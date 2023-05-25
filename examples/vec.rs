
// Uses a Property<[f32; 4]> as a color generator
use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    let p: Property<_> = ([0.0, 5.0]..=[100.0, 10.0]).into();
     
    for _ in 0..10 {
        let v: Vec2 = p.get_value().0.into();
        println!("{:?}", v);
    }

}
