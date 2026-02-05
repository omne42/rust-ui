use tachys::html::attribute::{NextAttribute, custom::custom_attribute};

#[test]
fn attribute_tuples_can_grow_beyond_26() {
    let attrs = (
        custom_attribute("data-a", "1"),
        custom_attribute("data-b", "1"),
        custom_attribute("data-c", "1"),
        custom_attribute("data-d", "1"),
        custom_attribute("data-e", "1"),
        custom_attribute("data-f", "1"),
        custom_attribute("data-g", "1"),
        custom_attribute("data-h", "1"),
        custom_attribute("data-i", "1"),
        custom_attribute("data-j", "1"),
        custom_attribute("data-k", "1"),
        custom_attribute("data-l", "1"),
        custom_attribute("data-m", "1"),
        custom_attribute("data-n", "1"),
        custom_attribute("data-o", "1"),
        custom_attribute("data-p", "1"),
        custom_attribute("data-q", "1"),
        custom_attribute("data-r", "1"),
        custom_attribute("data-s", "1"),
        custom_attribute("data-t", "1"),
        custom_attribute("data-u", "1"),
        custom_attribute("data-v", "1"),
        custom_attribute("data-w", "1"),
        custom_attribute("data-x", "1"),
        custom_attribute("data-y", "1"),
        custom_attribute("data-z", "1"),
    );

    let attrs = attrs.add_any_attr(custom_attribute("data-aa", "1"));

    assert_eq!(attrs.len(), 27);
}
