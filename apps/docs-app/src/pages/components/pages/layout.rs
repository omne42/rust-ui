use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::Snippet;
use ui_headless::A11yDirection;
use ui_layout::{
    AutoHeight, AutoHeightMotion, Card, CardVariant, Content, ContentTone, Divider, DividerMotion,
    DividerOrientation, Flex, FlexAlign, FlexDirection, FlexGap, FlexJustify, FlexMotion, FlexWrap,
    Footer, FooterTone, Header, HeaderTone, Heading, HeadingLevel, HeadingTone, ScrollShadow,
    Separator, SeparatorElementType, SeparatorMotion, SeparatorOrientation, Spacer, SpacerAxis,
    SpacerMotion, SpacerSize, View, ViewBackground, ViewBorder, ViewElement, ViewPadding,
    ViewRadius, ViewShadow, Well, WellDensity, WellTone,
};

#[path = "layout/auto_height.rs"]
mod auto_height;
#[path = "layout/card.rs"]
mod card;
#[path = "layout/content.rs"]
mod content;
#[path = "layout/divider.rs"]
mod divider;
#[path = "layout/flex.rs"]
mod flex;
#[path = "layout/footer.rs"]
mod footer;
#[path = "layout/header.rs"]
mod header;
#[path = "layout/heading.rs"]
mod heading;
#[path = "layout/scroll_shadow.rs"]
mod scroll_shadow;
#[path = "layout/separator.rs"]
mod separator;
#[path = "layout/spacer.rs"]
mod spacer;
#[path = "layout/view.rs"]
mod view;
#[path = "layout/well.rs"]
mod well;

pub(super) use auto_height::auto_height;
pub(super) use card::card;
pub(super) use content::content;
pub(super) use divider::divider;
pub(super) use flex::flex;
pub(super) use footer::footer;
pub(super) use header::header;
pub(super) use heading::heading;
pub(super) use scroll_shadow::scroll_shadow;
pub(super) use separator::separator;
pub(super) use spacer::spacer;
pub(super) use view::view;
pub(super) use well::well;
