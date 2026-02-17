use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant, SegmentedControl,
    SegmentedControlSize,
};

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
        vec![
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

    view! {
        <ComponentPage
            title="Item"
            slug="item"
            group="Collections"
            description="baseline-compatible item composition primitives (`Item*`) with stable slot/variant/size contracts for media-content-actions and header-footer layouts."
        >
            <Playground
                title="Media + Content + Actions"
                code_signal=basic_code
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

            <Playground title="Header + Footer Layout" code_signal=advanced_code>
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
        </ComponentPage>
    }
    .into_any()
}
