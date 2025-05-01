use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use crate::utils::deg_to_rad;

/// 摄像机结构体，存储俯仰(pitch)和偏航(yaw)
#[derive(Clone, Copy)]
pub struct Camera {
    pub(crate) pitch: f64,
    pub(crate) yaw: f64,
    radius: f64, // 新增：摄像机距离中心的距离
}


impl Camera {
    pub fn new() -> Self {
        Camera {
            pitch: 0.0,
            yaw: 0.0,
            radius: 1.0, // 默认距离
        }
    }

    pub fn rotate(&mut self, delta_pitch: f64, delta_yaw: f64) {
        self.pitch = (self.pitch + delta_pitch).clamp(-89.0, 89.0);
        self.yaw = (self.yaw + delta_yaw) % 360.0;
    }

    pub fn zoom(&mut self, delta_radius: f64) {
        self.radius = (self.radius + delta_radius).clamp(0.1, 10.0);
    }

    /// 获取摄像机世界坐标
    pub fn position(&self) -> [f64; 3] {
        let pitch_rad = deg_to_rad(self.pitch);
        let yaw_rad = deg_to_rad(self.yaw);
        [
            self.radius * pitch_rad.cos() * yaw_rad.cos(), // x
            self.radius * pitch_rad.sin(),                 // y
            self.radius * pitch_rad.cos() * yaw_rad.sin(),// z
        ]
    }

    /// 方向向量：从摄像机 -> 原点
    pub fn front_vector(&self) -> [f64; 3] {
        let pos = self.position();
        [-pos[0], -pos[1], -pos[2]]
    }
}


// 使用 thread_local + RefCell 存储摄像机实例
thread_local! {
    static CAMERA: RefCell<Camera> = RefCell::new(Camera::new());
}

/// 初始化摄像机（设置 panic hook）
pub fn init_camera() {
    console_error_panic_hook::set_once();
}

/// 内部：更新摄像机角度
pub fn update_camera_internal(delta_pitch: f64, delta_yaw: f64) {
    CAMERA.with(|c| {
        c.borrow_mut().rotate(delta_pitch, delta_yaw);
    });
}

/// 内部：读取摄像机，只读
pub fn with_camera<F, R>(f: F) -> R
where
    F: FnOnce(&Camera) -> R,
{
    CAMERA.with(|c| {
        let cam = *c.borrow();
        f(&cam)
    })
}

/// 内部：缩放摄像机距离
pub fn zoom_camera_internal(delta_radius: f64) {
    CAMERA.with(|c| {
        c.borrow_mut().zoom(delta_radius);
    });
}
