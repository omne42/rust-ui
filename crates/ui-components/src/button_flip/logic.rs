#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlipDirection {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl FlipDirection {
    pub fn as_attr(self) -> &'static str {
        match self {
            FlipDirection::Top => "top",
            FlipDirection::Bottom => "bottom",
            FlipDirection::Left => "left",
            FlipDirection::Right => "right",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr_mapping_is_stable() {
        assert_eq!(FlipDirection::Top.as_attr(), "top");
        assert_eq!(FlipDirection::Bottom.as_attr(), "bottom");
        assert_eq!(FlipDirection::Left.as_attr(), "left");
        assert_eq!(FlipDirection::Right.as_attr(), "right");
    }
}
