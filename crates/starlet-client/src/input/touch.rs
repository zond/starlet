use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::abstraction::{InputBackend, InputState};

struct State {
    alpha: f64,
    beta: f64,
    gamma: f64,
    base_alpha: f64,
    base_beta: f64,
    base_gamma: f64,
    calibrated: bool,
    speed: f32,
}

pub struct TouchInput {
    state: Rc<RefCell<State>>,
    // Keep closures alive
    _closures: Vec<Box<dyn std::any::Any>>,
}

impl TouchInput {
    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(State {
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
            base_alpha: 0.0,
            base_beta: 0.0,
            base_gamma: 0.0,
            calibrated: false,
            speed: 0.0,
        }));

        let window = web_sys::window().expect("window");
        let document = window.document().expect("document");
        let mut closures: Vec<Box<dyn std::any::Any>> = Vec::new();

        // Show the slider
        if let Some(slider) = document.get_element_by_id("thrust-slider") {
            let _ = slider.class_list().add_1("visible");
        }

        // Build the orientation listener (reused after permission grant)
        let orientation_cb = build_orientation_callback(state.clone());

        // Check if requestPermission is needed (iOS 13+)
        let doe = js_sys::Reflect::get(&window, &"DeviceOrientationEvent".into()).ok();
        let needs_permission = doe
            .as_ref()
            .and_then(|cls| js_sys::Reflect::get(cls, &"requestPermission".into()).ok())
            .map(|v| v.is_function())
            .unwrap_or(false);

        if needs_permission {
            // iOS: must request permission from a user gesture.
            // Show a prompt overlay and request on tap.
            let prompt = create_permission_prompt(&document);
            let prompt_clone = prompt.clone();
            let orientation_cb_clone = orientation_cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let tap_closure = Closure::<dyn FnMut()>::new(move || {
                // Remove the prompt
                if let Some(parent) = prompt_clone.parent_node() {
                    let _ = parent.remove_child(&prompt_clone);
                }

                // Call DeviceOrientationEvent.requestPermission()
                let window = web_sys::window().expect("window");
                let doe = js_sys::Reflect::get(&window, &"DeviceOrientationEvent".into())
                    .expect("DOE");
                let request_fn = js_sys::Reflect::get(&doe, &"requestPermission".into())
                    .expect("requestPermission");
                let request_fn = request_fn.dyn_into::<js_sys::Function>().expect("fn");
                let promise = request_fn.call0(&doe).expect("call");
                let promise = js_sys::Promise::from(promise);

                let cb = orientation_cb_clone.clone();
                let then_closure =
                    Closure::<dyn FnMut(JsValue)>::new(move |result: JsValue| {
                        let granted = result.as_string().map(|s| s == "granted").unwrap_or(false);
                        if granted {
                            let window = web_sys::window().expect("window");
                            window
                                .add_event_listener_with_callback("deviceorientation", &cb)
                                .expect("add deviceorientation");
                            web_sys::console::log_1(&"orientation permission granted".into());
                        } else {
                            web_sys::console::log_1(&"orientation permission denied".into());
                        }
                    });
                let _ = promise.then(&then_closure);
                then_closure.forget();
            });
            prompt
                .add_event_listener_with_callback("click", tap_closure.as_ref().unchecked_ref())
                .expect("add click");
            closures.push(Box::new(tap_closure));
        } else {
            // Android / desktop: just add the listener directly
            window
                .add_event_listener_with_callback(
                    "deviceorientation",
                    orientation_cb.as_ref().unchecked_ref(),
                )
                .expect("add deviceorientation");
        }
        closures.push(Box::new(orientation_cb));

        // Slider input
        let s = state.clone();
        let slider_closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(val) = input.value().parse::<f32>() {
                        s.borrow_mut().speed =
                            val / 100.0 * starlet_shared::constants::MAX_SPEED;
                    }
                }
            }
        });
        if let Some(slider) = document.get_element_by_id("thrust-slider") {
            slider
                .add_event_listener_with_callback("input", slider_closure.as_ref().unchecked_ref())
                .expect("add slider input");
        }
        closures.push(Box::new(slider_closure));

        Self {
            state,
            _closures: closures,
        }
    }
}

fn build_orientation_callback(
    state: Rc<RefCell<State>>,
) -> Closure<dyn FnMut(web_sys::DeviceOrientationEvent)> {
    Closure::new(move |e: web_sys::DeviceOrientationEvent| {
        let mut st = state.borrow_mut();
        let alpha = e.alpha().unwrap_or(0.0);
        let beta = e.beta().unwrap_or(0.0);
        let gamma = e.gamma().unwrap_or(0.0);

        if !st.calibrated {
            st.base_alpha = alpha;
            st.base_beta = beta;
            st.base_gamma = gamma;
            st.calibrated = true;
            web_sys::console::log_1(&"orientation calibrated".into());
        }

        st.alpha = alpha;
        st.beta = beta;
        st.gamma = gamma;
    })
}

fn create_permission_prompt(document: &web_sys::Document) -> web_sys::HtmlElement {
    let el = document
        .create_element("div")
        .expect("create div")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("cast");
    el.set_inner_html("Tap to enable gyro controls");
    let style = el.style();
    style.set_property("position", "fixed").ok();
    style.set_property("top", "60px").ok();
    style.set_property("left", "50%").ok();
    style.set_property("transform", "translateX(-50%)").ok();
    style.set_property("padding", "16px 32px").ok();
    style.set_property("background", "rgba(100, 200, 255, 0.2)").ok();
    style.set_property("border", "1px solid rgba(100, 200, 255, 0.5)").ok();
    style.set_property("border-radius", "8px").ok();
    style.set_property("color", "rgba(100, 200, 255, 0.9)").ok();
    style.set_property("font-family", "monospace").ok();
    style.set_property("font-size", "16px").ok();
    style.set_property("cursor", "pointer").ok();
    style.set_property("z-index", "100").ok();
    document
        .body()
        .expect("body")
        .append_child(&el)
        .expect("append");
    el
}

impl InputBackend for TouchInput {
    fn poll(&mut self) -> InputState {
        let st = self.state.borrow();
        let sensitivity = 0.03;

        // Relative to calibration pose
        let yaw = ((st.alpha - st.base_alpha) as f32).to_radians() * sensitivity;
        let pitch = ((st.beta - st.base_beta) as f32).to_radians() * sensitivity;
        let roll = ((st.gamma - st.base_gamma) as f32).to_radians() * sensitivity;

        InputState {
            yaw: -yaw,
            pitch: -pitch,
            roll: -roll,
            speed: st.speed,
        }
    }
}
