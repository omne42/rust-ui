const VIEW_SRC: &str = include_str!("../view.rs");
const MOD_SRC: &str = include_str!("../mod.rs");
const LOGIC_SRC: &str = include_str!("../logic.rs");
const STYLES_SRC: &str = include_str!("../styles.rs");
const PROTOCOL_SRC: &str = include_str!("../protocol.rs");
const COMPONENT_MANIFEST_SRC: &str = include_str!("../Component.toml");
const ITEM_RBI_SRC: &str = include_str!("../item.rbi");
const CARGO_TOML_SRC: &str = include_str!("../../Cargo.toml");

#[test]
fn view_mounts_stable_semantics_markers() {
    for needle in [
        "role=\"list\"",
        "role=\"listitem\"",
        "role=\"separator\"",
        "aria-orientation=\"horizontal\"",
        "data-slot=\"item-group\"",
        "data-slot=\"item\"",
        "data-slot=\"item-media\"",
        "data-slot=\"item-content\"",
        "data-slot=\"item-title\"",
        "data-slot=\"item-description\"",
        "data-slot=\"item-actions\"",
        "data-slot=\"item-header\"",
        "data-slot=\"item-footer\"",
        "data-variant=render_state.variant_attr",
        "data-size=render_state.size_attr",
        "data-variant-source=render_state.variant_source_attr",
        "data-size-source=render_state.size_source_attr",
        "let agent_attrs = protocol::agent_data_attrs(render_state);",
        "data-ui-schema=agent_attrs.schema",
        "data-ui-intent=agent_attrs.intent",
        "data-ui-action=agent_attrs.action",
        "data-ui-streaming-policy=agent_attrs.streaming_policy",
        "data-ui-streaming-fallback=agent_attrs.streaming_fallback",
        "data-ui-stream-mode=agent_attrs.stream_mode",
        "data-ui-output-mode=agent_attrs.output_mode",
        "data-ui-output-status=agent_attrs.output_status",
        "data-ui-state-variant=agent_attrs.state_variant",
        "data-ui-state-size=agent_attrs.state_size",
        "data-ui-source-variant=agent_attrs.source_variant",
        "data-ui-source-size=agent_attrs.source_size",
    ] {
        assert!(
            VIEW_SRC.contains(needle),
            "item view semantics contract must contain `{needle}`"
        );
    }
}

#[test]
fn public_api_surface_does_not_expose_platform_dom_types() {
    for forbidden in ["web_sys", "web-sys", "wasm_bindgen"] {
        assert!(
            !MOD_SRC.contains(forbidden),
            "public API surface must not expose `{forbidden}`"
        );
    }
}

#[test]
fn view_exposes_locale_attrs_hook_for_i18n_and_rtl() {
    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "logic::resolve_locale_attrs(lang, dir)",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            VIEW_SRC.contains(needle),
            "item view must expose locale hook `{needle}`"
        );
    }
}

#[test]
fn styles_avoid_fragile_dom_guessing_and_runtime_inline_logic() {
    for forbidden in [":nth-child", ":nth-of-type", "style=", " > "] {
        assert!(
            !STYLES_SRC.contains(forbidden),
            "styles contract must not rely on fragile selector `{forbidden}`"
        );
    }

    assert!(
        !VIEW_SRC.contains(" style="),
        "view must not encode business style logic through inline style attributes"
    );
}

#[test]
fn non_interactive_component_paths_are_explicitly_absent() {
    for forbidden in [
        "on:click",
        "on:keydown",
        "on:pointerdown",
        "on:pointermove",
        "tabindex",
        "aria-disabled",
        "data-disabled",
    ] {
        assert!(
            !VIEW_SRC.contains(forbidden),
            "item semantics contract should not expose interactive path `{forbidden}`"
        );
    }
}

#[test]
fn overlay_focus_stack_paths_are_explicitly_absent_for_item() {
    for forbidden in [
        "NodeRef",
        "document.body",
        "FallbackTo",
        "Selector",
        "role=\"dialog\"",
        "role=\"menu\"",
        "role=\"tooltip\"",
        "aria-modal",
    ] {
        assert!(
            !VIEW_SRC.contains(forbidden),
            "item is not an overlay; focus-stack path `{forbidden}` must stay absent"
        );
        assert!(
            !LOGIC_SRC.contains(forbidden),
            "item logic must not absorb overlay focus policy `{forbidden}`"
        );
    }
}

#[test]
fn foreign_zone_escape_hatch_paths_are_absent_for_item() {
    const CARGO_TOML_SRC: &str = include_str!("../../Cargo.toml");

    for forbidden in [
        "echarts",
        "mapbox",
        "leaflet",
        "amap",
        "google-maps",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "extern \"C\"",
    ] {
        assert!(
            !VIEW_SRC.contains(forbidden),
            "item view must not embed imperative third-party zone `{forbidden}`"
        );
        assert!(
            !LOGIC_SRC.contains(forbidden),
            "item logic must not absorb imperative foreign-state path `{forbidden}`"
        );
        assert!(
            !MOD_SRC.contains(forbidden),
            "item public api surface must not expose foreign integration token `{forbidden}`"
        );
        assert!(
            !CARGO_TOML_SRC.contains(forbidden),
            "item component must not add imperative third-party dependency `{forbidden}`"
        );
    }
}

