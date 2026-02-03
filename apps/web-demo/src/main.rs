use leptos::{mount::mount_to_body, prelude::*};
use ui_components::{
    provide_focus_visible, provide_overlay_stack, Button, Checkbox, ListBox, MenuItemKind,
    MenuTrigger, Modal, OnPress, Select, Switch, Theme, UiRoot,
};
use ui_core::overlay_trigger::{use_overlay_trigger_state, OverlayTriggerStateOptions};

#[component]
fn App() -> impl IntoView {
    provide_focus_visible();
    provide_overlay_stack();

    let (is_dark, set_is_dark) = signal(false);
    let theme = Signal::derive(move || {
        if is_dark.get() {
            Theme::dark()
        } else {
            Theme::light()
        }
    });
    let toggle_theme: OnPress = Callback::new(move |_| set_is_dark.update(|v| *v = !*v));

    let (count, set_count) = signal(0_i32);
    let on_press: OnPress = Callback::new(move |_| set_count.update(|n| *n += 1));

    let (overlay_state, set_overlay_state) = signal(use_overlay_trigger_state(
        OverlayTriggerStateOptions::default(),
    ));
    let open_overlay: OnPress = Callback::new(move |_| set_overlay_state.update(|s| s.open()));
    let close_overlay: OnPress = Callback::new(move |_| set_overlay_state.update(|s| s.close()));

    let (selected_index, set_selected_index) = signal(None::<usize>);

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

    let select_items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let (select_index, set_select_index) = signal(None::<usize>);

    let (checkbox_enabled, set_checkbox_enabled) = signal(false);
    let (checkbox_checked, set_checkbox_checked) = signal(true);
    let (checkbox_disabled_off, set_checkbox_disabled_off) = signal(false);
    let (checkbox_disabled_on, set_checkbox_disabled_on) = signal(true);

    let (switch_enabled, set_switch_enabled) = signal(false);
    let (switch_checked, set_switch_checked) = signal(true);
    let (switch_disabled_off, set_switch_disabled_off) = signal(false);
    let (switch_disabled_on, set_switch_disabled_on) = signal(true);

    view! {
        <UiRoot theme=theme safe_area=true>
        <div class="demo-shell">
            <header class="demo-header">
                <div>
                    <h1 class="demo-title">"rust-ui"</h1>
                    <div class="demo-subtitle">
                        "Leptos UI primitives: " <code>"ui-core"</code> " / " <code>"ui-headless"</code> " / " <code>"ui-theme"</code> " / " <code>"ui-components"</code>
                    </div>
                </div>
                <Button on_press=toggle_theme>
                    {move || if is_dark.get() { "Light theme" } else { "Dark theme" }}
                </Button>
            </header>

            <div class="demo-layout">
                <nav class="demo-nav">
                    <div class="demo-nav-title">"On this page"</div>
                    <a href="#architecture">"Architecture"</a>
                    <a href="#button">"Button"</a>
                    <a href="#overlay">"Overlay"</a>
                    <a href="#listbox">"ListBox"</a>
                    <a href="#menu">"MenuTrigger"</a>
                    <a href="#select">"Select"</a>
                    <a href="#forms">"Checkbox / Switch"</a>
                </nav>

                <main class="demo-main">
                    <section id="architecture" class="demo-card demo-hero">
                        <h2>"Architecture (Spectrum-like layering)"</h2>
                        <p>
                            "Rule of thumb: keep state pure, keep DOM behavior in headless hooks, keep visuals in components."
                        </p>
                        <ul>
                            <li><code>"ui-core"</code> " — state primitives (controlled/uncontrolled), no DOM."</li>
                            <li><code>"ui-headless"</code> " — interaction + a11y (press/focus-visible/roving tabindex), feature-gated."</li>
                            <li><code>"ui-theme"</code> " — tokens → CSS variables."</li>
                            <li><code>"ui-components"</code> " — composed Leptos components; no direct " <code>"web-sys"</code> " usage."</li>
                        </ul>
                        <div class="demo-divider"></div>
                        <div class="demo-kv">
                            "Try keyboard: " <code>"Tab"</code> ", " <code>"Enter"</code> "/" <code>"Space"</code> ", " <code>"Esc"</code> ". Focus ring should only appear for keyboard navigation."
                        </div>
                    </section>

                    <section id="button" class="demo-card">
                        <h2>"Button"</h2>
                        <p>"Pointer + keyboard press handling, disabled semantics, focus-visible outline."</p>
                        <div class="demo-row">
                            <Button on_press=on_press>"Press Me"</Button>
                            <Button disabled=true>"Disabled"</Button>
                            <Button on_press=open_overlay>"Open Overlay"</Button>
                            <span class="demo-kv">"count: " {count}</span>
                        </div>
                    </section>

                    <Show when=move || overlay_state.get().is_open()>
                        <Modal
                            id_base="demo-modal".to_string()
                            title="Overlay v2".to_string()
                            description="Esc / click outside closes. Tab is trapped; close returns focus.".to_string()
                            on_close=close_overlay
                        >
                            <div class="demo-row" style="justify-content: flex-end;">
                                <Button on_press=close_overlay>"Close"</Button>
                            </div>
                        </Modal>
                    </Show>

                    <section id="overlay" class="demo-card">
                        <h2>"Overlay"</h2>
                        <p>"Esc dismiss + click outside + focus trap (v0) + focus restore."</p>
                        <div class="demo-row">
                            <Button on_press=open_overlay>"Open Modal"</Button>
                            <span class="demo-kv">
                                "open: " {move || overlay_state.get().is_open().to_string()}
                            </span>
                        </div>
                    </section>

                    <section id="listbox" class="demo-card">
                        <h2>"ListBox"</h2>
                        <p>"Roving focus + aria-activedescendant semantics. Includes disabled item."</p>
                        <ListBox
                            id_base="demo-listbox".to_string()
                            items=vec![
                                "First".to_string(),
                                "Second (disabled)".to_string(),
                                "Third".to_string(),
                            ]
                            selected_index=selected_index
                            set_selected_index=set_selected_index
                            disabled_indices=vec![1]
                        />
                        <div class="demo-kv">
                            "selected_index: " {move || format!("{:?}", selected_index.get())}
                        </div>
                    </section>

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

                    <section id="select" class="demo-card">
                        <h2>"Select"</h2>
                        <p>"Composition: Button → Popover → ListBox → select/close. Includes disabled item + typeahead."</p>

                        <div class="demo-row">
                            <Select
                                id_base="demo-select".to_string()
                                items=select_items.clone()
                                selected_index=select_index
                                set_selected_index=set_select_index
                                disabled_indices=vec![3]
                            />
                            <div class="demo-kv">
                                "selected_index: " {move || format!("{:?}", select_index.get())}
                            </div>
                        </div>
                    </section>

                    <section id="forms" class="demo-card">
                        <h2>"Checkbox / Switch"</h2>
                        <p>"Tab to focus; Space to toggle. Focus-visible shows an outline."</p>

                        <div class="demo-grid-2">
                            <div class="demo-stack">
                                <div class="demo-kv">"Checkbox"</div>
                                <Checkbox checked=checkbox_enabled set_checked=set_checkbox_enabled>
                                    "Enabled (interactive)"
                                </Checkbox>
                                <Checkbox checked=checkbox_checked set_checked=set_checkbox_checked>
                                    "Enabled (checked)"
                                </Checkbox>
                                <Checkbox
                                    disabled=true
                                    checked=checkbox_disabled_off
                                    set_checked=set_checkbox_disabled_off
                                >
                                    "Disabled (unchecked)"
                                </Checkbox>
                                <Checkbox
                                    disabled=true
                                    checked=checkbox_disabled_on
                                    set_checked=set_checkbox_disabled_on
                                >
                                    "Disabled (checked)"
                                </Checkbox>
                                <div class="demo-kv">
                                    "enabled checked: " {move || checkbox_enabled.get().to_string()}
                                </div>
                            </div>

                            <div class="demo-stack">
                                <div class="demo-kv">"Switch"</div>
                                <Switch checked=switch_enabled set_checked=set_switch_enabled>
                                    "Enabled (interactive)"
                                </Switch>
                                <Switch checked=switch_checked set_checked=set_switch_checked>
                                    "Enabled (checked)"
                                </Switch>
                                <Switch
                                    disabled=true
                                    checked=switch_disabled_off
                                    set_checked=set_switch_disabled_off
                                >
                                    "Disabled (unchecked)"
                                </Switch>
                                <Switch
                                    disabled=true
                                    checked=switch_disabled_on
                                    set_checked=set_switch_disabled_on
                                >
                                    "Disabled (checked)"
                                </Switch>
                                <div class="demo-kv">
                                    "enabled checked: " {move || switch_enabled.get().to_string()}
                                </div>
                            </div>
                        </div>
                    </section>
                </main>
            </div>
        </div>
        </UiRoot>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
