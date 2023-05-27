use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    let p = Property::from_array_range([0.0f32, -100.0], [100.0, 0.0], true);
    let p2 = Property::from_array_choices(vec![[0.5f32, 5.0], [2.0, 10.0]]);

    for _ in 0..10 {
        let v: Vec2 = p.get_value().into();
        println!("{:?}", v);
        let v2: Vec2 = p2.get_value().into();
        println!("{:?}", v2);
    }
}
