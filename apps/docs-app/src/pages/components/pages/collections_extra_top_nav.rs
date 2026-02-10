use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{TopNav, TopNavItem, TopNavMotion};

pub(super) fn top_nav() -> AnyView {
    let default_items = vec![
        TopNavItem::new("brand", "Omne", "/"),
        TopNavItem::new("docs", "Docs", "/docs"),
        TopNavItem::new("components", "Components", "/components"),
        TopNavItem::new("changelog", "Changelog", "/changelog"),
    ];

    let controlled_items = vec![
        TopNavItem::new("home", "Home", "/home"),
        TopNavItem::new("workflows", "Workflows", "/workflows"),
        TopNavItem::new("reports", "Reports", "/reports").disabled(true),
        TopNavItem::new("settings", "Settings", "/settings"),
    ];

    let marker_items = vec![
        TopNavItem::new("brand", "Brand", "/"),
        TopNavItem::new("docs", "Docs", "/docs"),
        TopNavItem::new("api", "API", "/api"),
        TopNavItem::new("status", "Status", "/status"),
    ];

    let (last_selected, set_last_selected) = signal("none".to_string());
    let on_selected_id_change = Callback::new(move |next: Option<String>| {
        set_last_selected.set(next.unwrap_or_else(|| "none".to_string()))
    });

    let (controlled_selected_raw, set_controlled_selected_raw) = signal(Some("home".to_string()));
    let controlled_selected: Signal<Option<String>> =
        Signal::derive(move || controlled_selected_raw.get());
    let on_controlled_selected_change = Callback::new(move |next: Option<String>| {
        set_controlled_selected_raw.set(next);
    });

    let default_code = r#"<TopNav
  id_base=\"docs-top-nav\".to_string()
  items=items
  default_selected_id=\"components\".to_string()
  on_selected_id_change=Callback::new(move |next: Option<String>| {
    set_last_selected.set(next.unwrap_or_else(|| \"none\".to_string()));
  })
/>"#;

    let controlled_code = r#"let (selected, set_selected) = signal(Some(\"home\".to_string()));
let selected_signal: Signal<Option<String>> = Signal::derive(move || selected.get());

<TopNav
  id_base=\"docs-top-nav-controlled\".to_string()
  items=items
  selected_id=selected_signal
  on_selected_id_change=Callback::new(move |next| set_selected.set(next))
  activate_on_focus=false
  label=\"Main application sections\".to_string()
/>"#;

    let markers_code = r#"let mut marker_motion = TopNavMotion::default();
marker_motion.spring.stiffness = 320.0;
marker_motion.spring.damping = 24.0;

<TopNav
  id_base=\"docs-top-nav-markers\".to_string()
  items=items
  default_selected_id=\"docs\".to_string()
  activate_on_focus=false
  label=\"Primary sections\".to_string()
  class_name=\"docs-top-nav-state\".to_string()
  motion=marker_motion
/>"#;

    let marker_motion = {
        let mut motion = TopNavMotion::default();
        motion.spring.stiffness = 320.0;
        motion.spring.damping = 24.0;
        motion
    };

    view! {
        <ComponentPage
            title="TopNav"
            slug="top-nav"
            group="Collections"
            description="Spectrum-compatible TopNav alias for upstream naming parity, preserving NavigationMenu selection/accessibility contracts and HeroUI-level active-indicator spring motion behavior."
        >
            <Playground title="Default Selection + Roving Focus" code=default_code>
                <div class="docs-stack docs-stack--tight">
                    <TopNav
                        id_base="docs-top-nav-default".to_string()
                        items=default_items
                        default_selected_id="components".to_string()
                        on_selected_id_change=on_selected_id_change
                    />
                    <span class="ui-muted">"selected: " {move || last_selected.get()}</span>
                </div>
            </Playground>

            <Playground title="Controlled + Label + Disabled Item" code=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <TopNav
                        id_base="docs-top-nav-controlled".to_string()
                        items=controlled_items
                        selected_id=controlled_selected
                        on_selected_id_change=on_controlled_selected_change
                        activate_on_focus=false
                        label="Main application sections".to_string()
                        class_name="docs-top-nav-custom".to_string()
                    />
                    <span class="ui-muted">
                        "controlled selected: "
                        {move || controlled_selected_raw.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-selection-mode`, `data-default-selection`, `data-focus-activation`, `data-label-source`, `data-class-source`, and `data-motion-source` for TopNav contract stability."
                code=markers_code
            >
                <TopNav
                    id_base="docs-top-nav-markers".to_string()
                    items=marker_items
                    default_selected_id="docs".to_string()
                    activate_on_focus=false
                    label="Primary sections".to_string()
                    class_name="docs-top-nav-state".to_string()
                    motion=marker_motion
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
