use glam::{Mat4, Vec3};
use std::cell::RefCell;
use std::f32::consts::PI;

thread_local! {
    pub static CAMERA_STATE: RefCell<Camera> = RefCell::new(Camera::default());
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
    pub show_grid: bool,
    pub fov_x: f32,
    pub fov_y: f32,
    
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            yaw: 0.0,
            pitch: 0.0,
            radius: 10.0,
            show_grid: false,
            fov_x: 100.0_f32.to_radians(),
            fov_y: 100.0_f32.to_radians(),
        }
    }
}


pub fn init_camera() {
    CAMERA_STATE.with(|c| *c.borrow_mut() = Camera::default());
}

pub fn update_camera_internal(delta_pitch: f32, delta_yaw: f32) {
    CAMERA_STATE.with(|c| {
        let mut cam = c.borrow_mut();
        cam.pitch += delta_pitch * 0.005;
        cam.yaw += delta_yaw * 0.005;
        cam.pitch = cam.pitch.clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
    });
}

pub fn zoom_camera_internal(delta_radius: f32) {
    CAMERA_STATE.with(|c| {
        let mut cam = c.borrow_mut();
        cam.radius = (cam.radius - delta_radius * 0.1).clamp(1.0, 100.0);
    });
}

pub fn update_camera_fov_internal(fov_x: f32, fov_y: f32) {
    CAMERA_STATE.with(|c| {
        let mut cam = c.borrow_mut();
        cam.fov_x = fov_x.to_radians();
        cam.fov_y = fov_y.to_radians();
    });
}

pub fn toggle_grid() {
    CAMERA_STATE.with(|c| {
        let mut cam = c.borrow_mut();
        cam.show_grid = !cam.show_grid;
    });
}

impl Camera {
    /// 计算相机的位置
    pub fn position(&self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();

        Vec3::new(
            -self.radius * sy * cp,
            self.radius * sp,
            -self.radius * cy * cp,
        )
    }
}


/// 计算当前相机的观察矩阵
pub fn compute_view_matrix() -> Mat4 {
    CAMERA_STATE.with(|c| {
        let cam = c.borrow();
        let (sy, cy) = cam.yaw.sin_cos();
        let (sp, cp) = cam.pitch.sin_cos();

        let position = Vec3::new(
            -cam.radius * sy * cp,
            cam.radius * sp,
            -cam.radius * cy * cp,
        );

        Mat4::look_at_rh(position, Vec3::ZERO, Vec3::Y)
    })
}

/// 获取相机属性（用于外部）
pub fn get_camera() -> Camera {
    CAMERA_STATE.with(|c| c.borrow().clone())
}

