mod physics;
mod vector;

use physics::*;
use std::thread::sleep_ms;
use vector::*;

// TODO: Check https://wiki.geogebra.org/en/Reference:File_Format for geogebra files, to directly save the results of the test into a geogebra-compatible xml file

const MS: u32 = 100;
const MASS: f32 = 1 as f32;
const CHARGE: f32 = 1 as f32;

fn main() {
    println!("Starting calculation...");

    //* Initial velocity
    let mut velocity = Vec3::new(0.0, 1.0, 0.0);
    println!("Created velocity: {:?}", velocity);
    //* Initial position
    let mut position = Vec3::new(0.0, 0.0, 1.0); // Not actually a vector, just using it as point
    println!("Created position: {:?}", position);

    let mut force = Vec3::ZERO;
    let mut acceleration = Vec3::ZERO;

    //* Calculation loop
    for i in 0..=150 {
        force = magnetic_force(
            CHARGE,
            &velocity,
            &north_pole_magnetic_field(position.x, position.y, position.z),
        );
        acceleration = magnetic_acceleration(&force, MASS);
        //velocity = physics::velocity(&velocity, &acceleration, 0.1); //velocity + acceleration;
        velocity = velocity + acceleration; //+* ((MS / 100) as f32);
        position = position + velocity; //+* ((MS / 100) as f32); // You should somehow get time here
                                        //? You probably shouldn't get time here right? You're like doing it in a instant of time, doesn't make much sense to get time in there

        println!(
            "\n----- {}m -----\nForce: {:?}\nAcceleration: {:?}\nNew Velocity: {:?}\nPosition: {:?}",
            i, force, acceleration, velocity, position
        );
        sleep_ms(MS);
    }
}
