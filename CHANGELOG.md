# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

### Added

- `scripts` + `ui-components`: adds executable check2 auditing/ticking helpers (`scripts/check2-audit.sh`, `scripts/check2-tick.sh`) and marks the verified workspace layering items as checked in every component `check2.md`.

- `ui-components` + `docs-app`: completes the check2 contract remediation pass across component modules, restores missing compatibility exports/pages (`List`, `OverlaysRoot`, `ContextMenuMotion`, `Gridlist`, `Group`, `Rac`, `Resizable`, `SidebarMenuAction`), and reconciles inline-style policy enforcement by keeping `style=` only in allowlisted view files while preserving motion/state data contracts.

- `docs`: adds repository-wide docs governance and markdown index (`docs/DOCS_GOVERNANCE.md`, `docs/DOCS_INDEX.md`) plus new docs entrypoints for `docs/`, `docs/plan/`, and `docs/spec/`; refreshes philosophy/planning/research guidance and migrates root `todo.md` to the dedicated playground planning flow.

- `docs-app` + `ui-components`: aligns interactive docs playground contracts on `code_signal`, updates SearchInputButton/PickerButton/ToggleButton/ToggleButtonGroup/LinkButton and Item module snippet matrices, refreshes semantics assertions for current docs-source markers (including code-block/image/illustrated-message/ripple/ui-root contracts), and adapts `ButtonSpec` to the current `Button` prop/state contract.

- `docs-app` + `ui-components`: upgrades `SearchInputButton` interactive playground to control-driven, copy-ready reactive code (preset/shortcut/disabled/aria), and hardens `search_input_button_semantics` + `button_search_input_semantics` docs-contract coverage for the new dynamic controls.

- `docs-app` + `ui-components`: upgrades `ThemeToggleButton` default-cycle playground to control-driven, copy-ready reactive code (start mode/disabled/mode-cycle/aria label), and hardens both `theme_toggle_button_semantics` and `button_theme_toggle_semantics` docs-contract coverage.

- `docs-app` + `ui-components`: upgrades `ToggleButtonGroup` docs playground to control-driven, copy-ready reactive code (orientation/attached/variant/size), and hardens `toggle_button_group_semantics` coverage for the new dynamic docs contract.

- `docs-app` + `ui-components`: upgrades `ToggleButton` to Spectrum-style `xs/s/m/l/xl` sizing (with compatibility aliases), refreshes toggle size marker contracts, and makes the primary ToggleButton playground control-driven with copy-ready reactive code snippets.

- `docs-app` + `ui-components`: upgrades `LinkButton` docs playground to control-driven, copy-ready dynamic code output (`variant/size/disabled/target/rel` now live-sync snippet content), and hardens `link_button_semantics` assertions to lock the new interactive docs contract.

- `docs-app` + `ui-components`: upgrades `IconButton` docs playgrounds with control-driven, copy-ready code output (`variant/size/disabled` selections now live-sync snippet content), aligns icon-size demos to Spectrum-style `xs/s/m/l/xl`, and hardens `icon_button_semantics` + playground coverage assertions for the new dynamic code contract.

- `docs-app` + `ui-components`: upgrades Playground code panels to copy-paste-ready snippets (auto-prepending imports when missing and supporting control-driven dynamic code output), aligns Actions `Button` playground code with live control selections, and standardizes `Button` sizing contracts to Spectrum-style `xs/s/m/l/xl` (with icon-size counterparts and compatibility aliases), including updated docs/state-matrix semantics coverage for `button`, `icon-button`, `link-button`, and related size mappings.

- `ui-components`: fixes `Overlay`/`Sheet` closed-state pointer-event contracts so hidden backdrops no longer block page interaction, and hardens `overlay_semantics` coverage for closed-state click-through behavior.

- `ui-components`: upgrades `Select` with class/data dual source markers for class/motion contracts, aligns select source selectors in styles, and hardens `select_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `FlipButton` with class/data dual source markers for class/motion contracts, aligns button-flip source selectors in styles, and hardens `button_flip_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `DatePicker` with class/data dual source markers for class/motion contracts, aligns date-picker source selectors in styles, and hardens `date_picker_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `FileTrigger` with logic-derived motion source markers, aligns file-trigger source selectors in styles, and hardens `file_trigger_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `ContextualHelp` with class/data dual source markers for class/motion contracts, aligns contextual-help source selectors in styles, and hardens `contextual_help_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Autocomplete` with class/data dual source markers for label/description/error/placeholder/id/class/motion contracts, aligns autocomplete source selectors in styles, and hardens `autocomplete_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `ComboBox` with class/data dual source markers for label/description/error/placeholder/id/class/motion contracts, aligns combo-box source selectors in styles, and hardens `combo_box_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Combobox` with class/data dual source markers for label/description/error/placeholder/class/motion contracts, aligns combobox source selectors in styles, and hardens `combobox_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Dropzone` with class/data dual source markers for label/aria/drop-handler/class/motion contracts, aligns dropzone source selectors in styles, and hardens `dropzone_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Search` with class/data dual source markers for label/description/error/placeholder/submit-handler/clear-handler/class/motion contracts, aligns search source selectors in styles, and hardens `search_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Sonner` with class/data dual source markers for position/portal/max-toasts/aria/class/motion contracts, aligns sonner source selectors in styles, and hardens `sonner_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Toaster` with class/data dual source markers for position/portal/max-toasts/aria/class/motion contracts, aligns toaster source selectors in styles, and hardens `toaster_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Command` with class/data dual source markers for id/placeholder/empty-label/aria-label/class/disabled/action/motion contracts, aligns command source selectors in styles, and hardens `command_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `ContextMenu` with class/data dual source markers for id/aria-label/class/disabled/disabled-indices/item-kinds/close-on-action/placement/open/default-open/open-change/motion contracts, aligns context-menu source selectors in styles, and hardens `context_menu_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Carousel` with class/data dual source markers for id/aria-label/class/orientation/loop-navigation/selected-index/default-selected-index/selected-index-change/motion contracts, aligns carousel source selectors in styles, and hardens `carousel_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `ActionMenu` with class/data dual source markers for id/aria-label/class/disabled/disabled-indices/item-kinds/close-on-action/placement/open/default-open/open-change/motion contracts, aligns action-menu source selectors in styles, and hardens `action_menu_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `CommandDialog` with class/data dual source markers for id/title/description/placeholder/empty-label/aria-label/action/open-change/default-open/close-on-action/disabled/command-motion/overlay-motion contracts, aligns command-dialog source selectors in styles, and hardens `command_dialog_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `AlertDialog` with class/data dual source markers for variant/id/title/description/cancel/secondary/confirm/auto-focus/motion/exit contracts, aligns alert-dialog source selectors in styles, and hardens `alert_dialog_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Dialog` with class/data dual source markers for id/title/description/close/size/motion/exit contracts, aligns dialog source selectors in styles, and hardens `dialog_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Modal` with class/data dual source markers for id/title/description contracts, aligns modal source selectors in styles, and hardens `modal_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Tray` with class/data dual source markers for id/title/description/footer/close/size/dismiss/keyboard-dismiss contracts, aligns tray source selectors in styles, and hardens `tray_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Drawer` with class/data dual source markers for id/title/description/footer/close contracts, aligns drawer source selectors in styles, and hardens `drawer_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Sheet` with class/data dual source markers for `aria-labelledby`/`aria-describedby` contracts, aligns sheet source selectors in styles, and hardens `sheet_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Popover` with class/data dual source markers for modal source contracts, aligns popover modal selectors in styles, and hardens `popover_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Overlay` with class/data dual source markers for role/aria-labelledby/aria-describedby contracts, aligns overlay source selectors in styles, and hardens `overlay_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `Menubar` with class/data dual source markers for id/close-on-action/placement/open-index contracts, aligns menubar source selectors in styles, and hardens `menubar_semantics` coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `NavigationMenu` with class/data dual source markers for id/aria/activation/selection contracts, aligning root class composition with styles and hardening `navigation_menu_semantics` selector coverage.

- `ui-components`: upgrades `Tooltip` to memoized root/panel state derivation, emits panel-level `data-custom-*` motion/delay/trigger/press/id markers, and aligns tooltip styles + semantics coverage for stable Spectrum-compatible state-source contracts.

- `ui-components`: upgrades `TopNav` with explicit optional-handler forwarding (no forced no-op callback), adds `data-custom-label` marker coverage, and aligns top-nav class/source selectors plus semantics tests for stable state-source contracts.

- `ui-components`: upgrades `Toggle` to memoized logic-derived state attrs, standardizes `data-custom-aria-label` alongside legacy `data-custom-aria`, and strengthens `toggle_semantics` coverage for state-source contract stability.

- `ui-components`: upgrades `ThemeToggleButton` with shell-level `data-slot/state/source/motion` markers, aligns wrapper-aware motion selectors in `button_theme_toggle/styles.rs`, and strengthens `theme_toggle_button_semantics` regression coverage for state-source contracts.

- `ui-components`: upgrades core `TextField` with Spectrum-style `data-state/value/requirement/source` markers, adds marker selectors in `text_field/styles.rs`, and expands `forms` docs with a dedicated `State + Source Markers` playground backed by stronger `text_field_semantics` regression coverage.

- `ui-components`: hardens `upstream_name_parity` coverage by locking shadcn/animate/react-aria/HeroUI/Spectrum package-name parity checks behind standard docs-state contract test entrypoints in `upstream_name_parity_semantics`.

- `ui-components`: hardens `ToggleGroup` docs state matrix contracts by locking multiple-attached and single-vertical-disabled playground anchors in `toggle_group_semantics` regression coverage.

- `ui-components`: hardens `TimeField` docs state matrix contracts by locking controlled-step and strong-placeholder playground anchors in `time_field_semantics` regression coverage.

- `ui-components`: hardens `ThemeToggleButton` docs state matrix contracts by locking default-cycle and custom-modes-disabled playground anchors in `theme_toggle_button_semantics` regression coverage.

- `ui-components`: hardens `Text` docs state matrix contracts by locking tone-weight and alignment-truncate playground anchors in `text_semantics` regression coverage.

- `ui-components`: hardens `text_field` docs state matrix contracts by locking label-placeholder playground anchors in `text_field_semantics` regression coverage.

- `ui-components`: hardens `Underlay` docs state matrix contracts by locking state-source playground anchors in `underlay_semantics` regression coverage.

- `ui-components`: hardens `Tray` docs state matrix contracts by locking state-source/custom-motion playground anchors in `tray_semantics` regression coverage.

- `ui-components`: hardens `TopNav` docs state matrix contracts by locking controlled/state-source playground anchors in `top_nav_semantics` regression coverage.

- `ui-components`: hardens `Tooltip` docs state matrix contracts by locking state-source/custom-motion playground anchors in `tooltip_semantics` regression coverage.

- `ui-components`: hardens `Toggle` docs state matrix contracts by locking outline-disabled/state-source playground anchors in `toggle_semantics` regression coverage.

- `ui-components`: hardens `Toaster` docs state matrix contracts by locking state-source/custom-motion playground anchors in `toaster_semantics` regression coverage.

- `ui-components`: hardens `Toast` docs state matrix contracts by locking state-source/custom-motion playground anchors in `toast_semantics` regression coverage.

- `ui-components`: hardens `Thumbnail` docs state matrix contracts by locking default/state/custom-motion playground anchors in `thumbnail_semantics` regression coverage.

- `ui-components`: hardens `visually_hidden` compatibility docs contracts by locking dedicated docs playground anchors in `visually_hidden_module_semantics` regression coverage.

- `ui-components`: hardens `virtualizer` compatibility docs contracts by locking scroll-area reuse anchors in `virtualizer_module_semantics` regression coverage.

- `ui-components`: hardens `utils` compatibility docs contracts by locking button reuse anchors in `utils_module_semantics` regression coverage.

- `ui-components`: hardens `toolbar` compatibility docs contracts by locking action-bar reuse anchors in `toolbar_module_semantics` regression coverage.

- `ui-components`: hardens `theme_light` compatibility docs contracts by locking UiRoot reuse anchors in `theme_light_module_semantics` regression coverage.

- `ui-components`: hardens `theme_express` compatibility docs contracts by locking UiRoot reuse anchors in `theme_express_module_semantics` regression coverage.

- `ui-components`: hardens `theme_default` compatibility docs contracts by locking UiRoot reuse anchors in `theme_default_module_semantics` regression coverage.

- `ui-components`: hardens `theme_dark` compatibility docs contracts by locking UiRoot reuse anchors in `theme_dark_module_semantics` regression coverage.

- `ui-components`: hardens `test_utils` compatibility docs contracts by locking UiRoot reuse anchors in `test_utils_module_semantics` regression coverage.

- `ui-components`: hardens `textfield` compatibility docs state matrix contracts by locking required-invalid/state-source playground anchors in `textfield_semantics` regression coverage.

- `ui-components`: hardens `textarea` compatibility docs state matrix contracts by locking basic/state-source playground anchors in `textarea_semantics` regression coverage.

- `ui-components`: hardens `TextArea` docs state matrix contracts by locking state/source marker playground anchors in `text_area_semantics` regression coverage.

- `ui-components`: hardens `Tags` docs state matrix contracts by locking removable/disabled/state-source playground anchors in `tags_semantics` regression coverage.

- `ui-components`: hardens `Switch` docs state matrix contracts by locking `Controlled + on_change` and `State matrix` playground anchors in `switch_semantics` regression coverage.

- `ui-components`: hardens `SwitchGroup` docs state matrix contracts by locking `Required + Description` and `Horizontal + Invalid + Disabled + Custom Class` playground anchors in `switch_group_semantics` regression coverage.

- `ui-components`: hardens `Swatch` docs state matrix contracts by locking primary playground anchors (`Size + Shape + Rounding` / `Mixed + Nothing + Disabled + Controlled` / `Custom Motion Contract`) in `swatch_semantics` regression coverage.

- `ui-components`: hardens `Surface` docs state matrix contracts by locking tone-elevation-frame and custom-aria-class playground anchors in `surface_semantics` regression coverage.

- `ui-components`: hardens `style_macro_s1` compatibility docs state matrix contracts by locking UiRoot usage/state-contract playground anchors reused by `style_macro_s1_module_semantics` regression coverage.

- `ui-components`: hardens `story_utils` compatibility docs state matrix contracts by locking UiRoot usage/state-contract playground anchors reused by `story_utils_module_semantics` regression coverage.

- `ui-components`: hardens `SplitView` docs state matrix contracts by locking horizontal-default, controlled-vertical-bounds, and state-source-markers playground anchors in `split_view_semantics` regression coverage.

- `ui-components`: hardens `spinbutton` compatibility docs state matrix contracts by locking NumberField stepper playground anchors reused by `spinbutton_module_semantics` regression coverage.

- `ui-components`: hardens `Sonner` docs state matrix contracts by locking portal-queue-variants, inline-top-center-max-queue, and state-source-markers playground anchors in `sonner_semantics` regression coverage.

- `ui-components`: hardens `Slider` docs state matrix contracts by locking controlled-on_change and disabled-fine-step playground anchors in `slider_semantics` regression coverage.

- `ui-components`: hardens `SkeletonGroup` docs state matrix contracts by locking shimmer-pulse-layout and loaded-skeleton-only playground anchors in `skeleton_group_semantics` regression coverage.

- `ui-components`: hardens `Sidenav` docs state matrix contracts by locking controlled-floating, icon-collapsible-no-shortcut, and state-source-markers playground anchors in `sidenav_semantics` regression coverage.

- `ui-components`: hardens `SidebarTrigger` docs state matrix contracts by locking default-trigger and controlled-custom-label playground anchors in `sidebar_trigger_semantics` regression coverage.

- `ui-components`: hardens `Sidebar` docs state matrix contracts by locking offcanvas-slot-markers and controlled-right-inset-icon playground anchors in `sidebar_semantics` regression coverage.

- `ui-components`: hardens `SidebarRail` docs state matrix contracts by locking default-rail and controlled-right-rail playground anchors in `sidebar_rail_semantics` regression coverage.

- `ui-components`: hardens `SidebarMenu` docs state matrix contracts by locking default-menu, controlled-active-id+click-callback, keyboard+disabled+expanded, and state+source marker playground anchors in `sidebar_menu_semantics` regression coverage.

- `ui-components`: hardens `SidebarMenuBadge` docs state matrix contracts by locking default-numeric and muted-disabled-custom playground anchors in `sidebar_menu_badge_semantics` regression coverage.

- `ui-components`: hardens `SidebarMenuAction` docs state matrix contracts by locking default-hover-only and always-visible-disabled-custom playground anchors in `sidebar_menu_action_semantics` regression coverage.

- `ui-components`: hardens `SidebarInset` docs state matrix contracts by locking default-inset and compact-plain-disabled playground anchors in `sidebar_inset_semantics` regression coverage.

- `ui-components`: hardens `SidebarHeader` docs state matrix contracts by locking default-header and disabled-custom-class playground anchors in `sidebar_header_semantics` regression coverage.

- `ui-components`: hardens `SidebarGroup` docs state matrix contracts by locking label-action and controlled-collapsible group playground anchors in `sidebar_group_semantics` regression coverage.

- `ui-components`: hardens `SidebarFooter` docs state matrix contracts by locking default-footer and disabled-custom-class playground anchors in `sidebar_footer_semantics` regression coverage.

- `ui-components`: hardens `SidebarContent` docs state matrix contracts by locking default-scrollable and compact-static-custom playground anchors in `sidebar_content_semantics` regression coverage.

- `ui-components`: hardens `Sheet` docs state matrix contracts by locking bottom-sheet and state-source marker playground anchors in `sheet_semantics` regression coverage.

- `ui-components`: hardens `shared_element_transition` compatibility docs state matrix contracts by locking `View` surface-token and element-fluid-custom-class playground anchors plus `shared-element-transition` slug mapping in `shared_element_transition_module_semantics` regression coverage.

- `ui-components`: hardens `ShareButton` docs state matrix contracts by locking default-callback, icon-placement+custom-items, and custom-class+direction playground anchors plus `button-share` slug mapping in `share_button_semantics` regression coverage.

- `ui-components`: hardens `selection_indicator` compatibility docs state matrix contracts by locking listbox-item and menu-item playground anchors plus `selection-indicator` slug mapping in `selection_indicator_module_semantics` regression coverage.

- `ui-components`: hardens `Search` docs state matrix contracts by locking submit-clear, required-invalid, and state-source marker playground anchors in `search_semantics` regression coverage.

- `ui-components`: hardens `SearchInputButton` docs state matrix contracts by locking interactive-shortcut, placeholder-disabled matrix, and custom-class+aria-label playground anchors plus `button-search-input` slug mapping in `search_input_button_semantics` regression coverage.

- `ui-components`: hardens `SearchField` docs state matrix contracts by locking submit-clear, required-invalid, and state-source marker playground anchors in `search_field_semantics` regression coverage.

- `ui-components`: hardens `ScrollArea` docs state matrix contracts by locking vertical-max-height and horizontal/both-disabled playground anchors in `scroll_area_semantics` regression coverage.

- `ui-components`: hardens `s2` compatibility module docs state matrix contracts by locking UiRoot usage/state playground anchors and `s2`→`ui-root` mapping in `s2_module_semantics` regression coverage.

- `ui-components`: hardens `Resizable` docs state matrix contracts by locking horizontal-handle and controlled-vertical-bounds playground anchors in `resizable_semantics` regression coverage.

- `ui-components`: hardens `Radio` docs state matrix contracts by locking RadioGroup and standalone Radio playground anchors in `radio_semantics` regression coverage.

- `ui-components`: hardens `radio_group` compatibility module docs state matrix contracts by locking RadioGroup selection/horizontal-disabled-empty playground anchors in `radio_group_module_semantics` regression coverage.

- `ui-components`: hardens `rac` compatibility module docs state matrix contracts by locking UiRoot usage/state playground anchors and `rac`→`ui-root` mapping in `rac_module_semantics` regression coverage.

- `ui-components`: hardens `provider` compatibility module docs state matrix contracts by locking UiRoot usage/state playground anchors and `provider`→`ui-root` mapping in `provider_module_semantics` regression coverage.

- `ui-components`: hardens `PreviewLinkCard` docs state matrix contracts by locking snapshot, state-source marker, and default-fallback playground anchors in `preview_link_card_semantics` regression coverage.

- `ui-components`: hardens `PreviewCard` docs state matrix contracts by locking basic, state-source marker, and default-fallback playground anchors in `preview_card_semantics` regression coverage.

- `ui-components`: hardens `PressableFeedback` docs state matrix contracts by locking scale+highlight and custom-motion+ripple playground anchors in `pressable_feedback_semantics` regression coverage.

- `ui-components`: hardens `Popover` docs state matrix contracts by locking base and state-source marker playground anchors in `popover_semantics` regression coverage.

- `ui-components`: hardens `Picker` docs state matrix contracts by locking basic-selection, controlled-open, and state-source marker playground anchors in `picker_semantics` regression coverage.

- `ui-components`: hardens `PickerButton` docs state matrix contracts by locking interactive, state-matrix, and state-source marker playground anchors in `picker_button_semantics` regression coverage.

- `ui-components`: hardens `overlays` compatibility module docs state matrix contracts by locking overlay/popover/modal/tray primary playground anchors and `overlays` docs slug mapping in `overlays_module_semantics` regression coverage.

- `ui-components`: hardens `Overlay` docs state matrix contracts by locking presence and state-source marker playground anchors (role/dismiss/motion) in `overlay_semantics` regression coverage.

- `ui-components`: hardens `overlay_arrow` compatibility docs state matrix contracts by locking icon/popover playground anchors and `overlay-arrow` slug mapping in `overlay_arrow_module_semantics` regression coverage.

- `ui-components`: hardens `number` docs state matrix contracts by locking static/sliding number playground anchors, separator contracts, and motion marker values in `number_semantics` regression coverage.

- `ui-components`: hardens `NumberField` docs state matrix contracts by locking stepper playground id/label/min-max/value anchors in `number_field_semantics` regression coverage.

- `ui-components`: hardens `NavigationMenu` docs state matrix contracts by locking roving-focus default, controlled-manual activation, and state-source marker playground anchors in `navigation_menu_semantics` regression coverage.

- `ui-components`: hardens `NativeSelect` docs state matrix contracts by locking controlled-placeholder and required-invalid-disabled playground anchors in `native_select_semantics` regression coverage.

- `ui-components`: hardens `Modal` docs state matrix contracts by locking label+description and state-source marker playground anchors in `modal_semantics` regression coverage.

- `ui-components`: hardens `Menubar` docs state matrix contracts by locking desktop-dispatch, controlled-persistent, and state-source marker playground anchors in `menubar_semantics` regression coverage.

- `ui-components`: hardens `LogicButton` docs state matrix contracts by locking AND/OR and custom-class+disabled playground anchors in `logic_button_semantics` regression coverage.

- `ui-components`: hardens `list` compatibility module docs state matrix contracts by locking listbox+item primary playground anchors and `list` docs slug mapping in `list_module_semantics` regression coverage.

- `ui-components`: hardens `list_box` compatibility module docs state matrix contracts by locking listbox/listbox-item/listbox-section primary playground anchors and `list-box` docs slug mapping in `list_box_module_semantics` regression coverage.

- `ui-components`: hardens `Legend` docs state matrix contracts by locking `Fieldset` legend+description and horizontal+invalid+actions playground anchors in `legend_semantics` regression coverage.

- `ui-components`: hardens `layout` compatibility module docs state matrix contracts by locking Flex+Grid primary playground anchors and `layout`→`["flex", "grid"]` docs mapping in `layout_module_semantics` regression coverage.

- `ui-components`: hardens `LabeledValue` docs state matrix contracts by locking orientation+tone and description+custom-aria/class playground anchors in `labeled_value_semantics` regression coverage.

- `ui-components`: hardens `Label` docs state matrix contracts by locking emphasis+required and custom-indicator+class playground anchors in `label_semantics` regression coverage.

- `ui-components`: hardens `Keyboard` docs state matrix contracts by locking tone and compact+custom-aria/class playground anchors in `keyboard_semantics` regression coverage.

- `ui-components`: hardens `Input` docs state matrix contracts by locking clearable+validation playground anchors in `input_semantics` regression coverage.

- `ui-components`: hardens `InputOtp` docs state matrix contracts by locking otp playground id/label/length/value anchors in `input_otp_semantics` regression coverage.

- `ui-components`: hardens `InputGroup` docs state matrix contracts by locking attached-addons and detached+disabled playground anchors in `input_group_semantics` regression coverage.

- `ui-components`: hardens `inline_style_contract` docs state matrix contracts by locking styling-spec and RULES_ZH inline-style/CSS-variable contract anchors in `inline_style_contract_semantics` regression coverage.

- `ui-components`: hardens `InlineAlert` docs state matrix contracts by locking inline alerts playground tone/fill/content anchors in `inline_alert_semantics` regression coverage.

- `ui-components`: hardens `InfieldButton` docs state matrix contracts by locking default+quiet and invalid+active+disabled playground anchors in `infield_button_semantics` regression coverage.

- `ui-components`: hardens `IllustratedMessage` docs state matrix contracts by locking empty-state playground anchors and illustration/actions contract values in `illustrated_message_semantics` regression coverage.

- `ui-components`: hardens `Iconset` docs state matrix contracts by locking registry namespace resolution, fallback+source-state, and state+source marker playground anchors in `iconset_semantics` regression coverage.

- `ui-components`: hardens `IconsWorkflow` docs state matrix contracts by locking built-in workflow glyphs, custom workflow extension, and state+source marker playground anchors in `icons_workflow_semantics` regression coverage.

- `ui-components`: hardens `IconsUi` docs state matrix contracts by locking built-in ui glyphs, custom registry extension, and state+source marker playground anchors in `icons_ui_semantics` regression coverage.

- `ui-components`: hardens `Icons` docs state matrix contracts by locking set/scale/default+custom glyph and state+source marker playground anchors in `icons_semantics` regression coverage.

- `ui-components`: hardens `Icon` docs state matrix contracts by locking size+tone matrix and accessible+disabled+custom-class playground anchors in `icon_semantics` regression coverage.

- `ui-components`: hardens `IconButton` docs state matrix contracts by locking actions-page press/size matrix and state+source marker playground anchors in `icon_button_semantics` regression coverage.

- `ui-components`: hardens `HoverCard` docs state matrix contracts by locking hover-card baseline, state+source markers, and custom-motion playground anchors in `hover_card_semantics` regression coverage.

