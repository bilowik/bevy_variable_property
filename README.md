## Bevy Variable Property
A convenience library to genericize generating values for a given field. Includes a batteries-included component trait that 
can be utilized to automatically update another component with new values. 

## Examples

### Property
```rust
use bevy::prelude::*;

use bevy_variable_property::prelude::*;

fn main() {
    let p1 = Property::Static(Vec2::new(0.0, 1.0));
    let p2 = Property::RandomRange((Vec2::new(0.0, -100.0)..=Vec2::new(100.0, 0.0)).into());
    let p3 = Property::Random;

    for _ in 0..10 {
        for p in [&p1, &p2, &p3] {
            println!("{:?}", p.get_value());
        }
    }
}

```


### IntervalProperty
```rust, no_run
use bevy::prelude::*;
use bevy_variable_property::prelude::*;

#[derive(Component)]
struct MyComponent(pub IntervalProperty<Property<f32>>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MyComponent(IntervalProperty::new(
        (0.0..=100.0).into(),
        0.5,
    )));
}

fn tick(mut query: Query<&mut MyComponent>, time: Res<Time>) {
    for mut component in query.iter_mut() {
        if let Some(v) = component.0.tick_value(time.delta()) {
            println!("{:?}", v);
        }
    }
}

```

### IntervalPropertyComponent
```rust, no_run
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_variable_property::{interval_property::*, prelude::*};

#[derive(Component)]
pub struct MyIntervalProperty(pub IntervalProperty<Property<Vec2>>);

impl AsMut<IntervalProperty<Property<Vec2>>> for MyIntervalProperty {
    fn as_mut(&mut self) -> &mut IntervalProperty<Property<Vec2>> {
        &mut self.0
    }
}

impl IntervalPropertyComponent for MyIntervalProperty {
    type Property = Property<Vec2>;
    type TargetComponent = Transform;

    fn update(new_value: &Vec2, target: &mut Transform) {
        target.translation = new_value.extend(target.translation.z);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, MyIntervalProperty::system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(128.0, 128.0))))
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        },
        MyIntervalProperty(IntervalProperty::new(
            (Vec2::new(-250.0, -250.0)..=Vec2::new(250.0, 250.0)).into(),
            0.5,
        )),
    ));
}
```
