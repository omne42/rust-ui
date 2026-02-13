#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupMotion {
    pub enter_ms: u16,
    pub exit_ms: u16,
}

impl Default for GroupMotion {
    fn default() -> Self {
        Self {
            enter_ms: 160,
            exit_ms: 120,
        }
    }
}

pub fn sanitize_motion(motion: GroupMotion) -> GroupMotion {
    GroupMotion {
        enter_ms: motion.enter_ms.clamp(16, 2_000),
        exit_ms: motion.exit_ms.clamp(16, 2_000),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(node_ref: leptos::prelude::NodeRef<leptos::html::Div>, motion: GroupMotion) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();
        let motion = motion.get_value();
        let _ = style.set_property("--ui-group-enter-ms", &motion.enter_ms.to_string());
        let _ = style.set_property("--ui-group-exit-ms", &motion.exit_ms.to_string());
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(_node_ref: leptos::prelude::NodeRef<leptos::html::Div>, motion: GroupMotion) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_clamps_bounds() {
        assert_eq!(
            sanitize_motion(GroupMotion {
                enter_ms: 0,
                exit_ms: 5_000,
            }),
            GroupMotion {
                enter_ms: 16,
                exit_ms: 2_000,
            }
        );
    }
}