- `ui-components`: hardens `hidden_date_input` compatibility docs state matrix contracts by locking date-input-group playground anchors and docs slug mapping contract in `hidden_date_input_module_semantics` regression coverage.

- `ui-components`: hardens `HelpText` docs state matrix contracts by locking description/neutral and invalid+error-icon playground anchors in `help_text_semantics` regression coverage.

- `ui-components`: hardens `group` compatibility docs state matrix contracts by locking field-group playground anchors and docs slug mapping contract in `group_module_semantics` regression coverage.

- `ui-components`: hardens `gridlist` compatibility docs state matrix contracts by locking listbox/listbox-item/listbox-section playground anchors and docs slug mapping contracts in `gridlist_module_semantics` regression coverage.

- `ui-components`: hardens `Grid` docs state matrix contracts by locking columns+gap and autofit+dense+equal-rows playground anchors in `grid_semantics` regression coverage.

- `ui-components`: hardens `grid_list` compatibility docs state matrix contracts by locking listbox/listbox-item/listbox-section playground anchors and docs slug mapping contracts in `grid_list_module_semantics` regression coverage.

- `ui-components`: hardens `Form` docs state matrix contracts by locking label-layout-context playground anchors in `form_semantics` regression coverage.

- `ui-components`: hardens `FormField` docs state matrix contracts by locking switch-indicator and checkbox+quiet+invalid/disabled playground anchors in `form_field_semantics` regression coverage.

- `ui-components`: hardens `FlipCard` docs state matrix contracts by locking click+keyboard, state+source-markers, and disabled playground anchors in `flip_card_semantics` regression coverage.

- `ui-components`: hardens `FlipButton` docs state matrix contracts by locking top-flip, direction-matrix, and custom-class playground anchors in `flip_button_semantics` regression coverage.

- `ui-components`: hardens `FileTrigger` docs state matrix contracts by locking pick-files and custom-motion pick-files playground anchors in `file_trigger_semantics` regression coverage.

- `ui-components`: hardens `Field` docs state matrix contracts by locking required+description and horizontal+invalid+custom-class playground anchors in `field_semantics` regression coverage.

- `ui-components`: hardens `FieldLabel` docs state matrix contracts by locking tone+required and custom-indicator+aria+class playground anchors in `field_label_semantics` regression coverage.

- `ui-components`: hardens `FieldGroup` docs state matrix contracts by locking vertical+label+description and horizontal+compact+invalid+disabled playground anchors in `field_group_semantics` regression coverage.

- `ui-components`: hardens `FieldError` docs state matrix contracts by locking visible+tone and hidden+disabled+custom-class playground anchors in `field_error_semantics` regression coverage.

- `ui-components`: hardens `FieldButton` docs state matrix contracts by locking default+quiet and invalid+active+disabled playground anchors in `field_button_semantics` regression coverage.

- `ui-components`: hardens `example_theme` compatibility docs state matrix contracts by locking `example-theme`→`ui-root` mapping and UiRoot usage/state-contract playground anchors in `example_theme_module_semantics` regression coverage.

- `ui-components`: hardens `drag_and_drop` compatibility docs state matrix contracts by locking file-trigger and drop-zone playground anchors reused by `drag_and_drop_module_semantics` regression coverage.

- `ui-components`: hardens `dnd` compatibility docs state matrix contracts by locking file-trigger and drop-zone playground anchors reused by `dnd_module_semantics` regression coverage.

- `ui-components`: hardens `ErrorView` docs state matrix contracts by locking invalid-visibility and custom-content+motion+actions playground anchors in `error_view_semantics` regression coverage.

- `ui-components`: hardens `ErrorMessage` docs state matrix contracts by locking tone-variants and truncate+disabled+element+custom-class playground anchors in `error_message_semantics` regression coverage.

- `ui-components`: hardens `EmptyState` docs state matrix contracts by locking tone+alignment+actions and compact+bordered+custom-class playground anchors in `empty_state_semantics` regression coverage.

- `ui-components`: hardens `Empty` docs state matrix contracts by locking header+icon, content-action, and state+source-markers playground anchors in `empty_semantics` regression coverage.

- `ui-components`: hardens `Dropzone` docs state matrix contracts by locking drop/paste, disabled, and state+source-markers playground anchors in `dropzone_semantics` regression coverage.

- `ui-components`: hardens `DropZone` docs state matrix contracts by locking drop/paste and custom-motion drop/paste playground anchors in `drop_zone_semantics` regression coverage.

- `ui-components`: hardens `Drawer` docs state matrix contracts by locking right-drawer+slots and state+source-markers playground anchors in `drawer_semantics` regression coverage.

- `ui-components`: hardens `Fieldset` docs state matrix contracts by locking legend+description and horizontal+invalid+actions playground anchors in `fieldset_semantics` regression coverage.

- `ui-components`: hardens `DirectionProvider` docs state matrix contracts by locking ltr-direction and rtl+class playground anchors in `direction_semantics` regression coverage.

- `ui-components`: hardens `Dialog` docs state matrix contracts by locking default dialog and state+source-marker playground anchors in `dialog_semantics` regression coverage.

- `ui-components`: hardens `Description` docs state matrix contracts by locking tone-variants and truncate+element+disabled playground anchors in `description_semantics` regression coverage.

- `ui-components`: hardens `DateRangePicker` docs state matrix contracts by locking controlled+shared-month and strong-tone+invalid-range-hint playground anchors in `date_range_picker_semantics` regression coverage.

- `ui-components`: hardens `DatePicker` docs state matrix contracts by locking default+outside-days and monday-first+strong-tone playground anchors in `date_picker_semantics` regression coverage.

- `ui-components`: hardens `DateInputGroup` docs state matrix contracts by locking datefield-prefix/suffix and secondary+full-width+invalid playground anchors in `date_input_group_semantics` regression coverage.

- `ui-components`: hardens `DateField` docs state matrix contracts by locking controlled-value and strong-tone+custom-placeholder playground anchors in `date_field_semantics` regression coverage.

- `ui-components`: hardens `ContextualHelp` docs state matrix contracts by locking help-variant+slots and info-variant+controlled playground anchors in `contextual_help_semantics` regression coverage.

- `ui-components`: hardens `ContextMenu` docs state matrix contracts by locking right-click+keyboard-open, persistent+disabled+item-kinds, and state+source-marker playground anchors in `context_menu_semantics` regression coverage.

- `ui-components`: hardens `CommandDialog` docs state matrix contracts by locking controlled-open+action-close and state+source-marker playground anchors in `command_dialog_semantics` regression coverage.

- `ui-components`: hardens `Command` docs state matrix contracts by locking grouped-search, custom-placeholder+empty-label, and state+source-marker playground anchors in `command_semantics` regression coverage.

- `ui-components`: hardens `Combobox` docs state matrix contracts by locking basic-selection, invalid+disabled-option, and marker-source playground anchors in `combobox_semantics` regression coverage.

- `ui-components`: hardens `ClearButton` docs state contracts by locking default+over-background and inset+focus-mode+disabled playground anchors in `clear_button_semantics` regression coverage.

- `ui-components`: hardens `CloseButton` docs state contracts by locking default+over-background+custom-label and size-matrix+disabled+custom-class playground anchors in `close_button_semantics` regression coverage.

- `ui-components`: hardens `Coachmark` docs state contracts by locking step+cta+asset, controlled+image+actions, and state+source markers playground anchors in `coachmark_semantics` regression coverage.

- `ui-components`: hardens `Collapsible` docs state contracts by locking controlled, disabled+custom-motion, and state+source marker playground anchors in `collapsible_semantics` regression coverage.

- `ui-components`: hardens `Collection` module docs state contracts by locking Item primitives playground anchors used by `Collection`/`CollectionSection`/`CollectionSeparator` compatibility in `collection_module_semantics` regression coverage.

- `ui-components`: hardens `ColorArea` docs state contracts by locking controlled-grid and disabled+custom-grid+custom-class playground anchors in `color_area_semantics` regression coverage.

- `ui-components`: hardens `ColorEditor` docs state contracts by locking controlled-color+format and disabled+alpha-hidden+reduced-motion playground anchors in `color_editor_semantics` regression coverage.

- `ui-components`: hardens `ColorField` docs state contracts by locking controlled-value and invalid+disabled+custom-class playground anchors in `color_field_semantics` regression coverage.

- `ui-components`: hardens `ColorHandle` docs state contracts by locking focused+dragging+position and disabled+custom-class+loupe-off playground anchors in `color_handle_semantics` regression coverage.

- `ui-components`: hardens `ColorLoupe` docs state contracts by locking open+position-buckets and disabled+custom-label+custom-class playground anchors in `color_loupe_semantics` regression coverage.

- `ui-components`: hardens `color` compatibility module docs state contracts by locking forms/display color component and playground anchors in `color_module_semantics` regression coverage.

- `ui-components`: hardens `ColorPicker` docs state contracts by locking controlled-color+open and disabled+default-open+custom-class playground anchors in `color_picker_semantics` regression coverage.

- `ui-components`: hardens `ColorSlider` docs state contracts by locking controlled-hue and disabled-alpha+custom-track+reduced-motion playground anchors in `color_slider_semantics` regression coverage.

- `ui-components`: hardens `ColorSwatch` docs state contracts by locking size+rounding and transparency+accessible-label+shape playground anchors in `color_swatch_semantics` regression coverage.

- `ui-components`: hardens `ColorSwatchPicker` docs state contracts by locking basic-selection and transparency+disabled+custom-class playground anchors in `color_swatch_picker_semantics` regression coverage.

- `ui-components`: hardens `ColorThumb` docs state contracts by locking focused+dragging+position and disabled+custom-class+loupe-off playground anchors in `color_thumb_semantics` regression coverage.

- `ui-components`: hardens `ColorWheel` docs state contracts by locking controlled-hue-wheel and disabled+reduced-motion+custom-class playground anchors in `color_wheel_semantics` regression coverage.

- `ui-components`: hardens `CheckboxGroup` docs state contracts by locking validation+required and disabled+optional playground anchors in `checkbox_group_semantics` regression coverage.

- `ui-components`: hardens `CheckboxField` docs state contracts by locking controlled+description and indicator-end quiet invalid/disabled matrix playground anchors in `checkbox_field_semantics` regression coverage.

- `ui-components`: hardens `Checkbox` docs state contracts by locking controlled+on_change and variant+disabled matrix playground anchors in `checkbox_semantics` regression coverage.

- `ui-components`: hardens `Chart` docs state contracts by locking bar-action and controlled-line active-index playground anchors in `chart_semantics` regression coverage.

- `ui-components`: hardens `Carousel` docs state contracts by locking indicator-motion default, controlled-vertical no-loop, and state/source-marker playground anchors in `carousel_semantics` regression coverage.

- `ui-components`: hardens `Calendar` docs state contracts by locking default+outside-days and monday-first+strong-tone playground anchors in `calendar_semantics` regression coverage.

- `ui-components`: hardens `ThemeToggleButton` docs state contracts by locking default-cycle and custom-modes+disabled playground anchors in `button_theme_toggle_semantics` regression coverage.

- `ui-components`: hardens `ShareButton` docs state contracts by locking default-callback, icon-placement matrix, and custom-class+direction playground anchors in `button_share_semantics` regression coverage.

- `ui-components`: hardens `SearchInputButton` docs state contracts by locking interactive-shortcut, placeholder-disabled matrix, and custom-class+aria playground anchors in `button_search_input_semantics` regression coverage.

- `ui-components`: hardens `ButtonGroup` docs state contracts by locking attached-horizontal and vertical-detached playground anchors in `button_group_semantics` regression coverage.

- `ui-components`: hardens `FlipButton` docs state contracts by locking top-flip, direction-matrix, and custom-class playground anchors in `button_flip_semantics` regression coverage.

- `ui-components`: hardens `ButtonCopy` docs state contracts by locking label+variant and disabled+empty matrix playground anchors in `button_copy_semantics` regression coverage.

- `ui-components`: hardens `Breadcrumb` docs state contracts by locking trail+label-only/empty and primitive link+overflow+state/source-marker playground anchors in `breadcrumb_semantics` regression coverage.

- `ui-components`: hardens `BottomSheet` docs state contracts by locking semantic, detached-title-only-custom-class, and custom-motion playground anchors in `bottom_sheet_semantics` regression coverage.


- `ui-components`: hardens `Asset` docs state contracts by locking file+folder variants, custom-image+focused state, and state+source-marker playground anchors in `asset_semantics` regression coverage.


- `ui-components`: hardens `AspectRatio` docs state contracts by locking ratio-presets and bordered+fill+custom-aria-class playground anchors in `aspect_ratio_semantics` regression coverage.


- `ui-components`: hardens `Button` docs state contracts by locking variants+sizes playground anchors in `button_semantics` regression coverage.


- `ui-components`: hardens `AlertDialog` docs state contracts by locking alert-dialog and state+source-marker playground anchors in `alert_dialog_semantics` regression coverage.


- `ui-components`: hardens `AlertBanner` docs state contracts by locking tone+fill, bold+hidden-icon+custom-class, and custom-motion-contract playground anchors in `alert_banner_semantics` regression coverage.


- `ui-components`: hardens `Alert` docs state contracts by locking variants+live-region and custom-class+compact playground anchors in `alert_semantics` regression coverage.


- `ui-components`: hardens `ActionBar` docs state contracts by locking selection+clear, top-placement+custom-text+reduced-motion, and custom-motion playground anchors in `action_bar_semantics` regression coverage.


- `ui-components`: hardens `ActionMenu` docs state contracts by locking default+controlled+state-marker+disabled-empty playground anchors in `action_menu_semantics` regression coverage.


- `ui-components`: hardens `ActionGroup` docs state contracts by locking single-selection+action-callback and multiple+strong-tone playground anchors in `action_group_semantics` regression coverage.

- `ui-components`: hardens `Heading` docs state contracts by locking heading-levels+tone and strong+truncate+custom-aria-class playground anchors in `heading_semantics` regression coverage.

- `ui-components`: hardens `Footer` docs state contracts by locking semantic-footer+tone and bordered+custom-aria-class playground anchors in `footer_semantics` regression coverage.

- `ui-components`: hardens `Header` docs state contracts by locking semantic-header+tone and bordered+custom-aria-class playground anchors in `header_semantics` regression coverage.

- `ui-components`: hardens `Content` docs state contracts by locking semantic-section+tone and padded+custom-aria-class playground anchors in `content_semantics` regression coverage.

- `ui-components`: hardens `Flex` docs state contracts by locking direction+wrap+gap and inline+distribution playground anchors in `flex_semantics` regression coverage.

- `ui-components`: hardens `Card` docs state contracts by locking variants, padding-states, and custom-class playground anchors in `card_semantics` regression coverage.

- `ui-components`: hardens `View` docs state contracts by locking surface-tokens and element+fluid+custom-class playground anchors in `view_semantics` regression coverage.

- `ui-components`: hardens `UiRoot` docs state contracts by locking usage and root-state-contract playground anchors in `ui_root_semantics` regression coverage.

- `ui-components`: hardens `AutoHeight` docs state contracts by locking animated-height and static-motion+custom-class playground anchors in `auto_height_semantics` regression coverage.

- `ui-components`: hardens `ScrollShadow` docs state contracts by locking default-scrollable and custom-height+class playground anchors in `scroll_shadow_semantics` regression coverage.

- `ui-components`: hardens `Well` docs state contracts by locking tone+density+inset and custom-label+class playground anchors in `well_semantics` regression coverage.

- `ui-components`: hardens `Spacer` docs state contracts by locking axis+size and custom-class-marker playground anchors in `spacer_semantics` regression coverage.

- `ui-components`: hardens `Separator` docs state contracts by locking semantic+element-type and decorative+custom-class playground anchors in `separator_semantics` regression coverage.

- `ui-components`: hardens `Divider` docs state contracts by locking orientation and custom-class-marker playground anchors in `divider_semantics` regression coverage.

- `ui-components`: hardens `ProgressCircle` docs state contracts by locking determinate+indeterminate and custom-value-label+class playground anchors in `progress_circle_semantics` regression coverage.

- `ui-components`: hardens `ProgressBar` docs state contracts by locking variant+size matrix and custom-label+class playground anchors in `progress_bar_semantics` regression coverage.

- `ui-components`: hardens `Progress` docs state contracts by locking determinate+indeterminate and custom-label+motion+class playground anchors in `progress_semantics` regression coverage.

- `ui-components`: hardens `Spinner` docs state contracts by locking size-matrix and custom-label+class playground anchors in `spinner_semantics` regression coverage.

- `ui-components`: hardens `CircularProgress` docs state contracts by locking size+thickness matrix and custom-label+class playground anchors in `circular_progress_semantics` regression coverage.

- `ui-components`: hardens `Skeleton` docs state contracts by locking shimmer and still playground anchors in `skeleton_semantics` regression coverage.

- `ui-components`: hardens `Chip` docs state contracts by locking removable, variants+sizes, custom-label+class, and disabled+static playground anchors in `chip_semantics` regression coverage.

- `ui-components`: hardens `StatusLight` docs state contracts by locking variants, live-role, and custom-class+static playground anchors in `status_light_semantics` regression coverage.

- `ui-components`: hardens `Badge` docs state contracts by locking variant-matrix and custom-class+outline playground anchors in `badge_semantics` regression coverage.

- `ui-components`: hardens `Meter` docs state contracts by locking variant+size matrix and custom-label+motion+class playground anchors in `meter_semantics` regression coverage.

- `ui-components`: hardens `Code` docs state contracts by locking variant-matrix and custom-class+block playground anchors in `code_semantics` regression coverage.

- `ui-components`: hardens `Snippet` docs state contracts by locking copyable+copied-label and static+multiline+custom playground anchors in `snippet_semantics` regression coverage.

- `ui-components`: hardens `CodeBlock` docs state contracts by locking header+copy-motion and compact+no-copy playground anchors in `code_block_semantics` regression coverage.

- `ui-components`: hardens `Kbd` docs state contracts by locking size+keys matrix and custom-class+label-only playground anchors in `kbd_semantics` regression coverage.

- `ui-components`: hardens `Link` docs state contracts by locking state-matrix and custom-rel+class playground anchors in `link_semantics` regression coverage.

- `ui-components`: hardens `SlidingNumber` docs state contracts by locking animated-matrix and custom-separators+motion+class playground anchors in `sliding_number_semantics` regression coverage.

- `ui-components`: hardens `StaticNumber` docs state contracts by locking formatting-matrix and custom-separators+class playground anchors in `static_number_semantics` regression coverage.

- `ui-components`: hardens `MotionRipple` docs state contracts by locking animation-matrix and custom-boundary+class playground anchors (including unbounded origin trigger inputs) in `ripple_semantics` regression coverage.

- `ui-components`: hardens `IllustratedMessage` docs state contracts by locking empty-state playground anchors (`title`/`description`/`illustration`/`actions`) in `illustrated_message_semantics` regression coverage.

- `ui-components`: hardens `Image` docs state contracts by locking image playground anchors (`src`/`alt`/`radius`/`shadow`/`is_zoomed`) in `image_semantics` regression coverage.

- `ui-components`: hardens `AvatarGroup` docs state contracts by locking overflow-stack, size-without-overflow, and custom-aria+class playground anchors in `avatar_group_semantics` regression coverage.

- `ui-components`: hardens `Avatar` docs state contracts by locking image+fallback, size+label-source, and custom-class+normalized-props playground anchors in `avatar_semantics` regression coverage.

- `ui-components`: hardens `Item` docs state contracts by locking media+content+actions and header+footer layout playground anchors in `item_semantics` regression coverage.

- `ui-components`: hardens `Tag` docs state contracts by locking variant+size matrix and removable+disabled+custom-class playground anchors in `tag_semantics` regression coverage.

- `ui-components`: hardens `StepList` docs state contracts by locking controlled-selection and vertical+emphasized+disabled playground anchors in `step_list_semantics` regression coverage.

- `ui-components`: hardens `Table` docs state contracts by locking default+striped and compact+fixed+empty playground anchors in `table_semantics` regression coverage.

- `ui-components`: hardens `Tree` docs state contracts by locking default+expanded-root and strong+compact playground anchors in `tree_semantics` regression coverage.

- `ui-components`: hardens `DisclosureGroup` docs state contracts by locking multiple+controlled and single+disabled+custom-class playground anchors in `disclosure_group_semantics` regression coverage.

- `ui-components`: hardens `Dropdown` docs state contracts by locking default and controlled+persistent+motion playground anchors in `dropdown_semantics` regression coverage.

- `ui-components`: hardens `MenuSection` docs state contracts by locking default-section and quiet+sticky+divider+empty playground anchors in `menu_section_semantics` regression coverage.

- `ui-components`: hardens `MenuItem` docs state contracts by locking action+checkbox and radio+submenu+disabled playground anchors in `menu_item_semantics` regression coverage.

- `ui-components`: hardens `ListBoxSection` docs state contracts by locking default-section and quiet+sticky+divider+empty playground anchors in `listbox_section_semantics` regression coverage.

- `ui-components`: hardens `ListBoxItem` docs state contracts by locking selectable-option and focused+divider+disabled playground anchors in `listbox_item_semantics` regression coverage.

- `ui-components`: hardens `ListBox` docs state contracts by locking selection+typeahead and disabled+empty playground anchors in `listbox_semantics` regression coverage.

- `ui-components`: hardens `Menu` docs state contracts by locking kinds+selection and disabled+empty playground anchors in `menu_semantics` regression coverage.

- `ui-components`: hardens `Tabs` docs state contracts by locking automatic+controlled and manual+disabled playground anchors in `tabs_semantics` regression coverage.

- `ui-components`: hardens `Disclosure` docs state contracts by locking controlled and disabled playground anchors in `disclosure_semantics` regression coverage.

- `ui-components`: hardens `Accordion` docs state contracts by locking multiple+controlled and single+disabled playground anchors in `accordion_semantics` regression coverage.

- `ui-components`: hardens `Breadcrumbs` docs state contracts by locking trail and label-only+empty playground anchors in `breadcrumbs_semantics` regression coverage.

- `ui-components`: hardens `TagGroup` docs state contracts by locking removable+state, validation+required, and disabled+empty playground anchors in `tag_group_semantics` regression coverage.

- `ui-components`: hardens `Pagination` docs state contracts by locking pages+on_change and disabled+empty playground anchors in `pagination_semantics` regression coverage.

- `ui-components`: hardens `DropdownMenu` docs state contracts by locking default, controlled+persistent-open, and disabled+empty playground anchors in `dropdown_menu_semantics` regression coverage.

- `ui-components`: hardens `Autocomplete` docs state contracts by locking selection+validation, controlled-open, and disabled+empty playground anchors in `autocomplete_semantics` regression coverage.

- `ui-components`: hardens `ComboBox` docs state contracts by locking selection+validation, controlled-open, and disabled+empty playground anchors in `combo_box_semantics` regression coverage.

- `ui-components`: hardens `Select` docs state contracts by locking controlled-open+selection and disabled+empty playground anchors in `select_semantics` regression coverage.

- `ui-components`: hardens `MenuTrigger` docs state contracts by locking default, controlled+persistent-open, and disabled+empty playground anchors in `menu_trigger_semantics` regression coverage.

- `ui-components`: hardens `SegmentedControl` docs state contracts by locking selection-root-state and vertical+disabled+empty playground anchors in `segmented_control_semantics` regression coverage.

- `ui-components`: hardens `ActionButtonGroup` docs state contracts by locking default-compact and vertical+justified+disabled playground anchors in `action_button_group_semantics` regression coverage.

- `ui-components`: hardens `ActionButton` docs state contracts by locking default-callback and loading-placement+icon-only matrix playground anchors in `action_button_semantics` regression coverage.

- `ui-components`: hardens `ToggleButtonGroup` docs state contracts by locking attached-horizontal and vertical-detached playground anchors, counts, and alignment-label wiring in `toggle_button_group_semantics` regression coverage.

- `ui-components`: hardens `ToggleButton` docs state contracts by locking controlled on_change and variant+disabled matrix playground anchors in `toggle_button_semantics` regression coverage.

- `ui-components`: hardens `LinkButton` docs state contracts by locking external-target rel-hardening and variant+size+disabled matrix playground anchors in `link_button_semantics` regression coverage.

- `ui-components`: hardens `IconButton` docs state contracts by locking on_press+variant and size+disabled matrix playground anchors in `icon_button_semantics` regression coverage.

- `ui-components`: hardens `ButtonGroup` docs state contracts by locking attached-horizontal and vertical-detached playground anchors, counters, and aria-label wiring in `button_group_semantics` regression coverage.

- `ui-components`: hardens `Button` docs state contracts by locking variants+sizes playground anchors, segmented controls, and disabled/loading toggle wiring in `button_semantics` regression coverage.

- `ui-components`: hardens `ShareButton` docs state contracts by locking default-callback, icon-placement+items matrix, and custom-class+direction playground anchors in `button_share_semantics` regression coverage.

- `ui-components`: hardens `ThemeToggleButton` docs state contracts by locking default-cycle and custom-modes+disabled playground anchors in `button_theme_toggle_semantics` regression coverage.

- `ui-components`: hardens `SearchInputButton` docs state contracts by locking interactive-shortcut, placeholder-disabled matrix, and custom-class+aria playground anchors in `button_search_input_semantics` regression coverage.

- `ui-components`: hardens `FlipButton` docs state contracts by locking top-flip, direction-matrix, and custom-class playground anchors in `button_flip_semantics` regression coverage.

- `ui-components`: hardens `Dialog` docs state contracts by locking default-playground anchors (id/title/description/footer actions/exit callback wiring) in `dialog_semantics` regression coverage.

- `ui-components`: hardens `AlertDialog` docs state contracts by locking default-playground anchors (trigger variant/id/title/description/confirm/exit wiring) in `alert_dialog_semantics` regression coverage.

- `ui-components`: hardens `Overlay` docs state/source contracts by locking default and marker playground anchors (`dismiss`/`keyboard-dismiss`/`role`/`motion` source wiring + custom `initial_scale`/`initial_y_px`) in `overlay_semantics` regression coverage.

- `ui-components`: hardens `Tags` docs state/source contracts by locking removable/disabled default playground anchors plus marker-playground source wiring (`variant`/`size`/`invalid`/`aria`/`class`) in `tags_semantics` regression coverage.

- `ui-components`: hardens `SplitView` docs state/source contracts by locking horizontal/controlled default playground anchors plus marker-playground bounds/label/handler wiring in `split_view_semantics` regression coverage.

- `ui-components`: hardens `Empty` docs state/source contracts by locking icon/action default playground anchors plus marker-slot class/variant wiring in `empty_semantics` regression coverage.

