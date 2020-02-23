use std::rc::Rc;
use failure::{Fallible, format_err};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[derive(Clone, Debug)]
pub struct View {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl View {
    const SCALE: f64 = 10.;

    pub fn new() -> Fallible<Rc<Self>> {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("view").unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| format_err!(
                "Failed to convert #view to an HtmlCanvasElement"))?;
        let ctx = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| format_err!(
                "Failed to convert ctx to an CanvasRenderingContext2d"))?;

        ctx.scale(View::SCALE, View::SCALE)
            .map_err(|_| format_err!("Failed to scale ctx"))?;

        let view = Rc::new(View { canvas, ctx });
        let view_clone = view.clone();
        let step = Closure::wrap(Box::new(move || {
            view_clone.step()
        }) as Box<dyn Fn()>);

        // See https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html.
        window.request_animation_frame(step.as_ref().unchecked_ref())
            .map_err(|_| format_err!("Failed to call requestAnimationFrame"))?;

        Ok(view)
    }

    fn step(&self) {
        self.draw_pixel(10., 20., true);
    }

    pub fn draw_pixel(&self, x: f64, y: f64, is_filled: bool) {
        let fill_color = if is_filled {
            "rgb(0, 0, 0)"
        } else {
            "rgb(255, 255, 255)"
        };
        self.ctx.set_fill_style(&JsValue::from_str(&fill_color));
        self.ctx.fill_rect(x, y, 1., 1.);
    }

    pub fn clear(&self) -> Fallible<()> {
        // Save transformation matrix.
        self.ctx.save();

        // Use the identity matrix while clearing the canvas.
        self.ctx.set_transform(1., 0., 0., 1., 0., 0.)
            .map_err(|_| format_err!("Failed to transform canvas"))?;

        self.ctx.clear_rect(
            0., 0., self.canvas.width().into(), self.canvas.height().into());

        // Restore transformation matrix.
        self.ctx.restore();

        Ok(())
    }
}
