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
        (".ui-button", ".ui-button {"),
        (".ui-checkbox", ".ui-checkbox {"),
        (".ui-switch", ".ui-switch {"),
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
        (".ui-text-field", ".ui-text-field {"),
        (".ui-tabs", ".ui-tabs {"),
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
