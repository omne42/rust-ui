#[derive(Clone, Debug, Default)]
pub struct MotionKeyframe {
    pub offset: Option<f64>,
    pub props: Vec<MotionProp>,
}

impl MotionKeyframe {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_offset(mut self, offset: f64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn prop(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.props.push(MotionProp {
            name: name.into(),
            value: value.into(),
        });
        self
    }
}

#[derive(Clone, Debug)]
pub struct MotionProp {
    pub name: String,
    pub value: String,
}
