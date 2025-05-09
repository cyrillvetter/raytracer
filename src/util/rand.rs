use glam::Vec3A;
use fastrand::f32;

pub fn random_on_hemisphere(normal: Vec3A) -> Vec3A {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub fn random_unit_vector() -> Vec3A {
    loop {
        let p = Vec3A::new(signed_rand(), signed_rand(), signed_rand());
        let lensq = p.length_squared();
        if 1e-30 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

fn signed_rand() -> f32 {
    f32() * 2.0 - 1.0
}