#[test]
fn hydration_entropy_sources_are_absent_for_item() {
    const CARGO_TOML_SRC: &str = include_str!("../../Cargo.toml");

    for forbidden in [
        "Instant::now",
        "SystemTime::now",
        "UNIX_EPOCH",
        "js_sys::Date::now",
        "Math::random",
        "Uuid::new_v4",
        "uuid::Uuid",
        "thread_rng(",
        "rand::random",
        "getrandom",
        "IdProvider",
    ] {
        assert!(
            !VIEW_SRC.contains(forbidden),
            "item view must not introduce hydration entropy source `{forbidden}`"
        );
        assert!(
            !LOGIC_SRC.contains(forbidden),
            "item logic must stay deterministic without `{forbidden}`"
        );
        assert!(
            !MOD_SRC.contains(forbidden),
            "item public api must not leak hydration entropy path `{forbidden}`"
        );
        assert!(
            !CARGO_TOML_SRC.contains(forbidden),
            "item dependencies must not pull hydration entropy path `{forbidden}`"
        );
    }
}

#[test]
fn cross_platform_surface_is_browser_api_free_and_cfg_stable() {
    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, STYLES_SRC, PROTOCOL_SRC] {
        for forbidden in [
            "web_sys",
            "web-sys",
            "wasm_bindgen",
            "js_sys::",
            "window.",
            "document.",
            "navigator.",
            "HtmlElement",
            "NodeRef",
        ] {
            assert!(
                !source.contains(forbidden),
                "item source must stay non-browser-specific without `{forbidden}`"
            );
        }

        for forbidden in [
            "#[cfg(target_arch",
            "#[cfg(feature = \"web\")",
            "#[cfg(feature = \"ssr\")",
            "cfg!(target_arch",
        ] {
            assert!(
                !source.contains(forbidden),
                "item source should not fork platform branches with `{forbidden}`"
            );
        }
    }

    assert!(
        !CARGO_TOML_SRC.contains("target.'cfg(target_arch = \"wasm32\")'.dependencies"),
        "item cargo must not pull direct wasm-only browser dependency section"
    );
}

#[test]
fn ui_headless_web_ssr_mutual_exclusion_contract_is_preserved() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let headless_lib_src = std::fs::read_to_string(repo_root.join("crates/ui-headless/src/lib.rs"))
        .expect("must read ui-headless lib source");
    let headless_cargo_src =
        std::fs::read_to_string(repo_root.join("crates/ui-headless/Cargo.toml"))
            .expect("must read ui-headless Cargo.toml");

    assert!(
        headless_lib_src.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]"),
        "ui-headless must guard mutually enabled web/ssr features with cfg(all(...))"
    );
    assert!(
        headless_lib_src.contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless must hard-fail when web+ssr are enabled together"
    );

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_src.contains(needle),
            "ui-headless feature contract must contain `{needle}`"
        );
    }

    assert!(
        !CARGO_TOML_SRC.contains("ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }"),
        "item dependency wiring must not force-enable ui-headless web+ssr simultaneously"
    );
}

#[test]
fn tests_do_not_depend_on_visual_snapshot_tooling() {
    for forbidden in ["insta", "assert_snapshot", "snapbox"] {
        assert!(
            !CARGO_TOML_SRC.contains(forbidden),
            "semantic-contract tests must not require snapshot dependency `{forbidden}`"
        );
    }
}

#[test]
fn rust_hygiene_contract_is_kept_for_non_test_item_sources() {
    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, STYLES_SRC, PROTOCOL_SRC] {
        for forbidden in [".unwrap(", ".expect(", "let _ =", "let _="] {
            assert!(
                !source.contains(forbidden),
                "non-test item sources must not contain rust-hygiene violation `{forbidden}`"
            );
        }

        for hotspot in [".to_owned()", "String::from(", ".to_string()"] {
            assert!(
                !source.contains(hotspot),
                "string clone hotspot `{hotspot}` should be eliminated or justified with Cow<'static, str>"
            );
        }
    }
}

#[test]
fn file_responsibility_boundaries_are_explicit() {
    for needle in ["pub mod logic;", "pub mod styles;", "mod view;"] {
        assert!(
            MOD_SRC.contains(needle),
            "mod.rs must keep minimal export boundary `{needle}`"
        );
    }

    for forbidden in ["view!", "data-slot=", "role=\"", "aria-"] {
        assert!(
            !LOGIC_SRC.contains(forbidden),
            "logic.rs must stay renderer-agnostic without `{forbidden}`"
        );
    }

    for forbidden in ["view!", "data-slot=", "role=\"", "on:click"] {
        assert!(
            !STYLES_SRC.contains(forbidden),
            "styles.rs must stay static-css only without `{forbidden}`"
        );
    }

    for needle in ["view!", "data-slot=", "logic::derive_item_render_state"] {
        assert!(
            VIEW_SRC.contains(needle),
            "view.rs must own structure mounting and consume logic output `{needle}`"
        );
    }
}

#[test]
fn component_directory_standard_layout_is_kept_for_item() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "item component must keep required standard file `{required}`"
        );
    }

    assert!(
        !src_dir.join("render.rs").exists(),
        "item component should not drift to render.rs outside standard layout"
    );

    // item is a static composition primitive: motion/spec remain intentionally absent.
    assert!(
        !src_dir.join("motion.rs").exists(),
        "item should not add motion.rs without interactive motion contract"
    );
    assert!(
        !src_dir.join("spec.rs").exists(),
        "item should not add spec.rs without external schema evolution requirements"
    );

    for needle in ["pub mod logic;", "pub mod styles;", "mod view;"] {
        assert!(
            MOD_SRC.contains(needle),
            "mod.rs must preserve minimal stable export boundary `{needle}`"
        );
    }
}

