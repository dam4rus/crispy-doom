use euclid::{Box2D, Point2D, Rect, Size2D, Vector2D};

/* Frame buffer coordinate unit */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameBufferUnit;

/* Map coordinate unit */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapUnit;

pub type FrameBufferPoint = Point2D<i32, FrameBufferUnit>;
pub type FrameBufferSize = Size2D<i32, FrameBufferUnit>;

pub type MapPoint = Point2D<i64, MapUnit>;
pub type MapSize = Size2D<i64, MapUnit>;
pub type MapRect = Rect<i64, MapUnit>;
pub type MapVector = Vector2D<i64, MapUnit>;
pub type MapBox = Box2D<i64, MapUnit>;
