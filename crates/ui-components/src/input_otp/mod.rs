mod logic;
pub mod styles;
mod view;

pub use logic::{apply_otp_backspace, apply_otp_input, normalize_otp_value};
pub use view::InputOtp;
