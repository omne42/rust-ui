use super::{Direction, logic};

pub fn is_rtl(direction: Direction) -> bool {
    logic::is_rtl(direction)
}

pub fn use_locale(direction: Direction) -> &'static str {
    logic::use_locale(direction)
}

pub fn use_filter(value: &str, query: &str) -> bool {
    logic::use_filter(value, query)
}

pub fn get_localization_script(direction: Direction) -> String {
    logic::get_localization_script(direction)
}
