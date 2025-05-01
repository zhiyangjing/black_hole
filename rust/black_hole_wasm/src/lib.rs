mod utils;
mod camera;
mod skybox;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, CanvasRenderingContext2d};
use console_error_panic_hook::set_once;

/// 初始化（设置 panic hook、初始化摄像机）
#[wasm_bindgen]
pub fn initialize() {
    set_once();
    camera::init_camera();
    skybox::load_sky_texture(); // 直接在初始化时加载纹理
}


/// 更新摄像机角度（根据鼠标移动）
#[wasm_bindgen]
pub fn update_camera(delta_pitch: f64, delta_yaw: f64) {
    camera::update_camera_internal(delta_pitch, delta_yaw);
}

/// 缩放摄像机（根据滚轮事件）
#[wasm_bindgen]
pub fn zoom_camera(delta_radius: f64) {
    camera::zoom_camera_internal(delta_radius);
}

/// 渲染一帧
#[wasm_bindgen]
pub fn render(context: &CanvasRenderingContext2d) {
    skybox::render_skybox(context);
}
