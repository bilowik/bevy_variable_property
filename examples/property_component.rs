use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_variable_property::{
    prelude::*,
    property_component::*,
};

#[derive(Component)]
pub struct MyIntervalProperty(pub IntervalProperty<Property<f32>>);

impl AsMut<IntervalProperty<Property<f32>>> for MyIntervalProperty {
    fn as_mut(&mut self) -> &mut IntervalProperty<Property<f32>> {
        &mut self.0 
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_interval_property_system_gen::<Property<f32>, MyIntervalProperty, Transform, ()>(|value, t| {
            t.translation.x = value;
        }))
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
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(128.0, 128.0)))).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                ..default()
            },
            MyIntervalProperty(IntervalProperty::new((-250.0..250.0).into(), 0.10)),
        ));
}
