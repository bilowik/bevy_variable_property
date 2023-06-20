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

pub trait IntervalPropertyComponent: AsMut<IntervalProperty<Self::Property>> + Component + Sized {
    type Property: VariableProperty;
    type TargetComponent: Component;

    fn update(new_value: <Self::Property as VariableProperty>::Output, target: &mut Self::TargetComponent);


    fn system(
        mut query: Query<(&mut Self, &mut Self::TargetComponent, Option<&PauseIntervalProperty<Self>>)>, 
        time: Res<Time>,
    ) {
        let delta = time.delta();
        for (mut source, mut target, maybe_pause) in query.iter_mut() {
            if let Some(new_value) = AsMut::<IntervalProperty<Self::Property>>::as_mut(&mut *source).tick_value(delta) {
                if maybe_pause.is_none() {
                    Self::update(new_value, target.as_mut()); 
                }
           }
        }
    }
}

#[derive(Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PauseIntervalProperty<T>(std::marker::PhantomData<T>);

impl<T> Default for PauseIntervalProperty<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
