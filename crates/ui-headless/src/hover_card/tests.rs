use super::*;

fn reset_owner() -> Owner {
    drop(any_spawner::Executor::init_futures_executor());
    let owner = Owner::new();
    owner.set();
    test_timers::reset();
    owner
}

#[test]
fn opens_after_delay_and_closes_after_delay() {
    let _owner = reset_owner();

    let hover_card = use_hover_card_trigger(HoverCardTriggerOptions {
        open_delay_ms: 100,
        close_delay_ms: 50,
        ..Default::default()
    });

    hover_card.handlers.on_trigger_pointer_enter.run(());
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());

    test_timers::advance_by(99);
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());

    test_timers::advance_by(1);
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_trigger_pointer_leave.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    test_timers::advance_by(49);
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    test_timers::advance_by(1);
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());
}

#[test]
fn hovering_panel_cancels_pending_close() {
    let _owner = reset_owner();

    let hover_card = use_hover_card_trigger(HoverCardTriggerOptions {
        open_delay_ms: 0,
        close_delay_ms: 80,
        ..Default::default()
    });

    hover_card.handlers.on_trigger_pointer_enter.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_trigger_pointer_leave.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    test_timers::advance_by(40);
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_panel_pointer_enter.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    test_timers::advance_by(500);
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_panel_pointer_leave.run(());
    any_spawner::Executor::poll_local();
    test_timers::advance_by(79);
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    test_timers::advance_by(1);
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());
}

#[test]
fn focus_keeps_hover_card_open() {
    let _owner = reset_owner();

    let focus_visible = crate::provide_focus_visible();
    focus_visible.set_modality(crate::Modality::Keyboard);

    let hover_card = use_hover_card_trigger(HoverCardTriggerOptions {
        open_delay_ms: 0,
        close_delay_ms: 60,
        ..Default::default()
    });

    hover_card.handlers.on_trigger_focus_in.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_trigger_focus_out.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    test_timers::advance_by(60);
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());
}

#[test]
fn focus_does_not_open_when_focus_is_not_visible() {
    let _owner = reset_owner();

    let _focus_visible = crate::provide_focus_visible();

    let hover_card = use_hover_card_trigger(HoverCardTriggerOptions {
        open_delay_ms: 0,
        close_delay_ms: 0,
        ..Default::default()
    });

    hover_card.handlers.on_trigger_focus_in.run(());
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());
}

#[test]
fn dismiss_keeps_card_closed_until_pointer_leaves() {
    let _owner = reset_owner();

    let hover_card = use_hover_card_trigger(HoverCardTriggerOptions {
        open_delay_ms: 0,
        close_delay_ms: 0,
        ..Default::default()
    });

    hover_card.handlers.on_trigger_pointer_enter.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());

    hover_card.state.dismiss.run(());
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());

    test_timers::advance_by(500);
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_trigger_pointer_leave.run(());
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());

    hover_card.handlers.on_trigger_pointer_enter.run(());
    any_spawner::Executor::poll_local();
    assert!(hover_card.state.is_open.get_untracked());
}

#[test]
fn disabled_does_not_open() {
    let _owner = reset_owner();

    let hover_card = use_hover_card_trigger(HoverCardTriggerOptions {
        is_disabled: true,
        open_delay_ms: 0,
        close_delay_ms: 0,
    });

    hover_card.handlers.on_trigger_pointer_enter.run(());
    hover_card.handlers.on_trigger_focus_in.run(());
    any_spawner::Executor::poll_local();
    assert!(!hover_card.state.is_open.get_untracked());
}
