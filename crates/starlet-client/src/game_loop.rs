use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use crate::app::App;
use starlet_shared::constants::PHYSICS_DT;

const MAX_FRAME_TIME: f64 = 0.25;

struct LoopState {
    app: App,
    previous_time: f64,
    accumulator: f64,
}

pub fn start() {
    let state = Rc::new(RefCell::new(LoopState {
        app: App::new(),
        previous_time: -1.0,
        accumulator: 0.0,
    }));

    let closure: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let closure_clone = closure.clone();

    let state_clone = state.clone();
    *closure.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
        let mut ls = state_clone.borrow_mut();

        if ls.previous_time < 0.0 {
            ls.previous_time = timestamp;
        }

        let mut frame_time = (timestamp - ls.previous_time) / 1000.0;
        ls.previous_time = timestamp;

        if frame_time > MAX_FRAME_TIME {
            frame_time = MAX_FRAME_TIME;
        }

        ls.accumulator += frame_time;

        while ls.accumulator >= PHYSICS_DT as f64 {
            ls.app.update(PHYSICS_DT);
            ls.accumulator -= PHYSICS_DT as f64;
        }

        ls.app.draw();

        // Schedule next frame
        let window = web_sys::window().expect("window");
        let cb = closure_clone.borrow();
        let cb = cb.as_ref().expect("closure");
        window
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .expect("raf");
    }));

    // Kick off the first frame
    let window = web_sys::window().expect("window");
    let cb = closure.borrow();
    let cb = cb.as_ref().expect("closure");
    window
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .expect("raf");
}
