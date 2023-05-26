use eyre::eyre;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Debug)]
pub struct View {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl View {
    const SCALE: f64 = 10.;

    pub fn new() -> eyre::Result<Rc<Self>> {
        let window = window().ok_or_else(|| eyre!("window does not exist"))?;
        let document = window
            .document()
            .ok_or_else(|| eyre!("document does not exist"))?;
        let canvas = document
            .get_element_by_id("view")
            .ok_or_else(|| eyre!("#view does not exist"))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| eyre!("Failed to convert #view to an HtmlCanvasElement"))?;
        let ctx = canvas
            .get_context("2d")
            .map_err(|_| eyre!("Failed to get 2d canvas context"))?
            .ok_or_else(|| eyre!("Failed to get 2d canvas context"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| eyre!("Failed to convert ctx to an CanvasRenderingContext2d"))?;

        ctx.scale(Self::SCALE, Self::SCALE)
            .map_err(|_| eyre!("Failed to scale ctx"))?;

        let view = Rc::new(Self { canvas, ctx });
        Ok(view)
    }

    pub fn draw_pixel(&self, x: f64, y: f64, is_filled: bool) {
        let fill_color = if is_filled {
            "rgb(0, 0, 0)"
        } else {
            "rgb(255, 255, 255)"
        };
        self.ctx.set_fill_style(&JsValue::from_str(fill_color));
        self.ctx.fill_rect(x, y, 1., 1.);
    }

    pub fn clear(&self) -> eyre::Result<()> {
        // Save transformation matrix.
        self.ctx.save();

        // Use the identity matrix while clearing the canvas.
        self.ctx
            .set_transform(1., 0., 0., 1., 0., 0.)
            .map_err(|_| eyre!("Failed to transform canvas"))?;

        self.ctx.clear_rect(
            0.,
            0.,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        // Restore transformation matrix.
        self.ctx.restore();

        Ok(())
    }
}
