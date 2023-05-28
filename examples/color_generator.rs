// Uses a Property<[f32; 4]> as a color generator
use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    // We want the range inclusive bc in this made up case we want alpha to always be 1.0.
    let properties = [
        (
            "Generating colors in the (inclusive) range: R: 0.0-0.5, G: 0.5-1.0, B: 0.0-0.2, A: 1.0-1.0",
            Property::from_array_range([0.0f32, 0.5, 0.0, 1.0], [0.5, 1.0, 0.2, 1.0], true),
        ),
        (
            "Generating entirely random colors",
            Property::Random,
        ),
        (
            "Generating colors based on a predefined list",
            Property::RandomChoice(
                vec![
                    [0.0, 0.5, 1.0, 1.0],
                    [0.3, 0.9, 0.5, 1.0],
                    [0.5, 0.3, 0.3, 1.0],
                    [0.8, 0.2, 0.9, 1.0],
                    [1.0, 1.0, 0.9, 0.8],
                ]
                .into_iter()
                .map(|e| e.into())
                .collect()
            ),
        )
    ];

    for (description, p) in properties.into_iter() { 
        println!("{}", description);
        println!("===============================");
        for _ in 0..5 {
            let color: Color = p.get_value().into();
            println!("{:?}", color);
        }
        println!("===============================\n");
    }

}