- `ui-components`: hardens `FlipCard` docs state/motion contracts by locking click+keyboard/disabled anchors plus marker-playground motion/source values (`hover_scale`/`hover_tilt_deg`, id/class/flip-mode wiring) in `flip_card_semantics` regression coverage.

- `ui-components`: hardens `ActionBar` docs state/motion contracts by locking default/top-placement anchors plus custom-motion playground spring/opacity/offset values in `action_bar_semantics` regression coverage.

- `ui-components`: hardens `Iconset` docs state/source contracts by locking registry/fallback default anchors plus marker-playground namespace/label/class/tone wiring in `iconset_semantics` regression coverage.

- `ui-components`: hardens `Icons` docs state/source contracts by locking default/custom anchors plus marker-playground set/scale/glyph/aria/class/tone wiring in `icons_semantics` regression coverage.

- `ui-components`: hardens `IconsUi` docs state/source contracts by locking built-in/custom anchors plus marker-playground icon-reference/glyph/aria/class/size/tone wiring in `icons_ui_semantics` regression coverage.

- `ui-components`: hardens `IconsWorkflow` docs state/source contracts by locking built-in/custom anchors plus marker-playground icon-reference/glyph/aria/class/size/tone wiring in `icons_workflow_semantics` regression coverage.

- `ui-components`: hardens `Thumbnail` docs state/motion contracts by locking sizes+state anchors plus custom-motion contract values (`active_scale`/`active_ring_opacity`) in `thumbnail_semantics` regression coverage.

- `ui-components`: hardens `Swatch` docs state/motion contracts by locking size/state anchors plus custom-motion contract values (`selected_scale`/`selected_ring_opacity`) in `swatch_semantics` regression coverage.

- `ui-components`: hardens `Number` docs state/motion contracts by locking static+sliding playground anchors plus custom separator/motion values (`decimal_places`/`thousand_separator`/`animate`) in `number_semantics` regression coverage.

- `ui-components`: hardens `ButtonCopy` docs state contracts by locking label/variant and disabled-empty matrix playground anchors in `button_copy_semantics` regression coverage.

- `ui-components`: hardens `Dropzone` docs state/motion contracts by locking disabled and marker-playground anchors plus explicit custom motion values (`hover_scale`/`drop_scale`) in `dropzone_semantics` regression coverage.

- `ui-components`: hardens `PickerButton` docs state/source contracts by locking state-matrix and marker-playground anchors (quiet/invalid/disabled/active + aria/class/handler source wiring) in `picker_button_semantics` regression coverage.

- `ui-components`: hardens `Picker` docs state/motion contracts by locking controlled-open anchors and state/source marker popover motion values (`initial_scale`/`offset_y_px`) in `picker_semantics` regression coverage.

- `ui-components`: hardens `Search` docs state/motion contracts by locking required+invalid anchors and state/source marker motion values (`hidden_scale`/`hover_scale`/`tap_scale`) in `search_semantics` regression coverage.

- `ui-components`: hardens `TextArea` docs state/source contracts by locking marker playground anchors and state/source values (`rows`/description/error/placeholder/class wiring) in `text_area_semantics` regression coverage.

- `ui-components`: hardens `Textarea` docs state/source contracts by locking basic/marker playground anchors and state/source marker values (`rows`/description/error/placeholder/class wiring) in `textarea_semantics` regression coverage.

- `ui-components`: hardens `Textfield` docs state/source contracts by locking required+invalid anchors and state/source marker values (custom description/error/placeholder/type/class wiring) in `textfield_semantics` regression coverage.

- `ui-components`: hardens `Toggle` docs state/motion contracts by locking outline+disabled anchors and state/source marker values (`tap_scale`/custom aria+handler wiring) in `toggle_semantics` regression coverage.

- `ui-components`: hardens `Combobox` docs state/motion contracts by locking invalid+disabled anchors and state/source marker motion values (`offset_y_px`/`stiffness`) in `combobox_semantics` regression coverage.

- `ui-components`: hardens `Breadcrumb` docs state/source contracts by locking ellipsis-overflow anchors and state/source marker playground values (custom aria/class/link/page/separator content) in `breadcrumb_semantics` regression coverage.

- `ui-components`: hardens `Sidenav` docs state/source contracts by locking controlled-floating markers and state/source playground anchors (`shortcut_key`/`trigger_label`/`handler-source`) in `sidenav_semantics` regression coverage.

- `ui-components`: hardens `TopNav` docs state/motion contracts by locking controlled-selection markers and state/source playground motion values (`stiffness`/`damping`) in `top_nav_semantics` regression coverage.

- `ui-components`: hardens `CommandDialog` docs state/motion contracts by locking controlled-open markers and custom overlay-motion anchors (`initial_scale`/`initial_y_px`) in `command_dialog_semantics` regression coverage.

- `ui-components`: hardens `Carousel` docs state/motion contracts by locking controlled vertical/no-loop markers and custom indicator motion values (`stiffness`/`damping`) in `carousel_semantics` regression coverage.

- `ui-components`: hardens `NavigationMenu` docs state/motion contracts by locking controlled-selection/source markers and custom highlight motion values (`stiffness`/`damping`) in `navigation_menu_semantics` regression coverage.

- `ui-components`: hardens `Menubar` docs state/motion contracts by locking controlled-open/source markers and custom popover motion anchors (`initial_scale`/`offset_y_px`) in `menubar_semantics` regression coverage.

- `ui-components`: hardens `ContextMenu` docs state/motion contracts by locking persistent/open-source markers and custom popover motion anchors (`initial_scale`/`offset_y_px`) in `context_menu_semantics` regression coverage.

- `ui-components`: hardens `Command` docs state/motion contracts by locking custom-placeholder labels and state/source marker motion values (`stiffness`/`damping`) in `command_semantics` regression coverage.

- `ui-components`: hardens `Collapsible` motion/docs contracts by locking disabled/custom-motion (`panel_offset_y_px`) and state/source marker playground anchors in `collapsible_semantics` regression coverage.

- `ui-components`: hardens `HoverCard` motion docs contracts by locking state/source marker playground values (`open_delay_ms`/`close_delay_ms`/custom id/class) and custom-motion anchors (`initial_scale`/`offset_y_px`) in `hover_card_semantics` regression coverage.

- `ui-components`: hardens `Tooltip` motion docs contracts by locking state/source marker playground values (`delay_ms`/`close_delay_ms`/custom id/class) and custom-motion anchors (`initial_scale`/`offset_y_px`) in `tooltip_semantics` regression coverage.

- `ui-components`: hardens `Underlay` docs state/source contracts by locking source-marker playground anchors (`data-transparent-source`/`data-disabled-source`/`data-close-source`/`data-class-source`) in `underlay_semantics` regression coverage.

- `ui-components`: hardens `BottomSheet` motion docs contracts by locking docs custom-playground `BottomSheetMotion -> SheetMotion.initial_offset_px` values, docs route anchors, and css aggregation wiring in `bottom_sheet_semantics` regression coverage.

- `ui-components`: hardens `Tray` motion docs contracts by locking marker-playground `TrayMotion -> SheetMotion.initial_offset_px` values, fixed-height dismiss source markers, and custom docs state anchors in `tray_semantics` regression coverage.

- `ui-components`: hardens `Sonner` motion docs contracts by locking marker-playground `ToastMotion` values (`initial_y_px`/`initial_scale`), top-left queue/source markers, and custom docs state anchors in `sonner_semantics` regression coverage.

- `ui-components`: hardens `Toaster` motion docs contracts by locking marker-playground `ToastMotion` values (`initial_y_px`/`initial_scale`), top-left queue/source markers, and custom docs state anchors in `toaster_semantics` regression coverage.

- `ui-components`: hardens `Toast` motion docs contracts by locking marker-playground `ToastMotion` values (`initial_y_px`/`initial_scale`), danger variant source markers, and custom docs state anchors in `toast_semantics` regression coverage.

- `ui-components`: hardens `PreviewLinkCard` motion docs contracts by locking marker-playground `PreviewLinkCardMotion` values (`initial_scale`/`offset_y_px`), delay/site-label source markers, and custom docs state anchors in `preview_link_card_semantics` regression coverage.

- `ui-components`: hardens `PreviewCard` motion docs contracts by locking marker-playground `PreviewCardMotion` values (`initial_scale`/`offset_y_px`), delay/site-label source markers, and custom docs state anchors in `preview_card_semantics` regression coverage.

- `ui-components`: hardens `Modal` motion docs contracts by locking custom `OverlayMotion` marker values (`initial_scale`/`initial_y_px`), custom id/class source markers, and exit-presence wiring in `modal_semantics` regression coverage.

- `ui-components`: hardens `Sheet` motion docs contracts by locking marker-playground `SheetMotion.initial_offset_px`, dismiss/keyboard-dismiss source markers, and exit-presence wiring in `sheet_semantics` regression coverage.

- `ui-components`: hardens `ActionMenu` motion docs contracts by locking marker-playground `ActionMenuMotion -> PopoverMotion` values (`initial_scale`/`offset_y_px`), controlled-open markers, and custom source attrs in `action_menu_semantics` regression coverage.

- `ui-components`: hardens `Drawer` motion docs contracts by locking custom-motion playground values (`SheetMotion.initial_offset_px`), left-placement/source markers, and exit-presence wiring in `drawer_semantics` regression coverage.

- `ui-components`: hardens `Popover` motion docs contracts by locking custom-motion playground values (`initial_scale`/`offset_y_px`), modal/source markers, and exit-presence wiring in `popover_semantics` regression coverage.

- `ui-components`: hardens `ButtonCopy` motion/docs contracts by expanding semantics coverage for module/crate exports, CSS aggregation wiring, and docs playground stability anchors.

- `ui-components`: hardens `SearchInputButton` HeroUI-level motion contracts by expanding semantics coverage for spring/scale sanitize invariants, view-level motion+interaction source markers, and docs motion narrative stability.

- `ui-components`: hardens `FlipButton` HeroUI-level motion contracts by expanding semantics coverage for default/sanitized spring invariants, view-level motion source markers, and docs motion narrative stability across direction/custom-class variants.

- `ui-components`: hardens `ThemeToggleButton` HeroUI-level motion contracts by expanding semantics coverage for motion defaults/sanitization clamps, rotate+scale CSS-variable driver wiring, and docs motion narrative stability.

- `ui-components`: hardens `ShareButton` HeroUI-level motion contracts by expanding semantics coverage for motion sanitization delegation (`ShareButtonMotion -> FlipButtonMotion`), view-level `data-motion-source`/`data-custom-motion` markers, and docs motion narrative stability.

- `ui-components`: hardens `IconButton` motion docs contracts by locking marker-playground motion/source values (`ButtonMotion` hover/tap scale + class marker) in `icon_button_semantics` regression coverage.

- `ui-components`: hardens `Dialog` motion docs contracts by locking custom overlay-motion marker values (`initial_scale`/`initial_y_px`) in `dialog_semantics` regression coverage.

- `ui-components`: hardens `AlertDialog` motion docs contracts by locking custom overlay-motion marker values (`initial_scale`/`initial_y_px` + autofocus source) in `alert_dialog_semantics` regression coverage.

- `docs-app`/`ui-components`: adds a dedicated `Custom Motion Contract` hover-card playground (`overlays_hover_card::hover_card`) and extends `hover_card_semantics` to lock these motion-demo contracts against regression.

- `docs-app`/`ui-components`: adds a dedicated `Custom Motion Contract` tooltip playground (`overlays_tooltip::tooltip`) and extends `tooltip_semantics` to lock these motion-demo contracts against regression.

- `ui-components`: hardens `ActionBar` HeroUI-level motion regression coverage (defaults/disabled/sanitization/reduced-motion paths), and `apps/docs-app` adds a dedicated `Custom Motion Contract` action-bar playground (`actions_extra::action_bar`).

- `ui-components`: hardens `SlidingNumber` HeroUI-level motion behavior by enforcing reduced-motion sanitization in `sanitize_motion` and adds explicit spring/reduced-path regression coverage.

- `ui-components`: hardens `Thumbnail` HeroUI-level motion regression coverage (`ThumbnailMotion` defaults/sanitization/reduced-motion/disabled paths) and `apps/docs-app` adds a dedicated `Custom Motion Contract` thumbnail playground (`display_extra_thumbnail`).

- `ui-components`: hardens `Swatch` HeroUI-level motion regression coverage (`SwatchMotion` defaults/sanitization/reduced-motion paths) and `apps/docs-app` adds a dedicated `Custom Motion Contract` swatch playground (`display_extra_swatch`).

- `docs-app`: refactors `components/pages.rs` catalog entries to `component_doc!` form (reducing file size/maintenance overhead) and hardens `playground_coverage` parsing to support alias-based and delegated catalog functions without false negatives.

- `ui-components`: expands `upstream_name_parity_semantics` to enforce name-parity coverage across `react-aria-components`, `@react-aria`, `@react-spectrum`, `heroui`, `shadcn`, `animate-ui`, and `adobe-spectrum-web-components` sources for regression prevention.

- `ui-components`: adds `example_theme` compatibility module (mirroring `@react-aria/example-theme` with light-theme defaults), adds `example_theme_module_semantics` regression coverage, and maps `example-theme` docs-module coverage to the existing `ui-root` playground in `apps/docs-app`.

- `ui-components`: adds `visually_hidden` compatibility module (introducing a `VisuallyHidden` utility with focusable reveal semantics), adds `visually_hidden_module_semantics` regression coverage, and adds a dedicated `visually-hidden` docs playground in `apps/docs-app`.

- `ui-components`: adds `virtualizer` compatibility module (re-exporting `ScrollArea` contracts), adds `virtualizer_module_semantics` regression coverage, and maps `virtualizer` docs-module coverage to the existing `scroll-area` playground in `apps/docs-app`.

- `ui-components`: adds `shared_element_transition` compatibility module (re-exporting `View` contracts under SharedElementTransition naming), adds `shared_element_transition_module_semantics` regression coverage, and maps `shared-element-transition` docs-module coverage to the existing `view` playground in `apps/docs-app`.

- `ui-components`: adds `overlay_arrow` compatibility module (re-exporting icon contracts with popover placement aliasing), adds `overlay_arrow_module_semantics` regression coverage, and maps `overlay-arrow` docs-module coverage to existing `icon` and `popover` playgrounds in `apps/docs-app`.

- `ui-components`: adds `hidden_date_input` compatibility module (re-exporting `DateInputGroup` contracts), adds `hidden_date_input_module_semantics` regression coverage, and maps `hidden-date-input` docs-module coverage to the existing `date-input-group` playground in `apps/docs-app`.

- `ui-components`: adds `collection` compatibility module (re-exporting `Item`/`ItemGroup`/`ItemSeparator` contracts under Collection naming), adds `collection_module_semantics` regression coverage, and maps `collection` docs-module coverage to the existing `item` playground in `apps/docs-app`.

- `ui-components`: adds `selection_indicator` compatibility module (re-exporting listbox/menu selection-indicator contracts), adds `selection_indicator_module_semantics` regression coverage, and maps `selection-indicator` docs-module coverage to existing `listbox-item` and `menu-item` playgrounds in `apps/docs-app`.

- `ui-components`: adds `group` compatibility module (re-exporting `FieldGroup` contracts as `Group` aliases), adds `group_module_semantics` regression coverage, and maps `group` docs-module coverage to the existing `field-group` playground in `apps/docs-app`.

- `ui-components`: adds `grid_list` compatibility module (re-exporting `GridList` contracts under snake_case module naming), adds `grid_list_module_semantics` regression coverage, and maps `grid-list` docs-module coverage to existing listbox playgrounds in `apps/docs-app`.

- `ui-components`: adds `drag_and_drop` compatibility module (re-exporting `DropZone`/`FileTrigger` contracts and aliasing `DragAndDrop` motion contracts), adds `drag_and_drop_module_semantics` regression coverage, and maps `drag-and-drop` docs-module coverage to existing files playgrounds in `apps/docs-app`.

- `ui-components`: adds `list_box` compatibility module (re-exporting `ListBox`/`ListBoxItem`/`ListBoxSection` contracts under snake_case module naming), adds `list_box_module_semantics` regression coverage, and maps `list-box` docs-module coverage to existing listbox playgrounds in `apps/docs-app`.

- `ui-components`: adds `spinbutton` compatibility module (re-exporting `NumberField` as `SpinButton`), adds `spinbutton_module_semantics` regression coverage, and maps `spinbutton` docs-module coverage to the existing `number-field` playground in `apps/docs-app`.

- `ui-components`: adds `gridlist` compatibility module (re-exporting `ListBox`/`ListBoxItem`/`ListBoxSection` contracts as `GridList` aliases), adds `gridlist_module_semantics` regression coverage, and maps `gridlist` docs-module coverage to existing `listbox` playgrounds in `apps/docs-app`.

- `apps/docs-app`: adds `dev_css_hot_reload` regression coverage to lock `index.html` + `dev-overrides.css` ordering/workflow contracts, ensuring dev-mode style edits stay hot-swappable without Rust recompilation regressions.

- `ui-components`: updates `BreadcrumbPage` semantics to keep `aria-current="page"` while removing interactive-only `role="link"`/`aria-disabled="true"` markers, and adds regression coverage to lock the current-page non-interactive contract.

- `ui-components`: updates `NavigationMenu` item markup to rely on native anchor semantics (removing redundant `role="link"`) while preserving `aria-current` state markers, and adds regression coverage for the contract.

- `ui-components`: aligns `Item` list semantics by adding `role="listitem"` to `Item` when used with `ItemGroup role="list"`, and adds regression coverage to lock the accessibility contract.

- `ui-components`: adds dedicated `number_semantics` regression coverage for `StaticNumber`/`SlidingNumber` (`mod` exports, logic contracts, motion sanitization wiring, CSS aggregation, docs playground anchors) to prevent compatibility regressions.

- `ui-components`: adds dedicated `button_flip_semantics` regression coverage to lock `button_flip` module re-export contracts, crate-root compatibility exports, and docs playground anchors.

- `ui-components`: adds dedicated `button_search_input_semantics` regression coverage to lock `button_search_input` module re-export contracts, crate-root compatibility exports, and docs playground anchors.

- `ui-components`: adds dedicated `button_share_semantics` regression coverage to lock `button_share` module re-export contracts, crate-root compatibility exports, and docs playground anchors.

- `ui-components`: adds dedicated `button_theme_toggle_semantics` regression coverage to lock `button_theme_toggle` module re-export contracts, crate-root compatibility exports, and docs playground anchors.

- `ui-components`: adds dedicated `inline_style_contract_semantics` regression coverage to enforce `docs/spec/styling.md` rules (forbid `style:` prop bindings and require CSS-variable-based inline style contracts only).

- `ui-components`: adds dedicated `upstream_name_parity_semantics` regression coverage to lock same-name component parity for `examples/_upstream/shadcn-ui` (`new-york-v4/ui`) and `examples/_upstream/animate-ui`.

- `ui-components`: adds `toolbar` compatibility module (re-exporting `ActionBar`/`ActionBarMotion` as `Toolbar`/`ToolbarMotion`) and `toolbar_module_semantics` regression coverage; `apps/docs-app` module-coverage mapping now maps `toolbar` to the existing `action-bar` docs page.

- `ui-components`: aligns `Meter` ARIA semantics with a valid `role="meter"` contract (replacing invalid multi-role markup), updates semantics assertions, and preserves Spectrum-style state marker behavior.

- `ui-components`: upgrades `Toast` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring/entry contracts before runtime attachment, and adding regression tests to prevent NaN/invalid motion values from leaking into HeroUI-level toast transitions.

- `ui-components`: upgrades `Separator` motion safety by introducing `sanitize_motion` guards, validating entry-animation contracts before runtime attachment, and adding regression tests to keep SSR/wasm motion behavior aligned with HeroUI-level separator transitions.

- `ui-components`: upgrades `SegmentedControl` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime indicator attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level segmented-control motion.

- `ui-components`: upgrades `SlidingNumber` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level numeric rolling motion.

- `ui-components`: upgrades `ProgressCircle` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level circular progress motion.

- `ui-components`: upgrades `Progress` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level progress indicator motion.

- `ui-components`: upgrades `Meter` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level meter progress motion.

- `ui-components`: upgrades `InlineAlert` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level inline-alert reveal behavior.

- `ui-components`: upgrades `Image` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping custom spring/zoom-scale values before runtime attachment, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level image zoom feedback.

- `ui-components`: upgrades `IllustratedMessage` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping custom spring/offset values before runtime attachment, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level illustrated-message reveal behavior.

- `ui-components`: upgrades `FlipCard` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping custom spring/scale/tilt values before runtime attachment, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level flip-card interactions.

- `ui-components`: upgrades `CodeBlock` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring/flash-hold contracts before runtime attachment, and adding regression tests to prevent NaN/invalid motion contracts from leaking into HeroUI-level copy feedback behavior.

- `ui-components`: upgrades `ShareButton` motion safety by introducing `sanitize_motion` wrapper guards over nested flip-button contracts, sanitizing forwarded custom spring values before runtime handoff to `FlipButton`, and adding regression tests to prevent NaN/invalid motion contracts from leaking into HeroUI-level share interactions.

- `ui-components`: upgrades `FlipButton` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime attachment, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level flip interactions.

- `ui-components`: upgrades `ButtonCopy` motion safety by introducing `sanitize_motion` wrapper guards over nested button contracts, sanitizing forwarded custom spring/scale values before runtime handoff to `Button`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level copy-button interactions.

- `ui-components`: upgrades `DropZone` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping custom spring/scale/highlight values before runtime attachment, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level drag-hover feedback behavior.

- `ui-components`: upgrades `AutoHeight` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring contracts before runtime animation wiring, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level auto-height transitions.

- `ui-components`: upgrades `Select` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level select open/close behavior.

- `ui-components`: upgrades `Dropdown` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level dropdown open/close behavior.

- `ui-components`: upgrades `ComboBox` motion safety by introducing `sanitize_motion` guards over nested popover/highlight contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to popover and active-highlight drivers, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level combo-box interaction behavior.

- `ui-components`: upgrades `Autocomplete` motion safety by introducing `sanitize_motion` guards over nested popover/highlight contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to popover and active-highlight drivers, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level autocomplete interaction behavior.

- `ui-components`: upgrades `ColorPicker` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level color-picker open/close behavior.

- `ui-components`: upgrades `DatePicker` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level date-picker open/close behavior.

- `ui-components`: upgrades `ContextualHelp` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level contextual-help open/close behavior.

- `ui-components`: upgrades `MenuTrigger` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level menu-trigger open/close behavior.

- `ui-components`: upgrades `DropdownMenu` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level dropdown-menu open/close behavior.

- `ui-components`: upgrades `ActionMenu` motion safety by introducing `sanitize_motion` wrapper guards over nested popover contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Popover`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level action-menu open/close behavior.

- `ui-components`: upgrades `AlertBanner` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom spring parameters before runtime animation wiring, and adding regression tests to prevent NaN/invalid spring values from leaking into HeroUI-level alert-banner reveal behavior.

- `ui-components`: upgrades `AlertDialog` motion safety by introducing `sanitize_motion` wrapper guards over nested overlay contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Overlay`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level alert-dialog entry/exit behavior.

- `ui-components`: upgrades `Dialog` motion safety by introducing `sanitize_motion` wrapper guards over nested overlay contracts, sanitizing forwarded custom spring/scale/offset values before runtime handoff to `Overlay`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level dialog entry/exit behavior.

- `ui-components`: upgrades `Drawer` motion safety by introducing `sanitize_motion` wrapper guards over nested sheet contracts, sanitizing forwarded custom spring/offset values before runtime handoff to `Sheet`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level drawer entry/exit behavior.

- `ui-components`: upgrades `BottomSheet` motion safety by introducing `sanitize_motion` wrapper guards over nested sheet contracts, sanitizing forwarded custom spring/offset values before runtime handoff to `Sheet`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level bottom-sheet entry/exit behavior.

- `ui-components`: upgrades `Sheet` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level sheet entry/exit behavior.

- `ui-components`: upgrades `Tray` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards over nested sheet contracts, clamping invalid custom spring/offset values before runtime handoff to `Sheet`, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level tray entry/exit behavior.

- `ui-components`: upgrades `PreviewLinkCard` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/scale/offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level preview-link-card entry/exit behavior.

- `ui-components`: upgrades `PreviewCard` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/scale/offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level preview-card entry/exit behavior.

- `ui-components`: upgrades `Tooltip` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/scale/offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level tooltip entry/exit behavior.

- `ui-components`: upgrades `HoverCard` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/scale/offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level hover-card entry/exit behavior.

- `ui-components`: upgrades `Disclosure`/`Collapsible` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/rotation/panel-offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level panel/indicator behavior.

- `ui-components`: upgrades `Popover`/`Overlay` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/scale/offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level entry/exit behavior.

- `ui-components`: upgrades `Accordion` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/indicator-rotation/panel-offset values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into HeroUI-level disclosure behavior.

- `ui-components`: upgrades `Tabs` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards for indicator spring contracts, validating custom spring parameters before runtime animation wiring, and adding regression tests to prevent NaN/invalid spring values from leaking into indicator behavior.

- `ui-components`: upgrades `Button` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into button interaction behavior.

- `ui-components`: upgrades `Checkbox` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards (including indicator spring), clamping invalid custom spring/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into checkbox interaction behavior.

- `ui-components`: upgrades `Switch` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, validating custom pressed-thumb width input before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into switch thumb interaction behavior.

- `ui-components`: upgrades `Radio` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into radio interaction behavior.

- `ui-components`: upgrades `ToggleButton` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into toggle-button interaction behavior.

- `ui-components`: upgrades `ActionButton` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into action-button interaction behavior.

- `ui-components`: upgrades `Input` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/clear-button hidden/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into input clear-button interaction behavior.

- `ui-components`: upgrades `SearchField` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/clear-button hidden/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into search-field clear-button interaction behavior.

- `ui-components`: upgrades `SearchInputButton` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/hover/tap scale values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into search-input-button interaction behavior.

- `ui-components`: upgrades `ThemeToggleButton` motion safety by introducing `sanitize_motion`/`sanitize_spring` guards, clamping invalid custom spring/rotate/scale/delay values before runtime animation wiring, and adding regression tests to prevent NaN/overflow motion contracts from leaking into theme-toggle icon interaction behavior.

