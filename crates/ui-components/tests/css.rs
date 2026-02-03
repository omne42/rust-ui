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
        (".ui-avatar", ".ui-avatar {"),
        (".ui-avatar-group", ".ui-avatar-group {"),
        (".ui-button", ".ui-button {"),
        (".ui-button-group", ".ui-button-group {"),
        (".ui-toggle-button", ".ui-toggle-button {"),
        (".ui-toggle-button-group", ".ui-toggle-button-group {"),
        (".ui-checkbox", ".ui-checkbox {"),
        (".ui-checkbox-group", ".ui-checkbox-group {"),
        (".ui-chip", ".ui-chip {"),
        (".ui-tag-group", ".ui-tag-group {"),
        (".ui-card", ".ui-card {"),
        (".ui-switch", ".ui-switch {"),
        (".ui-divider", ".ui-divider {"),
        (".ui-overlay", ".ui-overlay {"),
        (".ui-popover", ".ui-popover {"),
        // Modal styles use `.ui-modal__*` selectors today, but we require the `.ui-modal` prefix to
        // appear in the aggregated output.
        (".ui-modal", ".ui-modal"),
        (".ui-listbox", ".ui-listbox {"),
        // Use the base selector to avoid false positives from `.ui-menu-trigger`.
        (".ui-menu", ".ui-menu {"),
        (".ui-select", ".ui-select {"),
        (".ui-badge", ".ui-badge {"),
        (".ui-circular-progress", ".ui-circular-progress {"),
        (".ui-spinner", ".ui-spinner {"),
        (".ui-combo-box", ".ui-combo-box {"),
        (".ui-skeleton", ".ui-skeleton {"),
        (".ui-link", ".ui-link {"),
        (".ui-link-button", ".ui-link-button"),
        (".ui-breadcrumbs", ".ui-breadcrumbs {"),
        (".ui-code", ".ui-code {"),
        (".ui-snippet", ".ui-snippet {"),
        (".ui-kbd", ".ui-kbd {"),
        (".ui-spacer", ".ui-spacer {"),
        (".ui-progress-bar", ".ui-progress-bar {"),
        (".ui-pagination", ".ui-pagination {"),
        (".ui-search-field", ".ui-search-field {"),
        (".ui-status-light", ".ui-status-light {"),
        (".ui-text-field", ".ui-text-field {"),
        (".ui-text-area", ".ui-text-area {"),
        (".ui-number-field", ".ui-number-field {"),
        (".ui-input-otp", ".ui-input-otp {"),
        (".ui-segmented-control", ".ui-segmented-control {"),
        (".ui-scroll-shadow", ".ui-scroll-shadow {"),
        (".ui-sheet", ".ui-sheet {"),
        (".ui-meter", ".ui-meter {"),
        (".ui-radio", ".ui-radio {"),
        (".ui-tabs", ".ui-tabs {"),
        (".ui-accordion", ".ui-accordion {"),
        (".ui-disclosure", ".ui-disclosure {"),
        (".ui-tooltip", ".ui-tooltip {"),
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
