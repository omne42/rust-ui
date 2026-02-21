pub use crate::button::normalize_optional_text;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DirectionMode {
    #[default]
    Ltr,
    Rtl,
}

impl DirectionMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

#[cfg(test)]
#[path = "test/direction.rs"]
mod tests;
