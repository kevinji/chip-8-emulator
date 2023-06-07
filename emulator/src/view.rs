use core::mem;
use gloo_render::request_animation_frame;
use gloo_utils::document;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const SIXTY_FPS_FRAME_MS: f64 = 1000. / 60.;

#[derive(Clone, Debug)]
pub struct View {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl View {
    const SCALE: f64 = 10.;

    pub fn new() -> Self {
        let canvas = document()
            .get_element_by_id("view")
            .unwrap_throw()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap_throw();
        let ctx = canvas
            .get_context("2d")
            .unwrap_throw()
            .unwrap_throw()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap_throw();

        ctx.scale(Self::SCALE, Self::SCALE).unwrap_throw();

        Self { canvas, ctx }
    }

    pub fn draw_pixel(&self, x: u8, y: u8, is_filled: bool) {
        let fill_color = if is_filled {
            "rgb(0, 0, 0)"
        } else {
            "rgb(255, 255, 255)"
        };
        self.ctx.set_fill_style(&JsValue::from_str(fill_color));
        self.ctx.fill_rect(x.into(), y.into(), 1., 1.);
    }

    pub fn clear(&self) {
        // Save transformation matrix.
        self.ctx.save();

        // Use the identity matrix while clearing the canvas.
        self.ctx
            .set_transform(1., 0., 0., 1., 0., 0.)
            .unwrap_throw();

        self.ctx.clear_rect(
            0.,
            0.,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        // Restore transformation matrix.
        self.ctx.restore();
    }
}

fn run_on_tick(mut last_time_ms: f64, mut f: impl FnMut() + 'static) -> impl FnOnce(f64) {
    move |time_ms| {
        if time_ms - last_time_ms >= SIXTY_FPS_FRAME_MS {
            f();
            last_time_ms = time_ms;
        }

        mem::forget(request_animation_frame(run_on_tick(last_time_ms, f)));
    }
}

pub fn set_up_render_loop(f: impl FnMut() + 'static) {
    mem::forget(request_animation_frame(run_on_tick(0., f)));
}