- `ui-components`: upgrades `FileTrigger` motion safety by introducing `sanitize_motion` wrapper guards over nested button motion contracts, sanitizing forwarded trigger motion before runtime wiring, and adding regression tests to prevent NaN/overflow values from leaking into file-trigger interaction behavior.

- `ui-components`: upgrades `ContextMenu` to a full `logic/styles/view` slice with centralized slot/open-mode/item/action/id/aria/class/disabled/placement/motion source-state normalization, stable Spectrum-style root+trigger `data-*` contracts, and preserved context-trigger + keyboard-open semantics with HeroUI-level popover spring motion reuse.
- `apps/docs-app`: upgrades `ContextMenu` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-aria-label-source`/`data-disabled-indices-source`/`data-close-on-action-source`/`data-open-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Menubar` to a full `logic/styles/view` slice with centralized slot/open-mode/menu/action/id/class/placement/open-index/motion source-state normalization, stable Spectrum-style root+menu+trigger `data-*` contracts, and preserved keyboard roving + HeroUI-level popover spring motion behavior.
- `apps/docs-app`: upgrades `Menubar` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-class-source`/`data-close-on-action-source`/`data-open-index-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `NavigationMenu` to a full `logic/styles/view` slice with centralized slot/state/selection/focus/activation/id/aria/class/motion source-state normalization, stable Spectrum-style root/list/item `data-*` contracts, and preserved HeroUI-level active-highlight spring motion behavior.
- `apps/docs-app`: upgrades `NavigationMenu` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-aria-label-source`/`data-activate-on-focus-source`/`data-selected-id-source`/`data-selected-id-change-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Carousel` to a full `logic/styles/view` slice with centralized slot/state/selection/focus/orientation/navigation/id/aria/class/motion source-state normalization, stable Spectrum-style root/item/indicator `data-*` contracts, and preserved HeroUI-level active-highlight spring motion behavior.
- `apps/docs-app`: upgrades `Carousel` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-aria-label-source`/`data-orientation-source`/`data-loop-navigation-source`/`data-selected-index-source`/`data-selected-index-change-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Command` to a full `logic/styles/view` slice with centralized slot/state/query/group/item/id/placeholder/empty-label/aria/class/disabled/action/motion source-state normalization, stable Spectrum-style root/list/item `data-*` contracts, and preserved HeroUI-level active-highlight spring motion behavior.
- `apps/docs-app`: upgrades `Command` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-placeholder-source`/`data-empty-label-source`/`data-aria-label-source`/`data-action-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `ActionMenu` to a full `logic/styles/view` slice with centralized slot/state/open-mode/action/id/aria/class/disabled/indices/item-kinds/placement/open-control/motion source-state normalization, stable Spectrum-style root `data-*` contracts, and preserved HeroUI-level popover spring motion behavior.
- `apps/docs-app`: upgrades `ActionMenu` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-aria-label-source`/`data-disabled-indices-source`/`data-item-kinds-source`/`data-open-source`/`data-open-change-source`/`data-motion-source` contracts.

- `ui-components`: adds `PreviewCard` as a new Spectrum-compatible hover/focus preview overlay component with full `logic/styles/motion/view` slice, centralized title/description/url/site-label/image/id/class/delay/motion source-state normalization, stable root/trigger/panel `data-*` contracts, and HeroUI-level spring enter/exit motion.
- `apps/docs-app`: adds `PreviewCard` docs with `Basic Preview`, `State + Source Markers`, and `Default Fallbacks` playgrounds to inspect `data-content`/`data-delay-source`/`data-title-source`/`data-description-source`/`data-url-source`/`data-site-label-source`/`data-motion-source` contracts.

- `ui-components`: adds `PreviewLinkCard` as a new Spectrum-compatible hover/focus preview-link overlay component with full `logic/styles/motion/view` slice, centralized title/description/url/site-label/image/id/class/delay/motion source-state normalization, stable root/trigger/panel `data-*` contracts, and HeroUI-level spring enter/exit motion.
- `apps/docs-app`: adds `PreviewLinkCard` docs with `Preview Snapshot`, `State + Source Markers`, and `Default Fallbacks` playgrounds to inspect `data-content`/`data-delay-source`/`data-title-source`/`data-description-source`/`data-url-source`/`data-site-label-source`/`data-motion-source` contracts.

- `ui-components`: adds `FlipCard` as a new Spectrum-compatible 3D front/back display component with full `logic/styles/motion/view` slice, centralized flip-state/flip-mode/id/class/motion source-state normalization, stable root/front/back `data-*` contracts, and HeroUI-level spring flip/hover motion.
- `apps/docs-app`: adds `FlipCard` docs with `Click + Keyboard Flip`, `State + Source Markers`, and `Disabled` playgrounds to inspect `data-state`/`data-flip-mode`/`data-motion-source`/`data-id-source`/face-level `data-visible`/`data-hidden` contracts.

- `ui-components`: upgrades `Sonner` to a full `logic/styles/view` slice with centralized slot/queue/position/portal/max-toasts/aria/class/motion/store source-state normalization, stable Spectrum-style root `data-*` contracts, and preserved HeroUI-level spring toast motion behavior via `ToastMotion`.
- `apps/docs-app`: extends `Sonner` docs with a `State + Source Markers` playground to inspect `data-state`/`data-queue`/`data-position-source`/`data-portal-source`/`data-max-toasts-source`/`data-store-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Toaster` to a full `logic/styles/view` slice with centralized slot/queue/position/portal/max-toasts/aria/class/motion/store source-state normalization, stable Spectrum-style root `data-*` contracts, and preserved HeroUI-level spring toast motion handoff through `Sonner`.
- `apps/docs-app`: extends `Toaster` docs with a `State + Source Markers` playground to inspect `data-state`/`data-queue`/`data-position-source`/`data-portal-source`/`data-max-toasts-source`/`data-store-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Underlay` to a full `logic/styles/view` slice with centralized slot/open/transparent/disabled/close/class source-state normalization, stable Spectrum-style root `data-*` contracts, and preserved close-interaction semantics.
- `apps/docs-app`: extends `Underlay` docs with a `State + Source Markers` playground to inspect `data-state`/`data-tone`/`data-close-mode`/`data-transparent-source`/`data-disabled-source`/`data-close-source`/`data-class-source` contracts.

- `ui-components`: upgrades `Toast` to a full `logic/styles/view` slice with centralized root + viewport slot/state/source normalization (`id`/`description`/`class`/`motion`/`close`/`exit`/`portal`/`max-toasts`/`store`), stable Spectrum-style `data-*` contracts, and preserved HeroUI-level spring entry/exit behavior.
- `apps/docs-app`: upgrades `Toast` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-description-source`/`data-close-source`/`data-exit-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Dialog` to a full `logic/styles/view` slice with centralized size/id/title/description/footer/close/class/motion/exit source-state normalization, stable Spectrum-style `data-*` contracts, and preserved Overlay-based a11y semantics.
- `apps/docs-app`: upgrades `Dialog` docs with a `State + Source Markers` playground to inspect `data-size-source`/`data-id-source`/`data-title-source`/`data-description-source`/`data-close-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `AlertDialog` to a full `logic/styles/view` slice with centralized variant/description/actions/auto-focus/id/title/motion/exit source-state normalization, stable Spectrum-style `data-*` contracts, and preserved alertdialog overlay semantics.
- `apps/docs-app`: upgrades `AlertDialog` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-title-source`/`data-description-source`/`data-cancel-source`/`data-secondary-source`/`data-auto-focus-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `CommandDialog` to a full `logic/styles/view` slice with centralized open-mode/description/close-on-action/id/title/placeholder/aria/handler/motion source-state normalization, stable Spectrum-style `data-*` contracts, and preserved Modal + Command composition semantics.
- `apps/docs-app`: upgrades `CommandDialog` docs with a `State + Source Markers` playground to inspect `data-id-source`/`data-title-source`/`data-description-source`/`data-placeholder-source`/`data-action-source`/`data-overlay-motion-source` contracts.

- `ui-components`: upgrades `Tray` to a full `logic/styles/view` slice with centralized description/footer/close/size/dismiss/keyboard/id/title/class/motion/exit source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior via `SheetMotion`.
- `apps/docs-app`: extends `Tray` docs with a `State + Source Markers` playground to inspect `data-state`/`data-size-source`/`data-dismiss-source`/`data-keyboard-dismiss-source`/`data-motion-source`/`data-exit-source` contracts.

- `ui-components`: upgrades `Drawer` to a full `logic/styles/view` slice with centralized placement/description/footer/close/id/title/class/motion/exit source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior via `SheetMotion`.
- `apps/docs-app`: extends `Drawer` docs with a `State + Source Markers` playground to inspect `data-state`/`data-placement-source`/`data-description-source`/`data-footer-source`/`data-motion-source`/`data-exit-source` contracts.

- `ui-components`: upgrades `Sheet` to a full `logic/styles/view` slice with centralized placement/dismiss/keyboard/aria/motion/exit source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior.
- `apps/docs-app`: extends `Sheet` docs with a `State + Source Markers` playground to inspect `data-state`/`data-placement-source`/`data-dismiss-source`/`data-keyboard-dismiss-source`/`data-motion-source`/`data-exit-source` contracts.

- `ui-components`: upgrades `Modal` to a full `logic/styles/view` slice with centralized id/title/description/class/motion/exit source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level overlay spring motion behavior.
- `apps/docs-app`: extends `Modal` docs with a `State + Source Markers` playground to inspect `data-state`/`data-id-source`/`data-title-source`/`data-description-source`/`data-motion-source`/`data-exit-source` contracts.

- `ui-components`: upgrades `Overlay` to a full `logic/styles/view` slice with centralized dismiss/keyboard/role/aria/class/motion/exit source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior.
- `apps/docs-app`: extends `Overlay` docs with a `State + Source Markers` playground to inspect `data-state`/`data-dismiss-source`/`data-keyboard-dismiss-source`/`data-role-source`/`data-motion-source`/`data-exit-source` contracts.

- `ui-components`: upgrades `Popover` to a full `logic/styles/view` slice with centralized modal/placement/exit/source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior.
- `apps/docs-app`: extends `Popover` docs with a `State + Source Markers` playground to inspect `data-state`/`data-modal`/`data-motion-source`/`data-placement-source`/`data-modal-source`/`data-exit-source` contracts.

- `ui-components`: upgrades `Toggle` to a full `logic/styles/view` slice with centralized interaction/variant/size/aria/handler source-state normalization, stable Spectrum-style `data-*` contracts, and preserved HeroUI-level press motion behavior.
- `apps/docs-app`: extends `Toggle` docs with a `State + Source Markers` playground to inspect `data-state`/`data-interaction`/`data-variant-source`/`data-motion-source`/`data-aria-source`/`data-handler-source` contracts.

- `ui-components`: upgrades `Tooltip` to a full `logic/styles/view` slice with centralized trigger/press/delay/id source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior.
- `apps/docs-app`: extends `Tooltip` docs with a `State + Source Markers` playground to inspect `data-state`/`data-delay-source`/`data-trigger-source`/`data-press-source`/`data-id-source` contracts.

- `ui-components`: upgrades `HoverCard` to a full `logic/styles/view` slice with centralized id/delay/motion/source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and preserved HeroUI-level spring motion behavior.
- `apps/docs-app`: extends `HoverCard` docs with a `State + Source Markers` playground to inspect root/trigger/panel `data-state`/`data-motion-source`/`data-delay-source`/`data-id-source` contracts.

- `ui-components`: upgrades `Empty` to a full `logic/styles/view` slice with centralized slot/media/source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition for `Empty*` primitives.
- `apps/docs-app`: extends `Empty` docs with a `State + Source Markers` playground to inspect `data-slot`/`data-state`/`data-class-source`/`data-variant`/`data-variant-source` contracts across `Empty*` slots.

- `ui-components`: upgrades `SplitView` to a full `logic/styles/view` slice with centralized split/bounds/aria/class/handler source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition over `Resizable`.
- `apps/docs-app`: extends `SplitView` docs with a `State + Source Markers` playground to inspect `data-state`/`data-orientation`/`data-split-mode`/`data-default-split-source`/`data-bounds-source`/`data-label-source`/`data-class-source`/`data-handler-source` contracts.
- `ui-components`: upgrades `Tags` to a full `logic/styles/view` slice with centralized tag/a11y/source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition over `TagGroup`.
- `apps/docs-app`: extends `Tags` docs with a `State + Source Markers` playground to inspect `data-state`/`data-content`/`data-removal`/`data-constraint`/`data-label-source`/`data-describedby-source`/`data-class-source`/`data-variant-source`/`data-size-source`/`data-handler-source` contracts.
- `ui-components`: upgrades `Iconset` to a full `logic/styles/view` slice with centralized namespace/glyph/label source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition over `Icon`.
- `apps/docs-app`: extends `Iconset` docs with a `State + Source Markers` playground to inspect `data-state`/`data-icon-source`/`data-iconset-source`/`data-label-source`/`data-class-source`/`data-size-source`/`data-tone-source` contracts.
- `ui-components`: upgrades `Icons` to a full `logic/styles/view` slice with centralized set/scale/source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened composition across `IconsUi`/`IconsWorkflow`.
- `apps/docs-app`: extends `Icons` docs with a `State + Source Markers` playground to inspect `data-state`/`data-set`/`data-scale`/`data-set-source`/`data-aria-source`/`data-class-source`/`data-glyph-source`/`data-tone-source` contracts.
- `ui-components`: upgrades `IconsUi` to a full `logic/styles/view` slice with centralized icon-reference/registry/source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition over `Iconset`.
- `apps/docs-app`: extends `IconsUi` docs with a `State + Source Markers` playground to inspect `data-state`/`data-icon-reference-source`/`data-aria-source`/`data-class-source`/`data-glyph-source`/`data-size-source`/`data-tone-source` contracts.
- `ui-components`: upgrades `IconsWorkflow` to a full `logic/styles/view` slice with centralized icon-reference/registry/source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition over `Iconset`.
- `apps/docs-app`: extends `IconsWorkflow` docs with a `State + Source Markers` playground to inspect `data-state`/`data-icon-reference-source`/`data-aria-source`/`data-class-source`/`data-glyph-source`/`data-size-source`/`data-tone-source` contracts.
- `ui-components`: upgrades `Asset` to a full `logic/styles/view` slice with centralized label/source-state normalization, stable Spectrum-style `data-*` contracts, and dedicated `asset` CSS layered through `ui-components/src/css.rs`.
- `apps/docs-app`: extends `Asset` docs with a `State + Source Markers` playground to inspect `data-state`/`data-label-source`/`data-content-source`/`data-class-source` contracts.
- `ui-components`: upgrades `Coachmark` to a full `logic/styles/view` slice with centralized heading/step/footer/asset source normalization, stable Spectrum-style `data-*` state/source contracts, and deduplicated controlled/uncontrolled composition through a shared ContextualHelp pipeline.
- `apps/docs-app`: extends `Coachmark` docs with a `State + Source Markers` playground to inspect `data-state`/`data-open-mode`/`data-label-source`/`data-class-source` and content-level `data-asset-source` contracts.
- `ui-components`: upgrades `Breadcrumb` primitives to a full `logic/styles/view` slice with centralized aria/href/separator content normalization, stable Spectrum-style `data-*` state/source contracts, and dedicated `breadcrumb` CSS layered through `ui-components/src/css.rs`.
- `apps/docs-app`: extends `BreadcrumbList` docs with a `State + Source Markers` playground to inspect `data-state`/`data-aria-source`/`data-class-source`/`data-href-state`/`data-content-source` contracts.
- `ui-components`: upgrades `Collapsible` to a full `logic/styles/view` slice with centralized id/label/source-state normalization, stable Spectrum-style `data-*` state/source/motion contracts, and hardened controlled/uncontrolled disclosure composition.
- `apps/docs-app`: extends `Collapsible` docs with a `State + Source Markers` playground to inspect `data-state`/`data-open-mode`/`data-label-source`/`data-class-source`/`data-motion-source` contracts.
- `ui-components`: upgrades `TopNav` to a full `logic/styles/view` slice with centralized label/default-selection/source-state normalization, stable Spectrum-style `data-*` state/source/motion contracts, and deduplicated controlled/uncontrolled NavigationMenu composition.
- `apps/docs-app`: extends `TopNav` docs with a `State + Source Markers` playground to inspect `data-state`/`data-selection-mode`/`data-default-selection`/`data-focus-activation`/`data-label-source`/`data-motion-source` contracts.
- `ui-components`: upgrades `Combobox` to a full `logic/styles/view` slice with centralized label/placeholder/error/source-state normalization, stable Spectrum-style `data-*` state/source/motion contracts, and hardened wrapper composition over `ComboBox`.
- `apps/docs-app`: extends `Combobox` docs with a `State + Source Markers` playground to inspect `data-state`/`data-selection`/`data-options`/`data-requirement`/`data-placeholder-source`/`data-motion-source` contracts.
- `ui-components`: upgrades `Dropzone` to a full `logic/styles/view` slice with centralized label/aria/handler source normalization, stable Spectrum-style `data-*` state/source/motion contracts, and hardened wrapper composition over `DropZone`.
- `apps/docs-app`: extends `Dropzone` docs with a `State + Source Markers` playground to inspect `data-state`/`data-label-source`/`data-aria-source`/`data-drop-handler-source`/`data-motion-source` contracts.
- `ui-components`: upgrades `Search` to a full `logic/styles/view` slice with centralized label/description/error/handler source normalization, stable Spectrum-style `data-*` state/source/motion contracts, and hardened wrapper composition over `SearchField`.
- `apps/docs-app`: extends `Search` docs with a `State + Source Markers` playground to inspect `data-state`/`data-value`/`data-requirement`/`data-submit-handler-source`/`data-clear-handler-source`/`data-motion-source` contracts.
- `ui-components`: upgrades `Textfield` to a full `logic/styles/view` slice with centralized label/input-type/source-state normalization, stable Spectrum-style `data-*` state/source contracts, and hardened wrapper composition over `TextField`.
- `apps/docs-app`: extends `Textfield` docs with a `State + Source Markers` playground to inspect `data-state`/`data-value`/`data-requirement`/`data-type-source`/`data-class-source` contracts.
- `ui-components`: upgrades `TextArea` to a full `logic/styles/view` slice with centralized label/rows/source-state normalization, stable Spectrum-style `data-*` state/source contracts, and preserved headless text-field semantics.
- `apps/docs-app`: extends `TextArea` docs with a `State + Source Markers` playground to inspect `data-state`/`data-value`/`data-requirement`/`data-rows-source`/`data-class-source` contracts.
- `ui-components`: upgrades `Textarea` to a full `logic/styles/view` slice with centralized label/rows/source-state normalization, stable Spectrum-style `data-*` state/source contracts, and preserved headless text-field semantics.
- `apps/docs-app`: extends `Textarea` docs with a `State + Source Markers` playground to inspect `data-state`/`data-value`/`data-requirement`/`data-rows-source`/`data-class-source` contracts.
- `ui-components`: upgrades `PickerButton` to a full `logic/styles/view` slice with centralized aria/class/handler source-state normalization, stable Spectrum-style `data-*` wrapper contracts, and hardened alias composition over `FieldButton`.
- `apps/docs-app`: extends `PickerButton` docs with a `State + Source Markers` playground to inspect `data-state`/`data-quiet`/`data-invalid`/`data-disabled`/`data-active`/`data-has-handler`/`data-aria-source`/`data-class-source`/`data-handler-source` contracts.
- `ui-components`: upgrades `Sidenav` to a full `logic/styles/view` slice with centralized aria/trigger/shortcut/class/handler source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and hardened alias composition over `Sidebar`.
- `apps/docs-app`: extends `Sidenav` docs with a `State + Source Markers` playground to inspect `data-state`/`data-open-mode`/`data-initial-open`/`data-trigger-mode`/`data-shortcut-mode`/`data-label-source`/`data-trigger-source`/`data-shortcut-source`/`data-class-source`/`data-handler-source` contracts.
- `ui-components`: upgrades `IconButton` to a full `logic/styles/view` slice with centralized aria/size/handler/class/motion source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and hardened alias composition over `Button`.
- `apps/docs-app`: adds a dedicated `IconButton` docs page with a `State + Source Markers` playground to inspect `data-state`/`data-size-mode`/`data-handler-source`/`data-label-source`/`data-class-source`/`data-motion-source` contracts.
- `ui-components`: upgrades `Picker` to a full `logic/styles/view` slice with centralized placeholder/open/placement/motion source-state normalization, stable Spectrum-style wrapper `data-*` contracts, and hardened alias composition over `Select`.
- `apps/docs-app`: extends `Picker` docs with a `State + Source Markers` playground to inspect `data-state`/`data-selection`/`data-disabled-options`/`data-open-mode`/`data-initial-open`/`data-placeholder-source`/`data-handler-source`/`data-placement-source`/`data-motion-source` contracts.

- `ui-components`: upgrades `Menu` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level active-highlight spring tuning.
- `ui-components`: upgrades `NavigationMenu` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level active-highlight spring tuning.
- `ui-components`: upgrades `TopNav` wrapper with root motion markers (`data-motion-source`/`data-custom-motion`) and stable custom-motion style selectors for HeroUI-level navigation-highlight spring tuning.
- `ui-components`: upgrades `Sonner` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level toast-stack spring orchestration tuning.
- `ui-components`: upgrades `Toggle` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level pressed-state spring tuning.
- `ui-components`: upgrades `ToggleButton` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level press/selection spring tuning.
- `ui-components`: upgrades `Toaster` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level toast-stack spring orchestration tuning.
- `ui-components`: upgrades `Toast` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level toast spring reveal tuning.
- `ui-components`: upgrades `InlineAlert` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `InlineAlertMotion` contract tests for HeroUI-level spring reveal tuning.
- `ui-components`: upgrades `ButtonCopy` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `ButtonCopyMotion` contract tests for HeroUI-level spring interaction tuning.
- `ui-components`: upgrades `FlipButton` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `FlipButtonMotion` contract tests for HeroUI-level spring flip tuning.
- `ui-components`: upgrades `SearchInputButton` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `SearchInputButtonMotion` contract tests for HeroUI-level spring interaction tuning.
- `ui-components`: upgrades `ShareButton` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `ShareButtonMotion` contract tests for HeroUI-level spring flip tuning.
- `ui-components`: upgrades `ThemeToggleButton` icon motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level theme-toggle micro-interaction tuning.
- `ui-components`: upgrades `Accordion` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level spring disclosure tuning.
- `ui-components`: upgrades `Button` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level press/hover spring tuning.
- `ui-components`: upgrades `Checkbox` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level toggle spring tuning.
- `ui-components`: upgrades `Separator` root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `SeparatorMotion` contract tests for HeroUI-level divider reveal tuning.
- `ui-components`: upgrades `Disclosure` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level panel spring disclosure tuning.
- `ui-components`: upgrades `Input` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level clear-button spring interaction tuning.
- `ui-components`: upgrades `Radio` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level selection spring feedback tuning.
- `ui-components`: upgrades `Switch` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level thumb spring interaction tuning.
- `ui-components`: upgrades `Tabs` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level indicator spring tracking tuning.
- `ui-components`: upgrades `Thumbnail` root motion markers (`data-motion-source`/`data-custom-motion`) with stable custom-motion style selectors for HeroUI-level selection ring spring feedback tuning.
- `ui-components`: upgrades `AutoHeight` with explicit root motion source markers (`data-motion-source`/`data-custom-motion`) and `AutoHeightMotion` contract tests for HeroUI-level spring height interpolation tuning.
- `ui-components`: upgrades `AlertBanner` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `AlertBannerMotion` contract tests for HeroUI-level spring reveal tuning.
- `apps/docs-app`: extends `AlertBanner` docs with a custom-motion playground showing `AlertBannerMotion` customization and motion marker inspection.
- `ui-components`: upgrades `ActionButton` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `ActionButtonMotion` contract tests for HeroUI-level spring interaction tuning.
- `ui-components`: upgrades `FileTrigger` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `FileTriggerMotion` contract tests for HeroUI-level trigger-motion tuning.
- `apps/docs-app`: extends `FileTrigger` docs with a custom-motion playground showing `FileTriggerMotion { trigger: ButtonMotion { hover_scale/tap_scale, .. } }` tuning.
- `ui-components`: upgrades `BottomSheet` with explicit root motion markers (`data-motion-source`/`data-custom-motion`), stable custom-motion style selectors, and `BottomSheetMotion` contract tests for HeroUI-level sheet-motion tuning.
- `apps/docs-app`: extends `BottomSheet` docs with a custom-motion playground showing `BottomSheetMotion { sheet: SheetMotion { initial_offset_px, .. } }` tuning.
- `ui-components`: upgrades `DropZone` with explicit root motion markers (`data-motion-source`/`data-custom-motion`) and `DropZoneMotion` contract tests for HeroUI-level spring hover/drop interaction tuning.
- `apps/docs-app`: extends `DropZone` docs with a custom-motion playground showing `DropZoneMotion` tuning (`hover_scale`/`drop_scale`/`hover_highlight`).
- `ui-components`: upgrades `Tray` with explicit root state/motion markers (`data-open`/`data-closed` + `data-motion-source`/`data-custom-motion`) and `TrayMotion` contract tests for HeroUI-level sheet-motion passthrough tuning.
- `apps/docs-app`: extends `Tray` docs with a custom-motion playground showing `TrayMotion { sheet: SheetMotion { initial_offset_px, .. } }` tuning.
- `ui-components`: upgrades `AlertDialog` with explicit root state/variant/motion markers (`data-state`/`data-open`/`data-closed` + `data-variant` + `data-motion-source`/`data-custom-motion`) and `AlertDialogMotion` contract tests for HeroUI-level overlay-motion tuning.
- `apps/docs-app`: extends `AlertDialog` docs with a custom-motion playground showing `AlertDialogMotion { overlay: OverlayMotion { initial_scale, initial_y_px, .. } }` tuning.
- `ui-components`: upgrades `Dialog` with explicit root state/motion markers (`data-state`/`data-open`/`data-closed` + `data-motion-source`/`data-custom-motion`) and `DialogMotion` contract tests for HeroUI-level overlay-motion tuning.
- `apps/docs-app`: extends `Dialog` docs with a custom-motion playground showing `DialogMotion { overlay: OverlayMotion { initial_scale, initial_y_px, .. } }` tuning.
- `ui-components`: upgrades `Drawer` with explicit root state/motion markers (`data-open`/`data-closed` + `data-motion-source`/`data-custom-motion`) and `DrawerMotion` contract tests for HeroUI-level sheet-motion passthrough tuning.
- `apps/docs-app`: extends `Drawer` docs playground with a custom-motion example demonstrating `DrawerMotion { sheet: SheetMotion { initial_offset_px, .. } }` tuning.
- `ui-components`: upgrades `Sheet` with explicit root state/dismiss/motion markers (`data-state`/`data-open`/`data-closed` + `data-dismissable`/`data-keyboard-dismiss-disabled` + `data-motion-source`/`data-custom-motion`) and `SheetMotion` spring/direction contract tests for HeroUI-level motion tuning.
- `ui-components`: upgrades `Tooltip` with explicit root state/motion markers (`data-state`/`data-open`/`data-closed` + `data-motion-source`/`data-custom-motion`) and `TooltipMotion` spring/placement contract tests for HeroUI-level motion tuning.
- `ui-components`: upgrades `HoverCard` with explicit root state/motion markers (`data-state`/`data-open`/`data-closed` + `data-motion-source`/`data-custom-motion`) and `HoverCardMotion` spring/placement contract tests for HeroUI-level interaction tuning.
- `ui-components`: upgrades `ComboBox` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) plus `ComboBoxMotion` contract tests (`PopoverMotion` + `ActiveHighlightMotion`) for HeroUI-level spring customization.
- `ui-components`: upgrades `Autocomplete` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) plus `AutocompleteMotion` contract tests (`PopoverMotion` + `ActiveHighlightMotion`) for HeroUI-level spring customization.
- `ui-components`: upgrades `Overlay` with explicit root state/dismiss/motion markers (`data-state`/`data-open`/`data-closed` + `data-dismissable`/`data-keyboard-dismiss-disabled` + `data-motion-source`/`data-custom-motion`) and `OverlayMotion` spring contract tests for HeroUI-level tuning.
- `ui-components`: upgrades `Popover` with explicit root state/motion markers (`data-state`/`data-open`/`data-closed` + `data-motion-source`/`data-custom-motion`) and `PopoverMotion` spring/placement contract tests for HeroUI-level motion tuning.
- `apps/docs-app`: extends `Popover` docs with a dedicated custom-motion playground showing `PopoverMotion` tuning (`initial_scale`/`offset_y_px`) without regressing presence-based unmount flow.
- `ui-components`: upgrades `Menubar` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) and motion alias contract tests, keeping `PopoverMotion` passthrough behavior compatible with HeroUI spring tuning.
- `ui-components`: upgrades `ContextMenu` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) and motion alias contract tests, keeping `PopoverMotion` passthrough behavior compatible with HeroUI spring tuning.
- `ui-components`: upgrades `ContextualHelp` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) and `ContextualHelpMotion` contract tests, keeping `PopoverMotion` passthrough behavior compatible with HeroUI spring tuning.
- `ui-components`: upgrades `DropdownMenu` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) and `DropdownMenuMotion` contract tests, keeping `PopoverMotion` passthrough behavior compatible with HeroUI spring tuning.
- `ui-components`: upgrades `ActionMenu` with explicit motion source markers (`data-motion-source`/`data-custom-motion`) and `ActionMenuMotion` contract tests, keeping `PopoverMotion` passthrough behavior compatible with HeroUI spring tuning.
- `ui-components`: upgrades `DatePicker` with explicit `DatePickerMotion` (`PopoverMotion` passthrough), exposing stable motion markers (`data-motion-source`/`data-custom-motion`) and configurable spring-style popover animation parity with HeroUI expectations.
- `apps/docs-app`: extends `DatePicker` docs playground with motion-tuned custom spring-style popover configuration.
- `ui-components`: upgrades `ColorPicker` with explicit `ColorPickerMotion` (`PopoverMotion` passthrough), exposing stable motion markers (`data-motion-source`/`data-custom-motion`) and configurable spring-style popover animation parity with HeroUI expectations.
- `ui-components`: upgrades `MenuTrigger` with explicit `MenuTriggerMotion` (`PopoverMotion` passthrough), exposing stable motion markers (`data-motion-source`/`data-custom-motion`) and configurable spring-style menu popover animation parity with HeroUI expectations.
- `ui-components`: upgrades `Select` with explicit `SelectMotion` (`PopoverMotion` passthrough), exposing stable motion markers (`data-motion-source`/`data-custom-motion`) and configurable spring-style overlay animation parity with HeroUI expectations.
- `ui-components`: upgrades `Dropdown` with explicit `DropdownMotion` (`PopoverMotion` passthrough), adding stable motion source markers (`data-motion-source`/`data-custom-motion`) for HeroUI-level spring tuning and regression-safe styling.
- `apps/docs-app`: extends `Dropdown` docs with a motion-tuned controlled playground showing custom spring-style popover configuration.

