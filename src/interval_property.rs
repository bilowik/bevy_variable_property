use bevy::{
    prelude::*,
    utils::Duration,
};

use crate::variable_property::VariableProperty;

/// A field that generates a new value on an interval.
///
/// Typically, this will be used in conjunction with [
#[derive(Reflect, FromReflect)]
pub struct IntervalProperty<T> {
    property: T,
    timer: Timer,
}

impl<T: VariableProperty> IntervalProperty<T> {
    // Will return the new value when the timer has finished.
    pub(crate) fn tick_value(&mut self, delta: Duration) -> Option<T::Output> {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            Some(self.property.get_value())
        }
        else {
            None
        }
    }
    
}

impl<T> IntervalProperty<T> {

    pub fn new(property: T, interval: f32) -> Self {
        Self {
            property,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
        }
    }
}


impl<T: Default> Default for IntervalProperty<T> {
    fn default() -> Self {
        Self {
            property: Default::default(),
            timer: Timer::new(bevy::utils::Duration::from_secs_f32(1.0), TimerMode::Repeating),
        }
    }
}

pub trait IntervalPropertyComponent: AsMut<IntervalProperty<Self::Property>> + Component {
    type Property: VariableProperty;
    type TargetComponent: Component;

    fn update(new_value: <Self::Property as VariableProperty>::Output, target: &mut Self::TargetComponent);
}

#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PauseIntervalProperty<T>(std::marker::PhantomData<T>);

impl<T> Default for PauseIntervalProperty<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

/// Creates a system that will tick the IntervalProperty field and update the component with the
/// given updater method. 
///
/// Typically, you will create a wrapper struct around [IntervalProperty] that implements
/// AsMut<IntervalProperty> then use this to create the system that will tick that
/// [InternvalProperty] and update the given component when the [IntervalProperty] generates a new
/// value.
pub fn interval_property_tick<T: IntervalPropertyComponent>(
    mut query: Query<(&mut T, &mut T::TargetComponent, Option<&PauseIntervalProperty<T>>)>, time: Res<Time>
) {
    let delta = time.delta();
    for (mut source, mut target, maybe_pause) in query.iter_mut() {
        if let Some(new_value) = AsMut::<IntervalProperty<T::Property>>::as_mut(&mut *source).tick_value(delta) {
            if maybe_pause.is_none() {
                T::update(new_value, target.as_mut()); 
            }
       }
    }
}


