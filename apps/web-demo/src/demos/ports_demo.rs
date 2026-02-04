use leptos::prelude::*;
use ui_components::{
    Autocomplete, Button, ButtonCopy, ButtonVariant, CodeBlock, DropdownMenu, Form, Input,
    InputSize, InputVariant, MenuItemKind, ThemeMode, ThemeToggleButton,
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
    let (last_action, set_last_action) = signal(None::<usize>);
    let on_menu_action = Callback::new(move |index: usize| set_last_action.set(Some(index)));

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

    // Form demo
    let (name_value, set_name_value) = signal(String::new());
    let (invalid, set_invalid) = signal(false);

    let is_invalid = Signal::derive(move || invalid.get());

    view! {
        <section id="ports" class="demo-card">
            <h2>"Ports (bb/ui-web)"</h2>
            <p>"Form / Autocomplete / DropdownMenu / CodeBlock / ButtonCopy / ThemeToggleButton"</p>

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
                            items=menu_items
                            item_kinds=menu_kinds
                            on_action=on_menu_action
                        >
                            "Actions"
                        </DropdownMenu>
                        <span class="demo-kv">
                            {move || last_action.get().map(|idx| format!("action: {idx}")).unwrap_or_else(|| "action: none".to_string())}
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
                    />
                    <div class="demo-kv">
                        {move || {
                            auto_selected
                                .get()
                                .map(|idx| format!("selected: {idx}"))
                                .unwrap_or_else(|| "selected: none".to_string())
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
        </section>
    }
}
