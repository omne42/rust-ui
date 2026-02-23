use super::*;

#[test]
fn sanitize_config_falls_back_for_non_finite_or_non_positive_values() {
    let fallback = SpringConfig::default();
    let sanitized = sanitize_config(
        SpringConfig {
            stiffness: f64::NAN,
            damping: -1.0,
            mass: 0.0,
            precision: f64::INFINITY,
        },
        fallback,
    );

    assert_eq!(sanitized.stiffness, fallback.stiffness);
    assert_eq!(sanitized.damping, fallback.damping);
    assert_eq!(sanitized.mass, fallback.mass);
    assert_eq!(sanitized.precision, fallback.precision);
}

#[test]
fn sanitize_config_keeps_valid_values() {
    let fallback = SpringConfig::default();
    let sanitized = sanitize_config(
        SpringConfig {
            stiffness: 240.0,
            damping: 22.0,
            mass: 1.1,
            precision: 0.002,
        },
        fallback,
    );

    assert_eq!(sanitized.stiffness, 240.0);
    assert_eq!(sanitized.damping, 22.0);
    assert_eq!(sanitized.mass, 1.1);
    assert_eq!(sanitized.precision, 0.002);
}

#[test]
fn spring_animator_triplet_updates_all_channels() {
    use std::cell::Cell;
    use std::rc::Rc;

    let first = Rc::new(Cell::new(0.0));
    let second = Rc::new(Cell::new(0.0));
    let third = Rc::new(Cell::new(0.0));

    let first_out = Rc::clone(&first);
    let second_out = Rc::clone(&second);
    let third_out = Rc::clone(&third);

    let triplet = SpringAnimatorTriplet::new(
        [0.0, 1.0, 2.0],
        SpringConfig::default(),
        move |value| first_out.set(value),
        move |value| second_out.set(value),
        move |value| third_out.set(value),
    );

    triplet.set_targets([10.0, 11.0, 12.0]);

    assert_eq!(first.get(), 10.0);
    assert_eq!(second.get(), 11.0);
    assert_eq!(third.get(), 12.0);
}

#[test]
fn spring_animator_triplet_supports_on_rest_and_clear_on_rest() {
    use std::cell::Cell;
    use std::rc::Rc;

    let second = Rc::new(Cell::new(0.0));
    let rest_calls = Rc::new(Cell::new(0_u32));

    let second_out = Rc::clone(&second);

    let triplet = SpringAnimatorTriplet::new(
        [0.0, 0.0, 0.0],
        SpringConfig::default(),
        |_| {},
        move |value| second_out.set(value),
        |_| {},
    );

    let rest_calls_for_hook = Rc::clone(&rest_calls);
    triplet.set_on_rest_second(move || {
        rest_calls_for_hook.set(rest_calls_for_hook.get() + 1);
    });
    triplet.set_targets([1.0, 2.0, 3.0]);

    assert_eq!(second.get(), 2.0);
    assert_eq!(rest_calls.get(), 1);

    triplet.clear_on_rest();
    triplet.set_targets([4.0, 5.0, 6.0]);

    assert_eq!(second.get(), 5.0);
    assert_eq!(rest_calls.get(), 1);
}
