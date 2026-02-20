use super::*;

fn pos(
    anchor: Rect,
    panel: Size,
    viewport: Size,
    preferred: TooltipPlacement,
    offset_px: f64,
    padding_px: f64,
) -> ComputedPosition {
    compute_tooltip_position(anchor, panel, viewport, preferred, offset_px, padding_px)
}

#[test]
fn top_centers_over_anchor() {
    let out = pos(
        Rect {
            top: 100.0,
            left: 200.0,
            width: 80.0,
            height: 20.0,
        },
        Size {
            width: 120.0,
            height: 40.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        TooltipPlacement::Top,
        7.0,
        8.0,
    );

    assert_eq!(out.placement, TooltipPlacement::Top);
    assert!((out.top - 53.0).abs() < 0.0001);
    assert!((out.left - 180.0).abs() < 0.0001);
}

#[test]
fn bottom_centers_over_anchor() {
    let out = pos(
        Rect {
            top: 100.0,
            left: 200.0,
            width: 80.0,
            height: 20.0,
        },
        Size {
            width: 120.0,
            height: 40.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        TooltipPlacement::Bottom,
        7.0,
        8.0,
    );

    assert_eq!(out.placement, TooltipPlacement::Bottom);
    assert!((out.top - 127.0).abs() < 0.0001);
    assert!((out.left - 180.0).abs() < 0.0001);
}

#[test]
fn flips_to_bottom_when_not_enough_space_above() {
    let out = pos(
        Rect {
            top: 10.0,
            left: 100.0,
            width: 50.0,
            height: 20.0,
        },
        Size {
            width: 100.0,
            height: 60.0,
        },
        Size {
            width: 300.0,
            height: 200.0,
        },
        TooltipPlacement::Top,
        7.0,
        8.0,
    );

    assert_eq!(out.placement, TooltipPlacement::Bottom);
    assert!((out.top - 37.0).abs() < 0.0001);
}

#[test]
fn flips_to_top_when_not_enough_space_below() {
    let out = pos(
        Rect {
            top: 170.0,
            left: 100.0,
            width: 50.0,
            height: 20.0,
        },
        Size {
            width: 100.0,
            height: 60.0,
        },
        Size {
            width: 300.0,
            height: 200.0,
        },
        TooltipPlacement::Bottom,
        7.0,
        8.0,
    );

    assert_eq!(out.placement, TooltipPlacement::Top);
    assert!((out.top - 103.0).abs() < 0.0001);
}

#[test]
fn clamps_left_within_viewport_padding() {
    let out = pos(
        Rect {
            top: 100.0,
            left: 0.0,
            width: 20.0,
            height: 20.0,
        },
        Size {
            width: 180.0,
            height: 40.0,
        },
        Size {
            width: 200.0,
            height: 200.0,
        },
        TooltipPlacement::Bottom,
        7.0,
        8.0,
    );

    assert!((out.left - 8.0).abs() < 0.0001);
}

#[test]
fn clamps_top_within_viewport_padding() {
    let out = pos(
        Rect {
            top: 2.0,
            left: 100.0,
            width: 50.0,
            height: 20.0,
        },
        Size {
            width: 120.0,
            height: 80.0,
        },
        Size {
            width: 300.0,
            height: 100.0,
        },
        TooltipPlacement::Top,
        7.0,
        8.0,
    );

    assert!(out.top >= 8.0);
}
