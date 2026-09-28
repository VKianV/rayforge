use rayforge_core::{
    app_error::AppError,
    camera::CameraBuilder,
    material::{Dielectric, Lambertian, Metal, SharedMaterial},
    shapes::{hittable::Shapes, hittable_list::HittableList, sphere::Sphere},
    vec3::{Point3, RGB},
};
use std::sync::Arc;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let mut world = HittableList::new();

    let material_ground: SharedMaterial = Arc::new(Lambertian::new(RGB::new(0.8, 0.8, 0.0)));
    let material_center: SharedMaterial = Arc::new(Lambertian::new(RGB::new(0.1, 0.2, 0.5)));
    let material_left: SharedMaterial = Arc::new(Dielectric::new(1.50));
    let material_bubble: SharedMaterial = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right: SharedMaterial = Arc::new(Metal::new(RGB::new(0.8, 0.6, 0.2), 0.0));

    // Ground
    world.add(Shapes::Sphere(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    // Center
    world.add(Shapes::Sphere(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    // Left: glass
    world.add(Shapes::Sphere(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    // Hollow bubble inside the glass
    world.add(Shapes::Sphere(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    // Right: metal
    world.add(Shapes::Sphere(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    CameraBuilder::from_config("config.env")
        .build()?
        .render(&world)?;

    Ok(())
}
