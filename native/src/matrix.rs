use rapier2d_f64::na::{Matrix3, Point2, Vector2};

#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    matrix: Matrix3<f64>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            matrix: Matrix3::identity(),
        }
    }

    pub fn translate(&mut self, translation: Vector2<f64>) {
        let transform = Matrix3::new_translation(&translation);
        self.matrix *= transform;
    }

    pub fn rotate_deg(&mut self, angle_degrees: f64) {
        let angle_radians = angle_degrees.to_radians();
        let rotation_matrix = Matrix3::new_rotation(angle_radians);
        self.matrix *= rotation_matrix;
    }

    pub fn rotate(&mut self, angle_radians: f64) {
        let rotation_matrix = Matrix3::new_rotation(angle_radians);
        self.matrix *= rotation_matrix;
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        let scale_matrix = Matrix3::new_nonuniform_scaling(&Vector2::new(sx, sy));
        self.matrix *= scale_matrix;
    }

    pub fn build_transform(
        &mut self,
        translation: Vector2<f64>,
        angle: f64,
        scale: Vector2<f64>,
        origin: Vector2<f64>,
    ) {
        let rotation_matrix = Matrix3::new_rotation(angle);
        let scale_matrix = Matrix3::new_nonuniform_scaling(&scale);
        let origin_matrix = Matrix3::new_translation(&-origin);
        let translation_matrix = Matrix3::new_translation(&translation);
        self.matrix = translation_matrix * rotation_matrix * scale_matrix * origin_matrix;
    }

    pub fn transform_point(&self, point: &Point2<f64>) -> Point2<f64> {
        self.matrix.transform_point(point)
    }

    pub fn transform_vector(&self, vector: &Vector2<f64>) -> Vector2<f64> {
        self.matrix.transform_vector(vector)
    }

    pub fn multiply(&self, other: &Transform) -> Transform {
        Transform {
            matrix: self.matrix * other.matrix,
        }
    }

    pub fn multiply_by(&mut self, other: &Transform) {
        self.matrix *= other.matrix;
    }

    pub fn try_inverse(&self) -> Option<Transform> {
        self.matrix
            .try_inverse()
            .map(|inverse| Transform { matrix: inverse })
    }

    pub fn try_inverse_mut(&mut self) -> bool {
        self.matrix.try_inverse_mut()
    }
}

#[cfg(test)]
mod tests_matrix {
    use super::*;

    fn assert_points_equal(a: &Point2<f64>, b: &Point2<f64>) {
        assert!((a.x - b.x).abs() < 0.0001, "{:?} != {:?}", a, b);
        assert!((a.y - b.y).abs() < 0.0001, "{:?} != {:?}", a, b);
    }

    #[test]
    fn test_default_transform() {
        let transform = Transform::new();
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        assert_points_equal(&transformed_point, &point);
    }

