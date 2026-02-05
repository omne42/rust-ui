use leptos::prelude::*;
use ui_components::{MenuItemKind, MenuTrigger};

#[component]
pub fn MenuDemo() -> impl IntoView {
    let menu_items = vec![
        "New".to_string(),
        "Open".to_string(),
        "Save".to_string(),
        "Close".to_string(),
    ];
    let (menu_selected, set_menu_selected) = signal(None::<usize>);
    let menu_selected_label = {
        let menu_items = menu_items.clone();
        move || {
            menu_selected
                .get()
                .and_then(|i| menu_items.get(i).cloned())
                .unwrap_or_else(|| "<none>".to_string())
        }
    };
    let on_menu_action = Callback::new(move |index: usize| set_menu_selected.set(Some(index)));

    let (controlled_open, set_controlled_open) = signal(false);
    let on_menu_open_change = Callback::new(move |next: bool| set_controlled_open.set(next));

    let checkbox_menu_items = vec![
        "Show Grid".to_string(),
        "Show Rulers".to_string(),
        "Snap to Grid".to_string(),
    ];
    let (show_grid, set_show_grid) = signal(false);
    let (show_rulers, set_show_rulers) = signal(true);
    let (snap_to_grid, set_snap_to_grid) = signal(false);
    let on_checkbox_menu_action = Callback::new(move |index: usize| match index {
        0 => set_show_grid.update(|v| *v = !*v),
        1 => set_show_rulers.update(|v| *v = !*v),
        2 => set_snap_to_grid.update(|v| *v = !*v),
        _ => {}
    });
    let checkbox_menu_kinds = vec![
        MenuItemKind::Checkbox {
            is_checked: show_grid.into(),
        },
        MenuItemKind::Checkbox {
            is_checked: show_rulers.into(),
        },
        MenuItemKind::Checkbox {
            is_checked: snap_to_grid.into(),
        },
    ];

    let radio_menu_items = vec![
        "Align Left".to_string(),
        "Align Center".to_string(),
        "Align Right".to_string(),
    ];
    let (alignment, set_alignment) = signal(0_usize);
    let on_radio_menu_action = Callback::new(move |index: usize| set_alignment.set(index));
    let radio_menu_kinds = vec![
        MenuItemKind::Radio {
            is_checked: Signal::derive(move || alignment.get() == 0),
        },
        MenuItemKind::Radio {
            is_checked: Signal::derive(move || alignment.get() == 1),
        },
        MenuItemKind::Radio {
            is_checked: Signal::derive(move || alignment.get() == 2),
        },
    ];

    view! {
        <section id="menu" class="demo-card">
            <h2>"MenuTrigger"</h2>
            <p>
                "Tab to focus; Enter/Space to open; Arrow keys to navigate; Enter/Space to activate. Includes disabled, checkbox, and radio items."
            </p>

            <div class="demo-row">
                <MenuTrigger
                    id_base="demo-menu".to_string()
                    items=menu_items.clone()
                    on_action=on_menu_action
                    disabled_indices=vec![2]
                >
                    "Open Menu"
                </MenuTrigger>
                <div class="demo-kv">
                    "selected: " {menu_selected_label}
                </div>
            </div>

            <div class="demo-row">
                <MenuTrigger
                    id_base="demo-menu-controlled".to_string()
                    items=menu_items.clone()
                    on_action=on_menu_action
                    open=controlled_open.into()
                    on_open_change=on_menu_open_change
                >
                    "Controlled Menu"
                </MenuTrigger>
                <div class="demo-kv">
                    "open: " {move || controlled_open.get().to_string()}
                </div>
            </div>

            <div class="demo-divider"></div>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"Checkbox Menu (stays open)"</div>
                    <MenuTrigger
                        id_base="demo-menu-checkbox".to_string()
                        items=checkbox_menu_items.clone()
                        item_kinds=checkbox_menu_kinds.clone()
                        on_action=on_checkbox_menu_action
                        close_on_action=false
                    >
                        "Options"
                    </MenuTrigger>
                    <div class="demo-kv">
                        "grid: " {move || show_grid.get().to_string()}
                        ", rulers: " {move || show_rulers.get().to_string()}
                        ", snap: " {move || snap_to_grid.get().to_string()}
                    </div>
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"Radio Menu (stays open)"</div>
                    <MenuTrigger
                        id_base="demo-menu-radio".to_string()
                        items=radio_menu_items.clone()
                        item_kinds=radio_menu_kinds.clone()
                        on_action=on_radio_menu_action
                        close_on_action=false
                    >
                        "Alignment"
                    </MenuTrigger>
                    <div class="demo-kv">
                        "selected: " {move || alignment.get().to_string()}
                    </div>
                </div>
            </div>
        </section>
    }
}