#[test]
fn motion_module_is_intentionally_absent_for_static_item_component() {
    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");
    assert!(
        !motion_path.exists(),
        "item component currently has no motion semantics; motion.rs should remain absent"
    );
}

#[test]
fn ui_motion_non_wasm_stub_contract_stays_available_without_item_motion_dependency() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ui_motion_lib = std::fs::read_to_string(repo_root.join("crates/ui-motion/src/lib.rs"))
        .expect("must read ui-motion lib source");

    assert!(
        !CARGO_TOML_SRC.contains("ui-motion"),
        "item component must not depend on ui-motion when no motion contract is exposed"
    );
    assert!(
        ui_motion_lib.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "ui-motion must keep non-wasm module gate for SSR/tooling builds"
    );
    assert!(
        ui_motion_lib.contains("pub fn prefers_reduced_motion() -> bool"),
        "ui-motion non-wasm backend must expose predictable reduced-motion fallback"
    );
    assert!(
        ui_motion_lib.contains("pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}"),
        "ui-motion non-wasm backend must keep no-op animate stub"
    );
}

#[test]
fn reduced_motion_ssr_wasm_contract_remains_static_for_item() {
    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, STYLES_SRC, PROTOCOL_SRC] {
        for forbidden in [
            "prefers_reduced_motion",
            "MotionOptions",
            "MotionKeyframe",
            "animate(",
            "attach_motion",
            "#[cfg(target_arch",
            "#[cfg(feature = \"ssr\")",
            "#[cfg(feature = \"web\")",
        ] {
            assert!(
                !source.contains(forbidden),
                "item should keep static cross-platform semantics without `{forbidden}`"
            );
        }
    }

    for needle in ["role=\"listitem\"", "data-slot=\"item\""] {
        assert!(
            VIEW_SRC.contains(needle),
            "item view must keep stable semantics marker `{needle}` across platforms"
        );
    }

    assert!(
        !CARGO_TOML_SRC.contains("ui-motion"),
        "item should not require motion runtime for reduced-motion/SSR/wasm compatibility"
    );
}

#[test]
fn motion_contract_surface_is_absent_for_static_item_component() {
    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");
    assert!(
        !motion_path.exists(),
        "item component should not define motion.rs when no motion contract is exposed"
    );

    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, STYLES_SRC, PROTOCOL_SRC] {
        for forbidden in [
            "attach_motion",
            "stiffness",
            "damping",
            "MotionOptions",
            "MotionKeyframe",
            "prefers_reduced_motion",
            "ui_motion::",
        ] {
            assert!(
                !source.contains(forbidden),
                "static item component must not leak motion-contract surface `{forbidden}`"
            );
        }
    }

    assert!(
        !CARGO_TOML_SRC.contains("ui-motion"),
        "item crate must not bind ui-motion runtime when motion contract is intentionally absent"
    );
}

#[test]
fn performance_budget_surface_is_static_and_bounded_for_item() {
    for source in [VIEW_SRC, LOGIC_SRC] {
        for forbidden in [
            "create_signal",
            "RwSignal",
            "create_memo",
            "Memo::new",
            "create_effect",
            "Effect::new",
            "spawn_local",
            "request_animation_frame",
            "set_timeout",
            "set_interval",
            "tokio::spawn",
            "wasm_bindgen_futures::spawn_local",
        ] {
            assert!(
                !source.contains(forbidden),
                "item should keep bounded render/update cost without `{forbidden}`"
            );
        }
    }

    for forbidden in ["transition:", "animation:", "@keyframes"] {
        assert!(
            !STYLES_SRC.contains(forbidden),
            "item static styles should avoid runtime animation cost via `{forbidden}`"
        );
    }
}

#[test]
fn view_macro_complexity_stays_shallow_and_split() {
    let src = VIEW_SRC;
    let starts: Vec<usize> = src.match_indices("view! {").map(|(idx, _)| idx).collect();
    assert_eq!(
        starts.len(),
        10,
        "item view should stay split into small view! blocks instead of one giant macro"
    );

    for start in starts {
        let mut depth = 0_i32;
        let mut end = None;
        for (off, ch) in src[start..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(start + off);
                        break;
                    }
                }
                _ => {}
            }
        }

        let end = end.expect("each view! block should have balanced braces");
        let block = &src[start..=end];
        let line_count = block.lines().count();
        assert!(
            line_count <= 20,
            "single view! block grew too large ({line_count} lines), split by semantic sub-blocks"
        );

        for forbidden in [
            "<For",
            "<Show",
            "<Suspense",
            "<Transition",
            "collect::<",
            ".map(",
        ] {
            assert!(
                !block.contains(forbidden),
                "view! block should avoid heavy macro/control-flow pattern `{forbidden}`"
            );
        }
    }
}

#[test]
fn componentization_is_limited_to_public_semantic_slots() {
    let component_count = VIEW_SRC.matches("#[component]").count();
    assert_eq!(
        component_count, 10,
        "item should not introduce extra private #[component] fragments beyond public slot API"
    );

    for name in [
        "ItemGroup",
        "ItemSeparator",
        "Item",
        "ItemMedia",
        "ItemContent",
        "ItemTitle",
        "ItemDescription",
        "ItemActions",
        "ItemHeader",
        "ItemFooter",
    ] {
        assert!(
            VIEW_SRC.contains(&format!("pub fn {name}(")),
            "view must define public semantic component `{name}`"
        );
        assert!(
            MOD_SRC.contains(name),
            "mod.rs must re-export semantic component `{name}` instead of hiding it as local fragment"
        );
    }

    for forbidden in ["fn render_", "fn slot_", "fn helper_"] {
        assert!(
            !VIEW_SRC.contains(forbidden),
            "view should avoid ad-hoc fragment component pattern `{forbidden}` without explicit API semantics"
        );
    }
}

