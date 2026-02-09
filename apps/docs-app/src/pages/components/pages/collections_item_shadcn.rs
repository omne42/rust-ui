use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
};

pub(super) fn item_primitives() -> AnyView {
    let basic_code = r#"<ItemGroup>
  <Item variant=ItemVariant::Outline size=ItemSize::Default>
    <ItemMedia variant=ItemMediaVariant::Icon>"📦"</ItemMedia>
    <ItemContent>
      <ItemTitle>"Build Artifact"</ItemTitle>
      <ItemDescription>"Generated from latest CI pipeline."</ItemDescription>
    </ItemContent>
    <ItemActions><button>"Open"</button></ItemActions>
  </Item>
</ItemGroup>"#;

    let advanced_code = r#"<Item variant=ItemVariant::Muted size=ItemSize::Sm>
  <ItemHeader>
    <ItemTitle>"Edge deployment"</ItemTitle>
    <ItemActions><button>"Retry"</button></ItemActions>
  </ItemHeader>
  <ItemContent>
    <ItemDescription>"2 minutes ago · US-East"</ItemDescription>
  </ItemContent>
  <ItemFooter>
    <span>"Status: degraded"</span>
  </ItemFooter>
</Item>"#;

    view! {
        <ComponentPage
            title="Item"
            slug="item"
            group="Collections"
            description="Shadcn-compatible item composition primitives (`Item*`) with stable slot/variant/size contracts for media-content-actions and header-footer layouts."
        >
            <Playground title="Media + Content + Actions" code=basic_code>
                <ItemGroup>
                    <Item variant=ItemVariant::Outline size=ItemSize::Default>
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
                    <Item variant=ItemVariant::Default size=ItemSize::Default>
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
            </Playground>

            <Playground title="Header + Footer Layout" code=advanced_code>
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
