use leptos::prelude::*;
use ui_components::{
    ActionButton, ActionButtonGroup, ActionButtonSize, ActionMenu, Autocomplete, Button,
    ButtonCopy, ButtonSize, ButtonVariant, CodeBlock, DropdownMenu, FlipButton, FlipDirection,
    Form, Input, InputSize, InputVariant, MenuItemKind, SearchInputButton, ShareButton,
    SharePlatform, ThemeMode, ThemeToggleButton,
};

#[component]
pub fn PortsDemo() -> impl IntoView {
    // Theme toggle
    let (theme_mode, set_theme_mode) = signal(ThemeMode::Light);

    // DropdownMenu demo
    let menu_items = vec![
        "Edit".to_string(),
        "Duplicate".to_string(),
        "Delete".to_string(),
    ];
    let menu_kinds = vec![
        MenuItemKind::Action,
        MenuItemKind::Action,
        MenuItemKind::Action,
    ];
    let (dropdown_open, set_dropdown_open) = signal(false);
    let (action_menu_open, set_action_menu_open) = signal(false);
    let (last_action, set_last_action) = signal(None::<usize>);
    let on_menu_action = Callback::new(move |index: usize| set_last_action.set(Some(index)));
    let on_dropdown_open_change = Callback::new(move |next: bool| set_dropdown_open.set(next));
    let on_action_menu_open_change =
        Callback::new(move |next: bool| set_action_menu_open.set(next));

    // Autocomplete demo
    let autocomplete_items = vec![
        "Apple".to_string(),
        "Apricot".to_string(),
        "Banana".to_string(),
        "Blueberry".to_string(),
        "Cantaloupe".to_string(),
        "Cherry".to_string(),
        "Grape".to_string(),
        "Kiwi".to_string(),
        "Mango".to_string(),
        "Orange".to_string(),
        "Peach".to_string(),
        "Strawberry".to_string(),
        "Watermelon".to_string(),
    ];
    let (auto_selected, set_auto_selected) = signal(None::<usize>);
    let (auto_open, set_auto_open) = signal(false);
    let on_auto_open_change = Callback::new(move |next: bool| set_auto_open.set(next));

    // Form demo
    let (name_value, set_name_value) = signal(String::new());
    let (invalid, set_invalid) = signal(false);

    let is_invalid = Signal::derive(move || invalid.get());

    // ShareButton demo
    let (share_platform, set_share_platform) = signal(None::<SharePlatform>);
    let on_share_icon_press = Callback::new(move |platform: SharePlatform| {
        set_share_platform.set(Some(platform));
    });

    view! {
        <section id="ports" class="demo-card">
            <h2>"Ports (bb/ui-web)"</h2>
            <p>"Form / Autocomplete / DropdownMenu / CodeBlock / ButtonCopy / ThemeToggleButton / ActionButton* / ActionMenu / SearchInputButton / ShareButton / FlipButton"</p>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"ThemeToggleButton"</div>
                    <div class="demo-row">
                        <ThemeToggleButton mode=theme_mode set_mode=set_theme_mode />
                        <span class="demo-kv">
                            {move || format!("mode: {}", theme_mode.get().label())}
                        </span>
                    </div>

                    <div class="demo-divider"></div>

                    <div class="demo-kv">"ButtonCopy"</div>
                    <ButtonCopy text="cargo add rust-ui".to_string() />
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"DropdownMenu"</div>
                    <div class="demo-row">
                        <DropdownMenu
                            id_base="demo-dropdown".to_string()
                            items=menu_items.clone()
                            item_kinds=menu_kinds.clone()
                            on_action=on_menu_action
                            open=dropdown_open.into()
                            on_open_change=on_dropdown_open_change
                        >
                            "Actions"
                        </DropdownMenu>
                        <span class="demo-kv">
                            {move || {
                                let open = dropdown_open.get();
                                let action = last_action
                                    .get()
                                    .map(|idx| format!("action: {idx}"))
                                    .unwrap_or_else(|| "action: none".to_string());
                                format!("open: {open}, {action}")
                            }}
                        </span>
                    </div>

                    <div class="demo-divider"></div>

                    <div class="demo-kv">"CodeBlock"</div>
                    <CodeBlock
                        label="Example".to_string()
                        language="rs".to_string()
                        code="fn main() {\n    println!(\"hello\");\n}\n".to_string()
                    />
                </div>
            </div>

            <div class="demo-divider"></div>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"Autocomplete"</div>
                    <Autocomplete
                        id_base="demo-autocomplete".to_string()
                        label="Fruit".to_string()
                        items=autocomplete_items
                        selected_index=auto_selected
                        set_selected_index=set_auto_selected
                        open=auto_open.into()
                        on_open_change=on_auto_open_change
                    />
                    <div class="demo-kv">
                        {move || {
                            let open = auto_open.get();
                            let selected = auto_selected
                                .get()
                                .map(|idx| format!("selected: {idx}"))
                                .unwrap_or_else(|| "selected: none".to_string());
                            format!("open: {open}, {selected}")
                        }}
                    </div>
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"Form"</div>
                    <Form required=true>
                        <div class="demo-stack">
                            <Input
                                id="demo-form-name".to_string()
                                label="Name".to_string()
                                value=name_value
                                set_value=set_name_value
                                placeholder="Type something…".to_string()
                                is_clearable=true
                                invalid=is_invalid
                                size=InputSize::Md
                                variant=InputVariant::Bordered
                            />
                            <div class="demo-row">
                                <Button
                                    variant=ButtonVariant::Secondary
                                    on_press=Callback::new(move |_| set_invalid.update(|v| *v = !*v))
                                >
                                    {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                                </Button>
                            </div>
                        </div>
                    </Form>
                </div>
            </div>

            <div class="demo-divider"></div>

            <div class="demo-grid-2">
                <div class="demo-stack">
                    <div class="demo-kv">"ActionButton / ActionButtonGroup"</div>
                    <ActionButtonGroup size=ActionButtonSize::M is_justified=true>
                        <ActionButton>"Edit"</ActionButton>
                        <ActionButton is_quiet=true>"Duplicate"</ActionButton>
                        <ActionButton>"Delete"</ActionButton>
                    </ActionButtonGroup>

                    <div class="demo-divider"></div>

                    <div class="demo-kv">"ActionMenu"</div>
                    <div class="demo-row">
                        <ActionMenu
                            id_base="demo-action-menu".to_string()
                            items=menu_items.clone()
                            item_kinds=menu_kinds.clone()
                            on_action=on_menu_action
                            open=action_menu_open.into()
                            on_open_change=on_action_menu_open_change
                            size=ActionButtonSize::M
                            is_quiet=true
                        />
                        <span class="demo-kv">
                            {move || {
                                let open = action_menu_open.get();
                                let action = last_action
                                    .get()
                                    .map(|idx| format!("action: {idx}"))
                                    .unwrap_or_else(|| "action: none".to_string());
                                format!("open: {open}, {action}")
                            }}
                        </span>
                    </div>

                    <div class="demo-divider"></div>

                    <div class="demo-kv">"SearchInputButton"</div>
                    <SearchInputButton
                        placeholder="Search docs...".to_string()
                        compact_placeholder="Search...".to_string()
                        meta_key_label="⌘".to_string()
                        key_label="K".to_string()
                        class_name="demo-search-input-button".to_string()
                    />
                </div>

                <div class="demo-stack">
                    <div class="demo-kv">"ShareButton"</div>
                    <ShareButton
                        size=ButtonSize::Sm
                        from=FlipDirection::Bottom
                        on_icon_press=on_share_icon_press
                    />
                    <div class="demo-kv">
                        {move || match share_platform.get() {
                            None => "clicked: none".to_string(),
                            Some(SharePlatform::Github) => "clicked: GitHub".to_string(),
                            Some(SharePlatform::X) => "clicked: X".to_string(),
                            Some(SharePlatform::Facebook) => "clicked: Facebook".to_string(),
                        }}
                    </div>

                    <div class="demo-divider"></div>

                    <div class="demo-kv">"FlipButton"</div>
                    <FlipButton
                        from=FlipDirection::Right
                        front=move || view! {
                            <Button variant=ButtonVariant::Secondary size=ButtonSize::Sm>"Front"</Button>
                        }
                        back=move || view! {
                            <Button variant=ButtonVariant::Accent size=ButtonSize::Sm>"Back"</Button>
                        }
                    />
                </div>
            </div>
        </section>
    }
}
