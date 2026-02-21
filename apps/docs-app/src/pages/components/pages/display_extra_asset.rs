use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Asset, AssetSize, AssetVariant};

const ASSET_PLAYGROUND_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui_components::{Asset, AssetSize, AssetVariant};";

pub(super) fn asset() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Asset />"#.to_string());

    let variant_code = Signal::derive(move || {
        r#"<Asset variant=AssetVariant::File size=AssetSize::Size600 label=\"Build Report\".into() />
<Asset variant=AssetVariant::Folder size=AssetSize::Size600 label=\"Design Assets\".into() />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Asset size=AssetSize::Size700 is_selected=true is_focused=true>
  <img src=\"https://picsum.photos/420/280\" alt=\"Preview image\" />
</Asset>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<Asset
  variant=AssetVariant::Custom
  size=AssetSize::Size800
  label=\"Featured Artwork\".into()
  lang=\"en\".into()
  dir=\"ltr\".into()
  is_selected=true
  is_focused=true
  class_name=\"docs-asset-state\".into()
>
  <img src=\"https://picsum.photos/500/360\" alt=\"Cover artwork\" />
</Asset>"#
            .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"<div class="docs-row">
  <Asset label="Controlled (external props)".into() is_selected=true is_focused=true />
  <Asset label="Uncontrolled axis: N/A".into() is_selected=false is_focused=false />
</div>

// Asset has no internal selection/focus state machine.
// It only consumes external props (`is_selected` / `is_focused`)."#
            .to_string()
    });

    let stream_policy_code = Signal::derive(move || {
        r#"<Asset
  variant=AssetVariant::Custom
  size=AssetSize::Size700
  label="Snapshot output".into()
>
  <img src="https://picsum.photos/360/240" alt="Snapshot preview" />
</Asset>

// Asset stream policy markers are rendered by default:
// data-ui-stream-support="optional"
// data-ui-stream-fallback="snapshot"
// data-ui-output-status="verified""#
            .to_string()
    });

    let (interactive_variant_key, set_interactive_variant_key) = signal("custom".to_string());
    let (interactive_size_key, set_interactive_size_key) = signal("700".to_string());
    let (interactive_selected, set_interactive_selected) = signal(false);
    let (interactive_focused, set_interactive_focused) = signal(false);
    let (interactive_use_custom_slot, set_interactive_use_custom_slot) = signal(true);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_label, set_interactive_label) = signal("Interactive Asset".to_string());

    let interactive_variant =
        Signal::derive(move || match interactive_variant_key.get().as_str() {
            "file" => AssetVariant::File,
            "folder" => AssetVariant::Folder,
            _ => AssetVariant::Custom,
        });
    let interactive_size = Signal::derive(move || match interactive_size_key.get().as_str() {
        "500" => AssetSize::Size500,
        "600" => AssetSize::Size600,
        "700" => AssetSize::Size700,
        "800" => AssetSize::Size800,
        _ => AssetSize::Size700,
    });

    let interactive_code = Signal::derive(move || {
        let variant_token = match interactive_variant_key.get().as_str() {
            "file" => "File",
            "folder" => "Folder",
            _ => "Custom",
        };
        let size_token = match interactive_size_key.get().as_str() {
            "500" => "Size500",
            "600" => "Size600",
            "800" => "Size800",
            _ => "Size700",
        };
        let escaped_label = interactive_label.get().replace('"', "\\\"");
        let mut lines = vec![
            "<Asset".to_string(),
            format!("  variant=AssetVariant::{variant_token}"),
            format!("  size=AssetSize::{size_token}"),
            format!("  label=\"{escaped_label}\".into()"),
            format!("  is_selected={}", interactive_selected.get()),
            format!("  is_focused={}", interactive_focused.get()),
        ];
        if interactive_custom_class.get() {
            lines.push("  class_name=\"docs-asset-interactive\".into()".to_string());
        }
        lines.push(">".to_string());
        if interactive_use_custom_slot.get() && interactive_variant_key.get() == "custom" {
            lines.push(
                "  <img src=\"https://picsum.photos/420/280\" alt=\"Interactive preview\" />"
                    .to_string(),
            );
        }
        lines.push("</Asset>".to_string());
        lines.join("\n")
    });

    let interactive_spec_preview = Signal::derive(move || {
        let class_name = if interactive_custom_class.get() {
            "\"docs-asset-interactive\""
        } else {
            "None"
        };
        let content_mode =
            if interactive_use_custom_slot.get() && interactive_variant_key.get() == "custom" {
                "custom-slot"
            } else {
                "builtin-icon"
            };
        format!(
            "AssetComponentSpecInput {{\n  variant: \"{}\",\n  size: {},\n  label: \"{}\",\n  is_selected: {},\n  is_focused: {},\n  class_name: {},\n  content_mode: \"{}\",\n}}",
            interactive_variant_key.get(),
            interactive_size_key.get(),
            interactive_label.get(),
            interactive_selected.get(),
            interactive_focused.get(),
            class_name,
            content_mode
        )
    });

    view! {
        <ComponentPage
            title="Asset"
            slug="asset"
            group="Display"
            description="baseline-compatible Asset primitive for file/folder/custom media representation, composed on Thumbnail state contracts with baseline-level spring focus-selection motion reuse."
        >
            <Playground
                title="Hello World (Default Path)"
                description="Single-line default usage: no state wiring, no headless/state-primitives setup."
                code_signal=hello_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
            >
                <Asset />
            </Playground>

            <Playground
                title="File + Folder Variants"
                code_signal=variant_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Asset
                        variant=AssetVariant::File
                        size=AssetSize::Size600
                        label="Build Report".to_string()
                    />
                    <Asset
                        variant=AssetVariant::Folder
                        size=AssetSize::Size600
                        label="Design Assets".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Custom Image + Focused State"
                code_signal=custom_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
            >
                <Asset size=AssetSize::Size700 is_selected=true is_focused=true>
                    <img src="https://picsum.photos/420/280" alt="Preview image" />
                </Asset>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-label-source`, `data-content-source`, and `data-class-source` on the Asset root for baseline-compatible style/source contracts."
                code_signal=state_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
            >
                <Asset
                    variant=AssetVariant::Custom
                    size=AssetSize::Size800
                    label="Featured Artwork".to_string()
                    lang="en".to_string()
                    dir="ltr".to_string()
                    is_selected=true
                    is_focused=true
                    class_name="docs-asset-state".to_string()
                >
                    <img src="https://picsum.photos/500/360" alt="Cover artwork" />
                </Asset>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A Axis)"
                description="Asset does not expose a controlled/uncontrolled toggle contract. Selection and focus are external props only."
                code_signal=controlled_uncontrolled_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Asset label="Controlled (external props)".to_string() is_selected=true is_focused=true />
                    <Asset label="Uncontrolled axis: N/A".to_string() is_selected=false is_focused=false />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional + Snapshot Fallback"
                description="Asset is not a text streaming surface. It renders full snapshot content while exposing optional-streaming/snapshot-fallback status markers."
                code_signal=stream_policy_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
            >
                <Asset
                    variant=AssetVariant::Custom
                    size=AssetSize::Size700
                    label="Snapshot output".to_string()
                >
                    <img src="https://picsum.photos/360/240" alt="Snapshot preview" />
                </Asset>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Spec Preview)"
                description="Edit props/state and observe semantic markers in real time. Includes spec-input to preview-output linkage and repeatable flow checkpoints."
                code_signal=interactive_code
                code_imports=ASSET_PLAYGROUND_IMPORTS.to_string()
                test_config_signal=interactive_spec_preview
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="asset-interactive-controls">
                        <label class="docs-search__label">
                            "Variant"
                            <select
                                prop:value=move || interactive_variant_key.get()
                                on:change=move |ev| set_interactive_variant_key.set(event_target_value(&ev))
                            >
                                <option value="custom">"Custom"</option>
                                <option value="file">"File"</option>
                                <option value="folder">"Folder"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || interactive_size_key.get()
                                on:change=move |ev| set_interactive_size_key.set(event_target_value(&ev))
                            >
                                <option value="500">"500"</option>
                                <option value="600">"600"</option>
                                <option value="700">"700"</option>
                                <option value="800">"800"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Label"
                            <input
                                type="text"
                                prop:value=move || interactive_label.get()
                                on:input=move |ev| set_interactive_label.set(event_target_value(&ev))
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || interactive_selected.get()
                                on:change=move |ev| set_interactive_selected.set(event_target_checked(&ev))
                            />
                            " Selected"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || interactive_focused.get()
                                on:change=move |ev| set_interactive_focused.set(event_target_checked(&ev))
                            />
                            " Focused"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || interactive_use_custom_slot.get()
                                on:change=move |ev| set_interactive_use_custom_slot.set(event_target_checked(&ev))
                            />
                            " Use custom slot"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || interactive_custom_class.get()
                                on:change=move |ev| set_interactive_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="asset-interactive-preview">
                    <span class="ui-muted">
                        "Repeatable flow: Selected on -> Focused on -> Variant folder -> back to custom."
                    </span>
                    {move || {
                        let class_name = if interactive_custom_class.get() {
                            "docs-asset-interactive".to_string()
                        } else {
                            String::new()
                        };

                        if interactive_use_custom_slot.get() && interactive_variant_key.get() == "custom" {
                            view! {
                                <Asset
                                    variant=interactive_variant.get()
                                    size=interactive_size.get()
                                    label=interactive_label.get()
                                    is_selected=interactive_selected.get()
                                    is_focused=interactive_focused.get()
                                    class_name=class_name
                                >
                                    <img src="https://picsum.photos/420/280" alt="Interactive preview" />
                                </Asset>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Asset
                                    variant=interactive_variant.get()
                                    size=interactive_size.get()
                                    label=interactive_label.get()
                                    is_selected=interactive_selected.get()
                                    is_focused=interactive_focused.get()
                                    class_name=class_name
                                />
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "variant: " {move || interactive_variant_key.get()}
                        " · size: " {move || interactive_size_key.get()}
                        " · selected: " {move || interactive_selected.get()}
                        " · focused: " {move || interactive_focused.get()}
                    </span>
                    <pre class="docs-code" data-slot="asset-interactive-spec-preview">
                        {move || interactive_spec_preview.get()}
                    </pre>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="asset-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p>
                    "Use any Asset Playground's "
                    <code>"Show code"</code>
                    " panel to get a copy-ready snippet. Imports are auto-composed so pasted code runs directly in docs-app/workspace context."
                </p>
                <ul data-slot="asset-source-first-paths">
                    <li><code>"components/asset/src/mod.rs"</code>" (public exports)"</li>
                    <li><code>"components/asset/src/view.rs"</code>" (Leptos structure + semantic mount)"</li>
                    <li><code>"components/asset/src/logic.rs"</code>" (state normalization + source markers)"</li>
                    <li><code>"components/asset/src/styles.rs"</code>" (token-first css contract)"</li>
                    <li><code>"components/asset/src/motion.rs"</code>" (motion contract mapping)"</li>
                    <li><code>"components/asset/src/protocol.rs"</code>" (schema/serde contract)"</li>
                    <li><code>"crates/ui-components/src/lib.rs"</code>" (feature-gated re-export entry)"</li>
                </ul>
                <p data-slot="asset-source-first-prerequisites">
                    "Dependency prerequisites: enable "
                    <code>"ui-components"</code>
                    " with "
                    <code>"component-asset"</code>
                    " in package mode, or import from "
                    <code>"components/asset"</code>
                    " in source mode."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="asset-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="asset-state-rows">
                    <li><code>"variant axis"</code>" = file | folder | custom"</li>
                    <li><code>"size axis"</code>" = 50..1000 (docs matrix samples: 600 | 700 | 800)"</li>
                    <li><code>"data-state"</code>" = selected | focused | default"</li>
                    <li><code>"control mode"</code>" = external-only (no uncontrolled state machine in Asset)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Asset has no disabled prop in public API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="asset-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="asset-parameter-rows">
                    <li><code>"variant: AssetVariant"</code>" default = AssetVariant::Custom"</li>
                    <li><code>"size: AssetSize"</code>" default = AssetSize::Size500"</li>
                    <li><code>"is_selected / is_focused: bool"</code>" default = false"</li>
                    <li><code>"label: Option&lt;String&gt;"</code>" fallback = File | Folder | Asset via resolve_label"</li>
                    <li><code>"class_name / lang / dir: Option&lt;String&gt;"</code>" default = None"</li>
                    <li><code>"motion: AssetMotion"</code>" default = AssetMotion::default()"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