#[test]
fn static_fragments_stay_lightweight_and_centrally_editable() {
    for forbidden in [
        "<svg",
        "<path",
        "<canvas",
        "<video",
        "inner_html",
        "data:image/",
    ] {
        assert!(
            !VIEW_SRC.contains(forbidden),
            "item view should not embed heavyweight static payload `{forbidden}`"
        );
    }

    for slot in [
        "item-group",
        "item-separator",
        "item",
        "item-media",
        "item-content",
        "item-title",
        "item-description",
        "item-actions",
        "item-header",
        "item-footer",
    ] {
        let count = VIEW_SRC.matches(&format!("data-slot=\"{slot}\"")).count();
        assert_eq!(
            count, 1,
            "semantic slot `{slot}` should be declared once in view.rs for a single static edit path"
        );
    }

    for needle in [
        "role=\"list\"",
        "role=\"listitem\"",
        "role=\"separator\"",
        "aria-orientation=\"horizontal\"",
    ] {
        assert!(
            VIEW_SRC.contains(needle),
            "accessibility semantics must remain mounted after static-fragment constraints `{needle}`"
        );
    }
}

#[test]
fn inner_html_injection_surface_is_absent_for_item_component() {
    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, STYLES_SRC, PROTOCOL_SRC] {
        for forbidden in [
            "inner_html=",
            ".set_inner_html(",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "from_html_unchecked",
            "innerHTML",
        ] {
            assert!(
                !source.contains(forbidden),
                "item component must not expose raw html injection surface `{forbidden}`"
            );
        }
    }

    for needle in [
        "role=\"list\"",
        "role=\"listitem\"",
        "role=\"separator\"",
        "data-slot=\"item\"",
    ] {
        assert!(
            VIEW_SRC.contains(needle),
            "semantic markers must remain stable while inner_html stays disabled `{needle}`"
        );
    }
}

#[test]
fn wasm_debug_tooling_surface_is_absent_for_static_item_component() {
    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, PROTOCOL_SRC] {
        for forbidden in [
            "TraceId",
            "trace_id",
            "state_transition",
            "record_event",
            "replay",
            "timeline",
            "devtools",
            "debug_panel",
            "debug_overlay",
        ] {
            assert!(
                !source.contains(forbidden),
                "item should not carry wasm debug/replay surface `{forbidden}`"
            );
        }
    }

    for forbidden in ["tracing", "wasm-logger", "console_log", "gloo-console"] {
        assert!(
            !CARGO_TOML_SRC.contains(forbidden),
            "item should not pull debug-only dependency `{forbidden}` into production surface"
        );
    }

    for needle in [
        "[features]",
        "default = []",
        "role=\"listitem\"",
        "data-slot=\"item\"",
    ] {
        assert!(
            CARGO_TOML_SRC.contains(needle) || VIEW_SRC.contains(needle),
            "item must keep feature-clean contract and stable semantics marker `{needle}`"
        );
    }
}

#[test]
fn dx_surface_uses_docs_playground_while_component_runtime_stays_clean() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docs_item_page = std::fs::read_to_string(
        repo_root.join("apps/docs-app/src/pages/components/pages/collections_item_primitives.rs"),
    )
    .expect("must read docs item playground page");

    for needle in [
        "Playground",
        "title=\"Hello World\"",
        "title=\"Media + Content + Actions\"",
        "title=\"State Matrix (Variant + Size)\"",
        "title=\"Controlled vs Uncontrolled (N/A for Item)\"",
        "title=\"Streaming / Snapshot Display\"",
        "code_imports=ITEM_DOC_IMPORTS.to_string()",
        "compose_copy_ready_code",
        "controls=move || view!",
        "signal(Some(0_usize))",
        "set_variant_index",
        "set_size_index",
    ] {
        assert!(
            docs_item_page.contains(needle),
            "docs page should provide isolated DX playground and stateful context `{needle}`"
        );
    }

    for source in [
        VIEW_SRC,
        LOGIC_SRC,
        MOD_SRC,
        STYLES_SRC,
        PROTOCOL_SRC,
        CARGO_TOML_SRC,
    ] {
        for forbidden in ["hot_reload", "leptos_hot_reload", "hmr", "vite_hmr"] {
            assert!(
                !source.contains(forbidden),
                "item component runtime should not be polluted by dev-only reload token `{forbidden}`"
            );
        }
    }
}

#[test]
fn docs_entry_is_beginner_friendly_and_default_first() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docs_item_page = std::fs::read_to_string(
        repo_root.join("apps/docs-app/src/pages/components/pages/collections_item_primitives.rs"),
    )
    .expect("must read docs item page source");

    for needle in [
        "slug=\"item\"",
        "data-slot=\"item-doc-onboarding\"",
        "Start with ",
        "\"Hello World\"",
        "Default API path comes first;",
        "title=\"Hello World\"",
        "title=\"Media + Content + Actions\"",
        "title=\"Header + Footer Layout\"",
        "title=\"State Matrix (Variant + Size)\"",
    ] {
        assert!(
            docs_item_page.contains(needle),
            "item docs entry should keep beginner-friendly token `{needle}`"
        );
    }

    let hello_pos = docs_item_page
        .find("title=\"Hello World\"")
        .expect("docs should contain Hello World entry");
    let media_pos = docs_item_page
        .find("title=\"Media + Content + Actions\"")
        .expect("docs should contain media/content/actions entry");
    let header_footer_pos = docs_item_page
        .find("title=\"Header + Footer Layout\"")
        .expect("docs should contain header/footer entry");
    let matrix_pos = docs_item_page
        .find("title=\"State Matrix (Variant + Size)\"")
        .expect("docs should contain state matrix entry");

    assert!(
        hello_pos < media_pos && hello_pos < header_footer_pos && hello_pos < matrix_pos,
        "default Hello World path should appear before advanced playground sections"
    );
}

