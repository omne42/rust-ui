use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_otp_uses_single_hidden_input_with_one_time_code_attributes() {
    let source = load_source("src/text_input/input_otp/view.rs");

    assert!(
        source.contains("class=\"ui-input-otp__input\""),
        "InputOtp should render a single hidden <input> (instead of N separate inputs) to match baseline OTP patterns."
    );
    assert!(
        source.contains("autocomplete=\"one-time-code\""),
        "InputOtp should enable `autocomplete=one-time-code` to support SMS autofill on mobile."
    );
    assert!(
        source.contains("inputmode=\"numeric\""),
        "InputOtp should use `inputmode=numeric` for mobile keypad parity."
    );
    assert!(
        source.contains("maxlength=length"),
        "InputOtp should set `maxlength` to prevent overfilling."
    );
}

#[test]
fn input_otp_slots_are_hidden_from_screen_readers_and_track_active_state() {
    let source = load_source("src/text_input/input_otp/view.rs");

    assert!(
        source.contains("aria-hidden=\"true\""),
        "InputOtp slot chrome should be aria-hidden so screen readers interact with the real input, not duplicate slot text."
    );
    assert!(
        source.contains("data-slot=\"input-otp-slot\""),
        "InputOtp should mark each visual slot with `data-slot=input-otp-slot` for styling/overrides."
    );
    assert!(
        source.contains("data-active"),
        "InputOtp slots should expose `data-active` for active-slot styling (Upstream parity)."
    );
}

#[test]
fn input_otp_integrates_headless_hooks_for_behavior_and_field_semantics() {
    let source = load_source("src/text_input/input_otp/view.rs");

    assert!(
        source.contains("use_input_otp"),
        "InputOtp should delegate value filtering/completion to a headless hook for testable behavior."
    );
    assert!(
        source.contains("use_text_field"),
        "InputOtp should integrate `use_text_field` to provide baseline-style aria-describedby/required/invalid wiring."
    );
    assert!(
        source.contains("use_focus_ring"),
        "InputOtp should use focus ring handling so focus-visible styling matches the rest of the system."
    );
    assert!(
        source.contains("locale_attrs"),
        "InputOtp should wire locale attrs through ui-headless a11y helpers."
    );
}

#[test]
fn input_otp_mounts_locale_and_i18n_default_label_contracts() {
    let source = load_source("src/text_input/input_otp/view.rs");
    let i18n_source = load_source("src/text_input/input_otp/i18n.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "i18n::use_ui_i18n()",
        "strings::<super::i18n::InputOtpStrings>()",
    ] {
        assert!(
            source.contains(needle),
            "InputOtp should include `{needle}` for locale + i18n contract wiring."
        );
    }

    assert!(
        i18n_source.contains("pub struct InputOtpStrings"),
        "InputOtp should source default copy from a dedicated i18n bundle struct."
    );
    assert!(
        i18n_source.contains("aria_label: \"One-time code\".into()"),
        "InputOtp default aria-label fallback should be defined in i18n bundle defaults."
    );
}

#[test]
fn input_otp_styles_define_caret_blink_and_focus_visible_outline() {
    let source = load_source("src/text_input/input_otp/styles.rs");

    assert!(
        source.contains("ui-input-otp__caret"),
        "InputOtp styles should define a caret element for the active slot."
    );
    assert!(
        source.contains("@keyframes ui-input-otp-caret-blink"),
        "InputOtp styles should define a caret blink animation (Upstream parity)."
    );
    assert!(
        source.contains("prefers-reduced-motion: reduce"),
        "InputOtp caret blink should respect prefers-reduced-motion to avoid forced animation."
    );
    assert!(
        source.contains("animation: none;"),
        "InputOtp should disable caret blink animation under prefers-reduced-motion."
    );
    assert!(
        source.contains("ui-input-otp--focus-visible"),
        "InputOtp styles should respond to the focus-visible class rather than relying on `:focus-visible` per-slot."
    );
}

#[test]
fn input_otp_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "pub(super) fn input_otp() -> AnyView",
        "title=\"InputOtp\"",
        "slug=\"input-otp\"",
        "description=\"baseline-style OTP input with a single hidden input and slot chrome.\"",
        "<Playground title=\"OTP\" code_signal=code>",
        "<InputOtp",
    ] {
        assert!(
            source.contains(needle),
            "forms docs should include `{needle}` for input_otp primary playground coverage.",
        );
    }
}

#[test]
fn input_otp_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "title=\"OTP\"",
        "id_base=\"docs-otp\".to_string()",
        "label=\"One-time code\".to_string()",
        "length=6",
        "value=value",
        "set_value=set_value",
        "\"value: \" {move || value.get()}",
    ] {
        assert!(
            source.contains(needle),
            "input_otp docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn input_otp_e2e_contract_uses_semantic_selectors() {
    let source = load_source("../../e2e/tests/docs_app_input_contract.spec.mjs");

    for needle in [
        "docs-app input-otp normalizes digits and preserves slot contracts",
        "docs-app input-otp comparison playground keeps disabled/invalid/default contracts",
        "/#/components/input-otp",
        "body:not(:has(#boot))",
        "data-slot=\"input-otp\"",
        "data-slot=\"input-otp-input\"",
        "data-slot=\"input-otp-slot\"",
        "Default OTP",
        "Disabled OTP",
        "Invalid OTP",
        "toBeDisabled()",
        "aria-invalid",
        "data-slot=\"input-otp-error\"",
        "toHaveCount(6)",
        "toHaveValue(\"123\")",
        "toHaveValue(\"123456\")",
        "Backspace",
    ] {
        assert!(
            source.contains(needle),
            "input_otp e2e contract should include `{needle}` for stable semantic selector coverage.",
        );
    }
}

#[test]
fn input_otp_docs_page_exposes_interactive_display_config_code_css_test_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms.rs");

    for needle in [
        "<Playground",
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"crates/ui-components/src/text_input/input_otp/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "<Playground title=\"State Comparison\" code_signal=state_compare_code>",
    ] {
        assert!(
            source.contains(needle),
            "input_otp docs page should include `{needle}` for interactive display/config/code/css-test + comparison coverage.",
        );
    }
}

#[test]
fn input_otp_readme_covers_display_config_code_css_test_and_comparison_sections() {
    let source = load_source("src/text_input/input_otp/README.md");

    for needle in [
        "## 展示区（Display）",
        "## Config 区",
        "## Code 区",
        "## CSS Test 区",
        "## 多种情况对比显示",
    ] {
        assert!(
            source.contains(needle),
            "input_otp README should include `{needle}` for required documentation structure.",
        );
    }
}

#[test]
fn input_otp_text_metrics_use_typography_tokens() {
    let source = load_source("src/text_input/input_otp/styles.rs");

    for needle in [
        "--ui-input-otp-label-font-size: var(--ui-font-size-150);",
        "--ui-input-otp-label-line-height: var(--ui-line-height-150);",
        "--ui-input-otp-slot-line-height: var(--ui-line-height-200);",
        "--ui-input-otp-meta-font-size: var(--ui-font-size-150);",
        "--ui-input-otp-meta-line-height: var(--ui-line-height-150);",
    ] {
        assert!(
            source.contains(needle),
            "InputOtp styles should include tokenized text metric `{needle}`."
        );
    }

    assert!(
        !source.contains("line-height: 1;"),
        "InputOtp styles should not hardcode `line-height: 1;` for slot text."
    );
}
