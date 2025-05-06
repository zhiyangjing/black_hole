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

pub fn set_viewport_size(width: u32, height: u32) {
    VIEWPORT.with(|vp| vp.replace((width, height)));
}

/// 计算投影矩阵，保证视场角正确
pub fn compute_projection_matrix(width: u32, height: u32) -> Mat4 {
    let cam = get_camera();

    // 根据窗口的宽高比，调整 fov_y 或 fov_x，保持视角一致
    let aspect_ratio = width as f32 / height as f32;

    let proj = if aspect_ratio >= 1.0 {
        // 长宽比大于等于1，基于 fov_x 调整 fov_y
        Mat4::perspective_rh(cam.fov_x, aspect_ratio, 0.1, 100.0)
    } else {
        // 长宽比小于1，基于 fov_y 调整 fov_x
        let adjusted_fov_x = cam.fov_x * aspect_ratio;
        Mat4::perspective_rh(adjusted_fov_x, aspect_ratio, 0.1, 100.0)
    };

    proj
}

fn lensing_effect(dir: &mut Vec3, origin: Vec3, center: Vec3, strength: f32) {
    let dist = origin.distance(center); // 光线与黑洞的距离
    let direction_to_center = (center - origin).normalize(); // 黑洞到光线的方向
    let force = strength / dist.powi(2); // 引力透镜效应的强度，随着距离的平方递减

    // 计算一个简单的引力偏移（简化版）
    let lens_effect = direction_to_center * force;

    // 修改光线方向
    *dir += lens_effect;
}

fn intersects_black_hole_with_lensing(
    origin: Vec3,
    dir: &mut Vec3,
    center: Vec3,
    radius: f32,
    strength: f32,
) -> bool {
    // 引入引力透镜效应
    lensing_effect(dir, origin, center, strength);

    let oc = origin - center;
    let a = dir.dot(*dir);
    let b = 2.0 * oc.dot(*dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

pub fn render_skybox_with_lensing(context: &CanvasRenderingContext2d) {
    let (tex_data, tex_w, tex_h) =
        TEXTURE_DATA.with(|t| t.borrow().clone().expect("Texture missing"));
    let (width, height) = VIEWPORT.with(|vp| *vp.borrow());
    let cam = get_camera();
    let view = compute_view_matrix();
    let proj = compute_projection_matrix(width, height); // 动态调整 fov

    let inv_proj_view = (proj * view).inverse();

    let black_hole_center = Vec3::ZERO;
    let black_hole_radius = 2.0;
    let lensing_strength = 10.0; // 引力透镜效应的强度，根据需要调整

    let mut buffer = vec![0u8; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let ndc_x = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0;
            let ndc_y = 1.0 - (y as f32 + 0.5) / height as f32 * 2.0;
            let clip4 = Vec4::new(ndc_x, ndc_y, 1.0, 1.0);
            let unproj = inv_proj_view * clip4;
            let mut dir = (Vec3::new(unproj.x, unproj.y, unproj.z) / unproj.w).normalize();

            let buf_idx = ((y * width + x) * 4) as usize;

            // 在这里加入引力透镜效应
            // if intersects_black_hole_with_lensing(
            //     cam.position(),
            //     &mut dir,
            //     black_hole_center,
            //     black_hole_radius,
            //     lensing_strength,
            // ) {
            //     buffer[buf_idx..buf_idx + 4].copy_from_slice(&[0, 0, 0, 255]);
            // } else 
            {
                let lon = dir.x.atan2(dir.z);
                let lat = dir.y.asin();

                let u = lon * (0.5 / PI) + 0.5;
                let v = 0.5 - lat / PI;

                let u_tex = (u * tex_w as f32).clamp(0.0, tex_w as f32 - 1.0) as u32;
                let v_tex = (v * tex_h as f32).clamp(0.0, tex_h as f32 - 1.0) as u32;

                let tex_idx = ((v_tex * tex_w + u_tex) * 4) as usize;
                buffer[buf_idx..buf_idx + 4].copy_from_slice(&tex_data[tex_idx..tex_idx + 4]);

                if cam.show_grid {
                    let lon_deg = lon.to_degrees().abs();
                    let lat_deg = lat.to_degrees().abs();
                    let lon_step = 360.0 / NUM_LONGITUDES as f32;
                    let lat_step = 180.0 / NUM_LATITUDES as f32;
                    let eps = 0.5;

                    let near_lon =
                        (lon_deg % lon_step) < eps || (lon_deg % lon_step) > lon_step - eps;
                    let near_lat =
                        (lat_deg % lat_step) < eps || (lat_deg % lat_step) > lat_step - eps;

                    if near_lon || near_lat {
                        buffer[buf_idx..buf_idx + 4].copy_from_slice(&[255, 255, 255, 255]);
                    }
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
