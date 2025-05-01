use crate::camera;
use crate::utils::deg_to_rad;
use std::cell::RefCell;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use image::io::Reader as ImageReader;
use image::GenericImageView;

const IMAGE_BYTES: &[u8] = include_bytes!("../assets/stars_2k.jpg");

thread_local! {
    static TEXTURE_DATA: RefCell<Option<(Vec<u8>, u32, u32)>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn load_sky_texture() {
    let img = ImageReader::new(std::io::Cursor::new(IMAGE_BYTES))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    let (width, height) = img.dimensions();
    let data = img.into_raw();

    TEXTURE_DATA.with(|tex| {
        tex.borrow_mut().replace((data, width, height));
    });

    console::log_1(&"Texture loaded into memory".into());
}

#[wasm_bindgen]
pub fn render_skybox(context: &CanvasRenderingContext2d) {
    let (tex_data, tex_width, tex_height) = TEXTURE_DATA.with(|tex| {
        tex.borrow().clone().expect("Texture not loaded; call load_sky_texture() first")
    });

    let canvas = context.canvas().unwrap();
    let width = canvas.width() as usize;
    let height = canvas.height() as usize;

    let mut buffer = vec![0u8; width * height * 4];

    let aspect_ratio = tex_width as f64 / tex_height as f64;  // 图片宽高比

    let (pitch, yaw) = camera::with_camera(|cam| (cam.pitch, cam.yaw));
    let pitch_rad = deg_to_rad(pitch);
    let yaw_rad = deg_to_rad(yaw);

    let cos_pitch = pitch_rad.cos();
    let sin_pitch = pitch_rad.sin();
    let cos_yaw = yaw_rad.cos();
    let sin_yaw = yaw_rad.sin();

    for y in 0..height {
        let v = 1.0 - (y as f64 + 0.5) / (height as f64);
        let theta = (v - 0.5) * PI;

        for x in 0..width {
            let u = (x as f64 + 0.5) / (width as f64);
            let phi = (u - 0.5) * 2.0 * PI;

            let dir = [
                theta.cos() * phi.cos(),
                theta.sin(),
                theta.cos() * phi.sin(),
            ];

            let rotated_dir = [
                dir[0] * cos_yaw + dir[2] * sin_yaw,
                dir[0] * sin_pitch * sin_yaw + dir[1] * cos_pitch - dir[2] * sin_pitch * cos_yaw,
                -dir[0] * sin_yaw + dir[2] * cos_yaw,
            ];

            let rdx = rotated_dir[0];
            let rdy = rotated_dir[1];
            let rdz = rotated_dir[2];

            let longitude = rdz.atan2(rdx);
            let latitude = rdy.asin();

            // 关键更新：对 longitude 按宽高比缩放
            let longitude_scaled = longitude * (aspect_ratio / 2.0);

            let u_tex = (longitude_scaled / (2.0 * PI) + 0.5) * tex_width as f64;
            let v_tex = (0.5 - latitude / PI) * tex_height as f64;

            let u_tex_clamped = u_tex.clamp(0.0, tex_width as f64 - 1.0) as u32;
            let v_tex_clamped = v_tex.clamp(0.0, tex_height as f64 - 1.0) as u32;

            let tex_idx = ((v_tex_clamped * tex_width + u_tex_clamped) * 4) as usize;
            let idx = (y * width + x) * 4;

            buffer[idx..idx + 4].copy_from_slice(&tex_data[tex_idx..tex_idx + 4]);
        }
    }

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut buffer),
        width as u32,
        height as u32,
    ).unwrap();

    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
