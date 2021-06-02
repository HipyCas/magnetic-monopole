use crate::vector::Vec3;
use mathrs::trigonometry::{arctan, sin};

pub fn magnetic_force(q: f32, v: &Vec3, B: &Vec3) -> Vec3 {
  q * (v.vectorial_product(B))
}

// Not really used, we understand that we take a "unit" of time
#[doc(deprecated)]
pub fn velocity(v: &Vec3, a: &Vec3, t: f32) -> Vec3 {
  v + a * t
}

pub fn magnetic_acceleration(Fm: &Vec3, m: f32) -> Vec3 {
  Fm / m
}

pub fn north_pole_magnetic_field(intensity: f32, x: f32, y: f32, z: f32) -> Vec3 {
  let modulus = intensity / ((x * x + y * y + z * z) as f32).sqrt().powi(2); // |B| = i /
  Vec3 {
    x: {
      let yz: f32 = ((y * y + z * z) as f32).sqrt();
      let angle = arctan(x as f64 / yz as f64);
      modulus * sin(angle) as f32
    },
    y: {
      let xz: f32 = ((x * x + z * z) as f32).sqrt();
      let angle = arctan(z as f64 / xz as f64);
      modulus * sin(angle) as f32
    },
    z: {
      let xy: f32 = ((x * x + y * y) as f32).sqrt();
      let angle = arctan(z as f64 / xy as f64);
      modulus * sin(angle) as f32
    },
  }
}
