pub mod physics;
pub mod vector;

use physics::{magnetic_acceleration, magnetic_force, north_pole_magnetic_field};
use vector::Vec3;

pub fn calc(
  velocity: &mut Vec3,
  position: &mut Vec3,
  acceleration: &mut Vec3,
  force: &mut Vec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  time_ms: f32,
) {
  *force = magnetic_force(
    charge,
    velocity,
    &north_pole_magnetic_field(intensity, position.x, position.y, position.z),
  );
  *acceleration = magnetic_acceleration(force, mass);
  //velocity = physics::velocity(&velocity, &acceleration, 0.1); //velocity + acceleration;
  *velocity += *acceleration * ((time_ms / 1000.0) as f32);
  *position += *velocity * ((time_ms / 1000.0) as f32); // You should somehow get time here
                                                        //? You probably shouldn't get time here right? You're like doing it in a instant of time, doesn't make much sense to get time in there
}

pub fn iterate(
  count: u32,
  velocity: &mut Vec3,
  position: &mut Vec3,
  acceleration: &mut Vec3,
  force: &mut Vec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  time_ms: f32,
  _print: bool,
) {
  //use std::io::Write;
  //let mut file = std::fs::File::create("geogebra/geogebra.xml").unwrap();

  for i in 0..count {
    //? 0..=count
    calc(
      velocity,
      position,
      acceleration,
      force,
      mass,
      charge,
      intensity,
      time_ms,
    );
    if _print {
      println!(
        "\n----- {}ms -----\nForce: {:?}\nAcceleration: {:?}\nNew Velocity: {:?}\nPosition: {:?}\nGeoGebra point: ({}, {}, {})",
        i as f32 * time_ms, force, acceleration, velocity, position, position.x, position.y, position.z
      );

      //sleep_ms(MS);

      //println!("26 % {} = {}", i, 26 % i);
      /*
      let mut label = String::new();
      for _ in 0..=(i as usize / 26) {
          label.push(ASCII_UPPER[i.rem_euclid(26)]);
      }
      */

      //file.write_all(format!("<expression label=\"{l}\" exp=\"({x}, {y}, {z})\" type=\"point\"/>\n<element type=\"point3d\" label=\"{l}\">\n\t<show object=\"true\" label=\"true\" ev=\"4\"/>\n\t<objColor r=\"77\" g=\"77\" b=\"255\" alpha=\"0\"/>\n\t<layer val=\"0\"/>\n\t<labelMode val=\"0\"/>\n\t<animation step=\"0.1\" speed=\"1\" type=\"1\" playing=\"false\"/>\n\t<coords x=\"{x}\" y=\"{y}\" z=\"{z}\" w=\"1\"/>\n\t<pointSize val=\"5\"/>\n</element>", l = label, x = position.x, y = position.y, z = position.z).as_bytes()).unwrap();
    }
  }
}

type Vectors = (Vec3, Vec3, Vec3, Vec3);

pub fn iterate_return(
  count: u32,
  velocity: &mut Vec3,
  position: &mut Vec3,
  acceleration: &mut Vec3,
  force: &mut Vec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  time_ms: f32,
  _print: bool,
) -> Vec<Vectors> {
  let mut vec: Vec<Vectors> = Vec::new();

  for i in 0..count {
    calc(
      velocity,
      position,
      acceleration,
      force,
      mass,
      charge,
      intensity,
      time_ms,
    );
    if _print {
      println!(
          "\n----- {}ms -----\nForce: {:?}\nAcceleration: {:?}\nNew Velocity: {:?}\nPosition: {:?}\nGeoGebra point: ({}, {}, {})",
          i as f32 * time_ms, force, acceleration, velocity, position, position.x, position.y, position.z
        );

      //sleep_ms(MS);

      //println!("26 % {} = {}", i, 26 % i);
      /*
      let mut label = String::new();
      for _ in 0..=(i as usize / 26) {
          label.push(ASCII_UPPER[i.rem_euclid(26)]);
      }
      */

      //file.write_all(format!("<expression label=\"{l}\" exp=\"({x}, {y}, {z})\" type=\"point\"/>\n<element type=\"point3d\" label=\"{l}\">\n\t<show object=\"true\" label=\"true\" ev=\"4\"/>\n\t<objColor r=\"77\" g=\"77\" b=\"255\" alpha=\"0\"/>\n\t<layer val=\"0\"/>\n\t<labelMode val=\"0\"/>\n\t<animation step=\"0.1\" speed=\"1\" type=\"1\" playing=\"false\"/>\n\t<coords x=\"{x}\" y=\"{y}\" z=\"{z}\" w=\"1\"/>\n\t<pointSize val=\"5\"/>\n</element>", l = label, x = position.x, y = position.y, z = position.z).as_bytes()).unwrap();
    }
    vec.push((
      velocity.clone(),
      position.clone(),
      acceleration.clone(),
      force.clone(),
    ));
  }

  vec
}

pub fn exec(
  count: u32,
  velocity: &mut Vec3,
  position: &mut Vec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  time_ms: f32,
  _print: bool,
) -> (Vec3, Vec3) {
  let mut force: Vec3 = Vec3::ZERO;
  let mut acceleration: Vec3 = Vec3::ZERO;

  iterate(
    count,
    velocity,
    position,
    &mut acceleration,
    &mut force,
    mass,
    charge,
    intensity,
    time_ms,
    _print,
  );

  (acceleration, force)
}

pub fn exec_return(
  count: u32,
  velocity: &mut Vec3,
  position: &mut Vec3,
  mass: f32,
  charge: f32,
  intensity: f32,
  time_ms: f32,
  _print: bool,
) -> (Vec3, Vec3, Vec<Vectors>) {
  let mut force: Vec3 = Vec3::ZERO;
  let mut acceleration: Vec3 = Vec3::ZERO;

  (
    acceleration,
    force,
    iterate_return(
      count,
      velocity,
      position,
      &mut acceleration,
      &mut force,
      mass,
      charge,
      intensity,
      time_ms,
      _print,
    ),
  )
}
