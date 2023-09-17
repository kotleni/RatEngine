use nalgebra::{Matrix4, Rotation3, Vector3};
use nalgebra_glm as glm;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::RelativeMouseState;

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Rotation3<f32>,

    pub move_speed: f32,
    pub mouse_sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,

    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        let position = Vector3::new(0.0, 0.0, 5.0);
        let view_matrix = glm::look_at(
            &position,
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        let projection_matrix = glm::perspective(
            800.0 / 600.0,
            70.0,
            0.1,
            1000.0,
        );

        Camera {
            position,
            rotation: Rotation3::from_euler_angles(0.0, 0.0, 0.0),

            move_speed: 0.1,
            mouse_sensitivity: 0.1,
            yaw: 0.0,
            pitch: 0.0,

            view_matrix,
            projection_matrix,
        }
    }

    pub fn look_at(&mut self, target: &Vector3<f32>, up: &Vector3<f32>) {
        self.view_matrix = glm::look_at(&self.position, target, up);
    }

    pub fn process_input(&mut self, ks: &KeyboardState, ms: &RelativeMouseState) {
        // look around
        let relx = ms.x() as f32 * self.mouse_sensitivity;
        let rely = ms.y() as f32 * self.mouse_sensitivity;

        self.yaw += relx;
        self.pitch -= rely;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        let front = Vector3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        ).normalize();

        self.rotation = Rotation3::face_towards(&front, &Vector3::new(0.0, 1.0, 0.0));

        // move
        let mut direction = Vector3::new(0.0, 0.0, 0.0);
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            direction += front;
        }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            direction -= front;
        }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            direction -= front.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            direction += front.cross(&Vector3::new(0.0, 1.0, 0.0)).normalize();
        }

        let to = direction * self.move_speed;
        self.position += to;
        self.view_matrix = glm::look_at(&self.position, &(self.position + front), &Vector3::new(0.0, 1.0, 0.0));
    }
}