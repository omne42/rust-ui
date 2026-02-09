# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

### Added

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
