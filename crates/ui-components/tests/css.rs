use ui_components::push_components_css;

#[test]
fn push_components_css_emits_expected_layer_and_selectors() {
    let mut css = String::new();
    push_components_css(&mut css);

    assert!(
        css.contains("@layer ui"),
        "expected aggregated CSS to contain `@layer ui`"
    );

    let required_selectors = [
        (".ui-alert", ".ui-alert {"),
        (".ui-inline-alert", ".ui-inline-alert {"),
        (".ui-avatar", ".ui-avatar {"),
        (".ui-avatar-group", ".ui-avatar-group {"),
        (".ui-button", ".ui-button {"),
        (".ui-action-button", ".ui-action-button {"),
        (".ui-action-button-group", ".ui-action-button-group {"),
        (".ui-button-copy", ".ui-button-copy {"),
        (".ui-button-group", ".ui-button-group {"),
        (".ui-search-input-button", ".ui-search-input-button {"),
        (".ui-flip-button", ".ui-flip-button {"),
        (".ui-share-button", ".ui-share-button {"),
        (".ui-theme-toggle-button", ".ui-theme-toggle-button {"),
        (".ui-toggle-button", ".ui-toggle-button {"),
        (".ui-toggle-button-group", ".ui-toggle-button-group {"),
        (".ui-checkbox", ".ui-checkbox {"),
        (".ui-checkbox-field", ".ui-checkbox-field {"),
        (".ui-checkbox-group", ".ui-checkbox-group {"),
        (".ui-chip", ".ui-chip {"),
        (".ui-color-area", ".ui-color-area {"),
        (".ui-color-field", ".ui-color-field {"),
        (".ui-color-handle", ".ui-color-handle {"),
        (".ui-color-loupe", ".ui-color-loupe {"),
        (".ui-color-editor", ".ui-color-editor {"),
        (".ui-color-picker", ".ui-color-picker {"),
        (".ui-color-slider", ".ui-color-slider {"),
        (".ui-color-thumb", ".ui-color-thumb {"),
        (".ui-color-wheel", ".ui-color-wheel {"),
        (".ui-color-swatch", ".ui-color-swatch {"),
        (".ui-color-swatch-picker", ".ui-color-swatch-picker {"),
        (".ui-image", ".ui-image {"),
        (".ui-tag-group", ".ui-tag-group {"),
        (".ui-card", ".ui-card {"),
        (".ui-switch", ".ui-switch {"),
        (".ui-divider", ".ui-divider {"),
        (".ui-separator", ".ui-separator {"),
        (".ui-auto-height", ".ui-auto-height {"),
        (".ui-ripple", ".ui-ripple {"),
        (".ui-overlay", ".ui-overlay {"),
        (".ui-underlay", ".ui-underlay {"),
        (".ui-popover", ".ui-popover {"),
        // Modal styles use `.ui-modal__*` selectors today, but we require the `.ui-modal` prefix to
        // appear in the aggregated output.
        (".ui-modal", ".ui-modal"),
        (".ui-listbox", ".ui-listbox {"),
        // Use the base selector to avoid false positives from `.ui-menu-trigger`.
        (".ui-menu", ".ui-menu {"),
        (".ui-select", ".ui-select {"),
        (".ui-dropdown-menu", ".ui-dropdown-menu {"),
        (".ui-action-menu", ".ui-action-menu {"),
        (".ui-badge", ".ui-badge {"),
        (".ui-circular-progress", ".ui-circular-progress {"),
        (".ui-spinner", ".ui-spinner {"),
        (".ui-autocomplete", ".ui-autocomplete {"),
        (".ui-combo-box", ".ui-combo-box {"),
        (".ui-skeleton", ".ui-skeleton {"),
        (".ui-skeleton-group", ".ui-skeleton-group {"),
        (".ui-link", ".ui-link {"),
        (".ui-legend", ".ui-legend {"),
        (".ui-link-button", ".ui-link-button"),
        (".ui-breadcrumbs", ".ui-breadcrumbs {"),
        (".ui-code", ".ui-code {"),
        (".ui-code-block", ".ui-code-block {"),
        (".ui-snippet", ".ui-snippet {"),
        (".ui-kbd", ".ui-kbd {"),
        (".ui-spacer", ".ui-spacer {"),
        (".ui-progress-bar", ".ui-progress-bar {"),
        (".ui-progress", ".ui-progress {"),
        (".ui-progress-circle", ".ui-progress-circle {"),
        (".ui-static-number", ".ui-static-number {"),
        (".ui-sliding-number", ".ui-sliding-number {"),
        (".ui-pagination", ".ui-pagination {"),
        (".ui-search-field", ".ui-search-field {"),
        (".ui-status-light", ".ui-status-light {"),
        (".ui-text-field", ".ui-text-field {"),
        (".ui-text-area", ".ui-text-area {"),
        (".ui-number-field", ".ui-number-field {"),
        (".ui-input", ".ui-input {"),
        (".ui-input-otp", ".ui-input-otp {"),
        (".ui-file-trigger", ".ui-file-trigger {"),
        (".ui-drop-zone", ".ui-drop-zone {"),
        (".ui-form", ".ui-form {"),
        (".ui-form-field", ".ui-form-field {"),
        (".ui-field-group", ".ui-field-group {"),
        (".ui-segmented-control", ".ui-segmented-control {"),
        (".ui-scroll-shadow", ".ui-scroll-shadow {"),
        (".ui-sheet", ".ui-sheet {"),
        (".ui-tray", ".ui-tray {"),
        (".ui-meter", ".ui-meter {"),
        (".ui-radio", ".ui-radio {"),
        (".ui-tabs", ".ui-tabs {"),
        (".ui-step-list", ".ui-step-list {"),
        (".ui-accordion", ".ui-accordion {"),
        (".ui-disclosure", ".ui-disclosure {"),
        (".ui-tooltip", ".ui-tooltip {"),
        (".ui-contextual-help", ".ui-contextual-help {"),
        (".ui-hover-card", ".ui-hover-card {"),
        (".ui-dialog", ".ui-dialog {"),
        (".ui-alert-dialog", ".ui-alert-dialog {"),
        (".ui-illustrated-message", ".ui-illustrated-message {"),
        (".ui-toast", ".ui-toast {"),
        (".ui-drawer", ".ui-drawer {"),
    ];

    let mut missing = Vec::new();
    for (name, pattern) in required_selectors {
        if !css.contains(pattern) {
            missing.push(format!("{name} (pattern: `{pattern}`)"));
        }
    }

    assert!(
        missing.is_empty(),
        "missing expected selectors in aggregated CSS:\n- {}",
        missing.join("\n- ")
    );
}
