mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use motion::IllustratedMessageMotion;
pub use view::IllustratedMessage;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IllustratedMessageOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl IllustratedMessageOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            IllustratedMessageOrientation::Vertical => {
                "ui-illustrated-message--orientation-vertical"
            }
            IllustratedMessageOrientation::Horizontal => {
                "ui-illustrated-message--orientation-horizontal"
            }
        }
    }
}
