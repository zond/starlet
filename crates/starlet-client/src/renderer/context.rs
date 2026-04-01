use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};

pub fn setup_canvas() -> (HtmlCanvasElement, GL) {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let canvas = document
        .get_element_by_id("starlet-canvas")
        .expect("canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("cast to canvas");

    let gl = canvas
        .get_context("webgl2")
        .expect("get_context")
        .expect("webgl2 support")
        .dyn_into::<GL>()
        .expect("cast to GL");

    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    resize_canvas(&canvas, &gl);
    (canvas, gl)
}

pub fn resize_canvas(canvas: &HtmlCanvasElement, gl: &GL) {
    let window = web_sys::window().expect("window");
    let dpr = window.device_pixel_ratio();

    // Use client dimensions, falling back to window inner dimensions
    // (mobile browsers can report 0 for client dimensions before layout)
    let mut cw = canvas.client_width();
    let mut ch = canvas.client_height();
    if cw == 0 || ch == 0 {
        cw = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0) as i32;
        ch = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0) as i32;
    }

    let w = (cw as f64 * dpr) as u32;
    let h = (ch as f64 * dpr) as u32;
    if w > 0 && h > 0 && (canvas.width() != w || canvas.height() != h) {
        canvas.set_width(w);
        canvas.set_height(h);
        gl.viewport(0, 0, w as i32, h as i32);
    }
}
