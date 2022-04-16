use console_error_panic_hook;
use enum_map::{enum_map, Enum, EnumMap};
use image::{ImageBuffer, Pixel, Rgba};
use num::{cast::AsPrimitive, Integer};
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Enum, Copy, Clone, PartialEq)]
enum Part {
    C,
    V,
    R,
}

const R: Part = Part::R; // reste
const C: Part = Part::C; // corps
const V: Part = Part::V; // visage

struct Pattern {
    description: Vec<Vec<Part>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn rev(&self) -> Self {
        let mut result = vec![vec![R; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                result[y][x] = self.description[y][self.width - 1 - x];
            }
        }

        Self {
            description: result,
            width: self.width,
            height: self.height,
        }
    }

    fn mul(&self, num: usize) -> Pattern {
        let mut result = vec![vec![R; num * self.width]; num * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                for dy in 0..num {
                    for dx in 0..num {
                        result[num * y + dy][num * x + dx] = self.description[y][x];
                    }
                }
            }
        }

        Pattern {
            description: result,
            width: num * self.width,
            height: num * self.height,
        }
    }
}

fn se<T: Integer + AsPrimitive<i32>>(a: &[T], b: &[T]) -> u32 {
    let mut error: i32 = 0;

    for i in 0..3 {
        error += ((a[i].as_()) - (b[i].as_())).abs();
    }

    return (error / 3) as u32;
}

fn amogus_score(canvas: &ImageBuffer<Rgba<u8>, Vec<u8>>, pattern: &Pattern, x: u32, y: u32) -> f64 {
    let mut mean_color = [0, 0, 0, 0];

    let H = pattern.height;
    let W = pattern.width;

    for dy in 0..H {
        for dx in 0..W {
            let pixel = canvas.get_pixel(x + dx as u32, y + dy as u32).channels();
            for i in 0..3 {
                mean_color[i] += pixel[i] as u32;
            }
        }
    }

    let count = (H * W) as u32;
    for i in 0..3 {
        mean_color[i] /= count;
    }

    let mut sum_squared_error = 0;

    for dy in 0..H {
        for dx in 0..W {
            let mut error = 0;

            for i in 0..3 {
                error += mean_color[i] as i32
                    - (canvas.get_pixel(x + dx as u32, y + dy as u32)[i] as i32);
            }

            sum_squared_error += error * error;
        }
    }

    let sqrt_mse = (sum_squared_error as f64 / (3 * count) as f64).sqrt();

    return 255. - sqrt_mse;
}

fn amogus_write(
    source_canvas: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    target_canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    pattern: &Pattern,
    x: u32,
    y: u32,
    contrast: i32,
    noise: u32,
) {
    let H = pattern.height;
    let W = pattern.width;

    let mut mean_colors = enum_map! {
        Part::R => [0, 0, 0, 0],
        Part::C => [0, 0, 0, 0],
        Part::V => [0, 0, 0, 0],
    };

    let mut rng = rand::thread_rng();

    let p_rev = pattern.rev();
    let pattern = if rng.gen() { pattern } else { &p_rev };

    for (dy, row) in pattern.description.iter().enumerate() {
        for (dx, item) in row.iter().enumerate() {
            let pixel = source_canvas
                .get_pixel(x + dx as u32, y + dy as u32)
                .channels();
            for i in 0..3 {
                mean_colors[*item][i] += pixel[i] as u32;
            }
            mean_colors[*item][3] += 1;
        }
    }

    for part in [R, C, V] {
        for i in 0..3 {
            mean_colors[part][i] /= mean_colors[part][3];
        }
    }

    let off = contrast
        + if noise > 0 {
            (rng.gen::<u32>() % noise) as i32
        } else {
            0
        };

    if se(&mean_colors[R], &mean_colors[C]) < (off * off) as u32 {
        for i in 0..3 {
            mean_colors[C][i] = (mean_colors[R][i] as i32 - off).max(0).min(255) as u32
        }
    }
    if se(&mean_colors[V], &mean_colors[C]) < (off * off) as u32 {
        for i in 0..3 {
            mean_colors[V][i] = (mean_colors[R][i] as i32 + off).max(0).min(255) as u32
        }
    }

    for (dy, row) in pattern.description.iter().enumerate() {
        for (dx, item) in row.iter().enumerate() {
            if *item != R {
                let u = mean_colors[*item];
                let pixel = Rgba::from_channels(u[0] as u8, u[1] as u8, u[2] as u8, 255);
                target_canvas.put_pixel(x + dx as u32, y + dy as u32, pixel);
            }
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn do_it(
    width: u32,
    height: u32,
    source_canvas: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    target_canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    amogus: &Pattern,
    count: usize,
    contrast: i32,
    noise: u32,
) {
    let mut rng = rand::thread_rng();
    for x in 0..(width - amogus.width as u32) {
        for y in 0..(height - amogus.height as u32) {
            let v = (rng.gen::<u32>() % (width * height)) as f64;
            if v < count as f64 {
                let score = amogus_score(&source_canvas, &amogus, x, y);
                if v < (score / 255.) * count as f64 {
                    amogus_write(source_canvas, target_canvas, amogus, x, y, contrast, noise);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn apply(
    width: f64,
    height: f64,
    input: web_sys::CanvasRenderingContext2d,
    output: web_sys::CanvasRenderingContext2d,
    size: usize,
    count: usize,
    contrast: i32,
    noise: u32,
) {
    init_panic_hook();

    let data = input.get_image_data(0., 0., width, height).unwrap();

    let vector: Vec<u8> = (*data.data()).clone();

    let width = width as u32;
    let height = height as u32;

    let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, vector).unwrap();
    let mut new_buffer: ImageBuffer<Rgba<u8>, _> = buffer.clone();

    let base_pattern: Pattern = Pattern {
        description: vec![
            vec![R, R, R, R, R],
            vec![R, C, C, C, R],
            vec![C, C, V, V, R],
            vec![C, C, C, C, R],
            vec![R, C, C, C, R],
            vec![R, C, R, C, R],
            vec![R, R, R, R, R],
        ],
        width: 5,
        height: 7,
    };

    let amogus = base_pattern.mul(size);

    do_it(
        width,
        height,
        &buffer,
        &mut new_buffer,
        &amogus,
        count,
        contrast,
        noise,
    );

    web_sys::console::log_1(&format!("Done.").into());

    let new_image_data: wasm_bindgen::Clamped<&[u8]> = wasm_bindgen::Clamped(&new_buffer);
    let new_image =
        web_sys::ImageData::new_with_u8_clamped_array_and_sh(new_image_data, width, height)
            .unwrap();
    output.put_image_data(&new_image, 0., 0.).unwrap();
}
