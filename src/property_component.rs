use bevy::{
    prelude::*,
    utils::Duration,
    ecs::query::ReadOnlyWorldQuery,
};

use crate::variable_property::VariableProperty;

/// A field that generates a new value on an interval.
///
/// Typically, this will be used in conjunction with [
#[derive(Reflect, FromReflect)]
pub struct IntervalProperty<U, T: VariableProperty<U>> {
    property: T,
    timer: Timer,
    _phantom: std::marker::PhantomData<U>,
}

impl<U, T: VariableProperty<U>> IntervalProperty<U, T> {
    // Will return the new value when the timer has finished.
    pub(crate) fn tick_value(&mut self, delta: Duration) -> Option<U> {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            Some(self.property.get_value())
        }
        else {
            None
        }
    }
}

pub struct IntervalPropertyBuilder<U: Reflect + Default, T: VariableProperty<U> + Reflect + Default> {
    property: T,
    secs: f32,
    _phantom: std::marker::PhantomData<U>,
}

impl<U: Reflect + Default, T: VariableProperty<U> + Reflect + Default> IntervalPropertyBuilder<U, T> {
    pub fn with_secs(mut self, secs: f32) -> Self {
        self.secs = secs;
        self
    }

    pub fn with_property(mut self, property: T) -> Self {
        self.property = property;
        self
    }

    pub fn build(self) -> IntervalProperty<U, T> {
        IntervalProperty {
            property: self.property,
            timer: Timer::from_seconds(self.secs, TimerMode::Repeating),
            _phantom: Default::default(),
        }
    }
}

impl<U: Default + Reflect, T: VariableProperty<U> + Reflect + Default> Default for IntervalProperty<U, T> {
    fn default() -> Self {
        Self {
            property: Default::default(),
            timer: Timer::new(bevy::utils::Duration::from_secs_f32(1.0), TimerMode::Repeating),
            _phantom: Default::default(),
        }
    }
}

/// Creates a system that will tick the IntervalProperty field and update the component with the
/// given updater method. 
///
/// Typically, you will create a wrapper struct around [IntervalProperty] that implements
/// AsMut<IntervalProperty> then use this to create the system that will tick that
/// [InternvalProperty] and update the given component when the [IntervalProperty] generates a new
/// value.
pub fn update_interval_property_system_gen<U, T, W, V, F>(updater: impl Fn(U, &mut V)) -> impl Fn(Query<(&mut V, &mut W), F>, Res<Time>)
    where V: Component, U: Reflect + Default, T: VariableProperty<U> + Reflect + Default, F: ReadOnlyWorldQuery,
          W: Component + AsMut<IntervalProperty<U, T>> {
    move |mut query, time| {
        let delta = time.delta();
       for (mut target, mut source) in query.iter_mut() {
           if let Some(new_value) = AsMut::<IntervalProperty<U, T>>::as_mut(&mut *source).tick_value(delta) {
                updater(new_value, target.as_mut()); 
           }
       }
    }
}


