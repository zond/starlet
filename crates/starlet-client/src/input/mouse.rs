use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use super::abstraction::{InputBackend, InputState};

/// Max pixels from center before the joystick is at full deflection.
const JOYSTICK_RANGE: f32 = 200.0;
/// Max turn rate in radians/sec at full joystick deflection.
const MAX_TURN_RATE: f32 = 1.0;

struct State {
    joy_x: f32,
    joy_y: f32,
    speed: f32,
    locked: bool,
}

pub struct MouseInput {
    state: Rc<RefCell<State>>,
    _move_closure: Closure<dyn FnMut(web_sys::MouseEvent)>,
    _wheel_closure: Closure<dyn FnMut(web_sys::WheelEvent)>,
    _click_closure: Closure<dyn FnMut()>,
}

impl MouseInput {
    pub fn new(canvas: &HtmlCanvasElement) -> Self {
        let state = Rc::new(RefCell::new(State {
            joy_x: 0.0,
            joy_y: 0.0,
            speed: 0.0,
            locked: false,
        }));

        let document = web_sys::window()
            .expect("window")
            .document()
            .expect("document");

        // Mouse move: accumulate joystick displacement (stays where you push it)
        let s = state.clone();
        let move_closure =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
                let mut st = s.borrow_mut();
                if st.locked {
                    st.joy_x = (st.joy_x + e.movement_x() as f32)
                        .clamp(-JOYSTICK_RANGE, JOYSTICK_RANGE);
                    st.joy_y = (st.joy_y + e.movement_y() as f32)
                        .clamp(-JOYSTICK_RANGE, JOYSTICK_RANGE);
                }
            });
        document
            .add_event_listener_with_callback("mousemove", move_closure.as_ref().unchecked_ref())
            .expect("add mousemove");

        // Wheel sets speed directly (~3 units per notch, 0 to MAX_SPEED)
        let s = state.clone();
        let wheel_closure =
            Closure::<dyn FnMut(web_sys::WheelEvent)>::new(move |e: web_sys::WheelEvent| {
                let mut st = s.borrow_mut();
                st.speed = (st.speed - e.delta_y() as f32 * 0.15)
                    .clamp(0.0, starlet_shared::constants::MAX_SPEED);
            });
        document
            .add_event_listener_with_callback("wheel", wheel_closure.as_ref().unchecked_ref())
            .expect("add wheel");

        // Click to lock pointer
        let canvas_clone = canvas.clone();
        let s = state.clone();
        let click_closure = Closure::<dyn FnMut()>::new(move || {
            let _ = canvas_clone.request_pointer_lock();
            s.borrow_mut().locked = true;
        });
        canvas
            .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
            .expect("add click");

        Self {
            state,
            _move_closure: move_closure,
            _wheel_closure: wheel_closure,
            _click_closure: click_closure,
        }
    }
}

impl InputBackend for MouseInput {
    fn poll(&mut self) -> InputState {
        let st = self.state.borrow();

        // Joystick deflection as fraction of range (-1.0 to 1.0)
        let joy_x_frac = st.joy_x / JOYSTICK_RANGE;
        let joy_y_frac = st.joy_y / JOYSTICK_RANGE;

        // Desired turn rate proportional to deflection
        InputState {
            yaw: -joy_x_frac * MAX_TURN_RATE,
            pitch: -joy_y_frac * MAX_TURN_RATE,
            roll: 0.0,
            speed: st.speed,
        }
    }
}
