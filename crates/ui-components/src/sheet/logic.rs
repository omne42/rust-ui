#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SheetPlacement {
    #[default]
    Bottom,
    Left,
    Right,
}

impl SheetPlacement {
    pub fn class_name(self) -> &'static str {
        match self {
            SheetPlacement::Bottom => "ui-sheet--placement-bottom",
            SheetPlacement::Left => "ui-sheet--placement-left",
            SheetPlacement::Right => "ui-sheet--placement-right",
        }
    }

    pub fn data_attr(self) -> &'static str {
        match self {
            SheetPlacement::Bottom => "bottom",
            SheetPlacement::Left => "left",
            SheetPlacement::Right => "right",
        }
    }
}