#[test]
fn e2e_contract_uses_semantic_selectors_and_wasm_stable_waits() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let e2e_source =
        std::fs::read_to_string(repo_root.join("e2e/tests/docs_app_item_contract.spec.mjs"))
            .expect("must read docs-app item e2e contract");

    for needle in [
        "page.goto(\"/#/components/item\")",
        "body:not(:has(#boot))",
        "[data-component=\"item\"]",
        "[data-slot=\"item\"][role=\"listitem\"]",
        "data-ui-schema=\"ui.item.agent-contract.v1\"",
        "data-ui-streaming-fallback=\"snapshot\"",
        "data-ui-output-status=\"validated\"",
        "[data-slot=\"segmented-control-option\"][data-index=\"1\"]",
    ] {
        assert!(
            e2e_source.contains(needle),
            "item e2e contract should keep semantic ready/selector marker `{needle}`"
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "nth-child",
        "locator(\"text=",
        "getByText(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "item e2e contract should avoid brittle selector/wait pattern `{forbidden}`"
        );
    }
}

#[test]
fn e2e_key_flow_is_repeatable_and_breakpoint_locatable() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let e2e_source =
        std::fs::read_to_string(repo_root.join("e2e/tests/docs_app_item_contract.spec.mjs"))
            .expect("must read docs-app item e2e contract");

    for needle in [
        "async function runItemInteractiveFlow(docsRoot)",
        "await variantControl",
        "await sizeControl",
        "await expect(previewItem).toHaveAttribute(\"data-variant\", \"outline\")",
        "await expect(previewItem).toHaveAttribute(\"data-size\", \"sm\")",
        "await expect(previewItem).toHaveAttribute(\"data-ui-output-status\", \"validated\")",
        "await page.reload();",
        "await runItemInteractiveFlow(reloadedRoot);",
        "docs-app item key flow is repeatable with semantic ready/settled breakpoints",
    ] {
        assert!(
            e2e_source.contains(needle),
            "item e2e key-flow regression should keep `{needle}` as a semantic breakpoint"
        );
    }
}

#[test]
fn docs_examples_and_matrix_are_synced_with_logic_api_contract() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docs_item_page = std::fs::read_to_string(
        repo_root.join("apps/docs-app/src/pages/components/pages/collections_item_primitives.rs"),
    )
    .expect("must read docs item page source");

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix (Variant + Size)\"",
        "title=\"Controlled vs Uncontrolled (N/A for Item)\"",
        "variant=ItemVariant::Default",
        "variant=ItemVariant::Outline",
        "variant=ItemVariant::Muted",
        "size=ItemSize::Default",
        "size=ItemSize::Sm",
        "variant=variant size=size",
    ] {
        assert!(
            docs_item_page.contains(needle),
            "docs examples/state matrix should keep `{needle}` aligned with item API"
        );
    }

    for needle in [
        "pub enum ItemVariant {",
        "Default,",
        "Outline,",
        "Muted,",
        "pub fn normalize_item_variant(variant: Option<ItemVariant>) -> ItemVariant",
        "variant.unwrap_or_default()",
        "pub enum ItemSize {",
        "Sm,",
        "pub fn normalize_item_size(size: Option<ItemSize>) -> ItemSize",
        "size.unwrap_or_default()",
    ] {
        assert!(
            LOGIC_SRC.contains(needle),
            "logic.rs should keep default and enum contract token `{needle}` for docs sync"
        );
    }
}

#[test]
fn docs_interactive_playground_supports_live_props_and_repeatable_flow() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docs_item_page = std::fs::read_to_string(
        repo_root.join("apps/docs-app/src/pages/components/pages/collections_item_primitives.rs"),
    )
    .expect("must read docs item page source");
    let e2e_source =
        std::fs::read_to_string(repo_root.join("e2e/tests/docs_app_item_contract.spec.mjs"))
            .expect("must read docs-app item e2e contract");

    for needle in [
        "title=\"Media + Content + Actions\"",
        "controls=move || view!",
        "let (variant_index, set_variant_index) = signal(Some(0_usize));",
        "let (size_index, set_size_index) = signal(Some(0_usize));",
        "id_base=\"docs-item-variant\".to_string()",
        "id_base=\"docs-item-size\".to_string()",
        "selected_index=variant_index",
        "set_selected_index=set_variant_index",
        "selected_index=size_index",
        "set_selected_index=set_size_index",
        "variant=variant size=size",
    ] {
        assert!(
            docs_item_page.contains(needle),
            "item docs interactive playground should keep `{needle}`"
        );
    }

    for needle in [
        "async function runItemInteractiveFlow(docsRoot)",
        "await variantControl",
        "await sizeControl",
        "await expect(previewItem).toHaveAttribute(\"data-variant\", \"outline\")",
        "await expect(previewItem).toHaveAttribute(\"data-size\", \"sm\")",
        "await page.reload();",
        "await runItemInteractiveFlow(reloadedRoot);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "item interactive e2e path should keep repeatable flow marker `{needle}`"
        );
    }
}

