//! `ui-headless` — interaction & accessibility primitives (A11y Baseline analogue).

#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one");

pub mod a11y;
pub mod aspect_ratio;
pub mod button;
pub mod chart;
pub mod checkbox;
pub mod clearable_text_field;
pub mod color_area;
pub mod combo_box;
pub mod controllable_state;
pub mod divider;
pub mod error_message;
pub mod field_label;
pub mod focus_ring;
pub mod focus_trap;
pub mod focus_visible;
pub mod focus_within;
pub mod hover;
pub mod hover_card;
pub mod i18n;
pub mod input_otp;
pub mod labeled_value;
pub mod legend;
pub mod listbox;
pub mod menu;
pub mod menu_item;
pub mod modal;
pub mod modality;
pub mod number_field;
pub mod overlay_stack;
pub mod perf;
pub mod popover_position;
pub mod presence;
pub mod press;
pub mod radio;
pub mod radio_group;
pub mod resizable;
pub mod roving_tabindex;
pub mod scroll_area;
pub mod search_field;
pub mod separator;
pub mod slider;
pub mod snippet;
pub mod spacer;
pub mod status_light;
pub mod step_list;
pub mod surface;
pub mod swatch;
pub mod switch;
pub mod tabs;
pub mod text_field;
pub mod time_field;
pub mod tooltip;
pub mod tooltip_position;
pub mod trace;
pub mod tree;
pub mod underlay;

