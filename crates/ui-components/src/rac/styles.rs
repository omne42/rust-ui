use super::Direction;

pub fn direction_data_attr(direction: Direction) -> &'static str {
    if matches!(direction, Direction::Rtl) {
        "rtl"
    } else {
        "ltr"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_data_attr_matches_direction_contract() {
        assert_eq!(direction_data_attr(Direction::Ltr), "ltr");
        assert_eq!(direction_data_attr(Direction::Rtl), "rtl");
    }
}