- `ui-components`: adds `rac` compatibility module mirroring HeroUI `rac` surface (`Direction`/`I18nProvider`/`Collection` reexports + locale/filter helpers) for shared component integration ergonomics.
- `apps/docs-app`: maps the new `rac` module to existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `test_utils` compatibility module exposing `snapshot_theme_css` for `-spectrum/test-utils` naming parity and snapshot-style theme contract reuse.
- `apps/docs-app`: maps the new `test-utils` module to existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `style_macro_s1` compatibility module that exposes layered CSS assembly (`build_s1_layer_css`) on top of `push_components_css` for `-spectrum/style-macro-s1` naming parity.
- `apps/docs-app`: maps the new `style-macro-s1` module to existing `ui-root` docs/playground coverage where layered CSS injection is documented.

- `ui-components`: adds `story_utils` compatibility module with `story_theme()` + `UiRoot` contracts for `-spectrum/story-utils` naming parity.
- `apps/docs-app`: maps the new `story-utils` module to existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `s2` compatibility module that re-exports `Theme` + `UiRoot` contracts for `-spectrum/s2` naming parity.
- `apps/docs-app`: maps the new `s2` module to existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `utils` compatibility module that re-exports core interaction hooks (`use_press`, `use_hover`, `use_focus_ring`) and option contracts from `ui_headless` for `-spectrum/utils` parity.
- `apps/docs-app`: maps the new `utils` module to existing `button` docs/playground coverage where these hook contracts are consumed.

- `ui-components`: adds `theme_express` compatibility module with `express_theme()` (currently mapped to `Theme::light()`) for `@react-spectrum/theme-express` naming parity.
- `apps/docs-app`: maps the new `theme-express` module to the existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `theme_dark` compatibility module with `dark_theme()` (`Theme::dark()`) for `@react-spectrum/theme-dark` naming parity.
- `apps/docs-app`: maps the new `theme-dark` module to the existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `theme_light` compatibility module with `light_theme()` (`Theme::light()`) for `@react-spectrum/theme-light` naming parity.
- `apps/docs-app`: maps the new `theme-light` module to the existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `theme_default` compatibility module with `default_theme()` (`Theme::light()`) for `@react-spectrum/theme-default` naming parity.
- `apps/docs-app`: maps the new `theme-default` module to the existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `dnd` compatibility module that re-exports `DropZone`/`FileTrigger` contracts for `@react-spectrum/dnd` naming parity in component space.
- `apps/docs-app`: maps the new `dnd` module to existing `drop-zone` + `file-trigger` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `list` compatibility module that re-exports `ListView` (via `ListBox`) and `Item` contracts for `@react-spectrum/list` naming parity.
- `apps/docs-app`: maps the new `list` module to existing `listbox` + `item` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `color` compatibility module that re-exports Spectrum color family contracts (`ColorArea`, `ColorWheel`, `ColorSlider`, `ColorField`, `ColorSwatch`, `ColorPicker`, `ColorEditor`, `ColorSwatchPicker`) for `@react-spectrum/color` naming parity.
- `apps/docs-app`: maps the new `color` module to existing color docs/playground coverage across forms and display catalogs.

- `ui-components`: adds `overlays` compatibility module that re-exports `Overlay`/`Popover`/`Modal`/`Tray` contracts for `@react-spectrum/overlays` naming parity.
- `apps/docs-app`: maps the new `overlays` module to existing `overlay` + `popover` + `modal` + `tray` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds `layout` compatibility module that re-exports `Flex`/`Grid` contracts for `@react-spectrum/layout` naming parity.
- `apps/docs-app`: maps the new `layout` module to existing `flex` + `grid` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds react-spectrum-compatible `Provider` alias that re-exports `UiRoot` for naming parity while preserving existing root theme/safe-area contracts.
- `apps/docs-app`: maps the new `provider` module to the existing `ui-root` docs/playground coverage for module-level catalog parity.

- `ui-components`: adds shadcn-compatible `item` primitive family (`Item`, `ItemGroup`, `ItemSeparator`, `ItemMedia`, `ItemContent`, `ItemTitle`, `ItemDescription`, `ItemActions`, `ItemHeader`, `ItemFooter`) with stable slot/variant contracts.
- `apps/docs-app`: adds item primitive docs/playground coverage for media-content-actions and header-footer composition layouts.

- `ui-components`: adds `radio_group` compatibility module that re-exports `RadioGroup` as-is and aliases `Radio` to `RadioGroupItem` for shadcn naming parity.
- `apps/docs-app`: reuses existing `RadioGroup` docs/playground coverage for `radio-group` compatibility semantics.

- `ui-components`: adds shadcn-compatible `breadcrumb` primitive family (`BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`, `BreadcrumbSeparator`, `BreadcrumbEllipsis`) with stable slot contracts.
- `apps/docs-app`: adds breadcrumb primitive docs/playground coverage for link/current-page and ellipsis-overflow compositions.

- `ui-components`: adds `<DirectionProvider>` as a shadcn/Radix-compatible direction context wrapper with normalized `direction`/`dir` props and stable slot/data-direction markers.
- `apps/docs-app`: adds a `DirectionProvider` docs page with LTR and RTL playground coverage.

- `ui-components`: adds shadcn-compatible `<Empty>` composition primitives (`Empty`, `EmptyHeader`, `EmptyTitle`, `EmptyDescription`, `EmptyContent`, `EmptyMedia`) with stable slot/variant contracts.
- `apps/docs-app`: adds an `Empty` docs page with icon-header and content-action playground coverage.

- `ui-components`: adds `<Icons>` as a Spectrum-compatible `@spectrum-web-components/icons` wrapper, mapping medium/large scale and ui/workflow set resolution onto `IconsUi`/`IconsWorkflow` with stable data-state markers.
- `apps/docs-app`: adds an `Icons` docs page with set-scale matrix and custom workflow glyph extension playground coverage.

- `ui-components`: adds `<IconsWorkflow>` as a Spectrum-compatible `icons-workflow` wrapper with workflow namespace normalization, default workflow glyph registry, and `Iconset` accessibility/source-state contracts.
- `apps/docs-app`: adds an `IconsWorkflow` docs page with built-in workflow glyph and custom-extension playground coverage.

- `ui-components`: adds `<IconsUi>` as a Spectrum-compatible `icons-ui` wrapper with built-in UI icon registry defaults, namespace normalization, and `Iconset` accessibility/source-state contracts.
- `apps/docs-app`: adds an `IconsUi` docs page with built-in glyph and custom-registry extension playground coverage.

- `ui-components`: adds `<Iconset>` as a Spectrum-compatible iconset registry wrapper, resolving `iconset:icon` references onto `Icon` with namespace/source state markers and accessibility fallbacks.
- `apps/docs-app`: adds an `Iconset` docs page with namespaced registry and fallback-source playground coverage.

- `ui-components`: adds `<Coachmark>` as a Spectrum-compatible guided-tour overlay primitive, composed on `ContextualHelp`/`Popover` contracts with optional asset, step, and CTA navigation semantics plus HeroUI-level spring motion reuse.
- `apps/docs-app`: adds a `Coachmark` docs page with step/cta/asset-variant and controlled image/actions playground coverage.

- `ui-components`: adds `<Asset>` as a Spectrum-compatible primitive for file/folder/custom media representation, composed on Thumbnail state contracts and HeroUI-level spring focus-selection motion reuse.
- `apps/docs-app`: adds an `Asset` docs page with file/folder variants and custom-image focused-state playground coverage.

- `ui-components`: adds `<TopNav>` as an upstream-name-compatible wrapper over `NavigationMenu`, preserving Spectrum top-nav selection/accessibility contracts and HeroUI-level active-indicator spring motion behavior.
- `apps/docs-app`: adds a `TopNav` docs page with default-selection/roving-focus and controlled-label/disabled-item playground coverage.

- `ui-components`: adds `<AlertBanner>` as a Spectrum-compatible banner alert primitive with centralized tone/fill/content contracts and HeroUI-grade spring reveal motion.
- `apps/docs-app`: adds an `AlertBanner` docs page with tone/fill and bold hidden-icon custom-class playground coverage.

- `ui-components`: adds `<Swatch>` as a Spectrum-compatible fill preview primitive with centralized size/shape/rounding/border/state contracts and HeroUI-grade spring selection motion.
- `apps/docs-app`: adds a `Swatch` docs page with size/shape/rounding and mixed/nothing/disabled controlled playground coverage.

- `ui-components`: adds `<Thumbnail>` as a Spectrum-compatible preview primitive with centralized size/background/cover/layer/state contracts and HeroUI-grade spring focus-selection motion.
- `apps/docs-app`: adds a `Thumbnail` docs page with sizes and cover/background/layer/selected playground coverage.

- `ui-components`: adds `<Combobox>` as an upstream-name-compatible wrapper over `ComboBox`, preserving Spectrum accessibility/state contracts and HeroUI-grade panel/highlight motion behavior.
- `apps/docs-app`: adds a `Combobox` docs page with basic selection and invalid/disabled-option playground coverage.

- `ui-components`: adds `<Textfield>` as an upstream-name-compatible wrapper over `TextField`, preserving Spectrum accessibility/state contracts.
- `apps/docs-app`: adds a `Textfield` docs page with basic and required/invalid playground coverage.

- `ui-components`: adds `<Dropzone>` as an upstream-name-compatible wrapper over `DropZone`, preserving Spectrum drag/drop + paste accessibility contracts and HeroUI-grade spring interaction motion behavior.
- `apps/docs-app`: adds a `Dropzone` docs page with drop/paste and disabled playground coverage.

- `ui-components`: adds `<Search>` as an upstream-name-compatible wrapper over `SearchField`, preserving Spectrum search accessibility/state contracts and HeroUI-grade clear-button spring motion behavior.
- `apps/docs-app`: adds a `Search` docs page with submit/clear and required/invalid playground coverage.

- `ui-components`: adds `<Tags>` as an upstream-name-compatible wrapper over `TagGroup`, preserving Spectrum tags collection accessibility/state contracts and HeroUI-level removable chip interaction patterns.
- `apps/docs-app`: adds a `Tags` docs page with removable and disabled playground coverage.

- `ui-components`: adds `<PickerButton>` as an upstream-name-compatible wrapper over `FieldButton`, preserving Spectrum picker-trigger accessibility/state contracts and HeroUI-level press/focus interaction behavior.
- `apps/docs-app`: adds a `PickerButton` docs page with interactive and state-matrix playground coverage.

- `ui-components`: adds `<Sidenav>` as an upstream-name-compatible wrapper over `Sidebar`, preserving Spectrum side-navigation controlled/uncontrolled accessibility contracts and HeroUI-level trigger/rail interaction behavior.
- `apps/docs-app`: adds a `Sidenav` docs page with controlled floating and icon-collapsible/no-shortcut playground coverage.

- `ui-components`: adds `<SplitView>` as an upstream-name-compatible wrapper over `Resizable`, preserving Spectrum split-pane controlled/uncontrolled accessibility contracts and HeroUI-level drag/keyboard handle interaction behavior.
- `apps/docs-app`: adds a `SplitView` docs page with horizontal default and controlled vertical-bounds playground coverage.

- `ui-components`: adds `<Picker>` as an upstream-name-compatible wrapper over `Select`, preserving Spectrum picker accessibility/state contracts and HeroUI-level trigger/listbox interaction behavior.
- `apps/docs-app`: adds a `Picker` docs page with basic selection and controlled-open/disabled-option playground coverage.

- `ui-components`: adds `<InfieldButton>` as a Spectrum-compatible in-field trigger button with centralized quiet/invalid/active/disabled state normalization and stable `data-*` markers.
- `apps/docs-app`: adds an `InfieldButton` docs page with default/quiet and invalid/active/disabled playground coverage.

- `ui-components`: adds `<FieldLabel>` as a Spectrum-compatible field label primitive with centralized tone/required/source-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `FieldLabel` docs page with tone/required and custom indicator/aria/class playground coverage.

- `ui-components`: adds `<SidebarMenuAction>` as a Shadcn-compatible sidebar menu action primitive with centralized visibility/disabled/source-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarMenuAction` docs page with hover-only action press and always-visible disabled custom-class playground coverage.

- `ui-components`: adds `<SidebarMenuBadge>` as a Shadcn-compatible sidebar menu badge primitive with centralized tone/disabled/source-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarMenuBadge` docs page with numeric and muted-disabled custom-class playground coverage.

- `ui-components`: adds `<SidebarInset>` as a Shadcn-compatible sidebar inset primitive with centralized side/padding/surface-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarInset` docs page with default inset-region and compact plain disabled playground coverage.

- `ui-components`: adds `<SidebarRail>` as a Shadcn-compatible sidebar rail primitive with controlled/uncontrolled open-state normalization, side-aware state contracts, and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarRail` docs page with default-rail and controlled right-rail playground coverage.

- `ui-components`: adds `<SidebarTrigger>` as a Shadcn-compatible sidebar trigger primitive with controlled/uncontrolled open-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarTrigger` docs page with default-trigger and controlled custom-label playground coverage.

- `ui-components`: adds `<SidebarContent>` as a Shadcn-compatible sidebar content region primitive with centralized padding/scroll/source-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarContent` docs page with default-scrollable and compact-static custom-class playground coverage.

- `ui-components`: adds `<SidebarFooter>` as a Shadcn-compatible sidebar footer region primitive with centralized border/disabled/source-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarFooter` docs page with bordered and disabled custom-class playground coverage.

- `ui-components`: adds `<SidebarHeader>` as a Shadcn-compatible sidebar header region primitive with centralized aria/disabled/source-state normalization and stable `data-*` markers.
- `apps/docs-app`: adds a `SidebarHeader` docs page with default and disabled custom-class playground coverage.

- `ui-components`: adds `<SidebarGroup>` as a Shadcn-compatible sidebar section primitive with label/action header regions, controlled/uncontrolled collapsible state, and Spectrum-style `data-*` markers.
- `apps/docs-app`: adds a `SidebarGroup` docs page with label+action and controlled collapsible-group playground coverage.

- `ui-components`: adds `<SidebarMenu>` as a Shadcn-compatible sidebar navigation menu with badges/actions/sub-items, controlled active-id state, collapsible submenu behavior, Spectrum-style `data-*` markers, and HeroUI-level active-highlight motion reuse.
- `apps/docs-app`: adds a `SidebarMenu` docs page with badge/action dispatch and controlled collapsible-submenu playground coverage.

- `ui-components`: adds `<Sidebar>` as a Shadcn-compatible navigation rail with controlled/uncontrolled open state, side/variant/collapsible contracts, keyboard shortcut toggling, and Spectrum-style `data-*` markers.
- `apps/docs-app`: adds a `Sidebar` docs page with offcanvas slot regions and controlled right-inset icon-collapse playground coverage.

- `ui-components`: adds `<Chart>` as a Shadcn-compatible chart primitive with bar/line modes, controlled/uncontrolled active-index flow, Spectrum-style `data-*` contracts, and HeroUI-level legend highlight motion.
- `apps/docs-app`: adds a `Chart` docs page with bar action-dispatch and controlled line-highlight playground coverage.

- `ui-components`: adds `<Resizable>` as a Shadcn-compatible panel splitter with controlled/uncontrolled split state, pointer+keyboard separator semantics, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `Resizable` docs page with horizontal grip and controlled-vertical bounded playground coverage.

- `ui-components`: adds `<CommandDialog>` as a Shadcn-compatible command overlay that composes `Modal` + `Command` with controllable open state, persistent-action mode, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `CommandDialog` docs page with controlled open/action-close and persistent-action playground coverage.

- `ui-components`: adds `<ScrollArea>` as a Shadcn-compatible scroll container with orientation/max-height/disabled normalization and stable `data-*` state contracts.
- `apps/docs-app`: adds a `ScrollArea` docs page with vertical list and horizontal/both-disabled playground coverage.

- `ui-components`: adds `<Toaster>` as a Shadcn-compatible host that composes `Sonner` with toaster-level position/portal/queue contracts and stable `data-*` state markers.
- `apps/docs-app`: adds a `Toaster` docs page with portal-host and inline top-center playground coverage.

- `ui-components`: adds `<Sonner>` as a Shadcn/HeroUI-compatible toast host that composes `ToastViewport` with position presets, queue limits, and stable `data-*` state contracts.
- `apps/docs-app`: adds a `Sonner` docs page with portal queue and inline top-center playground coverage.

- `ui-components`: adds `<Breadcrumb>` as a Shadcn-compatible singular breadcrumb primitive that wraps `<Breadcrumbs>` with identical props and Spectrum-style state data contracts.
- `apps/docs-app`: adds a `Breadcrumb` docs page and playground coverage without regressing existing `Breadcrumbs` docs.

- `ui-components`: adds `<ToggleGroup>` as a Shadcn-compatible grouped-toggle primitive with single/multiple selection modes, controllable selected-id sets, and Spectrum-style root `data-*` state contracts.
- `apps/docs-app`: adds a `ToggleGroup` docs page with multiple/attached and single/vertical/disabled-item playground coverage.

- `ui-components`: adds `<NativeSelect>` as a Spectrum-compatible native `<select>` primitive with controllable selected-index state, normalized options, and root `data-*` state contracts for styling/testing.
- `apps/docs-app`: adds a `NativeSelect` docs page with controlled placeholder and required/invalid/disabled playground coverage.

- `ui-components`: adds `<Carousel>` as a Shadcn-compatible carousel primitive with controllable slide index, orientation-aware keyboard navigation, Spectrum-style root `data-*` contracts, and HeroUI-level spring indicator-highlight motion reuse.
- `apps/docs-app`: adds a `Carousel` docs page with default indicator-motion and controlled vertical/no-loop playground coverage.

- `ui-components`: adds `<NavigationMenu>` as a Shadcn-compatible horizontal navigation primitive with roving keyboard focus, controllable selected-id state, Spectrum-style root `data-*` contracts, and HeroUI-level spring active-highlight motion reuse.
- `apps/docs-app`: adds a `NavigationMenu` docs page with default roving-selection and controlled-selection (`activate_on_focus=false`) playground coverage.

- `ui-components`: adds `<Menubar>` as a Shadcn-compatible persistent menubar with horizontal trigger navigation, controllable open-index state, Spectrum-style root `data-*` contracts, and HeroUI-level popover spring motion reuse.
- `apps/docs-app`: adds a `Menubar` docs page with desktop action-dispatch and controlled-open/persistent/disabled-menu playground coverage.

- `ui-components`: adds `<ContextMenu>` as a Shadcn-compatible context trigger menu with right-click + keyboard open semantics (`ContextMenu` / `Shift+F10`), Spectrum-style root `data-*` contracts, and HeroUI-level popover spring motion reuse.
- `apps/docs-app`: adds a `ContextMenu` docs page with right-click/keyboard-open and persistent-open/disabled-item playground coverage.

- `ui-components`: adds `<Command>` as a Shadcn-compatible command palette with grouped search/filtering, listbox keyboard semantics, and HeroUI-level spring active-highlight motion.
- `apps/docs-app`: adds a `Command` docs page with grouped keyboard-action and custom placeholder/empty-label playground coverage.

- `ui-components`: adds `<Toggle>` as a Shadcn-compatible toggle primitive with centralized pressed-state flow, Spectrum-style `data-*` contracts, and HeroUI-grade spring press motion reuse.
- `apps/docs-app`: adds a `Toggle` docs page with controlled and outline/ghost-disabled playground coverage.

- `ui-components`: adds `<Collapsible>` as a Shadcn-compatible disclosure primitive with HeroUI-level spring motion reuse, `ui-collapsible` state-class contracts, and Disclosure-composed semantics.
- `apps/docs-app`: adds a `Collapsible` docs page with controlled and disabled/custom-motion playground coverage.

- `ui-components`: adds `<Textarea>` as a Shadcn/HeroUI-compatible textarea primitive with Spectrum-style text-field semantics, stable `data-*` state contracts, and dedicated `ui-textarea` styling hooks.
- `apps/docs-app`: adds a `Textarea` docs page with basic and invalid-error playground coverage.

- `ui-components`: adds `<Toast>` as a Spectrum/HeroUI-style toast primitive with centralized title/description/class normalization, explicit open/closing state contracts, and spring-driven entry/exit motion hooks.
- `apps/docs-app`: adds a `Toast` docs page with basic dismiss and danger custom-motion playground coverage.

- `ui-components`: adds `<PressableFeedback>` as a Spectrum/HeroUI-style press feedback container with centralized effect/tone/boundary/source state derivation, spring-driven press/highlight motion, and optional ripple integration.
- `apps/docs-app`: adds a `PressableFeedback` docs page with highlight/scale and ripple/custom-motion playground coverage.

- `ui-components`: adds `<ErrorView>` as a Spectrum/HeroUI-style validation error container with centralized visibility/content/source state derivation, spring-driven motion contracts, and stable `slot` + `data-*` markers.
- `apps/docs-app`: adds an `ErrorView` docs page with invalid-visibility and custom-content/motion/actions playground coverage.

- `ui-components`: adds `<BottomSheet>` as a Spectrum/HeroUI-style bottom sheet primitive composed from `Sheet`, with centralized handle/description/footer/detached state derivation and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `BottomSheet` docs page with semantic action-footer and detached title-only playground coverage.

- `ui-components`: adds `<SkeletonGroup>` as a Spectrum/HeroUI-style skeleton coordination container with centralized loading/layout/variant visibility derivation and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `SkeletonGroup` docs page with shimmer/pulse layout and loaded/skeleton-only visibility playground coverage.

- `ui-components`: adds `<FormField>` as a Spectrum/HeroUI-style form-field primitive that composes switch/checkbox indicators with centralized tone/placement/message state derivation and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `FormField` docs page with switch-description and checkbox invalid/disabled playground coverage.

- `ui-components`: adds `<ColorSwatchPicker>` as a Spectrum-compatible selectable swatch group with centralized color normalization, single-selection state, keyboard roving, and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `ColorSwatchPicker` docs page with basic selection and transparency/disabled/custom-class playground coverage.

- `ui-components`: adds `<StepList>` as a Spectrum-compatible process-step primitive with centralized orientation/size/status normalization and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `StepList` docs page with controlled selection and vertical/emphasized/disabled-state playground coverage.

- `ui-components`: adds `<ColorField>` as a Spectrum-compatible color input primitive with centralized label/placeholder/aria/state normalization, sanitized preview rendering, and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `ColorField` docs page with controlled value and invalid/disabled/custom-class playground coverage.

- `ui-components`: adds `<ColorArea>` as a Spectrum-compatible two-axis color selection primitive with centralized step/grid normalization, keyboard navigation, and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `ColorArea` docs page with controlled grid selection and disabled/custom-grid/custom-class playground coverage.

- `ui-components`: adds `<ColorSlider>` as a Spectrum-compatible single-channel color slider with centralized channel/range/value normalization, spring motion integration, and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ColorSlider` docs page with controlled hue and disabled/custom-track/reduced-motion playground coverage.

