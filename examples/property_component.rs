use bevy::prelude::*;
use bevy_variable_property::{
    prelude::*,
    property_component::*,
};

#[derive(Component)]
pub struct MyIntervalProperty(IntervalProperty<f32, Property<f32>>);

impl AsMut<IntervalProperty<f32, Property<f32>>> for MyIntervalProperty {
    fn as_mut(&mut self) -> &mut IntervalProperty<f32, Property<f32>> {
        &mut self.0 
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(update_interval_property_system_gen::<f32, Property<f32>, MyIntervalProperty, Transform, ()>(|value, t| {
            t.translation.x = value;
        }))
        .run();
}
