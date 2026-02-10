use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};

pub(super) fn breadcrumb_primitives() -> AnyView {
    let basic_code = r##"<Breadcrumb>
  <BreadcrumbList>
    <BreadcrumbItem>
      <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbItem>
      <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
    </BreadcrumbItem>
  </BreadcrumbList>
</Breadcrumb>"##;

    let overflow_code = r#"<Breadcrumb>
  <BreadcrumbList>
    <BreadcrumbItem><BreadcrumbLink href="/">"Home"</BreadcrumbLink></BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbEllipsis />
    <BreadcrumbSeparator />
    <BreadcrumbItem><BreadcrumbPage>"Current"</BreadcrumbPage></BreadcrumbItem>
  </BreadcrumbList>
</Breadcrumb>"#;

    let markers_code = r##"<Breadcrumb
  aria_label="Documentation navigation".to_string()
  class_name="docs-breadcrumb-state".to_string()
>
  <BreadcrumbList class_name="docs-breadcrumb-list".to_string()>
    <BreadcrumbItem>
      <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator>
      <span>"→"</span>
    </BreadcrumbSeparator>
    <BreadcrumbItem>
      <BreadcrumbLink class_name="docs-breadcrumb-link".to_string()>
        "Collections"
      </BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbItem>
      <BreadcrumbPage class_name="docs-breadcrumb-page".to_string()>
        "Breadcrumb"
      </BreadcrumbPage>
    </BreadcrumbItem>
  </BreadcrumbList>
</Breadcrumb>"##;

    view! {
        <ComponentPage
            title="BreadcrumbList"
            slug="breadcrumb-list"
            group="Collections"
            description="Shadcn-compatible breadcrumb primitive family (`Breadcrumb*`) with slot-stable link/page/separator/ellipsis composition contracts and Spectrum-style source/state marker semantics."
        >
            <Playground title="Link + Current Page" code=basic_code>
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>
                                "Breadcrumb"
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Playground>

            <Playground title="Ellipsis Overflow" code=overflow_code>
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbEllipsis />
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage>
                                "Current"
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root/link/separator markers like `data-state`, `data-aria-source`, `data-class-source`, `data-href-state`, and `data-content-source` for Spectrum-compatible breadcrumb contracts."
                code=markers_code
            >
                <Breadcrumb
                    aria_label="Documentation navigation".to_string()
                    class_name="docs-breadcrumb-state".to_string()
                >
                    <BreadcrumbList class_name="docs-breadcrumb-list".to_string()>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#/components">"Components"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator>
                            <span>"→"</span>
                        </BreadcrumbSeparator>
                        <BreadcrumbItem>
                            <BreadcrumbLink class_name="docs-breadcrumb-link".to_string()>
                                "Collections"
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPage class_name="docs-breadcrumb-page".to_string()>
                                "Breadcrumb"
                            </BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
