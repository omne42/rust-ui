use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};

pub(super) fn empty() -> AnyView {
    let basic_code = r#"<Empty>
  <EmptyHeader>
    <EmptyMedia variant=EmptyMediaVariant::Icon>"📭"</EmptyMedia>
    <EmptyTitle>"No messages"</EmptyTitle>
    <EmptyDescription>"You're all caught up."</EmptyDescription>
  </EmptyHeader>
</Empty>"#;

    let content_code = r##"<Empty class_name="docs-empty-custom".to_string()>
  <EmptyHeader>
    <EmptyTitle>"No deployments"</EmptyTitle>
    <EmptyDescription>"Create your first release to populate this list."</EmptyDescription>
  </EmptyHeader>
  <EmptyContent>
    <a href="#/components/button">"Create deployment"</a>
  </EmptyContent>
</Empty>"##;

    view! {
        <ComponentPage
            title="Empty"
            slug="empty"
            group="Display"
            description="Shadcn-compatible empty-state composition primitives (`Empty*`) with stable slot contracts for header/media/title/description/content layering."
        >
            <Playground title="Header + Icon Variant" code=basic_code>
                <Empty>
                    <EmptyHeader>
                        <EmptyMedia variant=EmptyMediaVariant::Icon>
                            "📭"
                        </EmptyMedia>
                        <EmptyTitle>
                            "No messages"
                        </EmptyTitle>
                        <EmptyDescription>
                            "You're all caught up."
                        </EmptyDescription>
                    </EmptyHeader>
                </Empty>
            </Playground>

            <Playground title="Content Action Region" code=content_code>
                <Empty class_name="docs-empty-custom".to_string()>
                    <EmptyHeader>
                        <EmptyTitle>
                            "No deployments"
                        </EmptyTitle>
                        <EmptyDescription>
                            "Create your first release to populate this list."
                        </EmptyDescription>
                    </EmptyHeader>
                    <EmptyContent>
                        <a href="#/components/button">"Create deployment"</a>
                    </EmptyContent>
                </Empty>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
