use bevy::prelude::*;
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
    commands.spawn(Camera2d::default());
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Rectangle::new(128.0, 128.0)))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        MyIntervalProperty(IntervalProperty::new(
            (Vec2::new(-250.0, -250.0)..=Vec2::new(250.0, 250.0)).into(),
            0.5,
        )),
    ));
}
