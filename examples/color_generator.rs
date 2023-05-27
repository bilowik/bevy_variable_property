// Uses a Property<[f32; 4]> as a color generator
use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    // We want the range inclusive bc in this made up case we want alpha to always be 1.0.
    let mut p = Property::from_array_range([0.0f32, 0.5, 0.0, 1.0], [0.5, 1.0, 0.2, 1.0], true);
    let mut color: Color;
    println!("Generating colors in the (inclusive) range: R: 0.0-0.5, G: 0.5-1.0, B: 0.0-0.2, A: 1.0-1.0");
    println!("===============================");
    for _ in 0..5 {
        color = p.get_value().into();
        println!("{:?}", Color::from(p.get_value()));
    }

    println!("===============================\n");

    p = Property::Random;
    println!("Printing entirely random colors:");
    println!("===============================");
    for _ in 0..5 {
        println!("{:?}", Color::from(p.get_value()));
    }

    println!("===============================\n");

    p = Property::RandomChoice(
        vec![
            [0.0, 0.5, 1.0, 1.0],
            [0.3, 0.9, 0.5, 1.0],
            [0.5, 0.3, 0.3, 1.0],
            [0.8, 0.2, 0.9, 1.0],
            [1.0, 1.0, 0.9, 0.8],
        ]
        .into_iter()
        .map(|e| e.into())
        .collect(),
    );

    println!("Printing randomly from a preset list of colors:");
    println!("===============================");
    for _ in 0..5 {
        println!("{:?}", Color::from(p.get_value()));
    }
    println!("===============================");
}
