use euclid::{Point2D, UnknownUnit};
use std::convert::TryFrom;

use crate::{
    coords::{FrameBufferSize, MapBox, MapPoint, MapRect, MapSize, MapVector},
    fixed::{FrameBufferFixedPoint, MapFixedPoint},
    tables::{fine_cosine, fine_sine, Angle},
};

// Automap implementation for Doom
// It can be toggled by "tab" and follows the player by default
// Pressing "f" will unfollow the player and it can be panned by the arrow buttons
// It can be zoomed in and out with the mouse wheel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Automap {
    // Toggles whether the automap follows the player. Can be toggled with "f" by default
    follows_player: bool,
    // When the map is following the player the last position of the player is cached to skip logic
    // if the player hasn't moved at all
    follow_old_position: Option<Point2D<i32, UnknownUnit>>,
    // TODO: Implement
    clock: i32,
    // TODO: Implement
    light_level: i32,
    // The value the map should be panned. Set by using the keyboard arrow keys
    // TODO: Maybe it can be merged to a single pan_increase?
    pan_increase_keyboard: Option<MapVector>,
    // The value the map should be panned. Set by moving the mouse
    pan_increase_mouse: Option<MapVector>,
    // TODO: Implement
    frame_zoom_multiplier: FrameBufferFixedPoint,
    // TODO: Implement
    map_zoom_multiplier: MapFixedPoint,
    // The rect of the automap
    rect: MapRect,
    // Cached position and size of the automap
    old_rect: MapRect,
}

impl Automap {
    pub fn new(
        player_position: &Point2D<i32, UnknownUnit>,
        window_size: &FrameBufferSize,
        frame_buffer_scale: FrameBufferFixedPoint,
    ) -> Self {
        let width = frame_buffer_scale.transform_to_map(window_size.width);
        let height = frame_buffer_scale.transform_to_map(window_size.height);
        let x = player_position.x as i64 - (width / 2);
        let y = player_position.y as i64 - (height / 2);
        let position = MapPoint::new(x, y);
        let size = MapSize::new(width, height);
        let rect = MapRect::new(position, size);
        Self {
            follows_player: true,
            follow_old_position: None,
            clock: 0,
            light_level: 0,
            pan_increase_keyboard: None,
            pan_increase_mouse: None,
            frame_zoom_multiplier: FrameBufferFixedPoint::unit(),
            map_zoom_multiplier: MapFixedPoint::unit(),
            rect,
            old_rect: rect,
        }
    }

    pub fn change_window_location(&mut self, rotate: bool, boundaries: MapBox, map_angle: Angle) {
        let mut pan = match (self.pan_increase_keyboard, self.pan_increase_mouse) {
            (None, None) => return,
            (Some(pan), None) | (None, Some(pan)) => pan,
            (Some(pan_keyboard), Some(pan_mouse)) => pan_keyboard + pan_mouse,
        };

        self.follows_player = false;
        self.follow_old_position = None;

        if rotate {
            pan = self.rotate(&pan, map_angle);
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

    pub fn rotate(&mut self, point: &MapVector, map_angle: Angle) -> MapVector {
        let fixed_x = MapFixedPoint::from(i32::try_from(point.x).unwrap());
        let fixed_y = MapFixedPoint::from(i32::try_from(point.y).unwrap());
        let fixed_sine = MapFixedPoint::from(fine_sine(map_angle));
        let fixed_cosine = MapFixedPoint::from(fine_cosine(map_angle));
        let new_x = (fixed_x * fixed_cosine).0 - (fixed_y * fixed_sine).0;
        let new_y = (fixed_x * fixed_sine).0 + (fixed_y * fixed_cosine).0;
        MapVector::new(new_x as i64, new_y as i64)
    }

    pub fn activate_new_scale(
        &mut self,
        window_size: &FrameBufferSize,
        frame_buffer_scale: FrameBufferFixedPoint,
    ) {
        self.rect.origin += self.rect.size.to_vector() / 2;
        self.rect.size = frame_buffer_scale.transform_size_to_map(window_size);
        self.rect.origin -= self.rect.size.to_vector() / 2;
    }

    pub fn update_panning(
        &mut self,
        pan_increase_keyboard: Option<MapVector>,
        pan_increase_mouse: Option<MapVector>,
    ) {
        self.pan_increase_keyboard = pan_increase_keyboard;
        self.pan_increase_mouse = pan_increase_mouse;
    }

    pub fn save_rect(&mut self) {
        self.old_rect = self.rect;
    }

    pub fn restore_rect(&mut self, player_position: &Point2D<i32, UnknownUnit>) {
        let position = if !self.follows_player {
            self.old_rect.origin
        } else {
            let player_position = player_position.cast().cast_unit();
            let translate_vector = self.old_rect.size.to_vector() / 2;
            player_position - translate_vector
        };

        self.rect = MapRect::new(position, self.old_rect.size);
    }

    pub fn follow_player(&mut self, player_position: &Point2D<i32, UnknownUnit>) {
        if let Some(old_position) = self.follow_old_position {
            if old_position == *player_position {
                return;
            }
        }

        // TODO: dafuck?

        // let position = scale_frame_buffer_to_map.transform_point_to_map(
        //     &scale_map_to_frame_buffer.transform_point_to_frame_buffer(
        //         &MapPoint::new(player_position.x as i64, player_position.y as i64)));

        let position = player_position.cast().cast_unit();
        let translate_vector = self.rect.size.to_vector() / 2;

        self.rect.origin = position - translate_vector;
        self.follow_old_position = Some(*player_position);
    }

    pub fn rect(&self) -> &MapRect {
        &self.rect
    }
}
