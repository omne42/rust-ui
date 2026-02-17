use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    DateField, DateFieldTone, DateInputGroup, DateInputGroupVariant, Field, FieldGroup,
    FieldGroupDensity, FieldGroupOrientation, Switch, SwitchGroup, SwitchGroupOrientation,
    SwitchGroupTone, TimeField, TimeFieldTone,
};

pub(super) fn switch_group() -> AnyView {
    let (marketing, set_marketing) = signal(true);
    let (product_updates, set_product_updates) = signal(false);
    let (security_alerts, set_security_alerts) = signal(true);

    let (critical_alerts, set_critical_alerts) = signal(true);
    let (maintenance_mode, set_maintenance_mode) = signal(false);

    let base_code = Signal::derive(move || {
        r#"<SwitchGroup
  id_base="notifications".to_string()
  label="Notification channels".to_string()
  description="Choose which channels we can use to contact you.".to_string()
  required=true
>
  <Switch checked=marketing set_checked=set_marketing>"Marketing email"</Switch>
  <Switch checked=product_updates set_checked=set_product_updates>"Product updates"</Switch>
  <Switch checked=security_alerts set_checked=set_security_alerts>"Security alerts"</Switch>
</SwitchGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<SwitchGroup
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
</SwitchGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SwitchGroup"
            slug="switch-group"
            group="Forms"
            description="baseline-style switch grouping primitive with centralized orientation/tone/validation/message-state contracts and stable data markers."
        >
            <Playground title="Required + Description" code_signal=base_code>
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

            <Playground title="Horizontal + Invalid + Disabled + Custom Class" code_signal=states_code>
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

pub(super) fn field_group() -> AnyView {
    let base_code = Signal::derive(move || {
        r#"<FieldGroup
  id_base="account-fields".to_string()
  label="Account details".to_string()
  description="Group related fields to keep form scanning predictable.".to_string()
>
  <Field label="Name".to_string()>
    <input class="docs-search__input" type="text" placeholder="Ada Lovelace" />
  </Field>
  <Field label="Email".to_string()>
    <input class="docs-search__input" type="email" placeholder="ada@example.com" />
  </Field>
</FieldGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<FieldGroup
  orientation=FieldGroupOrientation::Horizontal
  density=FieldGroupDensity::Compact
  invalid=true
  disabled=true
  class_name="docs-field-group-custom".to_string()
  aria_label="Billing field cluster".to_string()
>
  <Field label="VAT ID".to_string() invalid=true disabled=true error_message="VAT ID is required".to_string()>
    <input class="docs-search__input" type="text" disabled />
  </Field>
  <Field label="Purchase Order".to_string() disabled=true>
    <input class="docs-search__input" type="text" disabled />
  </Field>
</FieldGroup>"#.to_string()
    });

    view! {
        <ComponentPage
            title="FieldGroup"
            slug="field-group"
            group="Forms"
            description="baseline-compatible field clustering primitive with centralized orientation/density/aria/class-state contracts and stable slot + data markers."
        >
            <Playground title="Vertical + Label + Description" code_signal=base_code>
                <FieldGroup
                    id_base="docs-field-group-account".to_string()
                    label="Account details".to_string()
                    description="Group related fields to keep form scanning predictable.".to_string()
                >
                    <Field label="Name".to_string()>
                        <input
                            class="docs-search__input"
                            type="text"
                            placeholder="Ada Lovelace"
                        />
                    </Field>

                    <Field label="Email".to_string()>
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="ada@example.com"
                        />
                    </Field>
                </FieldGroup>
            </Playground>

            <Playground title="Horizontal + Compact + Invalid + Disabled" code_signal=states_code>
                <FieldGroup
                    id_base="docs-field-group-billing".to_string()
                    orientation=FieldGroupOrientation::Horizontal
                    density=FieldGroupDensity::Compact
                    invalid=true
                    disabled=true
                    class_name="docs-field-group-custom".to_string()
                    aria_label="Billing field cluster".to_string()
                >
                    <Field
                        label="VAT ID".to_string()
                        invalid=true
                        disabled=true
                        error_message="VAT ID is required".to_string()
                    >
                        <input class="docs-search__input" type="text" disabled />
                    </Field>

                    <Field label="Purchase Order".to_string() disabled=true>
                        <input class="docs-search__input" type="text" disabled />
                    </Field>
                </FieldGroup>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_input_group() -> AnyView {
    let (invoice_date, set_invoice_date) = signal(Some("2026-03-14".to_string()));
    let on_invoice_date_change = Callback::new(move |next: Option<String>| {
        set_invoice_date.set(next);
    });

    let (ship_window, set_ship_window) = signal(Some("18:30".to_string()));
    let on_ship_window_change = Callback::new(move |next: Option<String>| {
        set_ship_window.set(next);
    });

    let code = Signal::derive(move || {
        r#"let (invoice_date, set_invoice_date) = signal(Some("2026-03-14".to_string()));
let on_invoice_date_change = Callback::new(move |next: Option<String>| {
  set_invoice_date.set(next);
});

<DateInputGroup
  aria_label="Invoice date controls".to_string()
  segmented=true
  prefix=move || view! { <span>"📅"</span> }
  suffix=move || view! { <span>"UTC+0"</span> }
>
  <DateField
    id_base="invoice-date".to_string()
    label="Invoice date".to_string()
    tone=DateFieldTone::Quiet
    value=invoice_date
    on_value_change=on_invoice_date_change
  />
</DateInputGroup>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (ship_window, set_ship_window) = signal(Some("18:30".to_string()));
let on_ship_window_change = Callback::new(move |next: Option<String>| {
  set_ship_window.set(next);
});

<DateInputGroup
  full_width=true
  variant=DateInputGroupVariant::Secondary
  invalid=true
  segmented=true
  aria_label="Ship window controls".to_string()
  class_name="docs-date-input-group-custom".to_string()
  prefix=move || view! { <span>"🕒"</span> }
  suffix=move || view! { <span>"5m"</span> }
>
  <TimeField
    id_base="ship-window".to_string()
    label="Ship window".to_string()
    tone=TimeFieldTone::Strong
    minute_step=5
    value=ship_window
    on_value_change=on_ship_window_change
  />
</DateInputGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="DateInputGroup"
            slug="date-input-group"
            group="Forms"
            description="baseline-style date-input grouping primitive with centralized variant/width/prefix-suffix state contracts and segmented slot markers."
        >
            <Playground title="DateField + Prefix/Suffix" code_signal=code>
                <div class="docs-stack">
                    <DateInputGroup
                        aria_label="Invoice date controls".to_string()
                        segmented=true
                        prefix=move || view! { <span>"📅"</span> }
                        suffix=move || view! { <span>"UTC+0"</span> }
                    >
                        <DateField
                            id_base="docs-date-input-group-invoice".to_string()
                            label="Invoice date".to_string()
                            tone=DateFieldTone::Quiet
                            value=invoice_date
                            on_value_change=on_invoice_date_change
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "invoice date: "
                        {move || invoice_date.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Secondary + Full Width + Invalid" code_signal=states_code>
                <div class="docs-stack">
                    <DateInputGroup
                        full_width=true
                        variant=DateInputGroupVariant::Secondary
                        invalid=true
                        segmented=true
                        aria_label="Ship window controls".to_string()
                        class_name="docs-date-input-group-custom".to_string()
                        prefix=move || view! { <span>"🕒"</span> }
                        suffix=move || view! { <span>"5m"</span> }
                    >
                        <TimeField
                            id_base="docs-date-input-group-time".to_string()
                            label="Ship window".to_string()
                            tone=TimeFieldTone::Strong
                            minute_step=5
                            value=ship_window
                            on_value_change=on_ship_window_change
                        />
                    </DateInputGroup>
                    <span class="ui-muted">
                        "ship window: "
                        {move || ship_window.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
