mod button;
mod combo_box;
mod rules;
mod select;
mod text_field;
mod welcome;

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocPage {
    Welcome,
    Rules,
    Button,
    TextField,
    Select,
    ComboBox,
}

pub fn page_view(page: DocPage) -> AnyView {
    match page {
        DocPage::Welcome => welcome::Welcome().into_any(),
        DocPage::Rules => rules::Rules().into_any(),
        DocPage::Button => button::ButtonPage().into_any(),
        DocPage::TextField => text_field::TextFieldPage().into_any(),
        DocPage::Select => select::SelectPage().into_any(),
        DocPage::ComboBox => combo_box::ComboBoxPage().into_any(),
    }
}