- `ui-components`: adds `<ColorWheel>` as a Spectrum-compatible hue wheel with centralized value/step/wrap-around normalization, spring-driven thumb motion, and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ColorWheel` docs page with controlled hue and disabled/reduced-motion/custom-class playground coverage.

- `ui-components`: adds `<ColorPicker>` as a Spectrum-compatible swatch-trigger + popover editor primitive with controllable color/open state and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `ColorPicker` docs page with controlled color/open and disabled/default-open/custom-class playground coverage.

- `ui-components`: adds `<ColorThumb>` as a Spectrum-compatible draggable thumb primitive with centralized focus/drag/loupe/position normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ColorThumb` docs page with focused/dragging positions and disabled/loupe-off/custom-class playground coverage.

- `ui-components`: adds `<ColorEditor>` as a Spectrum-compatible color composition primitive with centralized HSB channel normalization, format-state modeling, and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `ColorEditor` docs page with controlled color/format and disabled/alpha-hidden/reduced-motion playground coverage.

- `ui-components`: adds `<ColorHandle>` as a Spectrum-compatible draggable handle primitive with composed thumb/loupe behavior, centralized state derivation, and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `ColorHandle` docs page with focused/dragging positions and disabled/loupe-off/custom-class playground coverage.

- `ui-components`: adds `<ColorLoupe>` as a Spectrum-compatible color loupe overlay primitive with centralized open/disabled/position normalization, checkerboard alpha preview, and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ColorLoupe` docs page with open-position buckets and disabled/custom-label/custom-class playground coverage.

- `ui-components`: adds `<Underlay>` as a Spectrum-compatible full-viewport underlay primitive with centralized open/transparent/disabled state derivation, close-interaction contracts, and stable `slot` + `data-*` state markers.
- `apps/docs-app`: adds an `Underlay` docs page with scrim-dismiss and transparent/disabled/custom-class playground coverage.

- `ui-components`: adds `<Tray>` as a Spectrum-compatible bottom tray primitive composed from `Sheet`, with centralized description/footer/close/height contracts and stable `slot` + `data-*` state markers.
- `apps/docs-app`: adds a `Tray` docs page with semantic footer actions and fixed-height/title-only/custom-class playground coverage.

- `ui-components`: adds `<FieldGroup>` as a Spectrum/HeroUI-compatible field clustering primitive with centralized orientation/density/aria/class state derivation and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `FieldGroup` docs page with vertical labeled grouping and horizontal compact invalid/disabled playground coverage.

- `ui-components`: adds `<CheckboxField>` as a Spectrum/HeroUI-style checkbox-field primitive with centralized tone/indicator/aria/class state derivation and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `CheckboxField` docs page with controlled description and indicator-end quiet invalid/disabled playground coverage.

- `ui-components`: adds `<Legend>` as a Spectrum/HeroUI-compatible fieldset legend primitive with centralized tone/required/disabled state derivation and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds a `Legend` docs page with required semantics and tone/custom-indicator/disabled playground coverage.

- `ui-components`: adds `<ColorSwatch>` as a Spectrum-compatible color preview primitive with centralized size/rounding/shape/transparency/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ColorSwatch` docs page with size/rounding and transparency/accessible-label/shape playground coverage.

- `ui-components`: adds `<AspectRatio>` as a Shadcn/HeroUI-compatible media-frame primitive with centralized ratio/radius/frame/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds an `AspectRatio` docs page with ratio presets and bordered/fill/custom-aria playground coverage.

- `ui-components`: adds `<Icon>` as a Spectrum-compatible glyph primitive with centralized size/tone/accessibility/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds an `Icon` docs page with size/tone matrix and accessible/disabled/custom-class playground coverage.

- `ui-components`: aligns `<TagGroup>` to compose `<Tag>` primitives (instead of `<Chip>`), and unifies variant/size contracts to `TagVariant` + `TagSize` for better HeroUI/Spectrum parity.
- `apps/docs-app`: updates `TagGroup` docs wording/examples to reflect Tag-based composition semantics.

- `ui-components`: adds `<Tag>` as a Spectrum/HeroUI-style tag primitive with centralized variant/size/remove-action/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `Tag` docs page with variant/size matrix and removable/disabled/custom-class playground coverage.

- `ui-components`: adds `<DateInputGroup>` as a Spectrum/HeroUI-style date-input grouping primitive with centralized variant/width/prefix-suffix/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `DateInputGroup` docs page with DateField composition and secondary/full-width/invalid TimeField playground coverage.

