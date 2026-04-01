pub mod abstraction;
pub mod mouse;
pub mod touch;

use web_sys::HtmlCanvasElement;

use abstraction::InputBackend;
use mouse::MouseInput;
use touch::TouchInput;

pub fn create_input(canvas: &HtmlCanvasElement) -> Box<dyn InputBackend> {
    let window = web_sys::window().expect("window");
    let navigator = window.navigator();

    let max_touch = navigator.max_touch_points();
    let is_touch_device = max_touch > 0;

    web_sys::console::log_1(
        &format!("input detection: maxTouchPoints={max_touch} is_touch={is_touch_device}").into(),
    );

    if is_touch_device {
        Box::new(TouchInput::new())
    } else {
        Box::new(MouseInput::new(canvas))
    }
}
