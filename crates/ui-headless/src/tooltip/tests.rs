use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn reset_globals() -> Owner {
    let owner = Owner::new();
    owner.set();

    test_timers::reset();
    TOOLTIP_GLOBAL.with(|global| {
        let mut global = global.borrow_mut();
        global.tooltips.clear();
        global.warmed_up = false;
        global.next_id = 1;
        global.clear_warmup();
        global.clear_cooldown();
    });

    owner
}

#[test]
fn hover_opens_after_delay_and_closes_after_close_delay() {
    let _owner = reset_globals();

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 100,
            close_delay_ms: 50,
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    assert!(!tooltip.state.is_open().get_untracked());

    test_timers::advance_by(99);
    assert!(!tooltip.state.is_open().get_untracked());

    test_timers::advance_by(1);
    assert!(tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_pointer_leave.run(());
    assert!(tooltip.state.is_open().get_untracked());

    test_timers::advance_by(49);
    assert!(tooltip.state.is_open().get_untracked());

    test_timers::advance_by(1);
    assert!(!tooltip.state.is_open().get_untracked());
}

#[test]
fn opening_new_tooltip_closes_previous_and_skips_delay_when_warmed_up() {
    let _owner = reset_globals();

    let a = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 100,
            ..Default::default()
        },
    );
    let b = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 100,
            ..Default::default()
        },
    );

    a.handlers.on_pointer_enter.run(());
    test_timers::advance_by(100);
    assert!(a.state.is_open().get_untracked());

    b.handlers.on_pointer_enter.run(());
    assert!(b.state.is_open().get_untracked());
    assert!(!a.state.is_open().get_untracked());
}

#[test]
fn delay_returns_after_cooldown_expires() {
    let _owner = reset_globals();

    let a = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 100,
            close_delay_ms: 0,
            ..Default::default()
        },
    );
    let b = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 100,
            close_delay_ms: 0,
            ..Default::default()
        },
    );

    a.handlers.on_pointer_enter.run(());
    test_timers::advance_by(100);
    assert!(a.state.is_open().get_untracked());

    a.handlers.on_pointer_leave.run(());
    assert!(!a.state.is_open().get_untracked());

    // Wait out the global tooltip cooldown window before hovering the next trigger.
    test_timers::advance_by(TOOLTIP_COOLDOWN_MS);

    b.handlers.on_pointer_enter.run(());
    assert!(!b.state.is_open().get_untracked());

    test_timers::advance_by(99);
    assert!(!b.state.is_open().get_untracked());

    test_timers::advance_by(1);
    assert!(b.state.is_open().get_untracked());
}

#[test]
fn reopening_while_close_pending_cancels_close_timeout() {
    let _owner = reset_globals();

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 0,
            close_delay_ms: 80,
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    assert!(tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_pointer_leave.run(());
    assert!(tooltip.state.is_open().get_untracked());

    test_timers::advance_by(40);
    assert!(tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_pointer_enter.run(());
    assert!(tooltip.state.is_open().get_untracked());

    test_timers::advance_by(500);
    assert!(tooltip.state.is_open().get_untracked());
}

#[test]
fn press_does_not_close_when_should_close_on_press_is_false() {
    let _owner = reset_globals();

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 0,
            should_close_on_press: false,
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    assert!(tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_pointer_down.run(());
    assert!(tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_key_down.run("Enter".to_string());
    assert!(tooltip.state.is_open().get_untracked());
}

#[test]
fn press_closes_by_default() {
    let _owner = reset_globals();

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 0,
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    assert!(tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_pointer_down.run(());
    assert!(!tooltip.state.is_open().get_untracked());
}

#[test]
fn disabled_does_not_open_on_hover_or_focus() {
    let _owner = reset_globals();

    let focus_visible = crate::provide_focus_visible();
    focus_visible.set_modality(crate::Modality::Keyboard);

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            is_disabled: true,
            delay_ms: 0,
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    tooltip.handlers.on_focus.run(());
    assert!(!tooltip.state.is_open().get_untracked());
}

#[test]
fn focus_does_not_open_when_focus_is_not_visible() {
    let _owner = reset_globals();

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            trigger: TooltipTriggerMode::Focus,
            delay_ms: 0,
            ..Default::default()
        },
    );

    tooltip.handlers.on_focus.run(());
    assert!(!tooltip.state.is_open().get_untracked());
}

#[test]
fn focus_trigger_opens_only_when_focus_visible() {
    let _owner = reset_globals();

    let focus_visible = crate::provide_focus_visible();
    focus_visible.set_modality(crate::Modality::Keyboard);

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            trigger: TooltipTriggerMode::Focus,
            delay_ms: 100,
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    assert!(!tooltip.state.is_open().get_untracked());

    tooltip.handlers.on_focus.run(());
    assert!(tooltip.state.is_open().get_untracked());
}

#[test]
fn uncontrolled_open_respects_default_open_initialization() {
    let _owner = reset_globals();

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            default_open: Some(true),
            ..Default::default()
        },
    );

    assert!(tooltip.state.is_open().get_untracked());
}

#[test]
fn controlled_open_calls_on_change_without_internal_mutation() {
    let _owner = reset_globals();

    let (controlled_open, set_controlled_open) = signal(false);
    let called = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::clone(&called);

    let tooltip = use_tooltip_trigger(
        None,
        TooltipTriggerOptions {
            delay_ms: 0,
            open: Some(controlled_open.into()),
            on_open_change: Some(Callback::new(move |next: bool| {
                assert!(next);
                called2.fetch_add(1, Ordering::SeqCst);
            })),
            ..Default::default()
        },
    );

    tooltip.handlers.on_pointer_enter.run(());
    assert!(!tooltip.state.is_open().get_untracked());
    assert_eq!(called.load(Ordering::SeqCst), 1);

    set_controlled_open.set(true);
    assert!(tooltip.state.is_open().get_untracked());
}