#[test]
fn docs_source_first_copy_paste_contract_is_wired_and_stable() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let docs_item_page = std::fs::read_to_string(
        repo_root.join("apps/docs-app/src/pages/components/pages/collections_item_primitives.rs"),
    )
    .expect("must read docs item page source");
    let e2e_source =
        std::fs::read_to_string(repo_root.join("e2e/tests/docs_app_item_contract.spec.mjs"))
            .expect("must read docs-app item e2e contract");

    for needle in [
        "data-slot=\"item-copy-ready-hint\"",
        "compose_copy_ready_code",
        "code_imports=ITEM_DOC_IMPORTS.to_string()",
        "components/item/src/view.rs",
        "components/item/src/logic.rs",
        "component-item",
    ] {
        assert!(
            docs_item_page.contains(needle),
            "item docs copy-ready contract should keep `{needle}`"
        );
    }

    for needle in [
        "docs-app item playground source is copy-paste ready",
        "data-copyable\", \"true\"",
        "Copy to clipboard",
        "use leptos::prelude::*;",
        "use ui_components::{Item",
    ] {
        assert!(
            e2e_source.contains(needle),
            "item e2e copy-ready path should keep `{needle}`"
        );
    }
}

#[test]
fn heroui_strategy_and_docs_index_entry_are_synced_for_item() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let heroui_strategy =
        std::fs::read_to_string(repo_root.join("docs/spec/heroui-parameter-design-strategy.md"))
            .expect("must read heroui strategy doc");
    let item_catalog = std::fs::read_to_string(
        repo_root.join("apps/docs-app/src/pages/components/pages/collections_item_catalog.rs"),
    )
    .expect("must read item docs catalog source");
    let docs_pages =
        std::fs::read_to_string(repo_root.join("apps/docs-app/src/pages/components/pages.rs"))
            .expect("must read docs pages catalog");

    for needle in [
        "### Item 同步记录（2026-02-21）",
        "`Item` 参数主轴保持 `variant/size/class_name/lang/dir`",
        "collections_item_catalog.rs",
        "slug = \"item\"",
        "State Matrix (Variant + Size)",
        "compose_copy_ready_code",
        "`component-item`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "heroui strategy should keep item sync token `{needle}`"
        );
    }

    for needle in [
        "pub(super) const ITEM_DOC: ComponentDoc",
        "name: \"Item\"",
        "slug: \"item\"",
        "page: super::collections_item_primitives::item_primitives",
    ] {
        assert!(
            item_catalog.contains(needle),
            "item docs catalog should keep entry token `{needle}`"
        );
    }

    assert!(
        docs_pages.contains("collections_item_catalog::ITEM_DOC"),
        "docs pages catalog should aggregate item docs entry"
    );
}

#[test]
fn engineering_contract_stays_structured_and_runtime_neutral() {
    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum ItemComponentSchemaVersion",
        "pub struct ItemComponentSpec",
        "#[serde(default)]",
        "schema_version: ItemComponentSchemaVersion",
    ] {
        assert!(
            PROTOCOL_SRC.contains(needle),
            "item protocol should keep structured serde contract `{needle}`"
        );
    }

    for source in [VIEW_SRC, LOGIC_SRC, MOD_SRC, STYLES_SRC, PROTOCOL_SRC] {
        for forbidden in [
            "tracing::",
            "#[instrument",
            "tokio::",
            "async_std::",
            "async fn",
            "JoinHandle",
            "Runtime",
        ] {
            assert!(
                !source.contains(forbidden),
                "item component should remain runtime-neutral without `{forbidden}`"
            );
        }
    }

    for forbidden in ["tracing", "tracing-subscriber", "tokio", "async-std"] {
        assert!(
            !CARGO_TOML_SRC.contains(forbidden),
            "item Cargo dependencies should not bind concrete tracing/runtime crate `{forbidden}`"
        );
    }
}

#[test]
fn spec_module_is_intentionally_absent_for_simple_item_component() {
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "simple item component must not introduce spec.rs without stable external schema needs"
    );

    for forbidden in ["mod spec;", "pub mod spec;"] {
        assert!(
            !MOD_SRC.contains(forbidden),
            "mod.rs must not wire optional spec module through `{forbidden}`"
        );
    }
}

#[test]
fn context_compression_manifest_and_rbi_projection_are_present_for_item() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required in ["Component.toml", "item.rbi"] {
        assert!(
            src_dir.join(required).exists(),
            "item should keep context-compression sidecar `{required}`"
        );
    }

    for required in [
        "schema_version = \"1\"",
        "[component]",
        "name = \"Item\"",
        "crate = \"ui-item\"",
        "rbi = \"item.rbi\"",
        "context_compression_manifest",
        "rbi_signature_projection",
        "agent_contract_schema_markers",
        "schema = \"ui.item.agent-contract.v1\"",
        "output_mode_axis = [\"streaming\", \"snapshot\"]",
        "name = \"streaming_policy\"",
        "name = \"streaming_fallback\"",
        "name = \"stream_mode\"",
        "name = \"output_status\"",
        "name = \"render_path\"",
        "streaming_optional_with_snapshot_fallback_and_output_status_markers",
        "inner_html",
    ] {
        assert!(
            COMPONENT_MANIFEST_SRC.contains(required),
            "Component.toml should keep context-compression marker `{required}`"
        );
    }

    for required in [
        "pub type A11yDirection = ui_headless::A11yDirection;",
        "pub type ItemVariant = crate::logic::ItemVariant;",
        "pub type ItemSize = crate::logic::ItemSize;",
        "pub type ItemMediaVariant = crate::logic::ItemMediaVariant;",
        "pub const ITEM_AGENT_SCHEMA: &str = \"ui.item.agent-contract.v1\";",
        "pub enum ItemAgentIntent",
        "pub enum ItemAgentAction",
        "pub enum ItemStreamingPolicy",
        "pub enum ItemStreamingFallback",
        "pub enum ItemAgentStreamMode",
        "pub enum ItemAgentOutputMode",
        "pub enum ItemOutputStatus",
        "Streaming,",
        "Snapshot,",
        "pub fn ItemGroup(",
        "pub fn ItemSeparator(",
        "pub fn Item(",
        "pub fn ItemMedia(",
        "pub fn ItemContent(",
        "pub fn ItemTitle(",
        "pub fn ItemDescription(",
        "pub fn ItemActions(",
        "pub fn ItemHeader(",
        "pub fn ItemFooter(",
    ] {
        assert!(
            ITEM_RBI_SRC.contains(required),
            "item.rbi should keep public signature projection token `{required}`"
        );
    }
}

