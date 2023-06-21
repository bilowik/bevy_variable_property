use bevy_math::*;
use bevy_reflect::{Reflect, FromReflect};

use std::ops::{Range, RangeInclusive};

/// Wrapper around [std::ops::Range] to be utilized by PropRand.
#[derive(Clone, Default, Debug, Reflect, FromReflect)]
pub struct PropRange<T> {
    pub start: T,
    pub end: T,
    pub inclusive: bool,
}

impl<T> PropRange<T> {
    pub fn new(start: T, end: T, inclusive: bool) -> Self {
        Self {
            start,
            end,
            inclusive,
        }
    }
}

impl<T> From<Range<T>> for PropRange<T> {
    fn from(v: Range<T>) -> Self {
        Self {
            start: v.start,
            end: v.end,
            inclusive: false,
        }
    }
}

impl<T> From<RangeInclusive<T>> for PropRange<T> {
    fn from(v: RangeInclusive<T>) -> Self {
        let (start, end) = v.into_inner();
        Self {
            start,
            end,
            inclusive: true,
        }
    }
}
