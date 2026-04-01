use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

pub struct Hud {
    heading_el: HtmlElement,
    speed_el: HtmlElement,
}

impl Hud {
    pub fn new() -> Self {
        let document = web_sys::window()
            .expect("window")
            .document()
            .expect("document");

        let container = create_hud_container(&document);
        let heading_el = create_readout(&document, "HDG");
        let speed_el = create_readout(&document, "SPD");

        container
            .append_child(&heading_el)
            .expect("append heading");
        container.append_child(&speed_el).expect("append speed");

        document
            .body()
            .expect("body")
            .append_child(&container)
            .expect("append hud");

        Self {
            heading_el,
            speed_el,
        }
    }

    pub fn draw(&self, heading_deg: f32, pitch_deg: f32, speed: f32) {
        // Normalize heading to 0..360
        let hdg = ((heading_deg % 360.0) + 360.0) % 360.0;
        self.heading_el
            .set_inner_html(&format!("HDG {:03.0}\u{00b0} P {:+.0}\u{00b0}", hdg, pitch_deg));
        self.speed_el
            .set_inner_html(&format!("SPD {:.0}", speed));
    }
}

fn create_hud_container(document: &Document) -> HtmlElement {
    let el = document
        .create_element("div")
        .expect("create div")
        .dyn_into::<HtmlElement>()
        .expect("cast");
    let style = el.style();
    style.set_property("position", "fixed").ok();
    style.set_property("bottom", "20px").ok();
    style.set_property("left", "50%").ok();
    style.set_property("transform", "translateX(-50%)").ok();
    style.set_property("display", "flex").ok();
    style.set_property("gap", "40px").ok();
    style.set_property("pointer-events", "none").ok();
    style.set_property("z-index", "10").ok();
    el
}

fn create_readout(document: &Document, _label: &str) -> HtmlElement {
    let el = document
        .create_element("span")
        .expect("create span")
        .dyn_into::<HtmlElement>()
        .expect("cast");
    let style = el.style();
    style.set_property("font-family", "monospace").ok();
    style.set_property("font-size", "16px").ok();
    style.set_property("color", "rgba(100, 200, 255, 0.8)").ok();
    style
        .set_property("text-shadow", "0 0 6px rgba(100, 200, 255, 0.4)")
        .ok();
    style.set_property("letter-spacing", "1px").ok();
    el
}
