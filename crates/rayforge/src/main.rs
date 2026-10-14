use rayforge_core::{
    app_error::AppError,
    camera::CameraBuilder,
    color::RGB,
    hittable::{hittable_list::HittableList, shapes::Shapes, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    vec3::Point3,
};
use std::{process, sync::Arc};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(RGB::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(RGB::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(RGB::new(0.8, 0.6, 0.2), 0.0));

    // Ground
    CameraBuilder::from_config("config.env").build()?.render(
        &world
            .add(Shapes::Sphere(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground,
            )))
            // Center
            .add(Shapes::Sphere(Sphere::new(
                Point3::new(0.0, 0.0, -1.2),
                0.5,
                material_center,
            )))
            // Left: glass
            .add(Shapes::Sphere(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.5,
                material_left,
            )))
            // Hollow bubble inside the glass
            .add(Shapes::Sphere(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.4,
                material_bubble,
            )))
            // Right: metal
            .add(Shapes::Sphere(Sphere::new(
                Point3::new(1.0, 0.0, -1.0),
                0.5,
                material_right,
            ))),
    )?;

    Ok(())
}