pub use a11y::{
    A11yDirection, A11yLocaleAttrs, DisclosureTriggerA11yAttrs, ImageFallbackA11yAttrs,
    LabeledGroupA11yAttrs, LiveRegionA11yAttrs, LiveRegionPriority, OverlayDialogA11yAttrs,
    PopupTriggerA11yAttrs, RegionA11yAttrs, aria_controls_when_open, aria_expanded,
    disclosure_trigger_attrs, image_fallback_attrs, labeled_group_attrs, live_region_attrs,
    locale_attrs, overlay_dialog_attrs, popup_trigger_attrs, region_attrs,
};
pub use aspect_ratio::{
    AspectRatioAttrs, AspectRatioContract, AspectRatioHandlers, AspectRatioOptions,
    AspectRatioSemanticState, use_aspect_ratio,
};
pub use button::{
    ButtonAria, ButtonAttrs, ButtonElement, ButtonHandlers, ButtonOptions, use_button,
};
pub use chart::{
    ChartAttrs, ChartContract, ChartHandlers, ChartKeyAction, ChartOptions, ChartSemanticState,
    use_chart,
};
pub use checkbox::{CheckboxAria, CheckboxAttrs, CheckboxHandlers, CheckboxOptions, use_checkbox};
pub use clearable_text_field::{
    ClearableTextField, ClearableTextFieldAttrs, ClearableTextFieldHandlers,
    ClearableTextFieldOptions, ClearableTextFieldState, use_clearable_text_field,
};
pub use color_area::{
    ColorAreaAxisAttrs, ColorAreaContract, ColorAreaGridAttrs, ColorAreaHandlers,
    ColorAreaKeyboardInput, ColorAreaKeyboardResult, ColorAreaOptions, ColorAreaRootAttrs,
    ColorAreaSemanticState, use_color_area,
};
pub use combo_box::{
    ComboBoxAria, ComboBoxHandlers, ComboBoxInputAttrs, ComboBoxKeyDownResult,
    ComboBoxListBoxAttrs, ComboBoxOptions, use_combo_box,
};
pub use controllable_state::{
    ControllableOpenState, ControllableState, use_controllable_open_state_traced,
    use_controllable_state,
};
pub use divider::{
    DividerAttrs, DividerContract, DividerHandlers, DividerOptions, DividerSemanticState,
    use_divider,
};
pub use error_message::{
    ErrorMessageAttrs, ErrorMessageContract, ErrorMessageHandlers, ErrorMessageOptions,
    ErrorMessageSemanticState, use_error_message,
};
pub use field_label::{
    FieldLabelAttrs, FieldLabelContract, FieldLabelHandlers, FieldLabelOptions,
    FieldLabelSemanticState, use_field_label,
};
pub use focus_ring::{FocusRingHandlers, FocusRingOptions, FocusRingState, use_focus_ring};
pub use focus_trap::{FocusTrapHandlers, FocusTrapOptions, use_focus_trap};
pub use focus_visible::{FocusVisibleState, provide_focus_visible, use_focus_visible};
pub use focus_within::{
    FocusWithinHandlers, FocusWithinOptions, FocusWithinState, use_focus_within,
};
pub use hover::{HoverHandlers, HoverOptions, HoverState, use_hover};
pub use hover_card::{
    HoverCardTriggerAria, HoverCardTriggerHandlers, HoverCardTriggerOptions, HoverCardTriggerState,
    use_hover_card_trigger,
};
pub use i18n::{CommonStrings, UiI18n, provide_ui_i18n, use_ui_i18n};
pub use input_otp::{InputOtpAria, InputOtpHandlers, InputOtpOptions, use_input_otp};
pub use labeled_value::{
    LabeledValueAttrs, LabeledValueContract, LabeledValueHandlers, LabeledValueOptions,
    LabeledValueSemanticState, use_labeled_value,
};
pub use legend::{
    LegendAttrs, LegendContract, LegendHandlers, LegendOptions, LegendSemanticState, use_legend,
};
pub use listbox::{ListBoxAria, ListBoxAttrs, ListBoxHandlers, ListBoxOptions, use_listbox};
pub use menu::{
    MenuAria, MenuAttrs, MenuHandlers, MenuOnAction, MenuOpenFocusStrategy, MenuOptions,
    menu_trigger_open_focus_strategy, menu_trigger_open_focus_strategy_for_key, use_menu,
};
pub use menu_item::{
    MenuItemAria, MenuItemAttrs, MenuItemHandlers, MenuItemKind, MenuItemOptions, use_menu_item,
};
pub use modal::{ModalOptions, use_modal};
pub use modality::Modality;
pub use number_field::{
    NumberFieldAria, NumberFieldHandlers, NumberFieldInputAttrs, NumberFieldOptions,
    use_number_field,
};
pub use overlay_stack::{
    OverlayRegistration, provide_overlay_stack, use_overlay_stack, use_overlay_stack_registration,
};
pub use perf::{UiPerfBudget, UiPerfProbe};
pub use popover_position::{
    PopoverPlacement, PopoverPositionOptions, PopoverPositionState, use_popover_position,
};
pub use presence::{Presence, use_presence};
pub use press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, PressState, use_press};
pub use radio::{RadioAttrs, RadioContract, RadioOptions, RadioState, use_radio};
pub use radio_group::{RadioGroupAria, RadioGroupHandlers, RadioGroupOptions, use_radio_group};
pub use resizable::{
    ResizableAria, ResizableContractState, ResizableHandleAttrs, ResizableHandlers,
    ResizableOptions, ResizableRootAttrs, use_resizable,
};
pub use roving_tabindex::{
    RovingOrientation, RovingTabIndexHandlers, RovingTabIndexOptions, RovingTabIndexState,
    use_roving_tabindex,
};
pub use scroll_area::{
    ScrollAreaContract, ScrollAreaHandlers, ScrollAreaOptions, ScrollAreaRootAttrs,
    ScrollAreaSemanticState, ScrollAreaViewportAttrs, use_scroll_area,
};
pub use search_field::{
    SearchFieldAttrs, SearchFieldContract, SearchFieldHandlers, SearchFieldKeyDownResult,
    SearchFieldOptions, SearchFieldState, use_search_field,
};
pub use separator::{
    SeparatorAttrs, SeparatorContract, SeparatorHandlers, SeparatorOptions, SeparatorSemanticState,
    use_separator,
};
pub use slider::{
    SliderAria, SliderHandlers, SliderInputAttrs, SliderOptions, SliderState, use_slider,
};
pub use snippet::{
    SnippetCopyAttrs, SnippetCopyContract, SnippetCopyHandlers, SnippetCopyOptions,
    SnippetCopyState, use_snippet_copy,
};
pub use spacer::{
    SpacerAttrs, SpacerContract, SpacerHandlers, SpacerOptions, SpacerSemanticState, use_spacer,
};
pub use status_light::{
    StatusLightAttrs, StatusLightContract, StatusLightHandlers, StatusLightOptions,
    StatusLightSemanticState, use_status_light,
};
pub use step_list::{
    StepListItemA11yAttrs, StepListItemA11yInput, StepListItemContract, StepListItemSemanticState,
    StepListRootA11yAttrs, resolve_step_list_next_index, step_list_item_contract,
    step_list_root_a11y_attrs,
};
pub use surface::{
    SurfaceAttrs, SurfaceContract, SurfaceHandlers, SurfaceOptions, SurfaceSemanticState,
    use_surface,
};
pub use swatch::{SwatchAria, SwatchAttrs, SwatchHandlers, SwatchOptions, SwatchState, use_swatch};
pub use switch::{SwitchAria, SwitchAttrs, SwitchHandlers, SwitchOptions, use_switch};
pub use tabs::{
    TabsInteractionKind, TabsListA11yAttrs, TabsTabA11yAttrs, resolve_tabs_selection_intent,
    tabs_list_a11y_attrs, tabs_tab_a11y_attrs,
};
pub use text_field::{
    TextFieldAria, TextFieldInputAttrs, TextFieldLabelAttrs, TextFieldMessageAttrs,
    TextFieldOptions, use_text_field,
};
pub use time_field::{
    TimeFieldAria, TimeFieldAttrs, TimeFieldHandlers, TimeFieldOptions, TimeFieldState,
    use_time_field,
};
pub use tooltip::{
    TooltipFocusA11yOptions, TooltipFocusHandlers, TooltipTriggerAria, TooltipTriggerHandlers,
    TooltipTriggerMode, TooltipTriggerOptions, TooltipTriggerState, use_tooltip_focus_a11y,
    use_tooltip_trigger,
};
pub use tooltip_position::{
    TooltipPlacement, TooltipPositionOptions, TooltipPositionState, use_tooltip_position,
};
pub use trace::{UiTrace, UiTraceEvent, UiTraceEventKind, provide_ui_trace, use_ui_trace};
pub use tree::{
    TreeItemA11yInput, TreeItemAttrs, TreeItemContract, TreeItemHandlers, TreeItemOptions,
    TreeItemState, TreeRootAttrs, tree_root_attrs, use_tree_item,
};
pub use underlay::{
    UnderlayA11y, UnderlayAttrs, UnderlayHandlers, UnderlayOptions, UnderlayState, use_underlay,
};
