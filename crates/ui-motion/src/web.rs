use crate::{keyframes::MotionKeyframe, options::MotionOptions};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub fn prefers_reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
        })
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

pub fn animate(element: &web_sys::Element, keyframes: &[MotionKeyframe], options: MotionOptions) {
    if prefers_reduced_motion() {
        return;
    }

    let keyframes = keyframes_to_js(keyframes);
    let options = options_to_js(options);
    let Ok(animate) = js_sys::Reflect::get(element, &JsValue::from_str("animate")) else {
        return;
    };
    let Ok(animate) = animate.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = animate.call2(element, &keyframes, &options);
}

fn keyframes_to_js(frames: &[MotionKeyframe]) -> JsValue {
    let arr = js_sys::Array::new();
    for frame in frames {
        let obj = js_sys::Object::new();
        if let Some(offset) = frame.offset {
            let _ = js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("offset"),
                &JsValue::from_f64(offset),
            );
        }
        for prop in &frame.props {
            let _ = js_sys::Reflect::set(
                &obj,
                &JsValue::from_str(&prop.name),
                &JsValue::from_str(&prop.value),
            );
        }
        arr.push(&obj);
    }
    arr.into()
}

fn options_to_js(options: MotionOptions) -> JsValue {
    let obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("duration"),
        &JsValue::from_f64(f64::from(options.duration_ms)),
    );
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("easing"),
        &JsValue::from_str(options.easing),
    );
    let _ = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("fill"),
        &JsValue::from_str(options.fill.as_str()),
    );
    obj.into()
}
