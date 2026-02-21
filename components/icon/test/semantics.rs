use std::path::PathBuf;

fn load_source(path: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(path)).expect("source file should be readable")
}

#[test]
fn icon_component_keeps_layered_file_boundaries() {
    let source = load_source("src/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Icon;",
        "#[path = \"icons/mod.rs\"]",
        "#[path = \"set/mod.rs\"]",
        "#[path = \"ui/mod.rs\"]",
        "#[path = \"workflow/mod.rs\"]",
    ] {
        assert!(
            source.contains(needle),
            "icon component should keep `{needle}` in module boundary contracts."
        );
    }

    for banned in ["pub mod logic;", "pub mod view;", "web_sys"] {
        assert!(
            !source.contains(banned),
            "icon public API must not expose internal or web-specific type surface: `{banned}`."
        );
    }
}

#[test]
fn icon_view_mounts_semantic_markers_for_contract_tests() {
    let source = load_source("src/view.rs");

    for needle in [
        "let locale = locale_attrs(logic::normalize_optional_text(lang), dir);",
        "lang=locale.lang",
        "dir=locale.dir",
        "role=(!state.is_decorative).then_some(\"img\")",
        "aria-label=state.has_accessible_name.then_some(aria_label)",
        "aria-hidden=state.is_decorative.then_some(\"true\")",
        "data-slot=\"icon\"",
        "data-slot=\"icon-glyph\"",
        "data-state=state.data_state_attr",
        "data-size=state.size_attr",
        "data-tone=state.tone_attr",
        "data-class-source=state.class_source_attr",
        "data-aria-source=state.aria_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "icon view should preserve semantic contract marker `{needle}`."
        );
    }
}

#[test]
fn icon_family_views_forward_locale_contract() {
    for path in [
        "src/icons/view.rs",
        "src/set/view.rs",
        "src/ui/view.rs",
        "src/workflow/view.rs",
    ] {
        let source = load_source(path);

        for needle in [
            "A11yDirection",
            "locale_attrs(",
            "lang=locale.lang",
            "dir=locale.dir",
        ] {
            assert!(
                source.contains(needle),
                "{path} should include locale contract marker `{needle}`."
            );
        }
    }
}

#[test]
fn icon_family_views_expose_state_and_source_markers() {
    let cases = [
        (
            "src/icons/view.rs",
            [
                "data-state=state.state_attr",
                "data-set-source=state.set_source_attr",
                "data-aria-source=state.aria_source_attr",
                "data-class-source=state.class_source_attr",
                "data-glyph-source=state.glyph_source_attr",
                "data-tone-source=state.tone_source_attr",
                "data-disabled=state.is_disabled.then_some(\"true\")",
            ]
            .as_slice(),
        ),
        (
            "src/set/view.rs",
            [
                "data-state=state.state_attr",
                "data-icon-source=state.icon_source_attr",
                "data-iconset-source=state.iconset_source_attr",
                "data-label-source=state.label_source_attr",
                "data-class-source=state.class_source_attr",
                "data-size-source=state.size_source_attr",
                "data-tone-source=state.tone_source_attr",
                "data-disabled=state.is_disabled.then_some(\"true\")",
            ]
            .as_slice(),
        ),
        (
            "src/ui/view.rs",
            [
                "data-state=state.state_attr",
                "data-icon-reference-source=state.icon_reference_source_attr",
                "data-aria-source=state.aria_source_attr",
                "data-class-source=state.class_source_attr",
                "data-glyph-source=state.glyph_source_attr",
                "data-size-source=state.size_source_attr",
                "data-tone-source=state.tone_source_attr",
                "data-disabled=state.is_disabled.then_some(\"true\")",
            ]
            .as_slice(),
        ),
        (
            "src/workflow/view.rs",
            [
                "data-state=state.state_attr",
                "data-icon-reference-source=state.icon_reference_source_attr",
                "data-aria-source=state.aria_source_attr",
                "data-class-source=state.class_source_attr",
                "data-glyph-source=state.glyph_source_attr",
                "data-size-source=state.size_source_attr",
                "data-tone-source=state.tone_source_attr",
                "data-disabled=state.is_disabled.then_some(\"true\")",
            ]
            .as_slice(),
        ),
    ];

    for (path, needles) in cases {
        let source = load_source(path);
        for needle in needles {
            assert!(
                source.contains(needle),
                "{path} should expose semantic marker `{needle}`."
            );
        }
    }
}
