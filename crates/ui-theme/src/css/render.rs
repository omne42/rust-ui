use std::fmt::Write;

use crate::theme::{
    Theme, checkbox_group_motion_tokens, drop_zone_layout_tokens, flip_card_layout_tokens,
    label_motion_tokens, text_field_motion_tokens,
};
use crate::tokens::ColorScaleTokens;

macro_rules! css_writeln {
    ($target:expr $(, $arg:expr)*) => {{
        match writeln!($target $(, $arg)*) {
            Ok(()) | Err(_) => {}
        }
    }};
}

include!("render/theme_to_css_variables.inc");
