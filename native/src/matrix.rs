use rapier2d_f64::na::SimdValue;
use rapier2d_f64::na::{Matrix2x3, Point2, Vector2};
use simba::simd::f64x4;

#[derive(Debug, Clone, Copy, Default)]
pub struct TransformMatrix {
    pub matrix: Matrix2x3<f64>,
}

impl TransformMatrix {
    pub fn new() -> Self {
        Self {
            matrix: Matrix2x3::identity(),
            ..Default::default()
        }
    }

    pub fn multiply_transforms(
        transform1: &Matrix2x3<f64>,
        transform2: &Matrix2x3<f64>,
    ) -> Matrix2x3<f64> {
        let m11 = transform1[(0, 0)] * transform2[(0, 0)] + transform1[(0, 1)] * transform2[(1, 0)];
        let m12 = transform1[(0, 0)] * transform2[(0, 1)] + transform1[(0, 1)] * transform2[(1, 1)];
        let m21 = transform1[(1, 0)] * transform2[(0, 0)] + transform1[(1, 1)] * transform2[(1, 0)];
        let m22 = transform1[(1, 0)] * transform2[(0, 1)] + transform1[(1, 1)] * transform2[(1, 1)];
        let m31 = transform1[(0, 0)] * transform2[(0, 2)]
            + transform1[(0, 1)] * transform2[(1, 2)]
            + transform1[(0, 2)];
        let m32 = transform1[(1, 0)] * transform2[(0, 2)]
            + transform1[(1, 1)] * transform2[(1, 2)]
            + transform1[(1, 2)];

        Matrix2x3::new(m11, m12, m31, m21, m22, m32)
    }

    pub fn multiply_by_matrix(&mut self, matrix: &Matrix2x3<f64>) {
        let transform1 = self.matrix;
        let transform2 = matrix;
        let m11 = transform1[(0, 0)] * transform2[(0, 0)] + transform1[(0, 1)] * transform2[(1, 0)];
        let m12 = transform1[(0, 0)] * transform2[(0, 1)] + transform1[(0, 1)] * transform2[(1, 1)];
        let m21 = transform1[(1, 0)] * transform2[(0, 0)] + transform1[(1, 1)] * transform2[(1, 0)];
        let m22 = transform1[(1, 0)] * transform2[(0, 1)] + transform1[(1, 1)] * transform2[(1, 1)];
        let m31 = transform1[(0, 0)] * transform2[(0, 2)]
            + transform1[(0, 1)] * transform2[(1, 2)]
            + transform1[(0, 2)];
        let m32 = transform1[(1, 0)] * transform2[(0, 2)]
            + transform1[(1, 1)] * transform2[(1, 2)]
            + transform1[(1, 2)];

        self.matrix[(0, 0)] = m11;
        self.matrix[(0, 1)] = m12;
        self.matrix[(0, 2)] = m31;
        self.matrix[(1, 0)] = m21;
        self.matrix[(1, 1)] = m22;
        self.matrix[(1, 2)] = m32;
    }

    pub fn multiply_by(&mut self, other: &TransformMatrix) {
        self.multiply_by_matrix(&other.matrix);
    }

    pub fn translate(&mut self, translation: Vector2<f64>) {
        let new_matrix = Matrix2x3::new(1.0, 0.0, translation.x, 0.0, 1.0, translation.y);
        self.multiply_by_matrix(&new_matrix);
    }

    pub fn rotate_deg(&mut self, angle_degrees: f64) {
        let angle_radians = angle_degrees.to_radians();
        let rotation_matrix = Matrix2x3::new(
            angle_radians.cos(),
            -angle_radians.sin(),
            0.0,
            angle_radians.sin(),
            angle_radians.cos(),
            0.0,
        );
        self.multiply_by_matrix(&rotation_matrix);
    }

    pub fn rotate(&mut self, angle_radians: f64) {
        let rotation_matrix = Matrix2x3::new(
            angle_radians.cos(),
            -angle_radians.sin(),
            0.0,
            angle_radians.sin(),
            angle_radians.cos(),
            0.0,
        );
        self.multiply_by_matrix(&rotation_matrix);
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        let scale_matrix = Matrix2x3::new(sx, 0.0, 0.0, 0.0, sy, 0.0);
        self.multiply_by_matrix(&scale_matrix);
    }

