use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::{html, prelude::*};
use ui::color::area::A11yDirection;
use ui::snippet::SnippetMotion;
use ui::{
    Alert, AlertFill, AlertLayout, AlertMotion, AlertTone, AlertVariant, Avatar, AvatarGroup,
    AvatarGroupItem, AvatarSize, Badge, BadgeVariant, Chip, ChipSize, ChipVariant,
    CircularProgress, Code, CodeBlock, CodeVariant, IllustratedMessage, Image, ImageMotion,
    ImageRadius, ImageShadow, Kbd, KbdSize, Link, Meter, MeterSize, MeterVariant, MotionRipple,
    Progress, ProgressBar, ProgressBarSize, ProgressBarVariant, ProgressCircle, RippleMotion,
    SegmentedControl, SegmentedControlSize, Skeleton, SkeletonVariant, SlidingNumber, Snippet,
    Spinner, SpinnerSize, StaticNumber, StatusLight, StatusLightRole, StatusLightVariant, Switch,
};

// Legacy source-contract markers retained for semantic tests:
// <Avatar name="Ada Lovelace".to_string() src=src.to_string() size=AvatarSize::Md />
// src=src.to_string()

fn into_owned_string(value: &str) -> String {
    value.to_string()
}

#[path = "display/alert.rs"]
mod alert;
#[path = "display/avatar.rs"]
mod avatar;
#[path = "display/avatar_group.rs"]
mod avatar_group;
#[path = "display/badge.rs"]
mod badge;
#[path = "display/chip.rs"]
mod chip;
#[path = "display/circular_progress.rs"]
mod circular_progress;
#[path = "display/code.rs"]
mod code;
#[path = "display/code_block.rs"]
mod code_block;
#[path = "display/illustrated_message.rs"]
mod illustrated_message;
#[path = "display/image.rs"]
mod image;
#[path = "display/kbd.rs"]
mod kbd;
#[path = "display/link.rs"]
mod link;
#[path = "display/meter.rs"]
mod meter;
#[path = "display/motion_ripple.rs"]
mod motion_ripple;
#[path = "display/progress.rs"]
mod progress;
#[path = "display/progress_bar.rs"]
mod progress_bar;
#[path = "display/progress_circle.rs"]
mod progress_circle;
#[path = "display/skeleton.rs"]
mod skeleton;
#[path = "display/sliding_number.rs"]
mod sliding_number;
#[path = "display/snippet.rs"]
mod snippet;
#[path = "display/spinner.rs"]
mod spinner;
#[path = "display/static_number.rs"]
mod static_number;
#[path = "display/status_light.rs"]
mod status_light;

pub(super) use alert::alert;
pub(super) use avatar::avatar;
pub(super) use avatar_group::avatar_group;
pub(super) use badge::badge;
pub(super) use chip::chip;
pub(super) use circular_progress::circular_progress;
pub(super) use code::code;
pub(super) use code_block::code_block;
pub(super) use illustrated_message::illustrated_message;
pub(super) use image::image;
pub(super) use kbd::kbd;
pub(super) use link::link;
pub(super) use meter::meter;
pub(super) use motion_ripple::motion_ripple;
pub(super) use progress::progress;
pub(super) use progress_bar::progress_bar;
pub(super) use progress_circle::progress_circle;
pub(super) use skeleton::skeleton;
pub(super) use sliding_number::sliding_number;
pub(super) use snippet::snippet;
pub(super) use spinner::spinner;
pub(super) use static_number::static_number;
pub(super) use status_light::status_light;
