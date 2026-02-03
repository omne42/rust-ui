#[derive(Clone, Copy, Debug)]
pub struct MotionOptions {
    pub duration_ms: u32,
    pub easing: &'static str,
    pub fill: FillMode,
}

impl Default for MotionOptions {
    fn default() -> Self {
        Self {
            duration_ms: 120,
            easing: "cubic-bezier(0.2, 0, 0, 1)",
            fill: FillMode::None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl FillMode {
    pub fn as_str(self) -> &'static str {
        match self {
            FillMode::None => "none",
            FillMode::Forwards => "forwards",
            FillMode::Backwards => "backwards",
            FillMode::Both => "both",
        }
    }
}