    pub fn build_transform(
        &mut self,
        translation: Vector2<f64>,
        angle: f64,
        scale: Vector2<f64>,
        origin: Vector2<f64>,
    ) {
        let c = angle.cos();
        let s = angle.sin();
        let tx = translation.x - origin.x * scale.x * c + origin.y * scale.y * s;
        let ty = translation.y - origin.x * scale.x * s - origin.y * scale.y * c;

        self.matrix[(0, 0)] = scale.x * c;
        self.matrix[(0, 1)] = -scale.y * s;
        self.matrix[(0, 2)] = tx;
        self.matrix[(1, 0)] = scale.x * s;
        self.matrix[(1, 1)] = scale.y * c;
        self.matrix[(1, 2)] = ty;
    }

    pub fn transform_point_mut(&self, point: &mut Point2<f64>) {
        let x = point.x;
        let y = point.y;
        point.x = self.matrix[(0, 0)] * x + self.matrix[(0, 1)] * y + self.matrix[(0, 2)];
        point.y = self.matrix[(1, 0)] * x + self.matrix[(1, 1)] * y + self.matrix[(1, 2)];
    }

    pub fn transform_point(&self, point: &Point2<f64>) -> Point2<f64> {
        let mut result = *point;
        self.transform_point_mut(&mut result);
        result
    }

    pub fn transform_vector_mut(&self, vector: &mut Vector2<f64>) {
        let x = vector.x;
        let y = vector.y;
        vector.x = self.matrix[(0, 0)] * x + self.matrix[(0, 1)] * y;
        vector.y = self.matrix[(1, 0)] * x + self.matrix[(1, 1)] * y;
    }

    pub fn transform_vector(&self, vector: &Vector2<f64>) -> Vector2<f64> {
        let mut result = *vector;
        self.transform_vector_mut(&mut result);
        result
    }

    pub fn try_inverse(&self) -> Option<TransformMatrix> {
        let det =
            self.matrix[(0, 0)] * self.matrix[(1, 1)] - self.matrix[(1, 0)] * self.matrix[(0, 1)];
        if det == 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;
        let m11 = self.matrix[(1, 1)] * inv_det;
        let m12 = -self.matrix[(0, 1)] * inv_det;
        let m21 = -self.matrix[(1, 0)] * inv_det;
        let m22 = self.matrix[(0, 0)] * inv_det;
        let origin_x = -self.matrix[(0, 2)];
        let origin_y = -self.matrix[(1, 2)];
        let m31 = m11 * origin_x + m12 * origin_y;
        let m32 = m21 * origin_x + m22 * origin_y;
        let inverse_matrix = Matrix2x3::new(m11, m12, m31, m21, m22, m32);
        Some(TransformMatrix {
            matrix: inverse_matrix,
            ..Default::default()
        })
    }
}

pub fn bulk_transform_vectors_mut(transform: &Matrix2x3<f64>, input_vectors: &mut [Vector2<f64>]) {
    let m11 = transform[(0, 0)];
    let m12 = transform[(0, 1)];
    let m21 = transform[(1, 0)];
    let m22 = transform[(1, 1)];
    let m31 = transform[(0, 2)];
    let m32 = transform[(1, 2)];

    let m11s = f64x4::splat(m11);
    let m12s = f64x4::splat(m12);
    let m21s = f64x4::splat(m21);
    let m22s = f64x4::splat(m22);
    let m31s = f64x4::splat(m31);
    let m32s = f64x4::splat(m32);

    for chunk in input_vectors.chunks_mut(4) {
        let xs: Vec<_> = chunk.iter().map(|v| v.x).collect();
        let ys: Vec<_> = chunk.iter().map(|v| v.y).collect();
        let xs = f64x4::from_slice_unaligned(&xs);
        let ys = f64x4::from_slice_unaligned(&ys);

        let xt = m11s * xs + m12s * ys + m31s;
        let yt = m21s * xs + m22s * ys + m32s;

        for (i, v) in chunk.iter_mut().enumerate() {
            v.x = xt.extract(i);
            v.y = yt.extract(i);
        }
    }
}

