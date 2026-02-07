mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, TreeTone};
pub use view::Tree;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TreeDensity {
    #[default]
    Comfortable,
    Compact,
}

impl TreeDensity {
    pub fn class_name(self) -> &'static str {
        match self {
            TreeDensity::Comfortable => "ui-tree--density-comfortable",
            TreeDensity::Compact => "ui-tree--density-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            TreeDensity::Comfortable => "comfortable",
            TreeDensity::Compact => "compact",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub disabled: bool,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
            disabled: false,
        }
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeVisibleNode {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub has_children: bool,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub is_disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeStateInput {
    pub tone: TreeTone,
    pub density: TreeDensity,
    pub disabled: bool,
    pub node_count: usize,
    pub visible_count: usize,
    pub expanded_count: usize,
    pub has_selection: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeState {
    pub tone: TreeTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub density: TreeDensity,
    pub density_class: &'static str,
    pub density_attr: &'static str,
    pub is_disabled: bool,
    pub node_count: usize,
    pub visible_count: usize,
    pub expanded_count: usize,
    pub has_selection: bool,
    pub is_empty: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
