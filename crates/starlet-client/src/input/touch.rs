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
    event_count: u32,
    poll_count: u32,
    /// Screen orientation angle: 0 = portrait, 90/-90 = landscape, 180 = upside-down
    screen_angle: f64,
}

pub struct TouchInput {
    state: Rc<RefCell<State>>,
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
            event_count: 0,
            poll_count: 0,
            screen_angle: get_screen_angle(),
        }));

        let window = web_sys::window().expect("window");
        let document = window.document().expect("document");
        let mut closures: Vec<Box<dyn std::any::Any>> = Vec::new();

        // Show the slider
        if let Some(slider) = document.get_element_by_id("thrust-slider") {
            let _ = slider.class_list().add_1("visible");
        }

        // Track screen orientation changes
        let s = state.clone();
        let orient_closure = Closure::<dyn FnMut()>::new(move || {
            let mut st = s.borrow_mut();
            st.screen_angle = get_screen_angle();
            // Recalibrate on orientation change
            st.calibrated = false;
            web_sys::console::log_1(
                &format!("screen orientation changed: {}°", st.screen_angle).into(),
            );
        });
        // Listen on both the modern API and the legacy event
        if let Some(screen) = window.screen().ok() {
            if let Ok(orientation) = js_sys::Reflect::get(&screen, &"orientation".into()) {
                if !orientation.is_undefined() {
                    let _ = orientation.dyn_ref::<web_sys::EventTarget>().map(|t| {
                        t.add_event_listener_with_callback(
                            "change",
                            orient_closure.as_ref().unchecked_ref(),
                        )
                    });
                }
            }
        }
        window
            .add_event_listener_with_callback(
                "orientationchange",
                orient_closure.as_ref().unchecked_ref(),
            )
            .ok();
        closures.push(Box::new(orient_closure));

        // Build the orientation listener
        let orientation_cb = build_orientation_callback(state.clone());

        // Check if requestPermission is needed (iOS 13+)
        let doe = js_sys::Reflect::get(&window, &"DeviceOrientationEvent".into()).ok();
        let has_doe = doe.as_ref().map(|v| !v.is_undefined()).unwrap_or(false);
        let needs_permission = doe
            .as_ref()
            .and_then(|cls| js_sys::Reflect::get(cls, &"requestPermission".into()).ok())
            .map(|v| v.is_function())
            .unwrap_or(false);

        web_sys::console::log_1(
            &format!(
                "touch init: screen_angle={}, has_DOE={}, needs_permission={}, is_secure={}",
                get_screen_angle(),
                has_doe,
                needs_permission,
                window.is_secure_context(),
            )
            .into(),
        );

        if needs_permission {
            web_sys::console::log_1(&"iOS path: showing permission prompt".into());
            let prompt = create_permission_prompt(&document);
            let prompt_clone = prompt.clone();
            let orientation_cb_clone =
                orientation_cb.as_ref().unchecked_ref::<js_sys::Function>().clone();
            let tap_closure = Closure::<dyn FnMut()>::new(move || {
                if let Some(parent) = prompt_clone.parent_node() {
                    let _ = parent.remove_child(&prompt_clone);
                }

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
            web_sys::console::log_1(&"non-iOS path: attaching deviceorientation directly".into());
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

fn get_screen_angle() -> f64 {
    let window = web_sys::window().expect("window");
    // Try modern API first: screen.orientation.angle
    if let Ok(screen) = window.screen() {
        if let Ok(orientation) = js_sys::Reflect::get(&screen, &"orientation".into()) {
            if !orientation.is_undefined() {
                if let Ok(angle) = js_sys::Reflect::get(&orientation, &"angle".into()) {
                    if let Some(a) = angle.as_f64() {
                        return a;
                    }
                }
            }
        }
    }
    // Fallback: window.orientation (deprecated but widely supported)
    if let Ok(val) = js_sys::Reflect::get(&window, &"orientation".into()) {
        if let Some(a) = val.as_f64() {
            return a;
        }
    }
    0.0
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
            // Skip calibration on the first few events — alpha often starts at 0
            // before the compass settles.
            st.event_count += 1;
            if st.event_count < 5 {
                return;
            }
            st.base_alpha = alpha;
            st.base_beta = beta;
            st.base_gamma = gamma;
            st.calibrated = true;
            web_sys::console::log_1(
                &format!(
                    "orientation calibrated (screen_angle={}°, after {} events): a={:.1} b={:.1} g={:.1}",
                    st.screen_angle, st.event_count, alpha, beta, gamma
                )
                .into(),
            );
        }

        st.alpha = alpha;
        st.beta = beta;
        st.gamma = gamma;
        st.event_count += 1;
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

/// Wrap an angle delta to [-180, 180] to handle 0/360 wraparound.
fn wrap_angle(mut d: f64) -> f64 {
    d %= 360.0;
    if d > 180.0 {
        d -= 360.0;
    } else if d < -180.0 {
        d += 360.0;
    }
    d
}

impl InputBackend for TouchInput {
    fn poll(&mut self) -> InputState {
        let mut st = self.state.borrow_mut();
        st.poll_count += 1;

        if !st.calibrated {
            return InputState::default();
        }

        // Only use beta (front-back tilt) and gamma (left-right tilt).
        // No compass (alpha) — pure accelerometer.
        let d_beta = wrap_angle(st.beta - st.base_beta) as f32;
        let d_gamma = wrap_angle(st.gamma - st.base_gamma) as f32;

        let max_tilt = 15.0_f32;
        let max_turn = starlet_shared::constants::MAX_TURN_RATE;

        // Soft center: shift base if tilted beyond range
        if d_beta.abs() > max_tilt {
            st.base_beta += (d_beta - d_beta.signum() * max_tilt) as f64;
        }
        if d_gamma.abs() > max_tilt {
            st.base_gamma += (d_gamma - d_gamma.signum() * max_tilt) as f64;
        }

        // Recompute after center shift
        let d_beta = wrap_angle(st.beta - st.base_beta) as f32;
        let d_gamma = wrap_angle(st.gamma - st.base_gamma) as f32;

        // Remap axes based on screen orientation.
        // beta = tilt front/back, gamma = tilt left/right (in device portrait).
        // In landscape, these swap roles.
        // Joystick feel: push phone forward = nose down, tilt right = turn right.
        let (raw_yaw, raw_pitch) = match st.screen_angle as i32 {
            90 => (-d_beta, -d_gamma),
            -90 | 270 => (d_beta, d_gamma),
            180 => (d_gamma, d_beta),
            _ => (-d_gamma, -d_beta),
        };

        // Map tilt angle to turn rate: [-max_tilt, max_tilt] → [-max_turn, max_turn]
        let yaw = (raw_yaw / max_tilt).clamp(-1.0, 1.0) * max_turn;
        let pitch = (raw_pitch / max_tilt).clamp(-1.0, 1.0) * max_turn;

        if st.poll_count % 300 == 1 {
            web_sys::console::log_1(
                &format!(
                    "touch poll #{}: events={} db={:.1} dg={:.1} → yaw={:.2} pitch={:.2} screen={}°",
                    st.poll_count, st.event_count,
                    d_beta, d_gamma,
                    yaw, pitch, st.screen_angle,
                )
                .into(),
            );
        }

        InputState {
            yaw,
            pitch,
            roll: 0.0,
            speed: st.speed,
        }
    }
}