pub fn bulk_transform_points_mut(transform: &Matrix2x3<f64>, input_points: &mut [Point2<f64>]) {
    let m11 = transform[(0, 0)];
    let m12 = transform[(0, 1)];
    let m21 = transform[(1, 0)];
    let m22 = transform[(1, 1)];
    let m31 = transform[(0, 2)];
    let m32 = transform[(1, 2)];

    let m11s = f64x4::splat(m11);
    let m12s = f64x4::splat(m12);
    let m21s = f64x4::splat(m21);
    let m22s = f64x4::splat(m22);
    let m31s = f64x4::splat(m31);
    let m32s = f64x4::splat(m32);

    for chunk in input_points.chunks_mut(4) {
        let xs: Vec<_> = chunk.iter().map(|v| v.x).collect();
        let ys: Vec<_> = chunk.iter().map(|v| v.y).collect();
        let xs = f64x4::from_slice_unaligned(&xs);
        let ys = f64x4::from_slice_unaligned(&ys);

        let xt = m11s * xs + m12s * ys + m31s;
        let yt = m21s * xs + m22s * ys + m32s;

        for (i, v) in chunk.iter_mut().enumerate() {
            v.x = xt.extract(i);
            v.y = yt.extract(i);
        }
    }
}

pub fn bulk_transform_vectors_mut_n(
    transforms: &[Matrix2x3<f64>],
    input_vectors: &mut [Vector2<f64>],
    n: usize,
) {
    for (transform, chunk) in transforms.iter().zip(input_vectors.chunks_mut(n)) {
        let m11 = transform[(0, 0)];
        let m12 = transform[(0, 1)];
        let m21 = transform[(1, 0)];
        let m22 = transform[(1, 1)];
        let m31 = transform[(0, 2)];
        let m32 = transform[(1, 2)];

        let m11s = f64x4::splat(m11);
        let m12s = f64x4::splat(m12);
        let m21s = f64x4::splat(m21);
        let m22s = f64x4::splat(m22);
        let m31s = f64x4::splat(m31);
        let m32s = f64x4::splat(m32);

        for chunk in chunk.chunks_mut(4) {
            let xs: Vec<_> = chunk.iter().map(|v| v.x).collect();
            let ys: Vec<_> = chunk.iter().map(|v| v.y).collect();
            let xs = f64x4::from_slice_unaligned(&xs);
            let ys = f64x4::from_slice_unaligned(&ys);

            let xt = m11s * xs + m12s * ys + m31s;
            let yt = m21s * xs + m22s * ys + m32s;

            for (i, v) in chunk.iter_mut().enumerate() {
                v.x = xt.extract(i);
                v.y = yt.extract(i);
            }
        }
    }
}

