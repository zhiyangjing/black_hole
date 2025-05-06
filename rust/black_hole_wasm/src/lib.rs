mod utils;
mod camera;
mod skybox;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use console_error_panic_hook::set_once;

/// 初始化（设置 panic hook、初始化摄像机）
#[wasm_bindgen]
pub fn initialize() {
    set_once();
    camera::init_camera();
    skybox::load_sky_texture(); // 初始化加载纹理
}

/// 更新摄像机角度（鼠标移动）
#[wasm_bindgen]
pub fn update_camera(delta_pitch: f64, delta_yaw: f64) {
    camera::update_camera_internal(delta_pitch as f32, delta_yaw as f32);
}

/// 缩放摄像机（滚轮）
#[wasm_bindgen]
pub fn zoom_camera(delta_radius: f64) {
    camera::zoom_camera_internal(delta_radius as f32);
}

/// 更新摄像机 FOV
#[wasm_bindgen(js_name = updateCameraFov)]
pub fn update_camera_fov(fov_x: f64, fov_y: f64) {
    camera::update_camera_fov_internal(fov_x as f32, fov_y as f32);
}

/// 设置视口尺寸
#[wasm_bindgen]
pub fn set_viewport_size(width: u32, height: u32) {
    skybox::set_viewport_size(width, height);
}

/// 切换显示经纬线
#[wasm_bindgen]
pub fn toggle_grid() {
    camera::toggle_grid();
}

/// 渲染
#[wasm_bindgen]
pub fn render(context: &CanvasRenderingContext2d) {
    skybox::render_skybox_with_lensing(context);
}
