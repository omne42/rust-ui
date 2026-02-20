use super::*;

fn pos(
    anchor: Rect,
    panel: Size,
    viewport: Size,
    preferred: PopoverPlacement,
    offset_px: f64,
    padding_px: f64,
) -> ComputedPosition {
    compute_popover_position(anchor, panel, viewport, preferred, offset_px, padding_px)
}

#[test]
fn bottom_start_positions_below_and_aligns_start() {
    let out = pos(
        Rect {
            top: 10.0,
            left: 20.0,
            width: 100.0,
            height: 40.0,
        },
        Size {
            width: 200.0,
            height: 120.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        PopoverPlacement::BottomStart,
        8.0,
        8.0,
    );

    assert_eq!(out.placement, PopoverPlacement::BottomStart);
    assert_eq!(out.anchor_width, 100.0);
    assert!((out.top - 58.0).abs() < 0.0001);
    assert!((out.left - 20.0).abs() < 0.0001);
}

#[test]
fn bottom_end_aligns_end() {
    let out = pos(
        Rect {
            top: 0.0,
            left: 300.0,
            width: 120.0,
            height: 40.0,
        },
        Size {
            width: 200.0,
            height: 100.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        PopoverPlacement::BottomEnd,
        8.0,
        8.0,
    );

    assert_eq!(out.placement, PopoverPlacement::BottomEnd);
    assert!((out.left - 220.0).abs() < 0.0001);
}

#[test]
fn flips_to_top_when_bottom_does_not_fit() {
    let out = pos(
        Rect {
            top: 560.0,
            left: 20.0,
            width: 100.0,
            height: 30.0,
        },
        Size {
            width: 240.0,
            height: 120.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        PopoverPlacement::BottomStart,
        8.0,
        8.0,
    );

    assert_eq!(out.placement, PopoverPlacement::TopStart);
    // top = anchor.top - offset - panel.height = 560 - 8 - 120
    assert!((out.top - 432.0).abs() < 0.0001);
}

#[test]
fn clamps_left_within_viewport_padding() {
    let out = pos(
        Rect {
            top: 10.0,
            left: 760.0,
            width: 60.0,
            height: 40.0,
        },
        Size {
            width: 200.0,
            height: 100.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        PopoverPlacement::BottomStart,
        8.0,
        8.0,
    );

    // max_left = 800 - 200 - 8 = 592
    assert!((out.left - 592.0).abs() < 0.0001);
}

#[test]
fn clamps_top_when_panel_would_overflow() {
    let out = pos(
        Rect {
            top: 580.0,
            left: 20.0,
            width: 100.0,
            height: 10.0,
        },
        Size {
            width: 240.0,
            height: 200.0,
        },
        Size {
            width: 800.0,
            height: 600.0,
        },
        PopoverPlacement::TopStart,
        8.0,
        8.0,
    );

    // Preferred is top, but it doesn't fit above; it will choose bottom because more space below.
    assert!(matches!(
        out.placement,
        PopoverPlacement::TopStart | PopoverPlacement::BottomStart
    ));

    assert!(out.top >= 8.0);
    assert!(out.top <= 392.0); // 600 - 200 - 8
}
