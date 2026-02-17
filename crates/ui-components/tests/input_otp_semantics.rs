use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn input_otp_uses_single_hidden_input_with_one_time_code_attributes() {
    let source = load_source("src/input_otp/view.rs");

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
    let source = load_source("src/input_otp/view.rs");

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
    let source = load_source("src/input_otp/view.rs");

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
}

#[test]
fn input_otp_styles_define_caret_blink_and_focus_visible_outline() {
    let source = load_source("src/input_otp/styles.rs");

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
