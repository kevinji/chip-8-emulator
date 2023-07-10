use core::fmt;
use gloo_console::log;
use gloo_utils::{document, window};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const SIXTY_FPS_FRAME_MS: f64 = 1000. / 60.;
pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
const SCALE: u32 = 10;
const IMAGE_DATA_ENTRIES_PER_PIXEL: u32 = 4;

#[derive(Debug)]
enum ByteSlice {
    Whole,
    StartAt(u32),
    EndAt(u32),
}

impl ByteSlice {
    const fn start(&self) -> u32 {
        match *self {
            Self::Whole | Self::EndAt(_) => 0,
            Self::StartAt(start) => start,
        }
    }

    const fn end(&self) -> u32 {
        match *self {
            Self::Whole | Self::StartAt(_) => 8,
            Self::EndAt(end) => end,
        }
    }

    const fn len(&self) -> u32 {
        self.end() - self.start()
    }
}

#[derive(Clone, Debug)]
pub struct View {
    ctx: CanvasRenderingContext2d,
}

impl View {
    fn clear_canvas(ctx: &CanvasRenderingContext2d) {
        log!("Clearing canvas");
        ctx.clear_rect(0., 0., (WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64);
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
        Self { ctx }
    }

    fn draw_contiguous_sprite(
        &mut self,
        sprite: &[u8],
        x_slice: ByteSlice,
        n: u32,
        sx: u32,
        sy: u32,
    ) -> bool {
        let mut collision = false;
        let orig_image_data = self
            .ctx
            .get_image_data(
                (sx * SCALE) as f64,
                (sy * SCALE) as f64,
                (x_slice.len() * SCALE) as f64,
                (n * SCALE) as f64,
            )
            .unwrap_throw();
        let mut image_data = orig_image_data.data();

        for iy in 0..n {
            let byte = sprite[iy as usize];
            for ix in 0..x_slice.len() {
                let base_pos = (IMAGE_DATA_ENTRIES_PER_PIXEL
                    * SCALE
                    * (SCALE * iy * x_slice.len() + ix)) as usize;

                // Each pixel stores 4 values (RGBA), and R=0 is black
                let curr_is_filled = image_data[base_pos] == 0 && image_data[base_pos + 3] == 255;
                let mem_is_filled = (byte >> (7 - (x_slice.start() + ix))) & 1 == 1;

                let new_is_filled = curr_is_filled ^ mem_is_filled;

                if curr_is_filled && !new_is_filled {
                    collision = true;
                }

                for scale_dx in 0..SCALE {
                    for scale_dy in 0..SCALE {
                        let pos = base_pos
                            + (IMAGE_DATA_ENTRIES_PER_PIXEL
                                * (SCALE * scale_dy * x_slice.len() + scale_dx))
                                as usize;

                        if new_is_filled {
                            image_data[pos] = 0;
                            image_data[pos + 1] = 0;
                            image_data[pos + 2] = 0;
                            image_data[pos + 3] = 255;
                        } else {
                            image_data[pos] = 255;
                            image_data[pos + 1] = 255;
                            image_data[pos + 2] = 255;
                            image_data[pos + 3] = 255;
                        }
                    }
                }
            }
        }

        let new_image_data =
            ImageData::new_with_u8_clamped_array(Clamped(&image_data), x_slice.len() * SCALE)
                .unwrap_throw();

        self.ctx
            .put_image_data(&new_image_data, (sx * SCALE) as f64, (sy * SCALE) as f64)
            .unwrap_throw();

        collision
    }

    pub fn draw_sprite(&mut self, sprite: &[u8], n: u32, sx: u32, sy: u32) -> bool {
        let mut collision = false;

        if sx + 8 > WIDTH {
            if sy + n > HEIGHT {
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
            if sy + n > HEIGHT {
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