- `ui-components`: adds `<ListBoxSection>` as a Spectrum/HeroUI-style listbox grouping primitive with centralized heading/item/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ListBoxSection` docs page with default and quiet/sticky/divider/empty playground coverage.

- `ui-components`: adds `<ListBoxItem>` as a Spectrum/HeroUI-style listbox option primitive with centralized selection/focus/divider/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `ListBoxItem` docs page with selectable and focused/divider/disabled playground coverage.

- `ui-components`: adds `<MenuSection>` as a Spectrum/HeroUI-style menu grouping primitive with centralized heading/item/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `MenuSection` docs page with default and quiet/sticky/divider/empty playground coverage.

- `ui-components`: adds `<MenuItem>` as a Spectrum/HeroUI-style menu row primitive with centralized kind/checked/focus/source normalization and stable `slot` + `data-*` state contracts.
- `apps/docs-app`: adds a `MenuItem` docs page with action/checkbox and radio/submenu/disabled playground coverage.

- `ui-components`: adds `<Dropdown>` as a Spectrum/HeroUI-style trigger-driven menu primitive with centralized state/source normalization and `MenuTrigger` composition contracts.
- `apps/docs-app`: adds a `Dropdown` docs page with default and controlled/persistent/disabled-item playground coverage.

- `ui-components`: adds `<Surface>` as a Spectrum/HeroUI-style foundational container primitive with centralized tone/elevation/frame/source normalization and stable `data-*` state contracts.
- `apps/docs-app`: adds a `Surface` docs page with tone/elevation/frame and custom-aria/custom-class playground coverage.

- `ui-components`: adds `<DisclosureGroup>` as a Spectrum/HeroUI-style grouped disclosure primitive with centralized expanded-state normalization, controlled/uncontrolled contracts, and spring motion delegated through Accordion internals.
- `apps/docs-app`: adds a `DisclosureGroup` docs page with multiple-controlled and single-disabled/custom-class playground coverage.

- `ui-components`: adds `<SwitchGroup>` as a Spectrum/HeroUI-style grouped switch primitive with centralized orientation/tone/validation/message-state normalization and stable `data-*` contracts.
- `apps/docs-app`: adds a `SwitchGroup` docs page with required/description and horizontal-invalid-disabled/custom-class playground coverage.

- `ui-components`: adds `<EmptyState>` as a Spectrum/HeroUI-style empty-state primitive with centralized tone/align/layout/source normalization and stable `data-*` state contracts.
- `apps/docs-app`: adds an `EmptyState` docs page with tone/alignment/actions and compact/bordered/custom-class playground coverage.

- `ui-components`: adds `<ErrorMessage>` as a Spectrum/HeroUI-style inline error primitive with centralized tone/disabled/truncate/source normalization and stable `slot` + `data-*` contracts.
- `apps/docs-app`: adds an `ErrorMessage` docs page with tone variants and truncate/disabled/element/custom-class playground coverage.

- `ui-components`: adds `<FieldError>` as a Spectrum/HeroUI-style field validation primitive with centralized visibility/tone/message normalization and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `FieldError` docs page with visible/tone and hidden/disabled/custom-class playground coverage.

- `ui-components`: adds `<Description>` as a Spectrum/HeroUI-style form helper primitive with centralized tone/disabled/truncate normalization and stable `slot` + `data-*` state/source contracts.
- `apps/docs-app`: adds a `Description` docs page with tone variants and truncate/element/disabled playground coverage.

- `ui-components`: adds `<Fieldset>` as a Spectrum/HeroUI-style group primitive with centralized orientation/tone/validation/message/action-state normalization and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `Fieldset` docs page with legend/description and horizontal-invalid/actions playground coverage.

- `ui-components`: adds `<ClearButton>` as a Spectrum-style clear affordance with centralized variant/inset/focus-mode normalization, headless press/hover/focus integration, and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `ClearButton` docs page with default/over-background and inset/focus-mode/disabled playground coverage.

- `ui-components`: adds `<CloseButton>` as a Spectrum/HeroUI-style close affordance with centralized variant/size normalization, default icon fallback, and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `CloseButton` docs page with default/over-background/custom-label and size/disabled/custom-class playground coverage.

- `ui-components`: adds `<LogicButton>` as a Spectrum-style boolean operator primitive with centralized and/or variant normalization, headless press/hover/focus integration, and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `LogicButton` docs page with and/or variants and custom-class/disabled playground coverage.

- `ui-components`: adds `<FieldButton>` as a Spectrum-style field-trigger primitive with centralized quiet/invalid/active/disabled normalization, headless press/hover/focus integration, and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `FieldButton` docs page with default/quiet and invalid-active/disabled playground coverage.

- `ui-components`: adds `<HelpText>` as a Spectrum-style assistance primitive with centralized description/error resolution, tone/icon state derivation, and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `HelpText` docs page with description and invalid/icon/disabled error playground coverage.

- `ui-components`: adds `<Field>` as a Spectrum-style form field wrapper with centralized orientation/tone/validation/message-state normalization and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `Field` docs page with required/description and horizontal-invalid custom-class playground coverage.

- `ui-components`: adds `<Grid>` as a Spectrum-style layout primitive with centralized columns/rows/gap/alignment normalization and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `Grid` docs page with columns/gap and auto-fit/dense/equal-rows playground coverage.

- `ui-components`: adds `<Flex>` as a Spectrum-style layout primitive with centralized direction/wrap/justify/align/gap normalization and stable `data-*` state/source contracts.
- `apps/docs-app`: adds a `Flex` docs page with direction/wrap/gap and inline/distribution playground coverage.

- `ui-components`: adds `<Keyboard>` as a Spectrum-style keyboard command primitive (`<kbd>`) with centralized tone/compact normalization and stable `data-*` source/state contracts.
- `apps/docs-app`: adds a `Keyboard` docs page with tone and compact/custom-class playground coverage.
- `ui-components`: adds `<Heading>` as a Spectrum-style semantic heading primitive (`h1`..`h6`) with centralized level/tone/truncate normalization and stable `data-*` source/state contracts.
- `apps/docs-app`: adds a `Heading` docs page with level/tone and truncate/custom-class playground coverage.
- `ui-components`: adds `<Footer>` as a Spectrum-style semantic footer primitive with centralized tone/border normalization and stable `data-*` source/state contracts.
- `apps/docs-app`: adds a `Footer` docs page with semantic-tone and bordered-container playground coverage.
- `ui-components`: adds `<Header>` as a Spectrum-style semantic header primitive with centralized tone/border normalization and stable `data-*` source/state contracts.
- `apps/docs-app`: adds a `Header` docs page with semantic-tone and bordered-container playground coverage.
- `ui-components`: adds `<Content>` as a Spectrum-style semantic section primitive with centralized tone/padding normalization and stable `data-*` source/state contracts.
- `apps/docs-app`: adds a `Content` docs page with semantic-tone and padded-custom-class playground coverage.
- `ui-components`: adds `<View>` with centralized surface token normalization (`background/border/padding/radius/shadow/element`) and Spectrum-style `data-*` state/source contracts.
- `apps/docs-app`: adds a `View` docs page with surface-token matrix and element/fluid/custom-class playground coverage.
- `ui-components`: adds `<Tree>` with centralized hierarchy normalization, controllable expand/selection state, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `Tree` docs page with expanded-root and strong compact playground coverage.
- `ui-components`: adds `<TimeField>` with centralized hour/minute normalization, controllable value flow, and Spectrum-style `data-*` state/source contracts.
- `apps/docs-app`: adds a `TimeField` docs page with controlled step and strong-tone custom-placeholder playground coverage.
- `ui-components`: adds `<DateRangePicker>` with dual-DatePicker composition, centralized range validity/value-shape derivation, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `DateRangePicker` docs page with controlled shared-month and invalid-range hint playground coverage.
- `ui-components`: adds `<DateField>` with centralized year/month/day normalization, controllable value flow, and Spectrum-style `data-*` state/source contracts.
- `apps/docs-app`: adds a `DateField` docs page with controlled value and strong-tone custom-placeholder playground coverage.
- `ui-components`: adds `<ActionGroup>` with normalized item/selection state, controllable selection flow, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds an `ActionGroup` docs page with single/multiple selection and action callback playground coverage.
- `ui-components`: adds `<Text>` with centralized tone/align/weight normalization and Spectrum-style `data-*` state/source contracts.
- `apps/docs-app`: adds a `Text` docs page with tone/weight and alignment/truncate playground coverage.
- `ui-components`: adds `<DatePicker>` with centralized open/value controllable state, popover-calendar composition, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `DatePicker` docs page with outside-days and Monday-first strong-tone playground coverage.
- `ui-components`: adds `<Calendar>` with centralized Gregorian month-grid derivation, weekday policy normalization, and Spectrum-style `data-*` state contracts.
- `apps/docs-app`: adds a `Calendar` docs page with outside-days and Monday-first tone/state playground coverage.
- `ui-components`: adds `<Table>` with centralized column/row normalization, density/layout/variant state derivation, and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `Table` docs page with striped/default and compact/fixed empty-state playground coverage.
- `ui-components`: adds `<LabeledValue>` with centralized label/value/description/source state derivation and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `LabeledValue` docs page with orientation/tone and custom aria/class playground coverage.
- `ui-components`: `MotionRipple` now uses centralized phase/boundary/motion/class state derivation, emits stable Spectrum-style `data-*` contracts, supports bounded/unbounded rendering, and adds origin-aware ripple triggers with motion sanitization.
- `apps/docs-app`: `MotionRipple` docs now provide Animation Matrix and Custom Boundary + Class playgrounds covering default/slow/disabled and unbounded origin-triggered scenarios.
- `ui-components`: adds `<Slider>` with centralized bounds/step/value state derivation, Spectrum-style `data-*` contracts, and spring-driven motion for visual progress updates.
- `apps/docs-app`: adds a `Slider` docs page with controlled/on-change and disabled/reduced-motion playground scenarios.
- `ui-components`: adds `<InputGroup>` to compose inputs with shared addons, attachment/disabled/invalid state markers, and centralized aria/class state derivation.
- `apps/docs-app`: adds an `InputGroup` docs page with attached and detached playground scenarios.
- `ui-components`: adds `<ActionBar>` with centralized selection/visibility/label state derivation, Spectrum-style `data-*` contracts, and spring-driven visibility motion.
- `apps/docs-app`: adds an `ActionBar` docs page with selection controls, clear-action flow, top/bottom placement, and reduced-motion playground scenarios.
- `ui-components`: adds `<Well>` with centralized tone/density/inset/label state derivation and Spectrum-style `data-*` contracts for stable styling hooks.
- `apps/docs-app`: adds a `Well` docs page with tone+density matrix and custom label/class playground coverage.
- `ui-components`: adds `<Label>` with centralized emphasis/required/disabled/source state derivation and Spectrum-style `data-*` contracts.
- `apps/docs-app`: adds a `Label` docs page with emphasis/required matrix and custom indicator/class playground coverage.

- Cargo workspace scaffold with layered crates (`ui-core`, `ui-headless`, `ui-theme`, `ui-components`) and demo apps (`web-demo`, `tauri-demo`).
- `ui-core`: initial headless state primitive `use_toggle_state` with unit tests.
- `ui-core`: `use_overlay_trigger_state` (open/close/toggle + controlled/uncontrolled) with unit tests.
- `ui-core`: `use_controlled_state` helper (value/default + on_change pattern) for building controlled/uncontrolled primitives.
- `ui-core`: `use_single_selection_state` / `use_multiple_selection_state` for selection modeling.
- `ui-core`: `use_list_state` (items + selection) for list-based components.
- `ui-theme`: design tokens + CSS variable emitter, plus base/safe-area CSS helpers and unit tests.
- `ui-theme`: adds a dark theme preset and additional color tokens (`fg-muted`, `bg-muted`, `accent-soft`).
- `ui-theme`: switches preset colors to `oklch(...)` and adds an OLED theme preset (`Theme::oled()`).
- `ui-theme`: adds danger tokens (`--ui-danger`, `--ui-danger-fg`) for destructive UI.
- `ui-headless`: initial interaction primitives (focus-visible modality, press handling, button behavior) with `web`/`ssr` feature gating.
- `ui-headless`: `use_hover` (hover state + handlers).
- `ui-headless`: `use_focus_within` (container focus tracking).
- `ui-headless`: `use_roving_tabindex` (roving tabindex state + handlers).
- `ui-headless`: `use_listbox` (aria-activedescendant listbox semantics).
- `ui-headless`: `use_menu` (menu semantics + aria-activedescendant + keyboard navigation/activation).
- `ui-headless`: `use_menu` / `use_listbox` typeahead navigation when `item_text` is provided.
- `ui-headless`: per-item disabled support for roving navigation, activation, and typeahead (`is_item_disabled`).
- `ui-headless`: `use_menu_item` (Action/Checkbox/Radio roles + `aria-checked` + per-item handlers).
- `ui-headless`: `use_modal` (scroll lock + `aria-hidden` on non-portal content) for modal overlays.
- `ui-headless`: `use_popover_position` (anchor-rect positioning; fixed layout).
- `ui-components`: initial `<Button>` component integrating headless behavior + theme tokens.
- `ui-components`: `ListBox` (v0 demo component built on `use_listbox`).
- `ui-components`: `Popover` (v0 positioned popover using headless positioning + overlay stack + focus trap).
- `ui-components`: `Menu` / `MenuTrigger` (v0 popover-based menu composition).
- `ui-components`: `Select` (v0 Button + Popover + ListBox composition).
- `ui-components`: `Menu` / `ListBox` / `Select` provide `item_text` to enable typeahead navigation.
- `ui-components`: `Menu` / `MenuTrigger` / `ListBox` / `Select` support per-item disabled via `disabled_indices`.
- `ui-components`: `Menu` / `MenuTrigger` support checkbox/radio menu items via `item_kinds`, plus `close_on_action` for checkbox-style menus.
- `ui-components`: `Select` trigger now supports React Spectrum-style keyboard navigation when closed (ArrowLeft/ArrowRight + typeahead), and focus strategies when opening (ArrowDown/ArrowUp/Enter/Space).
- `ui-headless`: `use_checkbox` / `use_switch` (role + `aria-checked` + keyboard press handling via `PressActivationKeys`).
- `ui-components`: `<Checkbox>` / `<Switch>` components (built on headless press + focus ring).
- `ui-headless`: overlay primitives (topmost overlay stack + focus trap with focus restore).
- `ui-headless`: `use_focus_ring` hook for per-component focus ring handling.
- `ui-headless`: `use_tooltip_trigger` (Spectrum-style delay + warmup/cooldown + hover/focus-visible) and `use_tooltip_position` (anchor-rect positioning) with unit tests.
- `ui-components`: initial `<Overlay>` (portal + Esc/topmost + click-outside + focus trap).
- `ui-components`: `<Overlay>` now calls `use_modal`; `<Overlay>`/`<Popover>` mark portal content via `data-ui-overlay-portal` (used for `aria-hidden` exclusions).
- `ui-components`: `<Overlay>` now supports `aria-labelledby` / `aria-describedby`, and adds a `<Modal>` composition component.
- `ui-components`: adds `<UiRoot>` to inject theme CSS variables + base styles (including safe-area support).
- `apps/web-demo`: minimal Leptos CSR demo showcasing the initial primitives (Button + Modal/Overlay).
- `apps/web-demo`: adds MenuTrigger demo section (open/navigate/select).
- `apps/web-demo`: adds Select demo section (Button -> Popover -> ListBox).
- `apps/web-demo`: adds Checkbox/Switch demo section (Tab focus + Space toggle + focus-visible).
- `apps/web-demo`: adds a light/dark theme toggle powered by `<UiRoot>`.
- `apps/web-demo`: theme toggle now cycles Light/Dark/OLED.
- `apps/web-demo`: showcases bb-style Button variants and sizes.
- `apps/web-demo`: demonstrates per-item disabled options in Menu/ListBox/Select.
- `apps/web-demo`: demonstrates checkbox/radio menu items (role + aria-checked + stays-open behavior).
- `apps/web-demo`: Trunk entrypoint (`index.html`) and run instructions (`README.md`).
- `apps/tauri-demo`: Tauri v2 shell scaffold (config + build script + minimal command) for desktop verification.
- Dev tooling: `githooks/` (Conventional Commits + pre-commit gates) with `scripts/setup-githooks.sh`, plus gate runner scripts (`scripts/gate.sh`, `scripts/check.sh`) and `scripts/fetch_upstream.sh` for cloning upstream reference repos into `examples/` (ignored by git).
- Dev tooling: `scripts/fetch_upstream.sh` now also clones motion/heroui/shadcn-ui/animate-ui into `examples/_upstream` (ignored by git).
- Dev tooling: `scripts/dev-web-demo.sh` to run the web demo with sane defaults (unsets `NO_COLOR`, ensures wasm target/tooling).
- Dev tooling: adds Playwright-based WASM startup smoke scripts (`scripts/smoke-web-demo.sh`, `scripts/smoke-docs-app.sh`) to catch blank-screen regressions.
- Project docs: MVP/spec notes and a TODO/DAG-based implementation plan.
- Research: Android spike checklist and go/no-go criteria (`docs/research/android-spike.md`).
- Research: notes mapping `bb/packages/ui-web` architecture/colors/docs practices into this repo (`docs/research/bb_ui-web_notes.md`).
- Docs: add a consolidated rulebook covering layering, theming (OKLCH/OLED), motion, and workflow (`docs/RULES_ZH.md`).
- `ui-motion`: new crate providing native motion primitives (WAAPI-based on `wasm32`) with reduced-motion detection.
- `ui-motion`: adds a physics-based spring runtime (`SpringAnimator`) for Framer/HeroUI-style micro-interactions.
- `ui-components`: button styles moved into a dedicated style module and injected via `<UiRoot>`.
- `ui-components`: button motion interface reserved via `ButtonMotion` (no CSS transitions).
- `ui-components`: `Button` hover/tap scale now defaults to a spring (bb defaults: 1.05 / 0.95).
- `ui-components`: `Button` API aligned to bb-style variants/sizes.
- `ui-components`: refactors `<Checkbox>` / `<Switch>` into `logic/styles/motion/view` modules and adds spring-driven micro-interactions.
- `ui-motion`: adds spring presets (`ui_motion::presets`) and `SpringAnimator` rest callbacks (`set_on_rest`/`clear_on_rest`) for presence-style flows.
- `ui-components`: refactors `<Overlay>` / `<Popover>` / `<Modal>` into `logic/styles/motion/view` modules and adds spring enter/exit motion with `on_exit_complete` for presence.
- `ui-components`: adds spring-driven active highlight motion for `<ListBox>` / `<Menu>` / `<Select>` (HeroUI-style feel).
- `ui-components`: `Select` now composes `ListBox` and uses popover presence internally.
- `ui-components`: adds `aria_haspopup` / `aria_expanded` / `aria_controls` support on `Button` for trigger-style components.
- `ui-headless`: adds integration tests covering key hooks (hover, focus ring, press/button, checkbox/switch, listbox/menu, overlay stack).
- Docs: adds per-crate `README.md` files describing responsibilities and usage (`ui-core`, `ui-headless`, `ui-theme`, `ui-motion`, `ui-components`).
- CI: adds a GitHub Actions workflow that runs `cargo fmt`, `cargo clippy`, and `cargo test`.
- `apps/web-demo`: adds `dev-overrides.css` as a hot-reload-friendly place to prototype component style changes.
- `ui-headless`: adds `use_text_field` hook (+ unit tests) for wiring input `aria-describedby`/`aria-invalid`/`aria-required`.
- `ui-headless`: expands hook test coverage (focus-within, press cancel/blur, listbox selection sync, menu item roles).
- `ui-headless`: adds `use_hover_card_trigger` hook (hover/focus-within + open/close delays + dismiss) with unit tests.
- `ui-components`: adds new components: `IconButton`, `Badge`, `CircularProgress`, `TextField`, `Tabs`, and `Tooltip`.
- `ui-components`: adds tests enforcing styling rules (`style:` forbidden; `style=` allowlist) and validating CSS aggregation.
- `apps/web-demo`: adds demo sections for Badge/Spinner, Tooltip, Tabs, and TextField.
- `ui-motion`: adds reduced-motion unit tests for `SpringAnimator` (immediate apply + on-rest callbacks).
- `ui-headless`: adds `use_combo_box` and `use_radio_group` hooks with unit tests (combobox + radiogroup semantics).
- `ui-components`: adds new components: `Avatar`, `Divider`, `TextArea`, `RadioGroup`/`Radio`, and `ComboBox`.
- `apps/web-demo`: adds demo sections for Avatar, ComboBox, TextArea, RadioGroup, and Divider.
- `apps/docs-app`: adds a Leptos CSR docs site with a minimal playground (preview + code) and a rendered `docs/RULES_ZH.md` page.
- `ui-components`: adds new components: `Card`, `Alert`, `Chip`, `AvatarGroup`, `Skeleton`, and `Spinner`.
- `apps/web-demo`: adds a misc demo section covering Card/Alert/Chip/Skeleton/AvatarGroup/Spinner.
- `ui-components`: adds utility components: `Link`, `Breadcrumbs`, `Code`, `Kbd`, `Spacer`, and `ProgressBar`.
- `apps/web-demo`: adds a typography/utilities demo section for Link/Breadcrumbs/Code/Kbd/Spacer/ProgressBar.
- `ui-components`: adds new components: `ButtonGroup`, `CheckboxGroup`, `LinkButton`, `SearchField`, `Snippet`, and `StatusLight`.
- `apps/web-demo`: adds demo sections for ButtonGroup/CheckboxGroup/LinkButton/SearchField/Snippet/StatusLight.
- `ui-components`: adds new components: `Accordion`, `Disclosure`, `Pagination`, `TagGroup`, `ToggleButton`, and `ToggleButtonGroup`.
- `apps/web-demo`: adds demo sections for Pagination/TagGroup and Disclosure/Accordion; Button demo now includes ToggleButtonGroup.
- `ui-components`: adds new components: `InputOtp`, `NumberField`, `ScrollShadow`, `SegmentedControl`, `Sheet`, and `Meter`.
- `ui-headless`: adds `use_input_otp` (digits-only normalization, caret tracking, and `on_complete`).
- `ui-components`: `<InputOtp>` now renders a single hidden input with HeroUI-style slot chrome/caret and integrates Spectrum-style field semantics (required/invalid/description/error).
- `apps/web-demo`: adds a demo section for the new components (inputs + overlays).
- `ui-components`: adds new components: `Image`, `IllustratedMessage`, `FileTrigger`, `DropZone`, `HoverCard`, and `Toast`.
- `apps/web-demo`: adds an extras demo section (hover card + toasts + file picking + drag/drop + image + illustrated message).
- `ui-components`: adds new components: `Input`, `InlineAlert`, `Dialog`, `Drawer`, `AlertDialog`, and `ContextualHelp`.
- `apps/web-demo`: adds a demo section for dialogs/alerts and contextual help.
- `ui-components`: ports bb/ui-web components: `Form`, `Autocomplete`, `DropdownMenu`, `CodeBlock`, `ButtonCopy`, and `ThemeToggleButton`.
- `apps/web-demo`: adds a ports demo section showcasing the newly added components.
- `ui-components`: ports bb/ui-web components: `ActionButton`, `ActionButtonGroup`, `ActionMenu`, `SearchInputButton`, `FlipButton`, and `ShareButton`.
- `apps/web-demo`: extends the ports demo with action/share/search/flip components.
- `ui-components`: adds new components: `Separator`, `AutoHeight`, `Progress`, `ProgressCircle`, `MotionRipple`, `StaticNumber`, and `SlidingNumber`.
- `apps/web-demo`: extends the new components demo with separators, progress indicators, ripple surface, auto-height, and number formatting.

- `ui-components`: `RadioGroup` now resolves accessible names (`aria-label` / `aria-labelledby` + fallback), exposes Spectrum-style orientation/state `data-*` attributes, and hardens empty-option labeling semantics.
- `apps/docs-app`: `RadioGroup` docs now include playground scenarios for horizontal layout, disabled items, external labeling, and empty disabled groups.
- `ui-components`: `Select` now centralizes trigger disabled semantics (`disabled || items.is_empty()`), emits Spectrum-style root `data-*` state attrs, and marks its popover panel slot for stable styling/tests.
- `apps/docs-app`: `Select` docs now include playground scenarios for disabled options, fully disabled state, and empty-option behavior.
- `ui-components`: `ComboBox` now normalizes label/placeholder/message text, exposes option focus + empty-result slots/state attrs, and aligns root `data-empty` to filtered results.
- `apps/docs-app`: `ComboBox` docs now include validation toggling, disabled collection, and empty collection playground scenarios.
- `ui-components`: `Autocomplete` now normalizes label/placeholder/message text, exposes option focus + empty-result slots/state attrs, and aligns root `data-empty` to filtered results.
- `apps/docs-app`: `Autocomplete` docs now include validation toggling, disabled collection, and empty collection playground scenarios.
### Changed

- `ui-components`: `SlidingNumber` now snapshots decimal/thousand separators before reactive formatting derivation, fixing moved-value ownership errors during native/wasm compilation.
- `apps/web-demo`: replaced the full `<App />` construction unit test with a deterministic `DemoTheme` cycle test, removing stack-overflow failures from workspace test gates.
- `apps/docs-app`: rebuilds the docs site into a hash-routed component catalog (search + per-component playground pages) and adds a regression test that enforces 100% `ui-components` component coverage.
- `apps/docs-app`: add a Cmd/Ctrl+K command menu (SearchInputButton trigger + Dialog results), and switch the header theme toggle to `<ThemeToggleButton>`.
- `apps/docs-app`: command menu now uses a unified fuzzy search index (docs + doc sections + components).
- `apps/docs-app`: command menu search now indexes markdown body text and displays snippets; route changes now restore scroll position and update the document title.
- `apps/docs-app`: add a regression test enforcing that every component doc page contains at least one `<Playground>` section.
- `apps/docs-app`: the components index now supports group filtering (SegmentedControl) and the playground coverage test now enforces that each component page actually demos the component.
- `apps/docs-app`: add an additional catalog guard that checks every public `ui-components` module is represented in docs (with explicit alias mapping for renamed docs slugs).
- `apps/docs-app`: `Playground` now supports an optional controls sidebar (Spectrum-style prop knobs); Button docs page uses it as the reference pattern.
- `apps/docs-app`: adds an "On this page" TOC panel + deep-linkable sections (`section=`) and a mobile nav sheet.
- `apps/docs-app`: markdown doc headings now expose hoverable anchor buttons for deep-linking.
- `apps/docs-app`: renders project docs (起点 + spec + research markdown files) as first-class pages and adds a regression test for required doc routes.
- `apps/web-demo`: log Rust panics to the browser console (`console_error_panic_hook`) to reduce silent white-screen failures.
- `apps/docs-app`: log Rust panics to the browser console (`console_error_panic_hook`) to reduce silent white-screen failures.
- `apps/web-demo`: the loading overlay now captures `console.error` output so Rust panics show up in-page (not only in devtools).
- `apps/docs-app`: the loading overlay now captures `console.error` output so Rust panics show up in-page (not only in devtools).
- `apps/web-demo`/`apps/docs-app`: the loading overlay now preserves multiple error sources (console + runtime) instead of overwriting the details panel.
- `apps/web-demo`/`apps/docs-app`: CSR dev/smoke scripts now force `cfg(erase_components)` (and the apps fail fast without it) to avoid Tachys attribute tuple limit crashes.
- `apps/web-demo`: remove an intentionally invalid avatar image URL that produced `net::ERR_INVALID_URL` noise in the console.
- `apps/web-demo`: add a smoke test that constructs the demo `<App />` under a Leptos `Owner` to catch panics/regressions early.
- `ui-components`: `<UiRoot>` now supports disabling built-in component CSS injection by turning off the default `inject-css` feature (for apps that manage CSS separately).
- `ui-components`: adds ActionButton semantics regression tests (headless hooks + Spectrum-style state attributes + motion contract).
- `ui-components`: adds ActionButtonGroup semantics regression tests (toolbar semantics + context propagation contract).
- `ui-components`: adds ActionMenu semantics regression tests (Popover composition + overlay trigger ARIA contract).
- `ui-components`: adds SearchInputButton semantics regression tests (headless hooks + Spectrum-style state attributes + motion contract).
- `ui-components`: adds Dialog semantics regression tests (Overlay labeling contract + close button accessibility).
- `ui-components`: `<AlertDialog>` now supports secondary/cancel actions (close-before-callback), per-action disabled flags, auto-focus button selection, warning/error variants with icons, and adds a semantics regression test.
- `ui-components`: `<Accordion>` now emits Spectrum-style trigger state attributes (`data-open`, `data-focus-visible`, etc.) and adds a semantics regression test.
- `ui-components`: `SlidingNumber` now animates per-digit transitions via spring-driven CSS variables (animate-ui-style).
- `ui-components`: `CodeBlock` now provides spring-driven copy feedback via CSS variables (`--ui-code-block-copy-flash`).
- `ui-components`: `ThemeToggleButton` now animates icon micro-interactions via spring-driven CSS variables (`--ui-theme-toggle-*`).
- `ui-components`: `MotionRipple` no longer accepts an unused `motion` prop; configure ripple motion via `ripple::motion::trigger_ripple` instead.
- `ui-components`: `Form` no longer exposes a placeholder motion contract.
- `ui-components`: `SearchField` now clears on Escape (when non-empty), stops Escape propagation when clearing (so parent overlays don't dismiss), excludes its clear button from the tab order, adds spring-driven clear-button motion, and disables its control transitions when `prefers-reduced-motion` is enabled.
- `ui-components`: `InputOtp` caret blink now respects `prefers-reduced-motion` (disables the animation when reduced motion is enabled).
- `ui-components`: `Checkbox` now disables its color transitions when `prefers-reduced-motion` is enabled.
- `ui-components`: `Input` now clears on Escape when clearable, stops Escape propagation when clearing (so parent overlays don't dismiss), excludes its clear button from the tab order, and adds spring-driven clear-button reveal motion via CSS variables.
- `ui-components`: `TextField` now supports `read_only` via the native `readonly` attribute.
- `ui-components`: `TextField` now disables its transitions when `prefers-reduced-motion` is enabled.
- `ui-components`: `TextArea` now supports `read_only` via the native `readonly` attribute.
- `ui-components`: `TextArea` now disables its transitions when `prefers-reduced-motion` is enabled.
- `ui-components`: `<Meter>` now sets `role="meter progressbar"` to match React Spectrum's cross-browser fallback behavior, and adds a semantics regression test.
- `ui-components`: `<Progress>` now sets `aria-valuetext` for determinate values (and treats non-finite inputs as min) to match React Spectrum `useProgressBar` semantics.
- `ui-components`: `<ProgressCircle>` now sets `aria-valuetext` for determinate values (and treats non-finite inputs as min) to match React Spectrum `useProgressBar` semantics.
- `ui-components`: `<CircularProgress>` now uses `role="progressbar"` (indeterminate) and sanitizes custom size/thickness CSS variables.
- `ui-headless`: adds `use_number_field` (spinbutton semantics + raw input editing + keyboard stepping) and upgrades `<NumberField>` to match React Spectrum behavior more closely.
- `ui-headless`: callback/handler types now use Leptos `Callback` (Send+Sync) to support rendering inside `Portal`.
- `ui-headless`: `use_listbox` now supports `default_index` and optional `sync_active_index_to_selected` to enable Select focus strategies without mutating selection.
- `ui-headless`: roving tabindex now clamps disabled indices to the nearest enabled option (preferring previous when possible).
- `ui-headless`: `use_combo_box` keyboard behavior now aligns better with React Spectrum (ArrowDown/ArrowUp open first/last; Enter/Tab commit; Space no longer commits).
- `ui-headless`: `use_combo_box` now takes `is_open: Signal<bool>` + `set_open: Callback<bool>` to support controlled open state.
- `ui-components`: `ComboBox` popup panel now renders in a Portal and is positioned via `use_popover_position` (flip/clamp), keeping the menu aligned to the field control and avoiding overflow clipping.
- `ui-components`: `ComboBox` now supports `open/default_open/on_open_change`, labels its listbox via `aria-labelledby` (label `id`), and adds `ComboBoxMotion` (popover + highlight).
- `ui-components`: `ComboBox` now stops Escape propagation while open, preventing parent overlays from closing (React Spectrum parity), and adds a semantics regression test.
- `ui-components`: `Autocomplete` popup panel now renders in a Portal and is positioned via `use_popover_position` (flip/clamp), keeping the menu aligned to the field control and avoiding overflow clipping.
- `ui-components`: `Autocomplete` now supports `open/default_open/on_open_change` and labels its listbox via `aria-labelledby` (label `id`).
- `ui-components`: `Autocomplete` now stops Escape propagation while open, preventing parent overlays from closing (React Spectrum parity), and adds a semantics regression test.
- `ui-components`: `Disclosure` now supports `open/default_open/on_open_change`.
- `ui-components`: `Disclosure` panels now animate open/close via spring-driven `height/opacity/y`, deferring `hidden` until motion completes.
- `ui-components`: `Disclosure` panels now keep their measured height in sync while open (ResizeObserver), preventing clipping when content changes dynamically.
- `ui-components`: `Accordion` now supports `open_indices/default_open_indices/on_open_change`.
- `ui-components`: `Accordion` panels now animate open/close via spring-driven `height/opacity/y`, deferring `hidden` until motion completes.
- `ui-components`: `Accordion` panels now keep their measured height in sync while open (ResizeObserver), preventing clipping when content changes dynamically.
- `ui-components`: `Accordion` triggers now use headless press semantics (`data-pressed`) and avoid keyboard/click double firing.
- `ui-components`: `<SegmentedControl>` indicator motion now drives `x/y/width/height/opacity` via spring, fixing inset alignment and supporting vertical orientation.
- `ui-components`: `<Tabs>` indicator motion now refreshes its measured layout on resize (ResizeObserver), keeping the spring highlight aligned.
- `ui-components`: `<Tabs>` now emits Spectrum-style `data-*` state attributes and `data-slot` markers for styling and regression testing.
- `ui-components`: `ActiveHighlightMotion` now measures option layout via `offsetTop/offsetHeight` and refreshes on resize (ResizeObserver), keeping list/menu highlights aligned during reflow.
- `ui-components`: `<ScrollShadow>` now refreshes its shadow edges when the viewport size changes (ResizeObserver), not only on scroll.
- `ui-components`: overlay trigger buttons now set `aria-controls` only when open (aligns with React Spectrum `useOverlayTrigger`), enabled via `Button`/`ActionButton` `aria_controls_signal`.
- `ui-components`: `<Overlay>` now ignores Escape when the event is default-prevented or during IME composition, and adds a semantics regression test.
- `ui-components`: `<Overlay>` now supports `is_dismissable` and `is_keyboard_dismiss_disabled` to control backdrop/Escape dismissal, and adds semantics coverage.
- `ui-components`: `MenuTrigger` now supports `aria_label` and emits Spectrum-style root `data-*` state attributes, and adds a semantics regression test.
- WASM CSR builds no longer abort when attribute spreading adds a 27th HTML attribute (patched `tachys` and added a regression test).
- `tachys`: 26+ attribute chaining now keeps strongly typed tuples instead of erasing into `AnyAttribute`, avoiding CSR panics on non-erasable attributes.
- `ui-components`: `<Popover>` now ignores Escape when the event is default-prevented or during IME composition, and adds a semantics regression test.
- `ui-components`: `<Sheet>` now ignores Escape when the event is default-prevented or during IME composition, and adds a semantics regression test.
- `ui-components`: `<Sheet>` now supports `is_dismissable` and `is_keyboard_dismiss_disabled` to control backdrop/Escape dismissal, and adds semantics coverage.
- `ui-components`: `<ToastViewport>` now marks its portaled viewport as an overlay portal (`data-ui-overlay-portal`) so modal `aria-hidden` logic doesn't hide toasts.
- `apps/web-demo`: add a minimal boot loader and runtime error surface to avoid a blank screen during WASM startup.
- `apps/docs-app`: add a minimal boot loader and runtime error surface to avoid a blank screen during WASM startup.
- `build` (wasm): enable `erase_components` to avoid Tachys attribute tuple limits (prevents CSR panics on views with many attributes).
- `ui-components`: `<Tooltip>` now uses headless trigger state and portal positioning, with HeroUI-style spring motion (opacity/scale/y).
- `ui-components`: `<Tooltip>` no longer wraps children in a `<button>`; it listens via wrapper events and applies `aria-describedby` to the focused element, avoiding nested interactive markup.
- `ui-headless`: `PopoverPositionOptions` is now generic over the anchor element type (defaults to `Button`), allowing non-button triggers to opt into popover positioning.
- `ui-components`: `<ContextualHelp>` now supports `open/default_open/on_open_change`, adds proper dialog labeling (`aria-labelledby`/`aria-describedby`), and upgrades `footer` to a slot.
- `ui-components`: `<HoverCard>` now uses headless trigger state, marks its panel as an overlay portal, and positions via CSS variables (no direct `web-sys`).
- `ui-components`: `<HoverCard>` no longer wraps children in a `<button>`; it listens via a non-interactive wrapper and applies `aria-describedby` to the focused element, avoiding nested interactive markup.
- `ui-components`: `<HoverCard>` now intercepts Escape while open (stops propagation + prevents default) and ignores IME composition, and adds a semantics regression test.
- `ui-headless`: `use_hover_card_trigger` now opens on focus-visible (keyboard) rather than any focus, preventing hover cards from appearing on touch/pointer focus.
- `ui-headless`: `use_menu` now supports `default_index` to enable menu trigger focus strategies (e.g. open + focus last item).
- `ui-components`: `MenuTrigger` now supports Spectrum-style keyboard opening (ArrowDown opens focusing first; ArrowUp opens focusing last).
- `ui-components`: `MenuTrigger` now supports `open/default_open/on_open_change`, `disabled`, and labels its menu via `aria-labelledby` (trigger `id`).
- `ui-components`: adds Tooltip semantics regression tests (aria-describedby wiring + portal marker + spring CSS variables).
- `ui-components`: `DropdownMenu` now supports `open/default_open/on_open_change`, Spectrum-style ArrowUp/Down opening (focus first/last), and labels its menu via `aria-labelledby` (trigger `id`).
- `ui-components`: `ActionMenu` now supports `open/default_open/on_open_change`, Spectrum-style ArrowUp/Down opening (focus first/last), and labels its menu via `aria-labelledby` (trigger `id`).
- `ui-components`: `Select` now supports `open/default_open/on_open_change` and labels its listbox via `aria-labelledby` (trigger `id`).
- `ui-components`: `<Button>` now supports an optional `id` prop.
- `ui-components`: `<ActionButton>` now supports an optional `id` prop.
- `ui-components`: `<Menu>` now supports `aria_labelledby` for accessible labeling.
- `ui-components`: `<ListBox>` now supports `aria_labelledby` for accessible labeling.
- `ui-components`: `TagGroup` now supports Spectrum-style group semantics (`aria-labelledby`/`aria-describedby`/`aria-invalid`/`aria-required`), optional description/error messaging, and merged `aria-describedby` ids.
- `apps/docs-app`: `TagGroup` docs now include a validation playground (required + invalid state) in addition to removable tags.
- `ui-components`: `DropdownMenu` now centralizes trigger disabled state (`disabled || items.is_empty()`), emits Spectrum-style `data-open`/`data-disabled`, and keeps internal `logic` private with semantics regression coverage.
- `apps/docs-app`: `DropdownMenu` docs now include Default / Controlled Open State / Disabled + Empty playgrounds.
- `ui-components`: `Menu` now resolves accessible names (`aria-label` > `aria-labelledby` > default `"Menu"`), emits Spectrum-style root/item `data-*` state attrs, and adds semantics regression coverage.
- `apps/docs-app`: `Menu` docs now include Kinds + Selection and Disabled + Empty playgrounds.
- `ui-components`: `ListBox` now resolves accessible names (`aria-label` > `aria-labelledby` > default `"Listbox"`), emits Spectrum-style root/option `data-*` state attrs, and adds semantics regression coverage.
- `apps/docs-app`: `ListBox` docs now include Selection + Typeahead and Disabled + Empty playgrounds.
- `ui-components`: `CheckboxGroup` now normalizes label/description/error text, adds explicit legend labeling (`aria-labelledby`), emits Spectrum-style state `data-*` attrs, and adds semantics regression coverage.
- `apps/docs-app`: `CheckboxGroup` docs now include Validation + Required and Disabled playgrounds.
- `apps/web-demo`: adds a controlled `<MenuTrigger>` demo (external open state + `on_open_change`).
- `apps/web-demo`: updates `<DropdownMenu>` demo to exercise controlled open state.
- `apps/web-demo`: updates `<ActionMenu>` demo to exercise controlled open state.
- `apps/web-demo`: adds a controlled `<Select>` demo (external open state + `on_open_change`).
- `ui-motion`: `SpringAnimator` now runs `on_rest` immediately when reduced motion is enabled (fixes presence-style flows).
- `ui-components`: overlay/popover motion now animates enter when mounting while open (better presence UX).
- `ui-headless`: `use_popover_position` now exposes `anchor_width_px` for width-aligned panels.
- `ui-headless`: `use_popover_position` now recomputes when the anchor or panel resizes (ResizeObserver), keeping overlays aligned during reflow.
- `ui-headless`: `use_popover_position` and `use_tooltip_position` now recompute on captured scroll events (including scroll containers), preventing overlay drift in nested scrolling layouts.
- `ui-headless`: `PopoverPlacement` now includes top placements; `use_popover_position` now resolves/flips placement based on viewport space.
- `ui-headless`: `use_tooltip_position` now recomputes when the anchor or panel resizes (ResizeObserver), keeping tooltips aligned during reflow.
- `ui-components`: `Popover` now sets `--ui-popover-anchor-width` and uses it for the default min-width.
- `ui-components`: `Popover` now supports `is_modal` (defaults true) and sets `data-placement` for correct transform origin and motion direction.
- `ui-components`: re-exports `provide_focus_visible`, `provide_overlay_stack`, and `OnPress` to reduce app-layer coupling.
- `ui-components`: `Tabs` now supports manual keyboard activation and a spring-driven selection indicator (HeroUI-style feel), with hover/press/focus-visible states.
- `ui-core`: callback types are now `Send + Sync` (uses `Arc<dyn Fn(...) + Send + Sync>`).
- `ui-headless`: `use_press` now supports keyboard Enter/Space (with click de-duping) and exposes key handlers that indicate when callers should `preventDefault` (for custom elements).
- `ui-headless`: `use_button` now supports `ButtonElement` + returns `ButtonAttrs` (`role`/`tabindex`/`aria-disabled`) for custom button semantics.
- `ui-headless`: `use_listbox` now supports `on_action` to react to selection activation.
- `ui-headless` (wasm): wrap DOM/event handles in `send_wrapper::SendWrapper` to satisfy Leptos `on_cleanup` Send+Sync bounds.
- `ui-headless` (wasm): `use_popover_position` now uses `Element::get_bounding_client_rect` (required by `web-sys`) for anchor/panel rects.
- `ui-components`: core visuals now use CSS variables (`--ui-*`) for theming (Button, ListBox, Menu, Select, Popover, Overlay, Checkbox, Switch).
- `ui-components`: injects component CSS under `@layer ui` so app styles can override without `!important`/high specificity.
- `apps/web-demo`: polished the demo page layout and added a Trunk-loaded stylesheet (`app.css`) with in-page navigation.
- `apps/web-demo`: refactors `src/main.rs` into per-section demo components under `src/demos/` for readability.
- `apps/web-demo`: removes inline styles from demo markup (keeps layout rules in `app.css`).
- `ui-components`: refactored `Button` into `logic/styles/motion/view` modules (ARCHITECTURE_ZH-style separation).
- `ui-components`: `Button` now supports loading state (`is_loading`, `aria-busy`, and loading placement with a built-in spinner).
- `ui-components`: `FileTrigger` now clears the underlying input value before opening (allowing the same file to be selected twice), forwards trigger motion to the internal `Button`, supports `accept_directory`/`capture`, hides the input from focus, and keeps DOM helpers internal (private `logic` module).
- `ui-components`: `DropZone` now matches Spectrum-style semantics (headless hover/focus ring state, stable drag enter/leave tracking), supports pasting files, adds a hidden focus target for accessibility, and uses spring-driven highlight/scale feedback.
- `ui-components`: `AutoHeight` now animates height changes via a ResizeObserver-driven spring (updates `--ui-auto-height-height`) instead of ignoring its motion contract.
- `ui-components`: `Separator` now honors `SeparatorMotion` by optionally spring-animating in via CSS variables (`--ui-separator-scale-*`, `--ui-separator-opacity`).
- `ui-components`: `InlineAlert` now spring-animates reveal motion via CSS variables (`--ui-inline-alert-opacity`, `--ui-inline-alert-translate-y`, `--ui-inline-alert-scale`).
- `ui-theme`: adds `--ui-accent-fg` token to avoid hard-coded foreground colors in components.
- Dev tooling: `scripts/gate.sh` now runs WASM checks by default when the wasm target is installed (still auto-skips when missing).
- Dev tooling: pre-commit now refuses oversized Rust files (default 1000 lines; override via `RUST_UI_MAX_RS_LINES=<N>`).
- Dev tooling: `scripts/check.sh` now also checks `docs-app` for `wasm32-unknown-unknown`.
- Dev tooling: gate scripts are now invoked via `bash` to avoid executable-bit issues on some checkouts.
- `ui-components`: adds Spectrum-compat regression coverage for `Button` (semantics, state normalization, motion defaults).
- `ui-components`: simplifies `IconButton` composition and adds Spectrum-compat regression coverage.
- `ui-components`: makes `Select` internal logic private and adds Spectrum-compat regression coverage.
- `ui-components`: makes `Image` internal logic private and adds Spectrum-compat regression coverage.
- `ui-components`: makes `Toast` internal logic private and expands Spectrum-compat regression coverage.
- `ui-components`: makes `IllustratedMessage` internal logic private and adds Spectrum-compat regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `TextField` and expands regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `TextArea` and expands regression coverage.
- `ui-components`: adds Spectrum-compat regression coverage for `Link`.
- `ui-components`: adds Spectrum-compat regression coverage for `LinkButton`.
- `ui-components`: adds Spectrum-compat regression coverage for `Badge`.
- `ui-components`: `Link` now styles hover/focus-visible via headless-driven state attributes (no pseudo classes) and exposes focus state data attributes.
- `ui-components`: adds Spectrum-style state data attributes to `Input` and expands regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `SearchField` and expands regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `NumberField` and expands regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `ComboBox` and expands regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `Autocomplete` and expands regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `Checkbox` and adds regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `Switch` and adds regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `Radio` and adds regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `ToggleButton` and adds regression coverage.
- `ui-components`: adds Spectrum-style state data attributes to `SegmentedControl` and adds regression coverage.
- `ui-components`: `ActionMenu` now centralizes trigger-disabled state (`disabled || items.is_empty()`), emits Spectrum-style root state attrs (`data-open`/`data-disabled`/`data-empty`/`data-has-items`), expands semantics regression coverage, and upgrades docs playground with default/controlled/disabled-empty scenarios.
- `ui-components`: `MenuTrigger` now centralizes trigger-disabled state (`disabled || items.is_empty()`), emits Spectrum-style root state attrs (`data-open`/`data-disabled`/`data-empty`/`data-has-items`), expands semantics regression coverage, and upgrades docs playground with default/controlled/disabled-empty scenarios.
- `ui-components`: `DropdownMenu` now emits Spectrum-style root state attrs (`data-open`/`data-disabled`/`data-empty`/`data-has-items`), keeps trigger-disabled semantics centralized, expands regression coverage, and upgrades docs playground with richer action scenarios.
- `ui-components`: `Pagination` now uses a centralized state model for page clamping/prev-next disabling, emits richer Spectrum-style state attrs (`data-page`/`data-total-pages`/`data-empty`/`data-single-page`), adds regression coverage, and upgrades docs playground with on_change/disabled/empty scenarios.
- `ui-components`: `ListBox` now derives root state from a centralized model (`empty/items/selection/disabled-options`), emits additional Spectrum-style root attrs (`data-has-items`/`data-has-selection`/`data-selection-empty`/`data-has-disabled-options`), expands semantics regression coverage, and upgrades docs playground state visibility.
- `ui-components`: `Menu` now derives root state from a centralized model (`empty/items/checked-items/disabled-items`), emits richer Spectrum-style attrs (`data-has-items`/`data-has-checked-items`/`data-checked-empty`/`data-has-disabled-items`), expands semantics regression coverage, and upgrades docs playground state visibility.
- `ui-components`: `Tabs` now derives root state from a centralized model (`empty/items/selected-index/disabled-tabs`), emits richer Spectrum-style root attrs (`data-has-items`/`data-selected-index`/`data-selection-empty`/`data-has-disabled-tabs`/`data-keyboard-activation`), expands semantics regression coverage, and upgrades docs playground with automatic/manual controlled scenarios.
- `ui-components`: `Accordion` now derives root state from a centralized model (`empty/items/open-count/disabled-items`), emits richer Spectrum-style root attrs (`data-has-items`/`data-open-count`/`data-all-closed`/`data-multiple-open`/`data-has-disabled-items`/`data-selection-mode`), expands semantics regression coverage, and upgrades docs playground with controlled multiple/single-disabled scenarios.
- `ui-components`: `Disclosure` now derives root state from a centralized model (`open/closed/disabled`), emits Spectrum-style root and trigger/panel state attrs (`data-open`/`data-closed`/`data-disabled`), adds semantics regression coverage, and upgrades docs playground with controlled/disabled scenarios.
- `ui-components`: `Breadcrumbs` now derives root state from a centralized model (`empty/items/links/current-page`), emits Spectrum-style root/item attrs (`data-empty`/`data-has-items`/`data-has-links`/`data-has-current-page`/`data-count`/`data-index`/`data-last`), adds semantics regression coverage, and upgrades docs playground with label-only/empty scenarios.
- `ui-components`: `TagGroup` now derives root state from a centralized model (`empty/items/disabled/removable/invalid/required`), emits richer Spectrum-style root/item attrs (`data-empty`/`data-has-items`/`data-count`/`data-has-disabled-tags`/`data-has-removable-tags`/`data-tag-id`), expands semantics regression coverage, and upgrades docs playground with removable/validation/disabled-empty scenarios.
- `ui-components`: `CheckboxGroup` now derives root state from a centralized model (`disabled/invalid/required/description/error visibility`), emits richer Spectrum-style attrs (`data-enabled`/`data-valid`/`data-optional`/`data-shows-error`/`data-has-messages`), expands semantics regression coverage, and upgrades docs playground with disabled/optional scenarios.
- `ui-components`: `Select` now derives root state from a centralized model (`open/closed/items/selection/disabled-options`), emits richer Spectrum-style attrs (`data-closed`/`data-has-items`/`data-count`/`data-selection-empty`/`data-selected-index`/`data-disabled-option-count`), expands semantics regression coverage, and upgrades docs playground with controlled-open plus disabled/empty scenarios.
- `ui-components`: `RadioGroup` now derives root state from a centralized model (`empty/items/selection/disabled-options/orientation`), emits richer Spectrum-style attrs (`data-has-items`/`data-count`/`data-selection-empty`/`data-selected-index`/`data-disabled-option-count`/`data-horizontal`/`data-vertical`), expands semantics regression coverage, and upgrades docs playground state visibility.
- `ui-components`: `SegmentedControl` now derives root state from a centralized model (`empty/items/selection/disabled-options/orientation`), emits richer Spectrum-style attrs (`data-has-items`/`data-count`/`data-selection-empty`/`data-selected-index`/`data-disabled-option-count`/`data-horizontal`/`data-vertical`), expands semantics regression coverage, and upgrades docs playground state visibility.
- `ui-components`: `Switch` now derives root state from a centralized model (`checked/unchecked/disabled/enabled/pressed/hovered/focused/focus-visible`), emits richer Spectrum-style attrs (`data-checked`/`data-unchecked`/`data-enabled`), expands semantics regression coverage, and upgrades docs playground with controlled `on_change` plus disabled state-matrix scenarios.
- `ui-components`: `Checkbox` now derives root state from a centralized model (`checked/unchecked/disabled/enabled/pressed/hovered/focused/focus-visible`), emits richer Spectrum-style attrs (`data-checked`/`data-unchecked`/`data-enabled`), expands semantics regression coverage, and upgrades docs playground with controlled `on_change` plus variant/disabled state-matrix scenarios.
- `ui-components`: `ToggleButton` now derives root state from a centralized model (`selected/unselected/disabled/enabled/pressed/hovered/focused/focus-visible`), emits richer Spectrum-style attrs (`data-state`/`data-unselected`/`data-enabled`), expands semantics regression coverage, and upgrades docs playground with controlled `on_change` plus variant/disabled state-matrix scenarios.
- `ui-components`: `ToggleButtonGroup` now derives root state from a centralized model (`orientation/attached/label-source`), emits richer Spectrum-style attrs (`data-orientation`/`data-horizontal`/`data-vertical`/`data-attached`/`data-detached`), adds semantics regression coverage, and upgrades docs playground with attached + vertical/detached scenarios.
- `ui-components`: `ButtonGroup` now derives root state from a centralized model (`orientation/attached/label-source`), emits richer Spectrum-style attrs (`data-orientation`/`data-horizontal`/`data-vertical`/`data-attached`/`data-detached`), adds semantics regression coverage, and upgrades docs playground with attached + vertical/detached scenarios.
- `ui-components`: `IconButton` now derives wrapper state from a centralized model (size/disabled/handler/label-source/class-source), normalizes blank aria labels to a safe fallback, expands semantics regression coverage, and upgrades docs playground with on_press counters plus size/disabled scenarios.
- `ui-components`: `LinkButton` now derives anchor state from a centralized model (href/disabled/target/rel/aria-label/class-source), normalizes blank href and text props, emits richer Spectrum-style attrs (`data-state`/`data-enabled`/`data-target`/`data-rel`), expands semantics regression coverage, and upgrades docs playground with external hardening plus variant/disabled matrices.
- `ui-components`: `SearchInputButton` now derives root state from a centralized model (disabled/shortcut/placeholder-source/aria-label/class-source), normalizes blank text inputs to safe defaults, emits richer Spectrum-style attrs (`data-state`/`data-enabled`/`data-shortcut`/`data-placeholder`), expands semantics regression coverage, and upgrades docs playground with interactive shortcut + placeholder/disabled scenarios.
- `ui-components`: `ButtonCopy` now derives wrapper state from a centralized model (copyable/disabled/empty/label-source/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-state`/`data-copyable`/`data-empty`/`data-label`), adds dedicated semantics regression coverage, and upgrades docs playground with label/variant plus disabled-empty scenarios.
- `ui-components`: `FlipButton` now derives interaction/direction state from a centralized model (active/hover/focus-within/from/class-source), normalizes custom class input, emits richer Spectrum-style attrs (`data-state`/`data-active`/`data-inactive`/`data-focus-within`), adds dedicated semantics regression coverage, and upgrades docs playground with top + direction-matrix scenarios.
- `ui-components`: `ThemeToggleButton` now derives wrapper state from a centralized model (enabled/disabled/current-next-mode/custom-modes/aria-label/class-source), normalizes custom inputs, emits richer Spectrum-style attrs (`data-state`/`data-current-mode`/`data-next-mode`/`data-mode-count`), expands semantics regression coverage, and upgrades docs playground with default cycle plus custom-modes/disabled scenarios.
- `ui-components`: `ShareButton` now derives wrapper state from a centralized model (item-count/default-items/icon-placement/label-source/handler-source), normalizes optional text and custom share items, emits richer Spectrum-style attrs (`data-state`/`data-count`/`data-icon`/`data-default-items`), adds dedicated semantics regression coverage, and upgrades docs playground with callback plus icon-placement/custom-items scenarios.
- `ui-components`: `ActionButton` now derives wrapper state from a centralized model (size/loading-placement/quiet/icon-only/slot-content/handler-source), normalizes optional text props, emits richer Spectrum-style attrs (`data-state`/`data-size`/`data-loading-placement`/`data-has-start`/`data-has-end`), expands semantics regression coverage, and upgrades docs playground with callback plus loading-placement/icon-only scenarios.
- `ui-components`: `ActionButtonGroup` now derives toolbar wrapper state from a centralized model (orientation/density/justified/quiet/enablement/label-source/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-state`/`data-orientation`/`data-density`/`data-quiet`/`data-enabled`), expands semantics regression coverage, and upgrades docs playground with callback plus vertical/disabled/justified scenarios.
- `ui-components`: `ActionMenu` now derives wrapper state from a centralized model (item-count/open-strategy/control-mode/placement/label-source/class-source), normalizes id/label/disabled-index inputs, emits richer Spectrum-style attrs (`data-state`/`data-open`/`data-placement`/`data-controlled`/`data-close-on-action`), expands semantics regression coverage, and upgrades docs playground with controlled persistent-open plus disabled/empty scenarios.
- `ui-components`: `MenuTrigger` now derives wrapper state from a centralized model (item-count/open-strategy/control-mode/placement/label-source/class-source), normalizes id/label/disabled-index inputs, emits richer Spectrum-style attrs (`data-state`/`data-open`/`data-placement`/`data-controlled`/`data-close-on-action`), expands semantics regression coverage, and upgrades docs playground with controlled persistent-open plus disabled/empty scenarios.
- `ui-components`: `DropdownMenu` now derives wrapper state from a centralized model (item-count/open-strategy/control-mode/placement/class-source), normalizes id/disabled-index inputs, emits richer Spectrum-style attrs (`data-state`/`data-open`/`data-placement`/`data-controlled`/`data-close-on-action`), expands semantics regression coverage, and upgrades docs playground with controlled persistent-open plus disabled/empty scenarios.
- `ui-components`: `ComboBox` now derives wrapper state from a centralized model (item-count/disabled-options/control-mode/description-error/class-source), normalizes id/disabled-index inputs, emits richer Spectrum-style attrs (`data-state`/`data-open`/`data-count`/`data-filtered-count`/`data-controlled`), expands semantics regression coverage, and upgrades docs playground with controlled-open plus disabled/empty scenarios.
- `ui-components`: `Autocomplete` now derives wrapper state from a centralized model (item-count/disabled-options/control-mode/description-error/class-source), normalizes id/disabled-index inputs, emits richer Spectrum-style attrs (`data-state`/`data-open`/`data-count`/`data-filtered-count`/`data-controlled`), expands semantics regression coverage, and upgrades docs playground with controlled-open plus disabled/empty scenarios.
- `ui-components`: `Avatar` now derives wrapper state from a centralized model (size/image-source/label-source/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-state`/`data-size`/`data-image`/`data-label-source`), adds dedicated semantics regression coverage, and upgrades docs playground with image/fallback plus label-source scenarios.
- `ui-components`: `AvatarGroup` now derives wrapper state from a centralized model (size/overflow/empty/custom-aria/class-source), normalizes max/aria/class inputs, emits richer Spectrum-style attrs (`data-state`/`data-size`/`data-overflow-count`/`data-max-visible`), adds dedicated semantics regression coverage, and upgrades docs playground with overflow/size/empty scenarios.
- `ui-components`: `StatusLight` now derives wrapper state from a centralized model (variant/live-role/class-source), normalizes custom class input, emits richer Spectrum-style attrs (`data-state`/`data-variant`/`data-live`/`data-role`), adds dedicated semantics regression coverage, and upgrades docs playground with variant/live-role scenarios.
- `ui-components`: `Chip` now derives wrapper state from a centralized model (variant/size/removable/disabled/class-source), normalizes dismiss label + class inputs, emits richer Spectrum-style attrs (`data-state`/`data-variant`/`data-size`/`data-removable`/`data-dismiss-label`), adds dedicated semantics regression coverage, and upgrades docs playground with removable/variant/disabled scenarios.
- `ui-components`: `Card` now derives wrapper state from a centralized model (variant/padding/class-source), normalizes custom class input, emits richer Spectrum-style attrs (`data-state`/`data-variant`/`data-padded`/`data-flush`), adds dedicated semantics regression coverage, and upgrades docs playground with variant/padding/custom-class scenarios.
- `ui-components`: `Spinner` now derives wrapper state from a centralized model (size/aria-label/class-source), normalizes optional label + class inputs, emits richer Spectrum-style attrs (`data-state`/`data-size`/`data-custom-aria-label`/`data-custom-class`), adds dedicated semantics regression coverage, and upgrades docs playground with size + label/class scenarios.
- `ui-components`: `Skeleton` now derives wrapper state from a centralized model (variant/shimmer/class-source), normalizes custom class input, emits richer Spectrum-style attrs (`data-state`/`data-variant`/`data-shimmer`/`data-still`/`data-custom-class`), adds dedicated semantics regression coverage, and upgrades docs playground with shimmer/still scenarios.
- `ui-components`: `Divider` now derives wrapper state from a centralized model (orientation/class-source), normalizes custom class input, emits richer Spectrum-style attrs (`data-state`/`data-orientation`/`data-horizontal`/`data-vertical`/`data-custom-class`), adds dedicated semantics regression coverage, and upgrades docs playground with orientation/custom-class scenarios.
- `ui-components`: `Spacer` now derives wrapper state from a centralized model (axis/size/class-source), normalizes custom class input, emits richer Spectrum-style attrs (`data-state`/`data-axis`/`data-size`/`data-vertical`/`data-horizontal`/`data-custom-class`), adds dedicated semantics regression coverage, and upgrades docs playground with axis-size/custom-class scenarios.
- `ui-components`: `ScrollShadow` now derives wrapper state from a centralized model (edge-state/max-height/class-source), normalizes optional class + max-height inputs, emits richer Spectrum-style attrs (`data-state`/`data-scrollable`/`data-max-height`/`data-shadow-top`/`data-shadow-bottom`/`data-custom-class`) plus class markers, adds dedicated semantics regression coverage, and upgrades docs playground with default/custom-height scenarios.
- `ui-components`: `AutoHeight` now derives wrapper state from a centralized model (animation/class/motion-source), normalizes optional class input, emits richer Spectrum-style attrs (`data-state`/`data-animated`/`data-static`/`data-overflow-hidden`/`data-custom-class`/`data-custom-motion`) and state classes, upgrades semantics regression coverage, and expands docs playground with animated vs static-motion scenarios.
- `ui-components`: `UiRoot` now derives wrapper state from a centralized model (theme-scheme/safe-area), emits stable Spectrum-style root attrs (`data-slot`/`data-state`/`data-theme-scheme`/`data-safe-area`) plus safe-area class markers, adds dedicated semantics regression coverage, and upgrades docs playground with usage + state-contract scenarios.
- `ui-components`: `Separator` now derives wrapper state from a centralized model (orientation/element/decorative/class-source), normalizes optional class input, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-orientation`/`data-element`/`data-semantic`/`data-decorative`/`data-custom-class`) plus state class markers, expands semantics regression coverage, preserves spring-motion contract, and upgrades docs playground with semantic-element plus decorative-custom scenarios.
- `ui-components`: `Modal` now derives wrapper state from a centralized model (title/description/id-base/class-source), normalizes blank title/id/class inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-description`/`data-with-description`/`data-custom-class`) plus modal slot markers, forwards overlay motion/presence contracts, adds dedicated semantics regression coverage, and upgrades docs playground with described + title-only custom-class scenarios.
- `ui-components`: `Drawer` now derives wrapper state from a centralized model (placement/description/footer/close-button/id-base/class-source), normalizes blank title/id/class inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-placement`/`data-description`/`data-footer`/`data-close-button`/`data-custom-class`) plus drawer slot markers, forwards sheet motion/presence contracts, adds dedicated semantics regression coverage, and upgrades docs playground with right-slot and left-custom-class scenarios.
- `ui-components`: `ContextualHelp` now derives wrapper state from a centralized model (variant/placement/heading/footer/label-source/id-source/control-mode/class-source), normalizes optional heading/class/id/aria-label inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-variant`/`data-placement`/`data-heading`/`data-footer`/`data-open-mode`/`data-label-source`/`data-id-source`/`data-custom-class`), preserves non-modal popover semantics and motion/presence contracts, adds dedicated semantics regression coverage, and upgrades docs playground with help-slot plus info-controlled scenarios.
- `ui-components`: `Alert` now derives wrapper state from a centralized model (variant/title/description/actions/live-region/class-source), normalizes optional title/description/class inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-variant`/`data-title`/`data-description`/`data-actions`/`data-custom-class`), adds dedicated semantics regression coverage, and upgrades styles with class + `data-*` state-marker contracts.
- `apps/docs-app`: `Alert` docs now include richer playground coverage with variant/live-region matrix plus compact/custom-class scenarios.
- `ui-components`: `FlipButton` now derives wrapper state from a centralized model (direction/active/hover/focus-within/class-source), normalizes optional class input, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-from`/`data-hover`/`data-focus-within-state`/`data-custom-class`) plus stable class markers, and expands semantics regression coverage for logic + styles contracts.
- `apps/docs-app`: `FlipButton` docs now include richer playground coverage with top-flip, direction matrix, and custom-class scenarios.
- `ui-components`: `SearchInputButton` now derives wrapper state from a centralized model (enabled/disabled/shortcut/placeholder-source/aria-label-source/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-shortcut`/`data-placeholder`/`data-compact-placeholder`/`data-aria-label-source`/`data-custom-class`), and expands semantics regression coverage for logic + styles state-marker contracts.
- `apps/docs-app`: `SearchInputButton` docs now include richer playground coverage with interactive shortcut counters, placeholder+disabled matrix, and custom-class + aria-label scenarios.
- `ui-components`: `ShareButton` now derives wrapper state from a centralized model (provided/resolved item counts, items-source, icon-placement, label-source, handler-source, class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-provided-count`/`data-count`/`data-items-source`/`data-icon`/`data-label-source`/`data-handler-source`/`data-custom-class`), and expands semantics regression coverage for logic + styles state-marker contracts.
- `apps/docs-app`: `ShareButton` docs now include richer playground coverage with callback state, icon-placement + custom-items matrix, and custom-class + direction scenarios.
- `ui-components`: `Code` now derives wrapper state from a centralized model (variant/state/class-source), normalizes optional class input, emits richer Spectrum-style attrs (`data-slot`/`data-variant`/`data-state`/`data-inline`/`data-block`/`data-custom-class`) plus state class markers, and adds dedicated semantics regression coverage.
- `apps/docs-app`: `Code` docs now include richer playground coverage with inline+block variant matrix and custom-class block scenarios.
- `ui-components`: `Kbd` now derives wrapper state from a centralized model (size/keys/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-slot`/`data-size`/`data-state`/`data-keys`/`data-custom-class`) plus state class markers, and adds dedicated semantics regression coverage.
- `apps/docs-app`: `Kbd` docs now include richer playground coverage with size+keys matrix and custom-class + label-only scenarios.
- `ui-components`: `CodeBlock` now derives wrapper state from a centralized model (multiline/header/label/language/copyable/motion-source/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-header`/`data-multiline`/`data-empty`/`data-label`/`data-language`/`data-copyable`/`data-motion-source`/`data-custom-class`) plus state class markers, and expands semantics regression coverage.
- `apps/docs-app`: `CodeBlock` docs now include richer playground coverage with header+copy-motion and compact no-copy custom-class scenarios.
- `ui-components`: `Snippet` now derives wrapper state from a centralized model (multiline/copyability/label/copy-actionability/copied-label-source/class-source), normalizes optional text inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-copy`/`data-multiline`/`data-empty`/`data-label`/`data-copyable`/`data-copy-actionable`/`data-copied-label`/`data-custom-class`) plus state class markers, and adds dedicated semantics regression coverage.
- `apps/docs-app`: `Snippet` docs now include richer playground coverage with copyable + custom copied-label and static multiline custom-class scenarios.
- `ui-components`: `Link` now derives wrapper state from a centralized model (enabled/disabled/missing-href/target/rel-source/aria-label-source/class-source), normalizes href + optional text inputs, emits richer Spectrum-style attrs (`data-slot`/`data-state`/`data-enabled`/`data-disabled`/`data-missing-href`/`data-target`/`data-external`/`data-rel`/`data-aria-label`/`data-custom-class`) plus state class markers, and expands semantics regression coverage.
- `apps/docs-app`: `Link` docs now include richer playground coverage with state matrix and custom rel + class scenarios.
- `ui-components`: `Badge` now derives wrapper state from a centralized model (variant/fill/class-source), normalizes optional class input, emits richer Spectrum-style attrs (`data-slot`/`data-variant`/`data-fill`/`data-state`/`data-solid`/`data-outline`/`data-custom-class`) plus fill state class markers, and expands semantics regression coverage.
- `apps/docs-app`: `Badge` docs now include richer playground coverage with variant matrix and custom-class + outline scenarios.
- `ui-components`: `Avatar` now derives label-source classes from the centralized state model, adds explicit custom-class marker class (`ui-avatar--custom-class`), upgrades styles to class + `data-*` dual contracts (size/image/fallback/label-source/custom-class), and expands semantics regression coverage.
- `apps/docs-app`: `Avatar` docs now include richer playground coverage with image/fallback, size + label-source matrix, and custom-class + normalized-props scenarios.
- `ui-components`: `AvatarGroup` now derives stable/empty/overflow and aria-label-source classes from the centralized state model, emits richer source attrs (`data-aria-label-source`/`data-class-source`), adds explicit custom-class marker class (`ui-avatar-group--custom-class`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `AvatarGroup` docs now include richer playground coverage with overflow stack, size matrix, and custom aria-label + custom-class scenarios.
- `ui-components`: `StatusLight` now derives live/static and role-source classes from the centralized state model, emits richer source attrs (`data-role-source`/`data-class-source`), adds explicit custom-class marker class (`ui-status-light--custom-class`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `StatusLight` docs now include richer playground coverage with variants, live-role semantics, and custom-class static/live scenarios.
- `ui-components`: `Chip` now derives disabled/removable/static and dismiss-label-source classes from the centralized state model, emits richer source attrs (`data-dismiss-label-source`/`data-class-source`), adds explicit custom-class marker class (`ui-chip--custom-class`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `Chip` docs now include richer playground coverage with removable flow, variant-size matrix, custom dismiss-label + custom-class scenarios, and disabled/static scenarios.
- `ui-components`: `CircularProgress` now derives custom size/thickness/aria-label/class-source from a centralized state model, sanitizes numeric CSS-variable inputs, emits richer Spectrum-style attrs (`data-state`/`data-motion`/`data-size-source`/`data-thickness-source`/`data-label-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `CircularProgress` docs now include richer playground coverage with size+thickness matrix scenarios and custom label + custom-class cases.
- `ui-components`: `ProgressCircle` now derives label/value-label/size/stroke/motion/class source contracts from a centralized state model, sanitizes range + metric inputs, emits richer Spectrum-style attrs (`data-state`/`data-indeterminate`/`data-size-source`/`data-stroke-source`/`data-value-label-source`/`data-motion-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `ProgressCircle` docs now include richer playground coverage with determinate/indeterminate matrix scenarios and custom value-label + custom-class cases.
- `ui-components`: `Progress` now derives label/value-label/motion/class source contracts from a centralized state model, normalizes optional labels, emits richer Spectrum-style attrs (`data-state`/`data-indeterminate`/`data-determinate`/`data-label-source`/`data-value-label-source`/`data-motion-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `Progress` docs now include richer playground coverage with determinate/indeterminate matrix scenarios and custom value-label + motion + custom-class cases.
- `ui-components`: `ProgressBar` now derives variant/size/phase/label/class source contracts from a centralized state model, sanitizes `max`/`value`, emits richer Spectrum-style attrs (`data-variant`/`data-size`/`data-state`/`data-indeterminate`/`data-determinate`/`data-label-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and adds dedicated semantics regression coverage.
- `apps/docs-app`: `ProgressBar` docs now include richer playground coverage with variant+size matrix scenarios and custom aria-label + custom-class cases.
- `ui-components`: `Meter` now derives variant/size/phase plus label/value-label/motion/class source contracts from a centralized state model, normalizes optional labels, emits richer Spectrum-style attrs (`data-variant`/`data-size`/`data-state`/`data-indeterminate`/`data-determinate`/`data-label-source`/`data-value-label-source`/`data-motion-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and adds dedicated semantics regression coverage.
- `apps/docs-app`: `Meter` docs now include richer playground coverage with variant+size matrix scenarios and custom aria-label + value-label + motion + custom-class cases.
- `ui-components`: `Spinner` now derives size plus label/class source contracts from a centralized state model, normalizes optional labels/classes, emits richer Spectrum-style attrs (`data-size`/`data-state`/`data-indeterminate`/`data-label-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `Spinner` docs now include richer playground coverage with size matrix scenarios and custom aria-label + custom-class cases.
- `ui-components`: `StaticNumber` now derives sign plus decimal-separator/decimal-places/thousand-separator/class source contracts from a centralized state model, normalizes optional separators and non-finite values, emits richer Spectrum-style attrs (`data-sign`/`data-decimal-separator-source`/`data-decimal-places-source`/`data-thousand-separator-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and adds dedicated semantics regression coverage.
- `apps/docs-app`: `StaticNumber` docs now include richer playground coverage with formatting matrix scenarios and custom separators + custom-class cases.
- `ui-components`: `SlidingNumber` now derives sign/phase plus decimal-separator/decimal-places/thousand-separator/motion/class source contracts from a centralized state model, normalizes separator inputs and decimal precision, emits richer Spectrum-style attrs (`data-state`/`data-sign`/`data-animated`/`data-static`/`data-decimal-separator-source`/`data-decimal-places-source`/`data-thousand-separator-source`/`data-motion-source`/`data-class-source`), upgrades styles to class + `data-*` dual contracts, and expands semantics regression coverage.
- `apps/docs-app`: `SlidingNumber` docs now include richer playground coverage with animated matrix scenarios and custom separators + motion + custom-class cases.
