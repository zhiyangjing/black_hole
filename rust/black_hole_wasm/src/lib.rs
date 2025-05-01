use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_circle(width: u32, height: u32) -> Vec<u8> {
    let mut pixels = vec![255u8; (width * height * 4) as usize]; // 初始化全白（RGBA）

    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = (width.min(height) as f32) / 3.0;
    let radius_squared = radius * radius;

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            if dx * dx + dy * dy <= radius_squared {
                let idx = ((y * width + x) * 4) as usize;
                pixels[idx] = 0;     // R
                pixels[idx + 1] = 0; // G
                pixels[idx + 2] = 0; // B
                pixels[idx + 3] = 255; // A
            }
        }
    }

    pixels
}
