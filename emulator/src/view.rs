use core::fmt;
use gloo_utils::{document, window};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const SIXTY_FPS_FRAME_MS: f64 = 1000. / 60.;
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

fn clear_canvas(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) {
    // Save transformation matrix.
    ctx.save();

    // Use the identity matrix while clearing the canvas.
    ctx.set_transform(1., 0., 0., 1., 0., 0.).unwrap_throw();

    ctx.clear_rect(0., 0., canvas.width().into(), canvas.height().into());

    // Restore transformation matrix.
    ctx.restore();
}

#[derive(Clone, Debug)]
pub struct View {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    filled_pixels: [[bool; HEIGHT]; WIDTH],
}

impl View {
    const SCALE: f64 = 10.;

    #[allow(clippy::new_without_default)]
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

        clear_canvas(&canvas, &ctx);
        ctx.scale(Self::SCALE, Self::SCALE).unwrap_throw();

        let filled_pixels = [[false; HEIGHT]; WIDTH];

        Self {
            canvas,
            ctx,
            filled_pixels,
        }
    }

    pub fn is_pixel_filled(&self, x: u8, y: u8) -> bool {
        self.filled_pixels[x as usize][y as usize]
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8, is_filled: bool) {
        self.filled_pixels[x as usize][y as usize] = is_filled;

        let fill_color = if is_filled {
            "rgb(0, 0, 0)"
        } else {
            "rgb(255, 255, 255)"
        };
        self.ctx.set_fill_style(&JsValue::from_str(fill_color));
        self.ctx.fill_rect(x.into(), y.into(), 1., 1.);
    }

    pub fn clear(&mut self) {
        self.filled_pixels = [[false; HEIGHT]; WIDTH];
        clear_canvas(&self.canvas, &self.ctx);
    }
}

struct CallbackWrapper(Box<dyn Fn() + 'static>);

impl fmt::Debug for CallbackWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CallbackWrapper")
    }
}

#[derive(Debug)]
pub struct AnimationFrame {
    _closure: Rc<RefCell<Option<Closure<dyn FnMut(JsValue)>>>>,
    render_id: Rc<RefCell<Option<i32>>>,
}

impl Drop for AnimationFrame {
    fn drop(&mut self) {
        if let Some(render_id) = self.render_id.take() {
            window().cancel_animation_frame(render_id).unwrap_throw();
        }
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(JsValue)>) -> i32 {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap_throw()
}

pub fn set_up_render_loop(mut f: impl FnMut() + 'static) -> AnimationFrame {
    let mut last_time_ms = 0.;

    let closure = Rc::new(RefCell::new(None));
    let closure_internal = Rc::clone(&closure);

    let render_id = Rc::new(RefCell::new(None));
    let render_id_internal = Rc::clone(&render_id);

    *closure.borrow_mut() = Some(Closure::new(move |v: JsValue| {
        let time_ms: f64 = v.as_f64().unwrap_or(0.);
        if time_ms - last_time_ms >= SIXTY_FPS_FRAME_MS {
            f();
            last_time_ms = time_ms;
        }

        *render_id_internal.borrow_mut() = Some(request_animation_frame(
            &closure_internal.borrow().as_ref().unwrap(),
        ));
    }));

    *render_id.borrow_mut() = Some(request_animation_frame(&closure.borrow().as_ref().unwrap()));

    AnimationFrame {
        _closure: closure,
        render_id,
    }
}
