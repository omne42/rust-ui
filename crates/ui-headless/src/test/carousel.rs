use super::*;
use std::sync::{Arc, Mutex};

#[test]
fn carousel_root_contract_maps_region_attrs_and_locale() {
    let contract = use_carousel_root(CarouselRootOptions {
        aria_label: "Feature carousel".to_string(),
        orientation: CarouselA11yOrientation::Horizontal,
        lang: Some(" zh-CN ".to_string()),
        dir: Some(A11yDirection::Rtl),
        on_key_command: Callback::new(|_| {}),
    });

    assert_eq!(contract.attrs.role, "region");
    assert_eq!(contract.attrs.aria_label, "Feature carousel");
    assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
    assert_eq!(contract.attrs.dir, Some("rtl"));
    assert_eq!(contract.attrs.tabindex, 0);
    assert_eq!(
        contract.state.orientation,
        CarouselA11yOrientation::Horizontal
    );
    assert!(contract.state.is_rtl);
}

#[test]
fn carousel_key_command_resolution_honors_orientation_and_direction() {
    assert_eq!(
        resolve_carousel_key_command(
            "ArrowLeft",
            CarouselA11yOrientation::Horizontal,
            Some(A11yDirection::Ltr),
        ),
        Some(CarouselKeyCommand::StepBackward)
    );
    assert_eq!(
        resolve_carousel_key_command(
            "ArrowRight",
            CarouselA11yOrientation::Horizontal,
            Some(A11yDirection::Ltr),
        ),
        Some(CarouselKeyCommand::StepForward)
    );
    assert_eq!(
        resolve_carousel_key_command(
            "ArrowRight",
            CarouselA11yOrientation::Horizontal,
            Some(A11yDirection::Rtl),
        ),
        Some(CarouselKeyCommand::StepBackward)
    );
    assert_eq!(
        resolve_carousel_key_command(
            "ArrowLeft",
            CarouselA11yOrientation::Horizontal,
            Some(A11yDirection::Rtl),
        ),
        Some(CarouselKeyCommand::StepForward)
    );
    assert_eq!(
        resolve_carousel_key_command("ArrowUp", CarouselA11yOrientation::Vertical, None),
        Some(CarouselKeyCommand::StepBackward)
    );
    assert_eq!(
        resolve_carousel_key_command("ArrowDown", CarouselA11yOrientation::Vertical, None),
        Some(CarouselKeyCommand::StepForward)
    );
    assert_eq!(
        resolve_carousel_key_command("Home", CarouselA11yOrientation::Vertical, None),
        Some(CarouselKeyCommand::SelectFirst)
    );
    assert_eq!(
        resolve_carousel_key_command("End", CarouselA11yOrientation::Vertical, None),
        Some(CarouselKeyCommand::SelectLast)
    );
    assert_eq!(
        resolve_carousel_key_command("Enter", CarouselA11yOrientation::Vertical, None),
        None
    );
}

#[test]
fn carousel_root_keyboard_handler_emits_key_commands() {
    let commands: Arc<Mutex<Vec<CarouselKeyCommand>>> = Arc::new(Mutex::new(Vec::new()));
    let commands_capture = Arc::clone(&commands);
    let contract = use_carousel_root(CarouselRootOptions {
        aria_label: "Feature carousel".to_string(),
        orientation: CarouselA11yOrientation::Horizontal,
        lang: None,
        dir: None,
        on_key_command: Callback::new(move |command| match commands_capture.lock() {
            Ok(mut commands) => commands.push(command),
            Err(_) => panic!("mutex should not be poisoned"),
        }),
    });

    assert!(contract.handlers.on_key_down.run("ArrowRight".to_string()));
    assert!(contract.handlers.on_key_down.run("Home".to_string()));
    assert!(!contract.handlers.on_key_down.run("Enter".to_string()));
    let captured = match commands.lock() {
        Ok(commands) => commands,
        Err(_) => panic!("mutex should not be poisoned"),
    };
    assert_eq!(
        *captured,
        vec![
            CarouselKeyCommand::StepForward,
            CarouselKeyCommand::SelectFirst
        ]
    );
}

#[test]
fn carousel_slide_a11y_attrs_map_selected_state_to_aria_hidden() {
    let selected = carousel_slide_a11y_attrs(true);
    let hidden = carousel_slide_a11y_attrs(false);

    assert_eq!(selected.role, "group");
    assert_eq!(selected.aria_roledescription, "slide");
    assert_eq!(selected.aria_hidden, "false");
    assert_eq!(hidden.aria_hidden, "true");
}
