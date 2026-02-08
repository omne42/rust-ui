use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{Switch, SwitchGroup, SwitchGroupOrientation, SwitchGroupTone};

pub(super) fn switch_group() -> AnyView {
    let (marketing, set_marketing) = signal(true);
    let (product_updates, set_product_updates) = signal(false);
    let (security_alerts, set_security_alerts) = signal(true);

    let (critical_alerts, set_critical_alerts) = signal(true);
    let (maintenance_mode, set_maintenance_mode) = signal(false);

    let base_code = r#"<SwitchGroup
  id_base="notifications".to_string()
  label="Notification channels".to_string()
  description="Choose which channels we can use to contact you.".to_string()
  required=true
>
  <Switch checked=marketing set_checked=set_marketing>"Marketing email"</Switch>
  <Switch checked=product_updates set_checked=set_product_updates>"Product updates"</Switch>
  <Switch checked=security_alerts set_checked=set_security_alerts>"Security alerts"</Switch>
</SwitchGroup>"#;

    let states_code = r#"<SwitchGroup
  id_base="system-controls".to_string()
  label="System controls".to_string()
  orientation=SwitchGroupOrientation::Horizontal
  tone=SwitchGroupTone::Muted
  invalid=true
  disabled=true
  error_message="At least one critical channel must stay enabled.".to_string()
  class_name="docs-switch-group-custom".to_string()
>
  <Switch checked=critical_alerts set_checked=set_critical_alerts disabled=true>
    "Critical alerts"
  </Switch>
  <Switch checked=maintenance_mode set_checked=set_maintenance_mode disabled=true>
    "Maintenance mode"
  </Switch>
</SwitchGroup>"#;

    view! {
        <ComponentPage
            title="SwitchGroup"
            slug="switch-group"
            group="Forms"
            description="Spectrum/HeroUI-style switch grouping primitive with centralized orientation/tone/validation/message-state contracts and stable data markers."
        >
            <Playground title="Required + Description" code=base_code>
                <div class="docs-stack">
                    <SwitchGroup
                        id_base="docs-switch-group-notifications".to_string()
                        label="Notification channels".to_string()
                        description="Choose which channels we can use to contact you.".to_string()
                        required=true
                        aria_label="Notification switches".to_string()
                    >
                        <Switch checked=marketing set_checked=set_marketing>
                            "Marketing email"
                        </Switch>
                        <Switch checked=product_updates set_checked=set_product_updates>
                            "Product updates"
                        </Switch>
                        <Switch checked=security_alerts set_checked=set_security_alerts>
                            "Security alerts"
                        </Switch>
                    </SwitchGroup>
                    <span class="ui-muted">
                        "marketing="
                        {move || marketing.get().to_string()}
                        " · updates="
                        {move || product_updates.get().to_string()}
                        " · security="
                        {move || security_alerts.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Horizontal + Invalid + Disabled + Custom Class" code=states_code>
                <div class="docs-stack">
                    <SwitchGroup
                        id_base="docs-switch-group-system".to_string()
                        label="System controls".to_string()
                        orientation=SwitchGroupOrientation::Horizontal
                        tone=SwitchGroupTone::Muted
                        invalid=true
                        disabled=true
                        error_message="At least one critical channel must stay enabled.".to_string()
                        class_name="docs-switch-group-custom".to_string()
                    >
                        <Switch checked=critical_alerts set_checked=set_critical_alerts disabled=true>
                            "Critical alerts"
                        </Switch>
                        <Switch checked=maintenance_mode set_checked=set_maintenance_mode disabled=true>
                            "Maintenance mode"
                        </Switch>
                    </SwitchGroup>
                    <span class="ui-muted">
                        "critical="
                        {move || critical_alerts.get().to_string()}
                        " · maintenance="
                        {move || maintenance_mode.get().to_string()}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
