// Uses a Property<[f32; 4]> as a color generator
use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    let properties = [
        (
            "Generating colors in the (inclusive) range: R: 0.0-0.5, G: 0.5-1.0, B: 0.0-0.2, A: 1.0-1.0",
            ([0.0f32, 0.5, 0.0, 1.0]..=[0.5, 1.0, 0.2, 1.0]).into()
        ),
        (
            "Generating entirely random colors",
            Property::Random,
        ),
        (
            "Generating colors based on a predefined list",
                vec![
                    [0.0, 0.5, 1.0, 1.0],
                    [0.3, 0.9, 0.5, 1.0],
                    [0.5, 0.3, 0.3, 1.0],
                    [0.8, 0.2, 0.9, 1.0],
                    [1.0, 1.0, 0.9, 0.8],
                ].into()
        )
    ];

    for (description, p) in properties.into_iter() {
        println!("{}", description);
        println!("===============================");
        for _ in 0..5 {
            let color = Color::rgba_from_array(p.get_value());
            println!("{:?}", color);
        }
        println!("===============================\n");
    }
}
