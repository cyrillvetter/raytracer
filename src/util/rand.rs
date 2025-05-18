use glam::Vec3A;
use fastrand::f32;

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
