use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Button, ButtonVariant, Coachmark, CoachmarkAssetVariant, OnPress};

pub(super) fn coachmark() -> AnyView {
    let (last_action, set_last_action) = signal("none".to_string());

    let on_primary: OnPress = Callback::new(move |_| set_last_action.set("primary".to_string()));
    let on_secondary: OnPress =
        Callback::new(move |_| set_last_action.set("secondary".to_string()));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_controlled_open_change =
        Callback::new(move |next: bool| set_controlled_open_raw.set(next));
    let toggle_controlled: OnPress = Callback::new(move |_| {
        set_controlled_open_raw.update(|open| *open = !*open);
    });

    let basic_code = r#"<Coachmark
  title=\"Welcome to the tour\".to_string()
  default_open=true
  current_step=2
  total_steps=5
  primary_cta=\"Next\".to_string()
  secondary_cta=\"Back\".to_string()
  asset_variant=CoachmarkAssetVariant::Folder
>
  <div>Tour copy</div>
</Coachmark>"#;

    let controlled_code = r#"let (open, set_open) = signal(false);
let open_signal: Signal<bool> = Signal::derive(move || open.get());

<Coachmark
  title=\"Keyboard shortcuts\".to_string()
  open=open_signal
  on_open_change=Callback::new(move |next| set_open.set(next))
  primary_cta=\"Got it\".to_string()
  shortcut_key=\"K\".to_string()
  modifier_keys=vec![\"⌘\".to_string()]
  asset_src=\"https://picsum.photos/420/260\".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Coachmark"
            slug="coachmark"
            group="Overlays"
            description="Spectrum-compatible Coachmark primitive for guided tours, composed on ContextualHelp/Popover contracts with HeroUI-level spring overlay motion and optional asset + CTA navigation semantics."
        >
            <Playground title="Step + CTA + Asset Variant" code=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <Coachmark
                        title="Welcome to the tour".to_string()
                        default_open=true
                        current_step=2
                        total_steps=5
                        primary_cta="Next".to_string()
                        secondary_cta="Back".to_string()
                        on_primary=on_primary
                        on_secondary=on_secondary
                        asset_variant=CoachmarkAssetVariant::Folder
                        asset_label="Tour folder".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Discover navigation and command surfaces in this guided step."</div>
                            <div class="ui-muted">"Uses contextual popover semantics with footer CTA controls."</div>
                        </div>
                    </Coachmark>
                    <span class="ui-muted">"last action: " {move || last_action.get()}</span>
                </div>
            </Playground>

            <Playground title="Controlled + Image Asset + Actions" code=controlled_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button variant=ButtonVariant::Secondary on_press=toggle_controlled>
                            "Toggle controlled coachmark"
                        </Button>
                        <span class="ui-muted">"open: " {move || controlled_open_raw.get().to_string()}</span>
                    </div>

                    <Coachmark
                        title="Keyboard shortcuts".to_string()
                        open=controlled_open
                        on_open_change=on_controlled_open_change
                        primary_cta="Got it".to_string()
                        secondary_cta="Skip".to_string()
                        shortcut_key="K".to_string()
                        modifier_keys=vec!["⌘".to_string()]
                        asset_src="https://picsum.photos/420/260".to_string()
                        asset_alt="Dashboard preview".to_string()
                        actions=move || {
                            view! {
                                <Button variant=ButtonVariant::Secondary>
                                    "Restart"
                                </Button>
                            }
                        }
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Press ⌘ + K to quickly open command search from anywhere."</div>
                            <div class="ui-muted">"Controlled mode keeps parent state as source of truth."</div>
                        </div>
                    </Coachmark>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
