use crate::{
    app_error::AppError,
    color::{RGB, ray_color, write_color},
    config::Config,
    ray::Ray,
    shapes::hittable_list::HittableList,
    vec3::{Point3, Vec3},
};
use rander::Rng;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::Instant,
};

pub struct CameraBuilder {
    output_name: String,
    num_threads: Option<usize>,
    aspect_ratio: f64,
    image_width_pixels: usize,
    image_height_pixels: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    vfov: f64,

    // camera position & orientation
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,

    // defocus blur
    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            output_name: "render.ppm".to_string(),
            aspect_ratio: 1.777_777_777_8,
            num_threads: None,
            image_width_pixels: 1280,
            image_height_pixels: 720,
            samples_per_pixel: 100,
            max_depth: 10,
            vfov: 100.0,

            lookfrom: Point3::new(-2.0, 2.0, 1.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 3.92,
        }
    }
}

impl CameraBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from_config(path: &str) -> Self {
        let mut defaults = Self::default();

        let Ok(config) = Config::load_config(path) else {
            return defaults;
        };

        if let Ok(v) = config.get_usize("image_width_pixels") {
            defaults.image_width_pixels = v;
        }
        if let Ok(v) = config.get_usize("image_height_pixels") {
            defaults.image_height_pixels = v;
        }
        if let Ok(v) = config.get_f64("aspect_ratio") {
            defaults.aspect_ratio = v;
        }
        if let Ok(v) = config.get_string("output_name") {
            defaults.output_name = v.to_string();
        }
        if let Ok(v) = config.get_usize("samples_per_pixel") {
            defaults.samples_per_pixel = v;
        }
        if let Ok(v) = config.get_usize("max_depth") {
            defaults.max_depth = v;
        }
        if let Ok(v) = config.get_usize("num_threads") {
            defaults.num_threads = Some(v);
        }
        if let Ok(v) = config.get_f64("vfov") {
            defaults.vfov = v;
        }
        // --- position / orientation ---
        if let Ok(v) = config.get_f64("focus_dist") {
            defaults.focus_dist = v;
        }
        if let Ok(v) = config.get_f64("defocus_angle") {
            defaults.defocus_angle = v;
        }

        // Helper: parse a 3-vector from three scalar keys, leaving defaults untouched
        // if any component is missing.
        let parse_vec3 = |cfg: &Config, base: &str, fallback: Vec3| -> Vec3 {
            let x = cfg.get_f64(&format!("{base}_x"));
            let y = cfg.get_f64(&format!("{base}_y"));
            let z = cfg.get_f64(&format!("{base}_z"));
            match (x, y, z) {
                (Ok(x), Ok(y), Ok(z)) => Vec3::new(x, y, z),
                _ => fallback,
            }
        };

        defaults.lookfrom = parse_vec3(&config, "lookfrom", defaults.lookfrom);
        defaults.lookat = parse_vec3(&config, "lookat", defaults.lookat);
        defaults.vup = parse_vec3(&config, "vup", defaults.vup);

        defaults
    }

    pub fn output_name(mut self, name: impl Into<String>) -> Self {
        self.output_name = name.into();
        self
    }

    #[must_use]
    pub const fn image_width_pixels(mut self, n: usize) -> Self {
        self.image_width_pixels = n;
        self
    }

    #[must_use]
    pub const fn image_height_pixels(mut self, n: usize) -> Self {
        self.image_height_pixels = n;
        self
    }

    #[must_use]
    pub const fn vfov(mut self, deg: f64) -> Self {
        self.vfov = deg;
        self
    }

    #[must_use]
    pub const fn samples_per_pixel(mut self, n: usize) -> Self {
        self.samples_per_pixel = n;
        self
    }

    #[must_use]
    pub const fn max_depth(mut self, n: usize) -> Self {
        self.max_depth = n;
        self
    }

    #[must_use]
    pub const fn num_threads(mut self, n: usize) -> Self {
        self.num_threads = Some(n);
        self
    }

    pub fn aspect_ratio(mut self, n: f64) -> Self {
        self.aspect_ratio = n;
        self
    }

    #[must_use]
    pub const fn lookfrom(mut self, p: Point3) -> Self {
        self.lookfrom = p;
        self
    }

    #[must_use]
    pub const fn lookat(mut self, p: Point3) -> Self {
        self.lookat = p;
        self
    }

    #[must_use]
    pub const fn vup(mut self, v: Vec3) -> Self {
        self.vup = v;
        self
    }

    #[must_use]
    pub const fn focus_dist(mut self, d: f64) -> Self {
        self.focus_dist = d;
        self
    }

    #[must_use]
    pub const fn defocus_angle(mut self, deg: f64) -> Self {
        self.defocus_angle = deg;
        self
    }

    pub fn build(self) -> Result<Camera, AppError> {
        let camera_center = self.lookfrom;
        let focus_dist = self.focus_dist;
        let aspect_ratio = self.aspect_ratio;
        let image_height_pixels = (self.image_width_pixels as f64 / aspect_ratio) as usize;

        // --- camera frame basis (u, v, w), right-handed, w opposite view dir ---
        let w = (self.lookfrom - self.lookat).unit();
        let u = self.vup.cross(w).unit();
        let v = w.cross(u);

        // --- viewport geometry on the focus plane ---
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        // viewport_u points right (along +u), viewport_v points down (along -v).
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_width = viewport_u / self.image_width_pixels as f64;
        let pixel_delta_height = viewport_v / image_height_pixels as f64;

        let viewport_top_left =
            camera_center - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;

        let top_left_pixel_position =
            viewport_top_left + 0.5 * (pixel_delta_width + pixel_delta_height);

        // --- defocus disk basis (on the camera plane) ---
        let defocus_radius = focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let num_threads = self.num_threads.unwrap_or_else(|| {
            thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1)
                .min(image_height_pixels)
        });

        Ok(Camera {
            output_name: self.output_name,
            image_width_pixels: self.image_width_pixels,
            image_height_pixels,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            num_threads,
            defocus_angle: self.defocus_angle,

            pixel_delta_width,
            pixel_delta_height,
            camera_center,
            top_left_pixel_position,

            defocus_disk_u,
            defocus_disk_v,
        })
    }
}

