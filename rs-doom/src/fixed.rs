use std::{
    convert::TryFrom,
    marker::PhantomData,
    ops::{Div, Mul},
};

use crate::coords::{
    FrameBufferPoint, FrameBufferSize, FrameBufferUnit, MapPoint, MapSize, MapUnit,
};

// A strongly typed representation of a fixed-point. The generic parameter is the unit type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FixedPoint<U>(pub i32, PhantomData<U>);

impl<U> FixedPoint<U> {
    // The factor of fixed-points used in Doom
    const FRACTION_BITS: i32 = 16;

    // The value of 1.0
    pub fn unit() -> Self {
        Self(1 << Self::FRACTION_BITS, PhantomData)
    }
}

impl<U> Mul<FixedPoint<U>> for FixedPoint<U> {
    type Output = Self;

    fn mul(self, rhs: FixedPoint<U>) -> Self::Output {
        Self::from(
            i32::try_from((self.0 as i64 * rhs.0 as i64) >> Self::FRACTION_BITS)
                .expect("multiplication of fixed points wouldn't fit into i32"),
        )
    }
}

impl<U> Div<FixedPoint<U>> for FixedPoint<U> {
    type Output = Self;

    fn div(self, rhs: FixedPoint<U>) -> Self::Output {
        let result = if (self.0.abs() >> 14) >= rhs.0.abs() {
            if (self.0 ^ rhs.0) < 0 {
                i32::MIN
            } else {
                i32::MAX
            }
        } else {
            let value = ((self.0 as i64) << Self::FRACTION_BITS) / rhs.0 as i64;
            i32::try_from(value).expect("result of div of fixed points woudln't fit into i32")
        };
        Self::from(result)
    }
}

impl<U> From<i32> for FixedPoint<U> {
    fn from(value: i32) -> Self {
        Self(value, PhantomData)
    }
}

impl<U> Into<i32> for FixedPoint<U> {
    fn into(self) -> i32 {
        self.0
    }
}

pub type FrameBufferFixedPoint = FixedPoint<FrameBufferUnit>;
pub type MapFixedPoint = FixedPoint<MapUnit>;

impl FrameBufferFixedPoint {
    pub fn transform_to_map(&self, value: i32) -> i64 {
        (((value as i64) << Self::FRACTION_BITS) * i64::from(self.0)) >> Self::FRACTION_BITS
    }

    pub fn transform_point_to_map(&self, point: &FrameBufferPoint) -> MapPoint {
        MapPoint::new(
            self.transform_to_map(point.x),
            self.transform_to_map(point.y),
        )
    }

    pub fn transform_size_to_map(&self, size: &FrameBufferSize) -> MapSize {
        MapSize::new(
            self.transform_to_map(size.width),
            self.transform_to_map(size.height),
        )
    }
}

impl MapFixedPoint {
    pub fn transform_to_frame_buffer(&self, value: i64) -> i32 {
        let transformed_value =
            ((value as i64 * self.0 as i64) >> Self::FRACTION_BITS) >> Self::FRACTION_BITS;
        i32::try_from(transformed_value)
            .expect("value wouln't fit into i32 after converting to frame buffer unit")
    }

    pub fn transform_point_to_frame_buffer(&self, point: &MapPoint) -> FrameBufferPoint {
        FrameBufferPoint::new(
            self.transform_to_frame_buffer(point.x),
            self.transform_to_frame_buffer(point.y),
        )
    }
}
