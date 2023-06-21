use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    let p1: Property<_> = Vec2::new(0.0, 1.0).into();
    let p2: Property<_> = (Vec2::new(0.0, -100.0)..=Vec2::new(100.0, 0.0)).into();
    let p3: Property<Vec2> = Property::Random;

    for _ in 0..10 {
        for (desc, p) in [
            ("Static", &p1),
            ("Random range", &p2),
            ("Entirely random", &p3),
        ] {
            println!("{}: {:?}", desc, p.get_value());
        }
    }
}
