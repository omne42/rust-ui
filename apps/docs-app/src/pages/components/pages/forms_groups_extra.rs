use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    CheckboxField, CheckboxFieldIndicatorPlacement, CheckboxFieldTone, Legend, LegendTone,
};

pub(super) fn checkbox_field() -> AnyView {
    let (newsletter, set_newsletter) = signal(true);
    let (terms, set_terms) = signal(false);
    let (read_only, set_read_only) = signal(true);

    let code = r#"let (newsletter, set_newsletter) = signal(true);

<CheckboxField
  checked=newsletter
  set_checked=set_newsletter
  id_base="newsletter-checkbox-field".to_string()
  label="Subscribe to product updates".to_string()
  description="Receive release notes and occasional best-practice tips.".to_string()
/>"#;

    let states_code = r#"let (terms, set_terms) = signal(false);

<CheckboxField
  checked=terms
  set_checked=set_terms
  id_base="terms-checkbox-field".to_string()
  label="Accept terms of service".to_string()
  description="Required before continuing checkout.".to_string()
  indicator_placement=CheckboxFieldIndicatorPlacement::End
  tone=CheckboxFieldTone::Quiet
  invalid=true
  class_name="docs-checkbox-field-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="CheckboxField"
            slug="checkbox-field"
            group="Forms"
            description="Spectrum/HeroUI-style checkbox field primitive with centralized tone/indicator/state derivation and stable slot/data-state markers."
        >
            <Playground title="Controlled + Description" code=code>
                <div class="docs-stack">
                    <CheckboxField
                        checked=newsletter
                        set_checked=set_newsletter
                        id_base="docs-checkbox-field-newsletter".to_string()
                        label="Subscribe to product updates".to_string()
                        description="Receive release notes and occasional best-practice tips.".to_string()
                    />
                    <span class="ui-muted">
                        "newsletter: " {move || newsletter.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Indicator End + Quiet + Invalid/Disabled" code=states_code>
                <div class="docs-stack">
                    <CheckboxField
                        checked=terms
                        set_checked=set_terms
                        id_base="docs-checkbox-field-terms".to_string()
                        label="Accept terms of service".to_string()
                        description="Required before continuing checkout.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                        tone=CheckboxFieldTone::Quiet
                        invalid=true
                        class_name="docs-checkbox-field-custom".to_string()
                    />

                    <CheckboxField
                        checked=read_only
                        set_checked=set_read_only
                        id_base="docs-checkbox-field-read-only".to_string()
                        label="Enable maintenance window alerts".to_string()
                        description="Read-only preference inherited from organization policy.".to_string()
                        indicator_placement=CheckboxFieldIndicatorPlacement::End
                        disabled=true
                        aria_label="Maintenance alerts (read only)".to_string()
                    />

                    <span class="ui-muted">
                        "terms: " {move || terms.get().to_string()}
                        " · read-only: " {move || read_only.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn legend() -> AnyView {
    let required_code = r#"<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Notification settings\".to_string()
    required=true
  />
</fieldset>"#;

    let states_code = r#"<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Billing preferences\".to_string()
    tone=LegendTone::Muted
    required_indicator=\"(required)\".to_string()
    class_name=\"docs-legend-custom\".to_string()
  />
  <Legend
    text=\"Read-only group\".to_string()
    tone=LegendTone::Strong
    disabled=true
  />
</fieldset>"#;

    view! {
        <ComponentPage
            title="Legend"
            slug="legend"
            group="Forms"
            description="Spectrum/HeroUI-compatible fieldset legend primitive with centralized tone/required/disabled contracts and stable slot/data-state markers."
        >
            <Playground title="Required Legend" code=required_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() required=true />
                    <div class="ui-muted">
                        "Legend stays semantic inside fieldset and exposes required marker contracts."
                    </div>
                </fieldset>
            </Playground>

            <Playground title="Tone + Custom Indicator + Disabled" code=states_code>
                <fieldset class="docs-stack">
                    <Legend
                        text="Billing preferences".to_string()
                        tone=LegendTone::Muted
                        required=true
                        required_indicator="(required)".to_string()
                        class_name="docs-legend-custom".to_string()
                    />

                    <Legend
                        text="Read-only group".to_string()
                        tone=LegendTone::Strong
                        disabled=true
                    />
                </fieldset>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