#[test]
fn item_agent_contract_schema_is_typed_and_whitelisted() {
    for required in [
        "pub enum ItemAgentIntent",
        "pub enum ItemAgentAction",
        "pub enum ItemStreamingPolicy",
        "pub enum ItemStreamingFallback",
        "pub enum ItemAgentStreamMode",
        "pub enum ItemAgentOutputMode",
        "pub enum ItemOutputStatus",
        "pub struct ItemAgentDataAttrs",
        "pub const ITEM_AGENT_SCHEMA: &str = \"ui.item.agent-contract.v1\";",
        "pub fn agent_data_attrs(state: ItemRenderState) -> ItemAgentDataAttrs",
        "streaming_policy: ItemStreamingPolicy::Optional.as_attr()",
        "streaming_fallback: ItemStreamingFallback::Snapshot.as_attr()",
        "stream_mode: ItemAgentStreamMode::Snapshot.as_attr()",
        "output_mode: ItemAgentOutputMode::Snapshot.as_attr()",
        "output_status: ItemOutputStatus::Validated.as_attr()",
        "state_variant: state.variant_attr",
        "source_variant: state.variant_source_attr",
    ] {
        assert!(
            PROTOCOL_SRC.contains(required),
            "protocol.rs should keep typed agent-contract token `{required}`"
        );
    }

    for required in [
        "data-ui-schema=agent_attrs.schema",
        "data-ui-intent=agent_attrs.intent",
        "data-ui-action=agent_attrs.action",
        "data-ui-streaming-policy=agent_attrs.streaming_policy",
        "data-ui-streaming-fallback=agent_attrs.streaming_fallback",
        "data-ui-stream-mode=agent_attrs.stream_mode",
        "data-ui-output-mode=agent_attrs.output_mode",
        "data-ui-output-status=agent_attrs.output_status",
        "data-ui-state-variant=agent_attrs.state_variant",
        "data-ui-state-size=agent_attrs.state_size",
        "data-ui-source-variant=agent_attrs.source_variant",
        "data-ui-source-size=agent_attrs.source_size",
    ] {
        assert!(
            VIEW_SRC.contains(required),
            "view.rs should mount typed agent marker `{required}`"
        );
    }

    for required in [
        "[agent_contract]",
        "schema = \"ui.item.agent-contract.v1\"",
        "intent = \"collection-item\"",
        "output_mode_axis = [\"streaming\", \"snapshot\"]",
        "action_axis = [\"render\"]",
        "name = \"streaming_policy\"",
        "values = [\"optional\"]",
        "name = \"streaming_fallback\"",
        "values = [\"snapshot\"]",
        "name = \"stream_mode\"",
        "values = [\"streaming\", \"snapshot\"]",
        "name = \"output_mode\"",
        "values = [\"snapshot\"]",
        "name = \"output_status\"",
        "values = [\"validated\"]",
        "streaming_optional_with_snapshot_fallback_and_output_status_markers",
        "state_axes = [\"variant\", \"size\"]",
        "source_axes = [\"variant_source\", \"size_source\"]",
        "name = \"render_path\"",
        "typed_agent_contract_from_protocol::agent_data_attrs",
        "inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            COMPONENT_MANIFEST_SRC.contains(required),
            "Component.toml should keep agent-contract whitelist marker `{required}`"
        );
    }
}

#[test]
fn token_first_static_style_contract_is_preserved() {
    assert!(
        STYLES_SRC.contains("pub const CSS"),
        "styles.rs must expose static CSS contract for aggregation"
    );

    for forbidden in [
        "@apply",
        "tailwind",
        "css!(",
        "styled!(",
        "stylist::",
        "emotion",
        "linaria",
    ] {
        assert!(
            !STYLES_SRC.contains(forbidden),
            "styles.rs must avoid utility-first/css-in-rust default path `{forbidden}`"
        );
        assert!(
            !CARGO_TOML_SRC.contains(forbidden),
            "component dependencies must avoid utility-first/css-in-rust default path `{forbidden}`"
        );
    }
}

