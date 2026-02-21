//! `ui-headless` — interaction & accessibility primitives (A11y Baseline analogue).

#[cfg(all(feature = "web", feature = "ssr"))]
compile_error!("features `web` and `ssr` are mutually exclusive; enable exactly one");

pub mod a11y;
pub mod aspect_ratio;
pub mod button;
pub mod calendar;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod checkbox_group;
pub mod circular_progress;
pub mod clearable_text_field;
pub mod color_area;
pub mod color_swatch;
pub mod color_thumb;
pub mod color_wheel;
pub mod combo_box;
pub mod command;
pub mod controllable_state;
pub mod direction;
pub mod divider;
pub mod error_message;
pub mod field;
pub mod field_group;
pub mod field_label;
pub mod file_trigger;
pub mod flip_card;
pub mod focus_ring;
pub mod focus_trap;
pub mod focus_visible;
pub mod focus_within;
pub mod hover;
pub mod hover_card;
pub mod i18n;
pub mod id_provider;
pub mod input_otp;
pub mod keyboard;
pub mod labeled_value;
pub mod legend;
pub mod listbox;
pub mod menu;
pub mod menu_item;
pub mod modal;
pub mod modality;
pub mod native_select;
pub mod number_field;
pub mod overlay_stack;
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
#[cfg(feature = "logic-calendar")]
pub mod time_field;
pub mod tooltip;
pub mod tooltip_position;
pub mod trace;
pub mod tree;
pub mod underlay;

