use cgmath::{
    Angle,
    Deg,
    InnerSpace,
    Matrix4,
    Point3,
    SquareMatrix,
    Vector3,
    Vector4,
};
use collision::{
    Ray3,
};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    /// The distance from the eye to the near clipping plane.
    pub near: f32,

    /// The distance from the eye to the far clipping plane.
    pub far: f32,

    /// The field-of-view of the camera--specifically, the angle that the top and bottom of the
    /// image plane form with the eye.
    pub fov: Deg<f32>,

    /// The resolution of the image produced by this camera, which also determines the
    /// aspect ratio.
    pub image_resolution: (usize, usize),

    /// A matrix determining a transformation from view space to world space. The origin
    /// will be mapped to the position of the eye in world space. This transformation should consist
    /// only of translation and rotation components.
    pub eye: Matrix4<f32>,
}

impl Camera {
    pub fn new(
        near: f32,
        far: f32,
        fov: Deg<f32>,
        image_resolution: (usize, usize),
        eye: Matrix4<f32>
    ) -> Self {
        Camera{ near, far, fov, image_resolution, eye }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(0.1, 1000.0, Deg(70.0), (640, 480), Matrix4::identity())
    }
}

impl Camera {
    /// Returns the point in world space corresponding to the center of the specified pixel.
    pub fn pixel_to_world(&self, x: usize, y: usize) -> Point3<f32> {
        let pixel_size = 1.0 / self.image_resolution.1 as f32;
        // Adding pixel_size / 2.0 gives us the center of the pixel.
        let (screen_x, screen_y) = (
            x as f32 / self.image_resolution.0 as f32 + pixel_size / 2.0,
            y as f32 / self.image_resolution.1 as f32 + pixel_size / 2.0,
        );
        let (ndc_x, ndc_y) = (screen_x * 2.0 - 1.0, 1.0 -  2.0 * screen_y);
        let (image_width, image_height) = self.image_size();
        let (view_x, view_y, view_z) = (
            ndc_x * (image_width / 2.0),
            ndc_y * (image_height / 2.0),
            -1.0,
        );
        let view_coordinates = Vector4{ x: view_x, y: view_y, z: view_z, w: 1.0 };
        let world_coordinates = self.eye * view_coordinates;
        Point3::from((world_coordinates.x, world_coordinates.y, world_coordinates.z))
    }

    /// Returns a ray that originates at the camera's eye and passes through the point in world
    /// space corresponding to the center of the specified pixel.
    pub fn pixel_ray(&self, x: usize, y: usize) -> Ray3<f32> {
        let eye = self.world_eye();
        let dir = self.pixel_to_world(x, y) - eye;
        let normalized_dir = dir / dir.magnitude();
        Ray3::new(eye, normalized_dir)
    }

    /// Returns the position of the camera's eye in world space.
    pub fn world_eye(&self) -> Point3<f32> {
        let eye_coordinates = self.eye * Vector4{ x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        Point3{ x: eye_coordinates.x, y: eye_coordinates.y, z: eye_coordinates.z }
    }

    /// Returns the aspect ratio of the camera's image.
    pub fn aspect(&self) -> f32 {
        self.image_resolution.0 as f32 / self.image_resolution.1 as f32
    }

    /// Returns the size of the image plane in view space.
    pub fn image_size(&self) -> (f32, f32) {
        let image_height = 2.0 * (self.fov / 2.0).tan();
        let image_width = image_height * self.aspect();
        (image_width, image_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CameraTest {
        pub camera: Camera,
    }

    impl CameraTest {
        pub fn new() -> Self {
            CameraTest{
                camera: Camera::new(
                    0.0,
                    100.0,
                    Deg(90.0),
                    (2, 2),
                    Matrix4::identity(),
                ),
            }
        }
    }

    #[test]
    fn test_eye() {
        let mut camera = CameraTest::new().camera;
        assert_eq!(camera.world_eye(), Point3{x: 0.0, y: 0.0, z: 0.0});
        camera.eye = Matrix4::from_translation(Vector3{x: -1.0, y: 1.0, z: 1.0});
        assert_eq!(camera.world_eye(), Point3{x: -1.0, y: 1.0, z: 1.0});
    }

    #[test]
    fn test_aspect() {
        let camera = CameraTest::new().camera;
        assert_eq!(camera.aspect(), 1.0);
    }

    #[test]
    fn test_image_size() {
        let camera = CameraTest::new().camera;
        assert_eq!(camera.image_size(), (2.0, 2.0));
    }

    #[test]
    fn test_pixel_to_world() {
        let mut camera = CameraTest::new().camera;
        assert_eq!(camera.pixel_to_world(0, 0), Point3{ x: -0.5, y: 0.5, z: -1.0 });
        assert_eq!(camera.pixel_to_world(1, 0), Point3{ x: 0.5, y: 0.5, z: -1.0 });
        assert_eq!(camera.pixel_to_world(0, 1), Point3{ x: -0.5, y: -0.5, z: -1.0 });
        assert_eq!(camera.pixel_to_world(1, 1), Point3{ x: 0.5, y: -0.5, z: -1.0 });
        camera.eye = Matrix4::from_translation(Vector3{x: 1.0, y: 1.0, z: 1.0});
        assert_eq!(camera.pixel_to_world(0, 0), Point3{ x: 0.5, y: 1.5, z: 0.0 });
        assert_eq!(camera.pixel_to_world(1, 0), Point3{ x: 1.5, y: 1.5, z: 0.0 });
        assert_eq!(camera.pixel_to_world(0, 1), Point3{ x: 0.5, y: 0.5, z: 0.0 });
        assert_eq!(camera.pixel_to_world(1, 1), Point3{ x: 1.5, y: 0.5, z: 0.0 });
    }

    #[test]
    fn test_pixel_ray() {
        let camera = CameraTest::new().camera;
        assert_eq!(camera.pixel_ray(0, 0), Ray3::new(
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Vector3{x: -0.5, y: 0.5, z: -1.0},
        ));
    }
}