#[test]
fn defensive_variable_chain_for_item_min_inline_size_is_enforced() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ui_theme_css = std::fs::read_to_string(repo_root.join("crates/ui-theme/src/css.rs"))
        .expect("must read ui-theme css source");

    for needle in [
        "min-inline-size: var(--ui-item-min-inline-size, var(--ui-fallback-min-inline-size-none));",
        "--ui-fallback-min-inline-size-none",
    ] {
        assert!(
            STYLES_SRC.contains(needle) || ui_theme_css.contains(needle),
            "defensive variable contract must include `{needle}`"
        );
    }

    assert!(
        !STYLES_SRC.contains("min-inline-size: 0;"),
        "item styles should not hardcode bare min-inline-size terminal values"
    );

    for forbidden in ["#", "rgb(", "rgba(", "hsl("] {
        assert!(
            !STYLES_SRC.contains(forbidden),
            "item styles should avoid hardcoded color literal `{forbidden}`"
        );
    }
}

#[test]
fn item_styles_are_wired_into_uiroot_css_pipeline() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let css_src = std::fs::read_to_string(repo_root.join("crates/ui-components/src/css.rs"))
        .expect("must read ui-components css aggregator source");
    let root_src = std::fs::read_to_string(repo_root.join("crates/ui-components/src/root.rs"))
        .expect("must read ui-components root source");

    assert!(
        css_src.contains("out.push_str(crate::item::styles::CSS);"),
        "ui-components css aggregator must include item styles"
    );
    assert!(
        root_src.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot must call components css aggregation pipeline"
    );
}

#[test]
fn item_css_is_aggregated_under_ui_layer_without_plain_inline_style_paths() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let css_src = std::fs::read_to_string(repo_root.join("crates/ui-components/src/css.rs"))
        .expect("must read ui-components css aggregator source");

    assert!(
        css_src.contains("@layer ui {"),
        "components css aggregator must mount component styles under `@layer ui`"
    );
    assert!(
        css_src.contains("out.push_str(crate::item::styles::CSS);"),
        "item css must be routed through ui-components layered css aggregation"
    );

    for source in [VIEW_SRC, LOGIC_SRC, STYLES_SRC] {
        assert!(
            !source.contains(" style="),
            "item component must not use plain inline style attributes"
        );
    }
}

#[test]
fn ui_components_fixed_entry_files_remain_in_expected_locations() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ui_components_lib =
        std::fs::read_to_string(repo_root.join("crates/ui-components/src/lib.rs"))
            .expect("must read ui-components lib source");
    let ui_components_css =
        std::fs::read_to_string(repo_root.join("crates/ui-components/src/css.rs"))
            .expect("must read ui-components css source");
    let ui_components_root =
        std::fs::read_to_string(repo_root.join("crates/ui-components/src/root.rs"))
            .expect("must read ui-components root source");
    let active_highlight = std::fs::read_to_string(
        repo_root.join("crates/ui-visual-primitive/src/active_highlight.rs"),
    )
    .expect("must read ui-visual-primitive active_highlight source");

    assert!(
        ui_components_lib.contains("#[cfg(feature = \"component-item\")]")
            && ui_components_lib.contains("pub use ui_item as item;"),
        "ui-components lib.rs must keep feature-gated public item export boundary"
    );

    assert!(
        ui_components_css.contains("pub fn push_components_css(out: &mut String)")
            && ui_components_css.contains("@layer ui {")
            && ui_components_css.contains("out.push_str(crate::item::styles::CSS);"),
        "ui-components css.rs must keep layered component css aggregation entry"
    );

    for needle in [
        "pub fn UiRoot(",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            ui_components_root.contains(needle),
            "ui-components root.rs must keep centralized theme/css/i18n injection `{needle}`"
        );
    }

    for needle in [
        "pub const CSS: &str",
        "ActiveHighlightMotion",
        "attach_active_highlight_motion",
    ] {
        assert!(
            active_highlight.contains(needle),
            "active_highlight primitive should keep shared highlight style+motion contract `{needle}`"
        );
    }

    for forbidden in ["data-slot=\"item\"", "component-item", "ItemGroup"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight primitive should not absorb item-specific business semantic `{forbidden}`"
        );
    }

    for missing in [
        repo_root.join("crates/ui-components/src/overlay_open.rs"),
        repo_root.join("crates/ui-components/src/presence.rs"),
        repo_root.join("crates/ui-components/src/a11y.rs"),
    ] {
        assert!(
            !missing.exists(),
            "ui-components fixed-entry layout should not reintroduce forbidden file {:?}",
            missing.file_name().expect("must have file name")
        );
    }
}

#[test]
fn tree_shaking_feature_gates_for_item_are_wired() {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ui_components_cargo =
        std::fs::read_to_string(repo_root.join("crates/ui-components/Cargo.toml"))
            .expect("must read ui-components Cargo.toml");
    let ui_components_lib =
        std::fs::read_to_string(repo_root.join("crates/ui-components/src/lib.rs"))
            .expect("must read ui-components lib.rs");
    let ui_components_css =
        std::fs::read_to_string(repo_root.join("crates/ui-components/src/css.rs"))
            .expect("must read ui-components css.rs");

    assert!(
        ui_components_cargo.contains("component-item = [\"dep:ui-item\"]"),
        "ui-components must expose component-item feature gate"
    );
    assert!(
        ui_components_cargo
            .contains("ui-item = { path = \"../../components/item\", optional = true }"),
        "ui-item dependency must stay optional for package-level tree shaking"
    );

    assert!(
        ui_components_lib
            .contains("#[cfg(feature = \"component-item\")]\npub use ui_item as item;"),
        "lib.rs must gate item export by component-item feature"
    );
    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-item\")]\n    out.push_str(crate::item::styles::CSS);"
        ),
        "css.rs must gate item css aggregation by component-item feature"
    );
}
