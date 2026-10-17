use rayforge_core::{
    app_error::AppError,
    camera::CameraBuilder,
    color::RGB,
    geometries::{Geometries, hittable_list::HittableList, sphere::Sphere},
    material::{Material, dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    vec3::Point3,
};
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    CameraBuilder::from_config("config.env").build()?.render(
        &HittableList::default()
            .add(Geometries::Sphere(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                Material::Lambertian(Lambertian::new(RGB::new(0.8, 0.8, 0.0))),
            )))
            .add(Geometries::Sphere(Sphere::new(
                Point3::new(0.0, 0.0, -1.2),
                0.5,
                Material::Lambertian(Lambertian::new(RGB::new(0.1, 0.2, 0.5))),
            )))
            .add(Geometries::Sphere(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.5,
                Material::Dielectric(Dielectric::new(1.50)),
            )))
            .add(Geometries::Sphere(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.4,
                Material::Dielectric(Dielectric::new(1.00 / 1.50)),
            )))
            .add(Geometries::Sphere(Sphere::new(
                Point3::new(1.0, 0.0, -1.0),
                0.5,
                Material::Metal(Metal::new(RGB::new(0.8, 0.6, 0.2), 0.0)),
            ))),
    )?;
    Ok(())
}
