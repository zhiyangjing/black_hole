use crate::camera::{compute_view_matrix, get_camera};
use glam::{Mat4, Vec3, Vec4};
use image::GenericImageView;
use image::io::Reader as ImageReader;
use std::cell::RefCell;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, console};

const IMAGE_BYTES: &[u8] = include_bytes!("../assets/stars_2k.jpg");
const NUM_LONGITUDES: usize = 24;
const NUM_LATITUDES: usize = 12;

thread_local! {
    static TEXTURE_DATA: RefCell<Option<(Vec<u8>, u32, u32)>> = RefCell::new(None);
    static VIEWPORT: RefCell<(u32, u32)> = RefCell::new((800, 600));
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

    TEXTURE_DATA.with(|tex| tex.borrow_mut().replace((data, width, height)));
    console::log_1(&"Texture loaded".into());
}

#[wasm_bindgen]
pub fn set_viewport_size(width: u32, height: u32) {
    VIEWPORT.with(|vp| vp.replace((width, height)));
}

#[wasm_bindgen]
pub fn render_skybox(context: &CanvasRenderingContext2d) {
    let (tex_data, tex_w, tex_h) =
        TEXTURE_DATA.with(|t| t.borrow().clone().expect("Texture missing"));
    let (width, height) = VIEWPORT.with(|vp| *vp.borrow());
    let cam = get_camera();
    let view = compute_view_matrix();
    let proj = Mat4::perspective_rh(cam.fov_y, width as f32 / height as f32, 0.1, 100.0);

    let inv_proj_view = (proj * view).inverse();

    let mut buffer = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let ndc_x = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0;
            let ndc_y = 1.0 - (y as f32 + 0.5) / height as f32 * 2.0;
            // let clip = Vec3::new(ndc_x, ndc_y, 1.0);
            // let dir = inv_proj_view.transform_vector3(clip).normalize();

            let clip4 = Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
            let unproj = inv_proj_view * clip4;
            let dir = (Vec3::new(unproj.x, unproj.y, unproj.z) / unproj.w).normalize();

            let lon = dir.x.atan2(dir.z);
            let lat = dir.y.asin();

            let u = lon * (0.5 / PI) + 0.5;
            let v = 0.5 - lat / PI;

            let u_tex = (u * tex_w as f32).clamp(0.0, tex_w as f32 - 1.0) as u32;
            let v_tex = (v * tex_h as f32).clamp(0.0, tex_h as f32 - 1.0) as u32;

            let tex_idx = ((v_tex * tex_w + u_tex) * 4) as usize;
            let buf_idx = ((y * width + x) * 4) as usize;
            buffer[buf_idx..buf_idx + 4].copy_from_slice(&tex_data[tex_idx..tex_idx + 4]);

            if cam.show_grid {
                let lon_deg = lon.to_degrees().abs();
                let lat_deg = lat.to_degrees().abs();
                let lon_step = 360.0 / NUM_LONGITUDES as f32;
                let lat_step = 180.0 / NUM_LATITUDES as f32;
                let eps = 0.5;

                let near_lon = (lon_deg % lon_step) < eps || (lon_deg % lon_step) > lon_step - eps;
                let near_lat = (lat_deg % lat_step) < eps || (lat_deg % lat_step) > lat_step - eps;

                if near_lon || near_lat {
                    buffer[buf_idx..buf_idx + 4].copy_from_slice(&[255, 255, 255, 255]);
                }
            }
        }
    }

    let img_data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut buffer),
        width,
        height,
    )
    .unwrap();

    context.put_image_data(&img_data, 0.0, 0.0).unwrap();
}
