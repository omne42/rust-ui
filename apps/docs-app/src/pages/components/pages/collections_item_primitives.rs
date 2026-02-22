use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant, SegmentedControl,
    SegmentedControlSize,
};

const ITEM_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant};";

pub(super) fn item_primitives() -> AnyView {
    let variant_options = vec![
        "default".to_string(),
        "outline".to_string(),
        "muted".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ItemVariant::Outline,
        2 => ItemVariant::Muted,
        _ => ItemVariant::Default,
    });

    let size_options = vec!["m".to_string(), "s".to_string()];
    let (size_index, set_size_index) = signal(Some(0_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(0) {
        1 => ItemSize::Sm,
        _ => ItemSize::Default,
    });

    let basic_code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();

        let mut snippet = vec!["<ItemGroup>".to_string(), "  <Item".to_string()];

        if variant != ItemVariant::Default {
            snippet.push(format!("    variant=ItemVariant::{variant:?}"));
        }
        if size != ItemSize::Default {
            snippet.push(format!("    size=ItemSize::{size:?}"));
        }

        snippet.extend([
            "  >".to_string(),
            "    <ItemMedia variant=ItemMediaVariant::Icon>\"📦\"</ItemMedia>".to_string(),
            "    <ItemContent>".to_string(),
            "      <ItemTitle>\"Build Artifact\"</ItemTitle>".to_string(),
            "      <ItemDescription>\"Generated from latest CI pipeline.\"</ItemDescription>"
                .to_string(),
            "    </ItemContent>".to_string(),
            "    <ItemActions><button>\"Open\"</button></ItemActions>".to_string(),
            "  </Item>".to_string(),
            "</ItemGroup>".to_string(),
        ]);

        snippet.join("\n")
    });

    let advanced_code = Signal::derive(move || {
        [
            "<Item variant=ItemVariant::Muted size=ItemSize::Sm>".to_string(),
            "  <ItemHeader>".to_string(),
            "    <ItemTitle>\"Edge deployment\"</ItemTitle>".to_string(),
            "    <ItemActions><button>\"Retry\"</button></ItemActions>".to_string(),
            "  </ItemHeader>".to_string(),
            "  <ItemContent>".to_string(),
            "    <ItemDescription>\"2 minutes ago · US-East\"</ItemDescription>".to_string(),
            "  </ItemContent>".to_string(),
            "  <ItemFooter>".to_string(),
            "    <span>\"Status: degraded\"</span>".to_string(),
            "  </ItemFooter>".to_string(),
            "</Item>".to_string(),
        ]
        .join("\n")
    });

    let hello_code = Signal::derive(move || {
        [
            "<ItemGroup>".to_string(),
            "  <Item>".to_string(),
            "    <ItemTitle>\"Hello Item\"</ItemTitle>".to_string(),
            "  </Item>".to_string(),
            "</ItemGroup>".to_string(),
        ]
        .join("\n")
    });

    let state_matrix_code = Signal::derive(move || {
        [
            "<ItemGroup>".to_string(),
            "  <Item variant=ItemVariant::Default size=ItemSize::Default>".to_string(),
            "    <ItemTitle>\"Default / M\"</ItemTitle>".to_string(),
            "  </Item>".to_string(),
            "  <Item variant=ItemVariant::Outline size=ItemSize::Default>".to_string(),
            "    <ItemTitle>\"Outline / M\"</ItemTitle>".to_string(),
            "  </Item>".to_string(),
            "  <Item variant=ItemVariant::Muted size=ItemSize::Sm>".to_string(),
            "    <ItemTitle>\"Muted / S\"</ItemTitle>".to_string(),
            "  </Item>".to_string(),
            "</ItemGroup>".to_string(),
        ]
        .join("\n")
    });

    let controlled_na_code = Signal::derive(move || {
        [
            "// Item has no controllable value axis (no value/on_value_change/default_value triad)."
                .to_string(),
            "// Parent renders a full snapshot props set each time.".to_string(),
            "<Item variant=ItemVariant::Outline size=ItemSize::Default>".to_string(),
            "  <ItemTitle>\"Controlled vs Uncontrolled: N/A\"</ItemTitle>".to_string(),
            "</Item>".to_string(),
        ]
        .join("\n")
    });

    let streaming_snapshot_code = Signal::derive(move || {
        [
            "// Item is snapshot-first. Streaming input is optional and falls back to snapshot rendering."
                .to_string(),
            "<ItemGroup>".to_string(),
            "  <Item>".to_string(),
            "    <ItemTitle>\"Streaming feed (fallback=snapshot)\"</ItemTitle>".to_string(),
            "  </Item>".to_string(),
            "  <Item>".to_string(),
            "    <ItemTitle>\"Snapshot\"</ItemTitle>".to_string(),
            "  </Item>".to_string(),
            "</ItemGroup>".to_string(),
        ]
        .join("\n")
    });
    let basic_actual_config = Signal::derive(move || {
        format!(
            "ItemWorkbenchConfig {{\n  variant: {:?},\n  size: {:?},\n}}",
            variant.get(),
            size.get(),
        )
    });

    view! {
        <ComponentPage
            title="Item"
            slug="item"
            group="Collections"
            description="baseline-compatible item composition primitives (`Item*`) with stable slot/variant/size contracts for media-content-actions and header-footer layouts."
        >
            <section class="docs-card docs-prose" data-slot="item-doc-onboarding">
                <p>
                    "Start with "
                    <strong>"Hello World"</strong>
                    " to get a running Item immediately, then move to state and layout scenarios."
                </p>
                <p>
                    "Default API path comes first; advanced combinations (state matrix, header/footer, and streaming markers) follow after."
                </p>
            </section>

            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=ITEM_DOC_IMPORTS.to_string()
            >
                <ItemGroup>
                    <Item>
                        <ItemTitle>
                            "Hello Item"
                        </ItemTitle>
                    </Item>
                </ItemGroup>
            </Playground>

            <Playground
                title="Media + Content + Actions"
                code_signal=basic_code
                test_config_signal=basic_actual_config
                code_imports=ITEM_DOC_IMPORTS.to_string()
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-item-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Item variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-item-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Item size".to_string()
                        />
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let size = size.get();

                    view! {
                        <ItemGroup>
                            <Item variant=variant size=size>
                                <ItemMedia variant=ItemMediaVariant::Icon>
                                    "📦"
                                </ItemMedia>
                                <ItemContent>
                                    <ItemTitle>
                                        "Build Artifact"
                                    </ItemTitle>
                                    <ItemDescription>
                                        "Generated from latest CI pipeline."
                                    </ItemDescription>
                                </ItemContent>
                                <ItemActions>
                                    <button>"Open"</button>
                                </ItemActions>
                            </Item>
                            <ItemSeparator />
                            <Item>
                                <ItemMedia variant=ItemMediaVariant::Image>
                                    <span>"🖼"</span>
                                </ItemMedia>
                                <ItemContent>
                                    <ItemTitle>
                                        "Preview Image"
                                    </ItemTitle>
                                    <ItemDescription>
                                        "Updated by Design team."
                                    </ItemDescription>
                                </ItemContent>
                            </Item>
                        </ItemGroup>
                    }
                }}
            </Playground>

            <Playground
                title="Header + Footer Layout"
                code_signal=advanced_code
                code_imports=ITEM_DOC_IMPORTS.to_string()
            >
                <Item variant=ItemVariant::Muted size=ItemSize::Sm>
                    <ItemHeader>
                        <ItemTitle>
                            "Edge deployment"
                        </ItemTitle>
                        <ItemActions>
                            <button>"Retry"</button>
                        </ItemActions>
                    </ItemHeader>
                    <ItemContent>
                        <ItemDescription>
                            "2 minutes ago · US-East"
                        </ItemDescription>
                    </ItemContent>
                    <ItemFooter>
                        <span>"Status: degraded"</span>
                    </ItemFooter>
                </Item>
            </Playground>

            <Playground
                title="State Matrix (Variant + Size)"
                code_signal=state_matrix_code
                code_imports=ITEM_DOC_IMPORTS.to_string()
            >
                <ItemGroup>
                    <Item variant=ItemVariant::Default size=ItemSize::Default>
                        <ItemTitle>"Default / M"</ItemTitle>
                    </Item>
                    <ItemSeparator />
                    <Item variant=ItemVariant::Outline size=ItemSize::Default>
                        <ItemTitle>"Outline / M"</ItemTitle>
                    </Item>
                    <ItemSeparator />
                    <Item variant=ItemVariant::Muted size=ItemSize::Sm>
                        <ItemTitle>"Muted / S"</ItemTitle>
                    </Item>
                </ItemGroup>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A for Item)"
                code_signal=controlled_na_code
                code_imports=ITEM_DOC_IMPORTS.to_string()
                description="Item is a static composition primitive with no value/on_change/default_value axis."
            >
                <Item variant=ItemVariant::Outline>
                    <ItemTitle>"Controlled vs Uncontrolled: N/A"</ItemTitle>
                    <ItemDescription>
                        "Parent passes a full snapshot props set each render."
                    </ItemDescription>
                </Item>
            </Playground>

            <Playground
                title="Streaming / Snapshot Display"
                code_signal=streaming_snapshot_code
                code_imports=ITEM_DOC_IMPORTS.to_string()
                description="Item is snapshot-first; streaming stays optional with snapshot fallback."
            >
                <ItemGroup>
                    <Item>
                        <ItemTitle>"Streaming feed (fallback=snapshot)"</ItemTitle>
                        <ItemDescription>
                            "Shows stable markers while upstream output is still generating."
                        </ItemDescription>
                    </Item>
                    <ItemSeparator />
                    <Item>
                        <ItemTitle>"Snapshot"</ItemTitle>
                        <ItemDescription>
                            "Final output renders through the same semantic contract."
                        </ItemDescription>
                    </Item>
                </ItemGroup>
            </Playground>

            <section class="docs-card docs-prose" data-slot="item-copy-ready-hint">
                <p>
                    "Playground code panel supports one-click copy, and copied snippets auto-inject missing imports via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p>
                    "Source location: "
                    <code>"components/item/src/view.rs"</code>
                    " + "
                    <code>"components/item/src/logic.rs"</code>
                    "."
                </p>
                <p>
                    "Dependency prerequisite: use "
                    <code>"ui::{Item, ItemGroup, ...}"</code>
                    " from docs snippet imports (package mode: enable feature "
                    <code>"component-item"</code>
                    ")."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