pub use a11y::{
    A11yDirection, A11yLocaleAttrs, DisclosureTriggerA11yAttrs, ErrorViewA11yAttrs,
    FieldsetA11yAttrs, ImageFallbackA11yAttrs, LabeledGroupA11yAttrs, LabeledToolbarA11yAttrs,
    LiveRegionA11yAttrs, LiveRegionPriority, NavigationA11yAttrs, OverlayDialogA11yAttrs,
    PopupTriggerA11yAttrs, RegionA11yAttrs, aria_controls_when_open, aria_expanded,
    disclosure_trigger_attrs, error_view_attrs, fieldset_attrs, image_fallback_attrs,
    labeled_group_attrs, labeled_toolbar_attrs, live_region_attrs, locale_attrs, navigation_attrs,
    overlay_dialog_attrs, popup_trigger_attrs, region_attrs,
};
pub use aspect_ratio::{
    AspectRatioAttrs, AspectRatioContract, AspectRatioHandlers, AspectRatioOptions,
    AspectRatioSemanticState, use_aspect_ratio,
};
pub use button::{
    ButtonAria, ButtonAttrs, ButtonElement, ButtonHandlers, ButtonOptions, use_button,
};
pub use calendar::{
    CalendarDayA11yInput, CalendarDayAttrs, CalendarDayContract, CalendarDayHandlers,
    CalendarDayOptions, CalendarDayState, CalendarRootAttrs, CalendarRootContract,
    CalendarRootHandlers, CalendarRootOptions, CalendarRootState, use_calendar_day,
    use_calendar_root,
};
pub use carousel::{
    CarouselA11yOrientation, CarouselKeyCommand, CarouselRootAttrs, CarouselRootContract,
    CarouselRootHandlers, CarouselRootOptions, CarouselRootState, CarouselSlideA11yAttrs,
    carousel_slide_a11y_attrs, resolve_carousel_key_command, use_carousel_root,
};
pub use chart::{
    ChartAttrs, ChartContract, ChartHandlers, ChartKeyAction, ChartOptions, ChartSemanticState,
    use_chart,
};
pub use checkbox::{CheckboxAria, CheckboxAttrs, CheckboxHandlers, CheckboxOptions, use_checkbox};
pub use checkbox_group::{
    CheckboxGroupA11y, CheckboxGroupAttrs, CheckboxGroupFieldsetAttrs, CheckboxGroupHandlers,
    CheckboxGroupMessageAttrs, CheckboxGroupOptions, CheckboxGroupSemanticState,
    use_checkbox_group,
};
pub use circular_progress::{
    CircularProgressAttrs, CircularProgressContract, CircularProgressHandlers,
    CircularProgressOptions, CircularProgressSemanticState, use_circular_progress,
};
pub use clearable_text_field::{
    ClearableTextField, ClearableTextFieldAttrs, ClearableTextFieldHandlers,
    ClearableTextFieldOptions, ClearableTextFieldState, use_clearable_text_field,
};
pub use color_area::{
    ColorAreaAxisAttrs, ColorAreaCellAttrs, ColorAreaCellContract, ColorAreaCellInput,
    ColorAreaContract, ColorAreaGridAttrs, ColorAreaHandlers, ColorAreaKeyboardInput,
    ColorAreaKeyboardResult, ColorAreaOptions, ColorAreaRootAttrs, ColorAreaSemanticState,
    use_color_area,
};
pub use color_swatch::{
    ColorSwatchA11yAttrs, ColorSwatchA11yContract, ColorSwatchA11yHandlers, ColorSwatchA11yOptions,
    ColorSwatchA11yState, use_color_swatch_a11y,
};
pub use color_thumb::{
    ColorThumbContract, ColorThumbHandlers, ColorThumbOptions, ColorThumbRootAttrs,
    ColorThumbSemanticState, use_color_thumb,
};
pub use color_wheel::{
    ColorWheelContract, ColorWheelHandlers, ColorWheelInputAttrs, ColorWheelKeyboardResult,
    ColorWheelOptions, ColorWheelRootAttrs, ColorWheelSemanticState, ColorWheelTrackAttrs,
    use_color_wheel,
};
pub use combo_box::{
    ComboBoxAria, ComboBoxHandlers, ComboBoxInputAttrs, ComboBoxKeyDownResult,
    ComboBoxListBoxAttrs, ComboBoxOptionAttrs, ComboBoxOptions, use_combo_box,
};
pub use command::{
    CommandInputAttrs, CommandInputKeyDownResult, CommandOptionA11yAttrs, CommandOptionA11yInput,
    command_input_attrs, command_option_a11y_attrs, resolve_command_input_key_down,
};
pub use controllable_state::{
    ControllableOpenState, ControllableState, use_controllable_open_state_traced,
    use_controllable_state,
};
pub use direction::{
    DirectionAttrs, DirectionContract, DirectionHandlers, DirectionOptions, DirectionSemanticState,
    use_direction,
};
pub use divider::{
    DividerAttrs, DividerContract, DividerHandlers, DividerOptions, DividerSemanticState,
    use_divider,
};
pub use error_message::{
    ErrorMessageAttrs, ErrorMessageContract, ErrorMessageHandlers, ErrorMessageOptions,
    ErrorMessageSemanticState, use_error_message,
};
pub use field::{
    FieldAttrs, FieldContract, FieldHandlers, FieldOptions, FieldSemanticState, use_field,
};
pub use field_group::{
    FieldGroupAttrs, FieldGroupContract, FieldGroupHandlers, FieldGroupOptions,
    FieldGroupSemanticState, use_field_group,
};
pub use field_label::{
    FieldLabelAttrs, FieldLabelContract, FieldLabelHandlers, FieldLabelOptions,
    FieldLabelSemanticState, use_field_label,
};
pub use file_trigger::{
    FileTriggerAttrs, FileTriggerContract, FileTriggerHandlers, FileTriggerOptions,
    FileTriggerSemanticState, use_file_trigger,
};
pub use flip_card::{
    FlipCardA11y, FlipCardAttrs, FlipCardHandlers, FlipCardKeyDownResult, FlipCardOptions,
    FlipCardState, resolve_flip_card_key_down, use_flip_card,
};
pub use focus_ring::{FocusRingHandlers, FocusRingOptions, FocusRingState, use_focus_ring};
pub use focus_trap::{
    FocusTrapFrame, FocusTrapHandlers, FocusTrapOptions, RestorePolicy, use_focus_trap,
};
pub use focus_visible::{FocusVisibleState, provide_focus_visible, use_focus_visible};
pub use focus_within::{
    FocusWithinHandlers, FocusWithinOptions, FocusWithinState, use_focus_within,
};
pub use hover::{HoverHandlers, HoverOptions, HoverState, use_hover};
pub use hover_card::{
    HoverCardDismissA11y, HoverCardDismissAttrs, HoverCardDismissHandlers, HoverCardDismissOptions,
    HoverCardDismissState, HoverCardFocusA11y, HoverCardFocusA11yAttrs, HoverCardFocusA11yHandlers,
    HoverCardFocusA11yOptions, HoverCardFocusA11yState, HoverCardTriggerAria,
    HoverCardTriggerHandlers, HoverCardTriggerOptions, HoverCardTriggerState,
    should_dismiss_on_escape, use_hover_card_dismiss, use_hover_card_focus_a11y,
    use_hover_card_trigger,
};
pub use i18n::{CommonStrings, UiI18n, provide_ui_i18n, use_ui_i18n};
pub use id_provider::{UiIdProvider, provide_ui_id_provider, use_ui_id_provider};
pub use input_otp::{InputOtpAria, InputOtpHandlers, InputOtpOptions, use_input_otp};
pub use keyboard::{
    KeyboardAttrs, KeyboardContract, KeyboardHandlers, KeyboardOptions, KeyboardSemanticState,
    use_keyboard,
};
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
    MenubarKeyCommand, NavigationMenuKeyCommand, context_menu_open_focus_strategy,
    context_menu_open_focus_strategy_for_key, menu_trigger_open_focus_strategy,
    menu_trigger_open_focus_strategy_for_key, menubar_key_command, navigation_menu_key_command,
    use_menu,
};
pub use menu_item::{
    MenuItemAria, MenuItemAttrs, MenuItemHandlers, MenuItemKind, MenuItemOptions, use_menu_item,
};
pub use modal::{ModalOptions, use_modal};
pub use modality::Modality;
pub use native_select::{
    NativeSelectAttrs, NativeSelectContract, NativeSelectHandlers, NativeSelectOptions,
    NativeSelectSemanticState, resolve_native_select_change_index, use_native_select,
};
pub use number_field::{
    NumberFieldAria, NumberFieldHandlers, NumberFieldInputAttrs, NumberFieldOptions,
    use_number_field,
};
pub use overlay_stack::{
    OverlayRegistration, provide_overlay_stack, use_overlay_stack, use_overlay_stack_registration,
};
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
#[cfg(feature = "logic-calendar")]
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
