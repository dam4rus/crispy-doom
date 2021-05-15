use euclid::{Box2D, Point2D, Rect, Size2D, UnknownUnit, Vector2D};
use std::{convert::TryFrom, marker::PhantomData};

/* Frame buffer coordinate unit */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FrameBufferUnit;

/* Map coordinate unit */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MapUnit;

type FrameBufferPoint = Point2D<i32, FrameBufferUnit>;
type FrameBufferSize = Size2D<i32, FrameBufferUnit>;

type MapPoint = Point2D<i64, MapUnit>;
type MapSize = Size2D<i64, MapUnit>;
type MapRect = Rect<i64, MapUnit>;
type MapVector = Vector2D<i64, MapUnit>;
type MapBox = Box2D<i64, MapUnit>;

/* FixedPoint arithmetic */
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct FixedPoint<T>(i32, PhantomData<T>);

type FrameBufferFixed = FixedPoint<FrameBufferUnit>;

impl<T> FixedPoint<T> {
    const BITS: i32 = 16;

    fn unit() -> Self {
        Self(1 << Self::BITS, PhantomData)
    }
}

impl<T> From<i32> for FixedPoint<T> {
    fn from(value: i32) -> Self {
        Self(value, PhantomData)
    }
}

impl FrameBufferFixed {
    fn transform_to_map(&self, value: i32) -> i32 {
        i32::try_from((((value as i64) << Self::BITS) * i64::from(self.0)) >> Self::BITS).expect(
            format!(
                "value wouldn't fit into i32 after converting to map unit: {}",
                value
            )
            .as_str(),
        )
    }

    fn transform_to_map_i64(&self, value: i64) -> i64 {
        ((value << Self::BITS) * i64::from(self.0)) >> Self::BITS
    }

    fn transform_size_to_map(self, frame_buffer_size: &FrameBufferSize) -> MapSize {
        MapSize::new(
            self.transform_to_map(frame_buffer_size.width) as i64,
            self.transform_to_map(frame_buffer_size.height) as i64,
        )
    }
}

/* Automap implementation */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Automap {
    active: bool,
    follow_player: bool,
    follower_old_location: Option<FrameBufferPoint>,
    clock: i32,
    light_level: i32,
    pan_increase_keyboard: Option<MapVector>,
    pan_increase_mouse: Option<MapVector>, // TODO wtf?
    frame_zoom_multiplier: FixedPoint<FrameBufferUnit>,
    map_zoom_multiplier: FixedPoint<MapUnit>,
    rect: MapRect,
    old_rect: MapRect,
}

impl Automap {
    fn new(
        player_position: &Point2D<i32, UnknownUnit>,
        window_size: &FrameBufferSize,
        scale_frame_buffer_to_map: FrameBufferFixed,
    ) -> Self {
        let width = scale_frame_buffer_to_map.transform_to_map(window_size.width);
        let height = scale_frame_buffer_to_map.transform_to_map(window_size.height);
        let x = player_position.x - width / 2;
        let y = player_position.y - height / 2;
        let position = MapPoint::new(x as i64, y as i64);
        let size = MapSize::new(width as i64, height as i64);
        let rect = MapRect::new(position, size);
        Self {
            active: true,
            follow_player: true,
            follower_old_location: None,
            clock: 0,
            light_level: 0,
            pan_increase_keyboard: None,
            pan_increase_mouse: None,
            frame_zoom_multiplier: FixedPoint::unit(),
            map_zoom_multiplier: FixedPoint::unit(),
            rect,
            old_rect: rect,
        }
    }

    fn change_window_location(&mut self, rotate: bool, boundaries: MapBox) {
        let pan_increase_keyboard = self
            .pan_increase_keyboard
            .filter(|vec| *vec != MapVector::zero());
        let pan_increase_mouse = self
            .pan_increase_mouse
            .filter(|vec| *vec != MapVector::zero());
        let pan = match (pan_increase_keyboard, pan_increase_mouse) {
            (None, None) => return,
            (Some(pan), None) | (None, Some(pan)) => pan,
            (Some(pan_keyboard), Some(pan_mouse)) => pan_keyboard + pan_mouse,
        };

        self.follow_player = false;
        self.follower_old_location = None;

        if rotate {
            // TODO: implement
        }

        self.pan_increase_mouse = None;
        self.rect.origin = {
            let mut new_position = self.rect.origin + pan;
            if new_position.x + self.rect.size.width / 2 > boundaries.max.x {
                new_position.x = boundaries.max.x - self.rect.size.width / 2;
            } else if new_position.x + self.rect.size.width / 2 < boundaries.min.x {
                new_position.x = boundaries.min.x - self.rect.size.width / 2;
            }

            if new_position.y + self.rect.size.height / 2 > boundaries.max.y {
                new_position.y = boundaries.max.y - self.rect.size.height / 2;
            } else if new_position.x + self.rect.size.height / 2 < boundaries.min.y {
                new_position.y = boundaries.min.y - self.rect.size.height / 2;
            }

            new_position
        };
    }

    fn activate_new_scale(
        &mut self,
        window_size: &FrameBufferSize,
        scale_frame_buffer_to_map: FixedPoint<FrameBufferUnit>,
    ) {
        let translate_vector = MapVector::new(self.rect.size.width / 2, self.rect.size.height / 2);
        self.rect.origin += translate_vector;
        self.rect.size = scale_frame_buffer_to_map.transform_size_to_map(window_size);
        self.rect.origin -= translate_vector;

        println!("rect after scale: {:#?}", self.rect)
    }
}

#[no_mangle]
pub extern "C" fn automap_new(
    player_position_x: i32,
    player_position_y: i32,
    window_width: i32,
    window_height: i32,
    scale_frame_buffer_to_map: i32,
) -> *mut Automap {
    let automap = Automap::new(
        &Point2D::new(player_position_x, player_position_y),
        &FrameBufferSize::new(window_width, window_height),
        FixedPoint::from(scale_frame_buffer_to_map),
    );
    println!("{:#?}", automap);
    Box::into_raw(Box::new(automap))
}

#[no_mangle]
pub unsafe extern "C" fn automap_free(automap: *mut Automap) {
    let _ = Box::from_raw(automap);
}

#[no_mangle]
pub unsafe extern "C" fn automap_change_window_location(
    automap: *mut Automap,
    rotate: bool,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
) {
    automap
        .as_mut()
        .expect("null passed as Automap")
        .change_window_location(
            rotate,
            Box2D::new(Point2D::new(min_x, min_y), Point2D::new(max_x, max_y)),
        );
}

#[no_mangle]
pub unsafe extern "C" fn automap_activate_new_scale(
    automap: *mut Automap,
    window_width: i32,
    window_height: i32,
    scale_frame_buffer_to_map: i32,
) {
    automap
        .as_mut()
        .expect("null passed as Automap")
        .activate_new_scale(
            &FrameBufferSize::new(window_width, window_height),
            FrameBufferFixed::from(scale_frame_buffer_to_map),
        )
}