    #[test]
    fn test_translate() {
        let mut transform = Transform::new();
        let translation = Vector2::new(2.0, 3.0);
        transform.translate(translation);
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(3.0, 5.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_rotate() {
        let mut transform = Transform::new();
        let angle_degrees = 90.0f64;
        transform.rotate_deg(angle_degrees);
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(-2.0, 1.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_scale() {
        let mut transform = Transform::new();
        let scale = Vector2::new(2.0, 3.0);
        transform.scale(scale.x, scale.y);
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(2.0, 6.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_build_transform_identity() {
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
        );
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(1.0, 2.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_build_transform_translate() {
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(2.0, 3.0),
            0.0,
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
        );
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(3.0, 5.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_build_transform_rotate() {
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            90.0f64.to_radians(),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
        );
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(-2.0, 1.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_build_transform_scale() {
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(2.0, 3.0),
            Vector2::new(0.0, 0.0),
        );
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(2.0, 6.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_build_transform_origin() {
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(1.0, 1.0),
            Vector2::new(1.0, 2.0),
        );
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(0.0, 0.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);
    }

    #[test]
    fn test_build_transform_origin_scale() {
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        let point = Point2::new(0.0, 0.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new(-0.5 * 2.0, -0.5 * 3.0);
        assert_points_equal(&transformed_point, &expected_point);

        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        let expected_point = Point2::new((1.0 - 0.5) * 2.0, (2.0 - 0.5) * 3.0);
        assert_points_equal(&transformed_point, &expected_point);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_point = inverse.transform_point(&transformed_point);
        assert_points_equal(&inverse_transformed_point, &point);

        let inverse_transformed_point = inverse.transform_point(&Point2::new(0.0, 0.0));
        assert_points_equal(&inverse_transformed_point, &Point2::new(0.5, 0.5));
    }

    fn get_quad() -> [Point2<f64>; 4] {
        [
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 1.0),
        ]
    }
    #[test]
    fn test_build_transform_origin_scale_quad() {
        let points = get_quad();
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        let transformed_points = points
            .iter()
            .map(|point| transform.transform_point(point))
            .collect::<Vec<_>>();
        let expected_points = [
            Point2::new(-1.0, -1.5),
            Point2::new(1.0, -1.5),
            Point2::new(1.0, 1.5),
            Point2::new(-1.0, 1.5),
        ];
        assert_points_equal(&transformed_points[0], &expected_points[0]);
        assert_points_equal(&transformed_points[1], &expected_points[1]);
        assert_points_equal(&transformed_points[2], &expected_points[2]);
        assert_points_equal(&transformed_points[3], &expected_points[3]);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_points = transformed_points
            .iter()
            .map(|point| inverse.transform_point(point))
            .collect::<Vec<_>>();
        assert_points_equal(&inverse_transformed_points[0], &points[0]);
        assert_points_equal(&inverse_transformed_points[1], &points[1]);
        assert_points_equal(&inverse_transformed_points[2], &points[2]);
        assert_points_equal(&inverse_transformed_points[3], &points[3]);
    }

    #[test]
    fn test_build_transform_origin_scale_rotate_quad() {
        let points = get_quad();
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            90.0f64.to_radians(),
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        let transformed_points = points
            .iter()
            .map(|point| transform.transform_point(point))
            .collect::<Vec<_>>();
        let expected_points = [
            Point2::new(1.5, -1.0),
            Point2::new(1.5, 1.0),
            Point2::new(-1.5, 1.0),
            Point2::new(-1.5, -1.0),
        ];
        assert_points_equal(&transformed_points[0], &expected_points[0]);
        assert_points_equal(&transformed_points[1], &expected_points[1]);
        assert_points_equal(&transformed_points[2], &expected_points[2]);
        assert_points_equal(&transformed_points[3], &expected_points[3]);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_points = transformed_points
            .iter()
            .map(|point| inverse.transform_point(point))
            .collect::<Vec<_>>();
        assert_points_equal(&inverse_transformed_points[0], &points[0]);
        assert_points_equal(&inverse_transformed_points[1], &points[1]);
        assert_points_equal(&inverse_transformed_points[2], &points[2]);
        assert_points_equal(&inverse_transformed_points[3], &points[3]);
    }

    #[test]
    fn test_build_transform_origin_scale_rotate_translate_quad() {
        let points = get_quad();
        let mut transform = Transform::new();
        transform.build_transform(
            Vector2::new(1.0, 2.0),
            90.0f64.to_radians(),
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        let transformed_points = points
            .iter()
            .map(|point| transform.transform_point(point))
            .collect::<Vec<_>>();
        let expected_points = [
            Point2::new(2.5, 1.0),
            Point2::new(2.5, 3.0),
            Point2::new(-0.5, 3.0),
            Point2::new(-0.5, 1.0),
        ];
        assert_points_equal(&transformed_points[0], &expected_points[0]);
        assert_points_equal(&transformed_points[1], &expected_points[1]);
        assert_points_equal(&transformed_points[2], &expected_points[2]);
        assert_points_equal(&transformed_points[3], &expected_points[3]);

        // Inverse
        let inverse = transform.try_inverse().unwrap();
        let inverse_transformed_points = transformed_points
            .iter()
            .map(|point| inverse.transform_point(point))
            .collect::<Vec<_>>();
        assert_points_equal(&inverse_transformed_points[0], &points[0]);
        assert_points_equal(&inverse_transformed_points[1], &points[1]);
        assert_points_equal(&inverse_transformed_points[2], &points[2]);
        assert_points_equal(&inverse_transformed_points[3], &points[3]);
    }

    // More generic tests
    #[test]
    fn test_build_transform_360_degrees_of_rotations_quad() {
        let points = get_quad();
        for i in 0..360 {
            let mut transform = Transform::new();
            transform.build_transform(
                Vector2::new(0.0, 0.0),
                i as f64 * 1.0f64.to_radians(),
                Vector2::new(1.0, 1.0),
                Vector2::new(0.5, 0.5),
            );
            let transformed_points = points
                .iter()
                .map(|point| transform.transform_point(point))
                .collect::<Vec<_>>();
            // Manually calculate the expected points
            let radians = i as f64 * 1.0f64.to_radians();
            let expected_points = [
                Point2::new(
                    (0.0 - 0.5) * 1.0 * radians.cos() - (0.0 - 0.5) * 1.0 * radians.sin(),
                    (0.0 - 0.5) * 1.0 * radians.sin() + (0.0 - 0.5) * 1.0 * radians.cos(),
                ),
                Point2::new(
                    (1.0 - 0.5) * 1.0 * radians.cos() - (0.0 - 0.5) * 1.0 * radians.sin(),
                    (1.0 - 0.5) * 1.0 * radians.sin() + (0.0 - 0.5) * 1.0 * radians.cos(),
                ),
                Point2::new(
                    (1.0 - 0.5) * 1.0 * radians.cos() - (1.0 - 0.5) * 1.0 * radians.sin(),
                    (1.0 - 0.5) * 1.0 * radians.sin() + (1.0 - 0.5) * 1.0 * radians.cos(),
                ),
                Point2::new(
                    (0.0 - 0.5) * 1.0 * radians.cos() - (1.0 - 0.5) * 1.0 * radians.sin(),
                    (0.0 - 0.5) * 1.0 * radians.sin() + (1.0 - 0.5) * 1.0 * radians.cos(),
                ),
            ];
            assert_points_equal(&transformed_points[0], &expected_points[0]);
            assert_points_equal(&transformed_points[1], &expected_points[1]);
            assert_points_equal(&transformed_points[2], &expected_points[2]);
            assert_points_equal(&transformed_points[3], &expected_points[3]);

            // Inverse
            let inverse = transform.try_inverse().unwrap();
            let inverse_transformed_points = transformed_points
                .iter()
                .map(|point| inverse.transform_point(point))
                .collect::<Vec<_>>();
            assert_points_equal(&inverse_transformed_points[0], &points[0]);
            assert_points_equal(&inverse_transformed_points[1], &points[1]);
            assert_points_equal(&inverse_transformed_points[2], &points[2]);
            assert_points_equal(&inverse_transformed_points[3], &points[3]);
        }
    }

    #[test]
    fn test_build_transform_360_degrees_of_rotations_0_to_1_origin_0_to_1_translation_0_to_1_scale_quad(
    ) {
        let points = get_quad();
        for angle in 0..360 {
            for translation_x in 0..2 {
                for translation_y in 0..2 {
                    for scale_x in 0..2 {
                        for scale_y in 0..2 {
                            for origin_x in 0..2 {
                                for origin_y in 0..2 {
                                    let mut transform = Transform::new();
                                    transform.build_transform(
                                        Vector2::new(translation_x as f64, translation_y as f64),
                                        angle as f64 * 1.0f64.to_radians(),
                                        Vector2::new(scale_x as f64, scale_y as f64),
                                        Vector2::new(origin_x as f64, origin_y as f64),
                                    );
                                    let transformed_points = points
                                        .iter()
                                        .map(|point| transform.transform_point(point))
                                        .collect::<Vec<_>>();
                                    // Manually calculate the expected points
                                    let radians = angle as f64 * 1.0f64.to_radians();
                                    let expected_points = [
                                        Point2::new(
                                            (0.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.cos()
                                                - (0.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.sin()
                                                + translation_x as f64,
                                            (0.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.sin()
                                                + (0.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.cos()
                                                + translation_y as f64,
                                        ),
                                        Point2::new(
                                            (1.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.cos()
                                                - (0.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.sin()
                                                + translation_x as f64,
                                            (1.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.sin()
                                                + (0.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.cos()
                                                + translation_y as f64,
                                        ),
                                        Point2::new(
                                            (1.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.cos()
                                                - (1.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.sin()
                                                + translation_x as f64,
                                            (1.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.sin()
                                                + (1.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.cos()
                                                + translation_y as f64,
                                        ),
                                        Point2::new(
                                            (0.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.cos()
                                                - (1.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.sin()
                                                + translation_x as f64,
                                            (0.0 - origin_x as f64)
                                                * scale_x as f64
                                                * radians.sin()
                                                + (1.0 - origin_y as f64)
                                                    * scale_y as f64
                                                    * radians.cos()
                                                + translation_y as f64,
                                        ),
                                    ];
                                    assert_points_equal(
                                        &transformed_points[0],
                                        &expected_points[0],
                                    );
                                    assert_points_equal(
                                        &transformed_points[1],
                                        &expected_points[1],
                                    );
                                    assert_points_equal(
                                        &transformed_points[2],
                                        &expected_points[2],
                                    );
                                    assert_points_equal(
                                        &transformed_points[3],
                                        &expected_points[3],
                                    );

                                    // Inverse
                                    let inverse = transform.try_inverse();
                                    if inverse.is_none() {
                                        continue;
                                    }
                                    let inverse = inverse.unwrap();
                                    let inverse_transformed_points = transformed_points
                                        .iter()
                                        .map(|point| inverse.transform_point(point))
                                        .collect::<Vec<_>>();
                                    assert_points_equal(&inverse_transformed_points[0], &points[0]);
                                    assert_points_equal(&inverse_transformed_points[1], &points[1]);
                                    assert_points_equal(&inverse_transformed_points[2], &points[2]);
                                    assert_points_equal(&inverse_transformed_points[3], &points[3]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
