#[cfg(target_arch = "wasm32")]
pub const TRACK_WIDTH_PX: f64 = 32.0;

#[cfg(target_arch = "wasm32")]
pub const TRACK_PADDING_PX: f64 = 2.0;

#[cfg(target_arch = "wasm32")]
pub const THUMB_WIDTH_PX: f64 = 16.0;

#[cfg(target_arch = "wasm32")]
pub fn checked_thumb_x_px(thumb_width_px: f64) -> f64 {
    let inner_width = TRACK_WIDTH_PX - (TRACK_PADDING_PX * 2.0);
    (inner_width - thumb_width_px).max(0.0)
}
