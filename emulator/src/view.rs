use core::{fmt, ops::Range};
use gloo_console::log;
use gloo_utils::{document, window};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const SIXTY_FPS_FRAME_MS: f64 = 1000. / 60.;
pub const WIDTH: u8 = 64;
pub const HEIGHT: u8 = 32;
const SCALE: f64 = 10.;

#[derive(Debug)]
enum ByteSlice {
    Whole,
    StartAt(u8),
    EndAt(u8),
}

impl ByteSlice {
    const fn start(&self) -> u8 {
        match *self {
            Self::Whole | Self::EndAt(_) => 0,
            Self::StartAt(start) => start,
        }
    }

    const fn end(&self) -> u8 {
        match *self {
            Self::Whole | Self::StartAt(_) => 8,
            Self::EndAt(end) => end,
        }
    }

    fn into_range(&self) -> Range<u8> {
        self.into()
    }
}

impl From<&ByteSlice> for Range<u8> {
    fn from(byte_slice: &ByteSlice) -> Self {
        byte_slice.start()..byte_slice.end()
    }
}

#[derive(Clone, Debug)]
pub struct View {
    ctx: CanvasRenderingContext2d,
    filled_pixels: [[bool; HEIGHT as usize]; WIDTH as usize],
}

impl View {
    pub fn init_canvas() {
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

        ctx.scale(SCALE, SCALE).unwrap_throw();
    }

    fn clear_canvas(ctx: &CanvasRenderingContext2d) {
        log!("Clearing canvas");

        // Save transformation matrix.
        ctx.save();

        // Use the identity matrix while clearing the canvas.
        ctx.set_transform(1., 0., 0., 1., 0., 0.).unwrap_throw();

        ctx.clear_rect(0., 0., (WIDTH as f64) * SCALE, (HEIGHT as f64) * SCALE);

        // Restore transformation matrix.
        ctx.restore();
    }

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

        Self::clear_canvas(&ctx);

        let filled_pixels = [[false; HEIGHT as usize]; WIDTH as usize];

        Self { ctx, filled_pixels }
    }

    pub fn is_pixel_filled(&self, x: u8, y: u8) -> bool {
        self.filled_pixels[x as usize][y as usize]
    }

    fn draw_pixel(&mut self, x: u8, y: u8, is_filled: bool) {
        self.filled_pixels[x as usize][y as usize] = is_filled;

        let fill_color = if is_filled {
            "rgb(0, 0, 0)"
        } else {
            "rgb(255, 255, 255)"
        };
        self.ctx.set_fill_style(&JsValue::from_str(fill_color));
        self.ctx.fill_rect(x.into(), y.into(), 1., 1.);
    }

    fn draw_contiguous_sprite(
        &mut self,
        sprite: &[u8],
        x_slice: ByteSlice,
        n: u8,
        sx: u8,
        sy: u8,
    ) -> bool {
        let mut collision = false;
        for iy in 0..n {
            let byte = sprite[iy as usize];
            for ix in x_slice.into_range() {
                let x = sx + ix;
                let y = sy + iy;

                let curr_is_filled = self.is_pixel_filled(x, y);
                let mem_is_filled = (byte >> (7 - ix)) & 1 == 1;

                let new_is_filled = curr_is_filled ^ mem_is_filled;

                if curr_is_filled && !new_is_filled {
                    collision = true;
                }

                self.draw_pixel(x, y, new_is_filled);
            }
        }

        collision
    }

    pub fn draw_sprite(&mut self, sprite: &[u8], n: u8, sx: u8, sy: u8) -> bool {
        let mut collision = false;

        if sx + 8 >= WIDTH {
            if sy + n >= HEIGHT {
                collision |= self.draw_contiguous_sprite(
                    sprite,
                    ByteSlice::EndAt(WIDTH - sx),
                    HEIGHT - sy,
                    sx,
                    sy,
                );
                collision |= self.draw_contiguous_sprite(
                    sprite,
                    ByteSlice::StartAt(sx + 8 - WIDTH),
                    HEIGHT - sy,
                    0,
                    sy,
                );
                collision |= self.draw_contiguous_sprite(
                    sprite,
                    ByteSlice::EndAt(WIDTH - sx),
                    sy + n - HEIGHT,
                    sx,
                    0,
                );
                collision |= self.draw_contiguous_sprite(
                    sprite,
                    ByteSlice::StartAt(sx + 8 - WIDTH),
                    sy + n - HEIGHT,
                    0,
                    0,
                );
            } else {
                collision |=
                    self.draw_contiguous_sprite(sprite, ByteSlice::EndAt(WIDTH - sx), n, sx, sy);
                collision |= self.draw_contiguous_sprite(
                    sprite,
                    ByteSlice::StartAt(sx + 8 - WIDTH),
                    n,
                    0,
                    sy,
                );
            }
        } else {
            if sy + n >= HEIGHT {
                collision |=
                    self.draw_contiguous_sprite(sprite, ByteSlice::Whole, HEIGHT - sy, sx, sy);
                collision |=
                    self.draw_contiguous_sprite(sprite, ByteSlice::Whole, sy + n - HEIGHT, sx, 0);
            } else {
                collision |= self.draw_contiguous_sprite(sprite, ByteSlice::Whole, n, sx, sy);
            }
        }

        collision
    }

    pub fn clear(&mut self) {
        self.filled_pixels = [[false; HEIGHT as usize]; WIDTH as usize];
        Self::clear_canvas(&self.ctx);
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
        self._closure.take();
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
    let render_id = Rc::new(RefCell::new(None));

    let closure_internal = Rc::clone(&closure);
    let render_id_internal = Rc::clone(&render_id);

    *closure.borrow_mut() = Some(Closure::new(move |v: JsValue| {
        let time_ms = v.as_f64().unwrap_or(0.);
        if time_ms - last_time_ms >= SIXTY_FPS_FRAME_MS {
            f();
            last_time_ms = time_ms;
        }

        if let Some(closure_internal) = closure_internal.borrow().as_ref() {
            *render_id_internal.borrow_mut() = Some(request_animation_frame(closure_internal));
        }
    }));

    *render_id.borrow_mut() = Some(request_animation_frame(&closure.borrow().as_ref().unwrap()));

    AnimationFrame {
        _closure: closure,
        render_id,
    }
}
