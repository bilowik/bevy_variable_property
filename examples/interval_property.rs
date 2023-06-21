use bevy::prelude::*;
use bevy_variable_property::prelude::*;

#[derive(Component)]
struct MyComponent(pub IntervalProperty<Property<f32>>);



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}


fn setup(
    mut commands: Commands, 

) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
            MyComponent(IntervalProperty::new((0.0..=100.0).into(), 0.5)),
    );
}


fn tick(
    mut query: Query<&mut MyComponent>,
    time: Res<Time>,
) {
    for mut component in query.iter_mut() {
        if let Some(v) = component.0.tick_value(time.delta()) {
            println!("{:?}", v);
        }
    }

}
