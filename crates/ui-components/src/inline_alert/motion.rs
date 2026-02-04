#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InlineAlertMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for InlineAlertMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            },
        }
    }
}
