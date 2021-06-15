use magnetic_monopole::{exec, vector::Vec3};

// TODO: Check https://wiki.geogebra.org/en/Reference:File_Format for geogebra files, to directly save the results of the test into a geogebra-compatible xml file

/*static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];*/

const TIME_MS: f32 = 100 as f32;
const MASS: f32 = 1 as f32;
const CHARGE: f32 = 1 as f32;
const INTENSITY: f32 = 1 as f32; // The monopole magnetic "intensity" (B = intensity / R²)

fn main() {
    println!("Starting calculation...");

    //* Initial velocity
    let mut velocity = Vec3::new(0.0, 1.0, 0.0);
    println!("Created velocity: {:?}", velocity);
    //* Initial position
    let mut position = Vec3::new(0.0, 0.0, 1.0); // Not actually a vector, just using it as point
    println!("Created position: {:?}", position);

    exec(
        5,
        &mut velocity,
        &mut position,
        MASS,
        CHARGE,
        INTENSITY,
        TIME_MS,
        true,
    );
}
