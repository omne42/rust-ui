use super::MockDom;
use crate::{
    html::element,
    renderer::{Renderer, mock_dom::node_eq},
};

#[test]
fn html_debugging_works() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    MockDom::set_attribute(&p, "id", "foo");
    let text = MockDom::create_text_node("Hello, world!");
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&p, text.as_ref(), None);
    assert_eq!(
        main.to_debug_html(),
        "<main><p id=\"foo\">Hello, world!</p></main>"
    );
}

#[test]
fn remove_attribute_works() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    MockDom::set_attribute(&p, "id", "foo");
    let text = MockDom::create_text_node("Hello, world!");
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&p, text.as_ref(), None);
    MockDom::remove_attribute(&p, "id");
    assert_eq!(main.to_debug_html(), "<main><p>Hello, world!</p></main>");
}

#[test]
fn remove_node_works() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    MockDom::set_attribute(&p, "id", "foo");
    let text = MockDom::create_text_node("Hello, world!");
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&p, text.as_ref(), None);
    MockDom::remove_node(&main, p.as_ref());
    assert_eq!(main.to_debug_html(), "<main></main>");
}

#[test]
fn insert_before_works() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    let span = MockDom::create_element(element::Span);
    let text = MockDom::create_text_node("Hello, world!");
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&span, text.as_ref(), None);
    MockDom::insert_node(&main, span.as_ref(), Some(p.as_ref()));
    assert_eq!(
        main.to_debug_html(),
        "<main><span>Hello, world!</span><p></p></main>"
    );
}

#[test]
fn insert_before_sets_parent() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    MockDom::insert_node(&main, p.as_ref(), None);
    let parent = MockDom::get_parent(p.as_ref()).expect("p should have parent set");
    assert!(node_eq(parent, main));
}

#[test]
fn insert_before_moves_node() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    let span = MockDom::create_element(element::Span);
    let text = MockDom::create_text_node("Hello, world!");
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&span, text.as_ref(), None);
    MockDom::insert_node(&main, span.as_ref(), Some(p.as_ref()));
    MockDom::insert_node(&main, p.as_ref(), Some(span.as_ref()));
    assert_eq!(
        main.to_debug_html(),
        "<main><p></p><span>Hello, world!</span></main>"
    );
}

#[test]
fn first_child_gets_first_child() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    let span = MockDom::create_element(element::Span);
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&p, span.as_ref(), None);
    assert_eq!(
        MockDom::first_child(main.as_ref()).as_ref(),
        Some(p.as_ref())
    );
    assert_eq!(
        MockDom::first_child(&MockDom::first_child(main.as_ref()).unwrap()).as_ref(),
        Some(span.as_ref())
    );
}

#[test]
fn next_sibling_gets_next_sibling() {
    let main = MockDom::create_element(element::Main);
    let p = MockDom::create_element(element::P);
    let span = MockDom::create_element(element::Span);
    let text = MockDom::create_text_node("foo");
    MockDom::insert_node(&main, p.as_ref(), None);
    MockDom::insert_node(&main, span.as_ref(), None);
    MockDom::insert_node(&main, text.as_ref(), None);
    assert_eq!(
        MockDom::next_sibling(p.as_ref()).as_ref(),
        Some(span.as_ref())
    );
    assert_eq!(
        MockDom::next_sibling(span.as_ref()).as_ref(),
        Some(text.as_ref())
    );
}
