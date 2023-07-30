use bevy_utils::Duration;
use bevy_ecs::{
    reflect::ReflectComponent,
    prelude::*,
};
use bevy_time::{Time, Timer, TimerMode};
use bevy_reflect::Reflect;

use crate::variable_property::VariableProperty;

/// A field that generates a new value on an interval.
#[derive(Reflect)]
pub struct IntervalProperty<T: VariableProperty> {
    property: T,
    timer: Timer,
    curr: Option<T::Output>,
}

impl<T: VariableProperty> IntervalProperty<T> {
    /// Ticks the internal timer by the given delta and will generate a new value if the timer
    /// has finished and return a reference to it.
    pub fn tick_value(&mut self, delta: Duration) -> Option<&T::Output> {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            self.curr = Some(self.property.get_value());
            self.get_curr_value()
        } else {
            None
        }
    }

    /// Returns a reference to the current value if one has been set yet.
    pub fn get_curr_value(&self) -> Option<&T::Output> {
        self.curr.as_ref()
    }
}

impl<T: VariableProperty> IntervalProperty<T> {
    pub fn new(property: T, interval: f32) -> Self {
        Self {
            property,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            curr: None,
        }
    }

    /// Explicitly set a starting value, which will be returned from [IntervalProperty::get_curr_value] until
    /// the internal timer finishes the first time.
    pub fn new_with_initial_value(property: T, interval: f32, init: T::Output) -> Self {
        Self {
            property,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            curr: Some(init),
        }
    }

    /// Explicitly set a starting value generated from the given Property, which will be returned from
    /// [IntervalProperty::get_curr_value] until the internal timer finishes the first time.
    pub fn new_with_generated_inital_value(property: T, interval: f32) -> Self {
        let curr = Some(property.get_value());
        Self {
            property,
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            curr,
        }
    }
}

impl<T: VariableProperty + Default> Default for IntervalProperty<T> {
    fn default() -> Self {
        Self {
            property: Default::default(),
            timer: Timer::new(
                Duration::from_secs_f32(1.0),
                TimerMode::Repeating,
            ),
            curr: None,
        }
    }
}

/// A convenience trait for automatically ticking the timer of an [IntervalProperty] and updating
/// some other Component when new values are generated. To be utilized directly as a bevy_ecs
/// System.
pub trait IntervalPropertyComponent:
    AsMut<IntervalProperty<Self::Property>> + Component + Sized
{
    type Property: VariableProperty;
    type TargetComponent: Component;

    fn update(
        new_value: &<Self::Property as VariableProperty>::Output,
        target: &mut Self::TargetComponent,
    );

    /// The system that will tick the given component's IntervalProperty and run the defined update
    /// function when a new value is generated.
    fn system(
        mut query: Query<(
            &mut Self,
            &mut Self::TargetComponent,
            Option<&PauseIntervalProperty<Self>>,
        )>,
        time: Res<Time>,
    ) {
        let delta = time.delta();
        for (mut source, mut target, maybe_pause) in query.iter_mut() {
            if let Some(new_value) =
                AsMut::<IntervalProperty<Self::Property>>::as_mut(&mut *source).tick_value(delta)
            {
                if maybe_pause.is_none() {
                    Self::update(new_value, target.as_mut());
                }
            }
        }
    }
}

/// Marker component to pause the generation of new values from an IntervalPropertyComponent
/// The timer will still tick and turn over, but [IntervalPropertyComponent::update] will not be
/// called.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PauseIntervalProperty<T: IntervalPropertyComponent>(std::marker::PhantomData<T>);

impl<T: IntervalPropertyComponent> Default for PauseIntervalProperty<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
