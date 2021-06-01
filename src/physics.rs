use crate::vector::Vec3;

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

pub fn north_pole_magnetic_field(x: f32, y: f32, z: f32) -> Vec3 {
  Vec3 { x: x, y: y, z: z }
}
