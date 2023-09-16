use nalgebra::{Matrix4, Rotation3, Vector3};
use crate::utils::{get_position_from_matrix, get_rotation_from_matrix};

pub(crate) struct Camera {
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
    pub fn new() -> Self {
        Camera {
            position: Vector3::new(0.0, 0.0, 3.0),
            rotation: Rotation3::from_euler_angles(0.0, 0.0, 0.0),

            mouse_sensitivity: 0.1,
            move_speed: 3.0,
            yaw: 0.0,
            pitch: 0.0,

            view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::new_perspective(800.0 / 600.0, 70.0, 0.1, 1000.0),
        }
    }

    pub fn get_forward_vector(&self) -> Vector3<f32> {
        let direction = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        );

        direction.normalize()
    }

    pub fn get_right_vector(&self) -> Vector3<f32> {
        let right = self.rotation * Vector3::new(1.0, 0.0, 0.0);
        right.normalize()
    }

    pub fn get_up_vector(&self) -> Vector3<f32> {
        let up = self.rotation * Vector3::new(0.0, 1.0, 0.0);
        up.normalize()
    }

    pub fn process_input(&mut self, keys: &sdl2::keyboard::KeyboardState, mouse_state: &sdl2::mouse::RelativeMouseState) {
        // Keyboard input
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            self.position += self.get_forward_vector() * self.move_speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            self.position -= self.get_forward_vector() * self.move_speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            self.position -= self.get_right_vector() * self.move_speed;
        }
        if keys.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            self.position += self.get_right_vector() * self.move_speed;
        }

        // Mouse input
        let xrel = -mouse_state.x();
        let yrel = mouse_state.y();
        self.yaw += xrel as f32 * self.mouse_sensitivity;
        self.pitch += yrel as f32 * self.mouse_sensitivity;

        // Limit pitch to prevent camera flipping
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        // Update camera's rotation based on yaw and pitch
        // println!("yaw: {}, pitch: {}", self.yaw, self.pitch);
        self.update();
    }

    pub fn update(&mut self) {
        let direction = self.get_forward_vector();
        let up = self.get_up_vector();

        self.rotation = Rotation3::face_towards(&direction, &up);
        self.view_matrix = self.rotation.to_homogeneous() * Matrix4::new_translation(&-self.position);
    }

    pub fn look_at(&mut self, eye: &Vector3<f32>, center: &Vector3<f32>, up: &Vector3<f32>) {
        let f = (center - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        let new_matrix = Matrix4::new(
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -eye.dot(&s), -eye.dot(&u), eye.dot(&f), 1.0,
        );

        let new_rotation = get_rotation_from_matrix(&new_matrix);
        self.yaw = new_rotation.euler_angles().0.to_degrees();
        self.pitch = new_rotation.euler_angles().1.to_degrees();

        self.update();
    }
}