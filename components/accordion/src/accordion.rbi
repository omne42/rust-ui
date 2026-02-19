pub type AccordionSelectionMode = ui_state_primitives::expansion::ExpansionMode;

pub enum AccordionVariant {
    Light,
    Shadow,
    Bordered,
    Splitted,
}

impl AccordionVariant {
    pub const fn as_str(self) -> &'static str;
}

pub fn open_set<const N: usize>(keys: [usize; N]) -> std::collections::BTreeSet<usize>;

pub struct AccordionMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub indicator_closed_rotation_deg: f64,
    pub indicator_open_rotation_deg: f64,
    pub panel_offset_y_px: f64,
}

pub const ACCORDION_COMPONENT_SCHEMA_NAME: &str;

pub enum AccordionComponentSchemaVersion {
    V1,
}

impl AccordionComponentSchemaVersion {
    pub const fn as_str(self) -> &'static str;
}

pub enum AccordionSelectionModeSpec {
    Single,
    Multiple,
}

pub enum AccordionVariantSpec {
    Light,
    Shadow,
    Bordered,
    Splitted,
}

pub struct AccordionComponentItemSpec {
    pub key: Option<usize>,
    pub label: String,
    pub body: String,
    pub is_disabled: bool,
    pub is_open: bool,
}

pub struct AccordionComponentSpec {
    pub schema_name: String,
    pub schema_version: AccordionComponentSchemaVersion,
    pub id_base: Option<String>,
    pub selection_mode: AccordionSelectionModeSpec,
    pub variant: AccordionVariantSpec,
    pub disallow_empty_selection: bool,
    pub is_disabled: bool,
    pub items: Vec<AccordionComponentItemSpec>,
}

pub struct ResolvedAccordionComponentItemSpec {
    pub key: usize,
    pub label: String,
    pub body: String,
    pub is_disabled: bool,
    pub is_open: bool,
}

pub struct ResolvedAccordionComponentSpec {
    pub schema_name: String,
    pub schema_version: AccordionComponentSchemaVersion,
    pub id_base: Option<String>,
    pub selection_mode: AccordionSelectionMode,
    pub variant: AccordionVariant,
    pub disallow_empty_selection: bool,
    pub is_disabled: bool,
    pub items: Vec<ResolvedAccordionComponentItemSpec>,
    pub open_keys: std::collections::BTreeSet<usize>,
}

pub enum AccordionComponentSpecError {
    UnsupportedSchemaName {
        expected: &'static str,
        actual: String,
    },
}

impl AccordionComponentSpec {
    pub fn resolve(self) -> Result<ResolvedAccordionComponentSpec, AccordionComponentSpecError>;
}

pub struct AccordionStreamingItem {
    pub label: String,
    pub text: String,
    pub is_complete: bool,
}

pub struct AccordionStreamingProjection {
    pub has_root_start: bool,
    pub has_root_open: bool,
    pub has_root_close: bool,
    pub items: Vec<AccordionStreamingItem>,
}

impl AccordionStreamingProjection {
    pub fn is_complete(&self) -> bool;
}

pub fn project_streaming_accordion_markup(input: &str) -> AccordionStreamingProjection;

pub fn AccordionItem(
    label: String,
    key: Option<usize>,
    is_disabled: bool,
    open: Option<leptos::prelude::Signal<bool>>,
    default_open: bool,
    on_open_change: Option<leptos::prelude::Callback<bool>>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;

pub fn Accordion(
    id_base: Option<String>,
    selection_mode: AccordionSelectionMode,
    variant: AccordionVariant,
    disallow_empty_selection: bool,
    is_disabled: bool,
    lang: Option<String>,
    dir: Option<ui_headless::a11y::A11yDirection>,
    motion: AccordionMotion,
    class_name: Option<String>,
    children: leptos::children::Children,
) -> impl leptos::prelude::IntoView;