pub struct Camera {
    output_name: String,
    num_threads: usize,
    image_width_pixels: usize,
    image_height_pixels: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    defocus_angle: f64,

    // derived
    pixel_delta_width: Vec3,
    pixel_delta_height: Vec3,
    camera_center: Point3,
    top_left_pixel_position: Point3,

    // NEW: defocus disk radii along u and v
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    #[inline]
    fn sample_square(rng: &mut Rng) -> (f64, f64) {
        (rng.next_f64() - 0.5, rng.next_f64() - 0.5)
    }

    /// Random point on the camera's defocus disk (in world space).
    #[inline]
    fn defocus_disk_sample(&self, rng: &mut Rng) -> Point3 {
        let p = Vec3::random_in_unit_disk(rng);
        self.camera_center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }

    /// Build a camera ray aimed at a randomly-jittered point inside `pixel_center`,
    /// originating from the defocus disk (or its center when `defocus_angle <= 0`).
    #[inline]
    fn get_ray(&self, pixel_center: &Point3, rng: &mut Rng) -> Ray {
        let (ox, oy) = Self::sample_square(rng);
        let pixel_sample =
            *pixel_center + ox * self.pixel_delta_width + oy * self.pixel_delta_height;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample(rng)
        };

        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    pub fn render(&self, world: &HittableList) -> Result<(), AppError> {
        let start = Instant::now();

        let file = File::create(&self.output_name)?;
        let mut out = BufWriter::new(file);

        println!(
            "Rendering with {} threads (dynamic scheduling)...",
            self.num_threads
        );

        eprint!("\x1b[?25l");
        eprint!("Scanlines remaining: {}", self.image_height_pixels);

        const CHUNK_ROWS: usize = 8;

        let next_row = AtomicUsize::new(0);
        let (tx, rx) = mpsc::channel::<(usize, usize, Vec<u8>)>();

        thread::scope(|s| -> Result<(), AppError> {
            let mut master_rng = Rng::default();
            for _ in 0..self.num_threads {
                let tx = tx.clone();
                let next_row = &next_row;

                let mut rng = master_rng.split();

                s.spawn(move || {
                    loop {
                        let start_row = next_row.fetch_add(CHUNK_ROWS, Ordering::Relaxed);

                        if start_row >= self.image_height_pixels {
                            break;
                        }

                        let end_row = (start_row + CHUNK_ROWS).min(self.image_height_pixels);
                        let rows = end_row - start_row;

                        let mut chunk = Vec::with_capacity(rows * self.image_width_pixels * 3);

                        let inv_samples = 1.0 / self.samples_per_pixel as f64;

                        for h in start_row..end_row {
                            let mut pixel_center =
                                self.top_left_pixel_position + h as f64 * self.pixel_delta_height;

                            for _ in 0..self.image_width_pixels {
                                let mut pixel_color = RGB::ZERO;

                                for _ in 0..self.samples_per_pixel {
                                    let ray = self.get_ray(&pixel_center, &mut rng);
                                    pixel_color += ray_color(&ray, world, self.max_depth, &mut rng);
                                }

                                write_color(&mut chunk, &(inv_samples * pixel_color))
                                    .expect("couldn't write color");

                                pixel_center += self.pixel_delta_width;
                            }
                        }

                        tx.send((start_row, end_row, chunk))
                            .expect("couldn't send data from workers");
                    }
                });
            }

            drop(tx);

            let num_chunks = self.image_height_pixels.div_ceil(CHUNK_ROWS);
            let mut chunks: Vec<Option<Vec<u8>>> = (0..num_chunks).map(|_| None).collect();

            let mut rows_received = 0;
            while rows_received < self.image_height_pixels {
                let (start_row, end_row, data) =
                    rx.recv().expect("couldn't recive data from workers");

                let chunk_idx = start_row / CHUNK_ROWS;
                chunks[chunk_idx] = Some(data);

                rows_received += end_row - start_row;

                let remaining = self.image_height_pixels - rows_received;
                eprint!("\x1b[21G\x1b[K {remaining}",);
            }

            writeln!(out, "P6")?;
            writeln!(
                out,
                "{} {}",
                self.image_width_pixels, self.image_height_pixels
            )?;
            writeln!(out, "255")?;

            for chunk in chunks.into_iter().flatten() {
                out.write_all(&chunk)?;
            }

            Ok(())
        })?;

        eprintln!("\x1b[?25h");
        eprintln!("Done in {:.3}s!", start.elapsed().as_secs_f64());

        Ok(())
    }
}
