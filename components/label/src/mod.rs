mod logic;
mod motion;
pub mod styles;
mod view;

pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, LABEL_AGENT_SCHEMA, LABEL_AGENT_SCHEMA_VERSION,
    LabelAgentAction, LabelAgentContractAttrs, LabelAgentIntent, LabelAgentOutputStatus,
    LabelAgentSource, LabelAgentState, LabelAgentStreamFallback, LabelAgentStreamSupport,
    LabelEmphasis, LabelState, LabelStateInput, resolve_agent_contract_attrs,
};
pub use motion::LabelMotion;
pub use view::Label;
