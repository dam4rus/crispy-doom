use crate::{
    automap::Automap,
    coords::{FrameBufferSize, MapVector},
    fixed::{FixedPoint, FrameBufferFixedPoint},
};
use euclid::{Box2D, Point2D};

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
    // println!("{:#?}", automap);
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
            0,
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
            FrameBufferFixedPoint::from(scale_frame_buffer_to_map),
        )
}

#[no_mangle]
pub unsafe extern "C" fn automap_update_panning(
    automap: *mut Automap,
    pan_increase_keyboard_x: i64,
    pan_increase_keyboard_y: i64,
    pan_increase_mouse_x: i64,
    pan_increase_mouse_y: i64,
) {
    let pan_increase_keyboard = match (pan_increase_keyboard_x, pan_increase_keyboard_y) {
        (0, 0) => None,
        (x, y) => Some(MapVector::new(x, y)),
    };

    let pan_increase_mouse = match (pan_increase_mouse_x, pan_increase_mouse_y) {
        (0, 0) => None,
        (x, y) => Some(MapVector::new(x, y)),
    };

    automap
        .as_mut()
        .expect("null passed as Automap")
        .update_panning(pan_increase_keyboard, pan_increase_mouse);
}

#[no_mangle]
pub unsafe extern "C" fn automap_save_rect(automap: *mut Automap) {
    automap
        .as_mut()
        .expect("null passed as Automap")
        .save_rect();
}

#[no_mangle]
pub unsafe extern "C" fn automap_restore_rect(
    automap: *mut Automap,
    player_position_x: i32,
    player_position_y: i32,
) {
    automap
        .as_mut()
        .expect("null passed as Automap")
        .restore_rect(&Point2D::new(player_position_x, player_position_y));
}

#[no_mangle]
pub unsafe extern "C" fn automap_follow_player(
    automap: *mut Automap,
    player_position_x: i32,
    player_position_y: i32,
) {
    automap
        .as_mut()
        .expect("null passed as Automap")
        .follow_player(&Point2D::new(player_position_x, player_position_y));
}

#[no_mangle]
pub unsafe extern "C" fn automap_print_rect(automap: *const Automap) {
    let automap = automap.as_ref().expect("null passed as Automap");

    println!("{:#?}", automap.rect());
}