pub fn bulk_transform_points_mut_n(
    transforms: &[Matrix2x3<f64>],
    input_points: &mut [Point2<f64>],
    n: usize,
) {
    for (transform, chunk) in transforms.iter().zip(input_points.chunks_mut(n)) {
        let m11 = transform[(0, 0)];
        let m12 = transform[(0, 1)];
        let m21 = transform[(1, 0)];
        let m22 = transform[(1, 1)];
        let m31 = transform[(0, 2)];
        let m32 = transform[(1, 2)];

        let m11s = f64x4::splat(m11);
        let m12s = f64x4::splat(m12);
        let m21s = f64x4::splat(m21);
        let m22s = f64x4::splat(m22);
        let m31s = f64x4::splat(m31);
        let m32s = f64x4::splat(m32);

        for chunk in chunk.chunks_mut(4) {
            let xs: Vec<_> = chunk.iter().map(|v| v.x).collect();
            let ys: Vec<_> = chunk.iter().map(|v| v.y).collect();
            let xs = f64x4::from_slice_unaligned(&xs);
            let ys = f64x4::from_slice_unaligned(&ys);

            let xt = m11s * xs + m12s * ys + m31s;
            let yt = m21s * xs + m22s * ys + m32s;

            for (i, v) in chunk.iter_mut().enumerate() {
                v.x = xt.extract(i);
                v.y = yt.extract(i);
            }
        }
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
        let transform = TransformMatrix::new();
        let point = Point2::new(1.0, 2.0);
        let transformed_point = transform.transform_point(&point);
        assert_points_equal(&transformed_point, &point);
    }

    #[test]
    fn test_translate() {
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
        let mut transform = TransformMatrix::new();
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
            let mut transform = TransformMatrix::new();
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
                                    let mut transform = TransformMatrix::new();
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

    #[test]
    fn test_build_transform_bulk_identity() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(0.0, 0.0));
        assert_points_equal(&points[1], &Point2::new(1.0, 0.0));
        assert_points_equal(&points[2], &Point2::new(1.0, 1.0));
        assert_points_equal(&points[3], &Point2::new(0.0, 1.0));
    }

    #[test]
    fn test_build_transform_bulk_translation() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(1.0, 2.0),
            0.0,
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(1.0, 2.0));
        assert_points_equal(&points[1], &Point2::new(2.0, 2.0));
        assert_points_equal(&points[2], &Point2::new(2.0, 3.0));
        assert_points_equal(&points[3], &Point2::new(1.0, 3.0));
    }

    #[test]
    fn test_build_transform_bulk_scale() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(2.0, 3.0),
            Vector2::new(0.0, 0.0),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(0.0, 0.0));
        assert_points_equal(&points[1], &Point2::new(2.0, 0.0));
        assert_points_equal(&points[2], &Point2::new(2.0, 3.0));
        assert_points_equal(&points[3], &Point2::new(0.0, 3.0));
    }

    #[test]
    fn test_build_transform_bulk_rotation() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            90.0f64.to_radians(),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(0.0, 0.0));
        assert_points_equal(&points[1], &Point2::new(0.0, 1.0));
        assert_points_equal(&points[2], &Point2::new(-1.0, 1.0));
        assert_points_equal(&points[3], &Point2::new(-1.0, 0.0));
    }

    #[test]
    fn test_build_transform_bulk_origin() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(1.0, 1.0),
            Vector2::new(1.0, 1.0),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(-1.0, -1.0));
        assert_points_equal(&points[1], &Point2::new(0.0, -1.0));
        assert_points_equal(&points[2], &Point2::new(0.0, 0.0));
        assert_points_equal(&points[3], &Point2::new(-1.0, 0.0));
    }

    #[test]
    fn test_build_transform_bulk_origin_scale() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            0.0,
            Vector2::new(2.0, 3.0),
            Vector2::new(1.0, 1.0),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(-2.0, -3.0));
        assert_points_equal(&points[1], &Point2::new(0.0, -3.0));
        assert_points_equal(&points[2], &Point2::new(0.0, 0.0));
        assert_points_equal(&points[3], &Point2::new(-2.0, 0.0));
    }

    #[test]
    fn test_build_transform_bulk_origin_rotation() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            90.0f64.to_radians(),
            Vector2::new(1.0, 1.0),
            Vector2::new(0.5, 0.5),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(0.5, -0.5));
        assert_points_equal(&points[1], &Point2::new(0.5, 0.5));
        assert_points_equal(&points[2], &Point2::new(-0.5, 0.5));
        assert_points_equal(&points[3], &Point2::new(-0.5, -0.5));
    }

    #[test]
    fn test_build_transform_bulk_origin_scale_and_rotation() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(0.0, 0.0),
            90.0f64.to_radians(),
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(1.5, -1.0));
        assert_points_equal(&points[1], &Point2::new(1.5, 1.0));
        assert_points_equal(&points[2], &Point2::new(-1.5, 1.0));
        assert_points_equal(&points[3], &Point2::new(-1.5, -1.0));
    }

    #[test]
    fn test_build_transform_bulk_origin_scale_and_rotation_and_translation() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(1.0, 1.0),
            90.0f64.to_radians(),
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        bulk_transform_points_mut(&transform.matrix, &mut points);

        assert_points_equal(&points[0], &Point2::new(2.5, 0.0));
        assert_points_equal(&points[1], &Point2::new(2.5, 2.0));
        assert_points_equal(&points[2], &Point2::new(-0.5, 2.0));
        assert_points_equal(&points[3], &Point2::new(-0.5, 0.0));
    }

    #[test]
    fn test_build_transform_bulk_n_origin_scale_and_rotation_and_translation() {
        let mut points = get_quad();
        let mut transform = TransformMatrix::new();
        transform.build_transform(
            Vector2::new(1.0, 1.0),
            90.0f64.to_radians(),
            Vector2::new(2.0, 3.0),
            Vector2::new(0.5, 0.5),
        );
        bulk_transform_points_mut_n(&[transform.matrix; 4], &mut points, 4);

        assert_points_equal(&points[0], &Point2::new(2.5, 0.0));
        assert_points_equal(&points[1], &Point2::new(2.5, 2.0));
        assert_points_equal(&points[2], &Point2::new(-0.5, 2.0));
        assert_points_equal(&points[3], &Point2::new(-0.5, 0.0));
    }
}
