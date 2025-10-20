use ray_tracer::{matrix::Matrix, tuple::Tuple};

#[test]
fn matrix() {
    let m = Matrix::from_iter([
        [1., 2., 3., 4.],
        [5.5, 6.5, 7.5, 8.5],
        [9., 10., 11., 12.],
        [13.5, 14.5, 15.5, 16.5],
    ]);
    assert_eq!(m[0][0], 1.);
    assert_eq!(m[0][3], 4.);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}

#[test]
fn matrix_2x2() {
    let m = Matrix::from_iter([[-3., 5.], [1., -2.]]);

    assert_eq!(m[0][0], -3.);
    assert_eq!(m[0][1], 5.);
    assert_eq!(m[1][0], 1.);
    assert_eq!(m[1][1], -2.);
}

#[test]
fn matrix_3x3() {
    let m = Matrix::from_iter([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);

    assert_eq!(m[0][0], -3.);
    assert_eq!(m[1][1], -2.);
    assert_eq!(m[2][2], 1.);
}

#[test]
fn matrix_equality() {
    let m1 = Matrix::from_iter([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    ]);
    let m2 = Matrix::from_iter([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 10., 11., 12.],
        [13., 14., 15., 16.],
    ]);

    assert_eq!(m1, m2);

    let m1 = Matrix::from_iter([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    ]);
    let m2 = Matrix::from_iter([
        [2., 3., 4., 5.],
        [6., 7., 8., 9.],
        [8., 7., 6., 5.],
        [4., 3., 2., 1.],
    ]);

    assert_ne!(m1, m2);
}

#[test]
fn matrix_mul() {
    let m1 = Matrix::from_iter([
        [1., 2., 3., 4.],
        [5., 6., 7., 8.],
        [9., 8., 7., 6.],
        [5., 4., 3., 2.],
    ]);
    let m2 = Matrix::from_iter([
        [-2., 1., 2., 3.],
        [3., 2., 1., -1.],
        [4., 3., 6., 5.],
        [1., 2., 7., 8.],
    ]);

    assert_eq!(
        m1 * m2,
        Matrix::from_iter([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.]
        ])
    );
}

#[test]
fn matrix_mul_tuple() {
    let m = Matrix::from_iter([
        [1., 2., 3., 4.],
        [2., 4., 4., 2.],
        [8., 6., 4., 1.],
        [0., 0., 0., 1.],
    ]);
    let t = Tuple::new(1., 2., 3., 1.);
    assert_eq!(m * t, Tuple::new(18., 24., 33., 1.));
}

#[test]
fn matrix_multiple_by_identity_matrix() {
    let m = Matrix::from_iter([
        [0., 1., 2., 4.],
        [1., 2., 4., 8.],
        [2., 4., 8., 16.],
        [4., 8., 16., 32.],
    ]);
    let identity = Matrix::identity(4, 4);
    assert_eq!(m.clone() * identity, m);
}

#[test]
fn matrix_transpose() {
    let m = Matrix::from_iter([
        [0., 9., 3., 0.],
        [9., 8., 0., 8.],
        [1., 8., 5., 3.],
        [0., 0., 5., 8.],
    ]);
    let t = Matrix::from_iter([
        [0., 9., 1., 0.],
        [9., 8., 8., 0.],
        [3., 0., 5., 5.],
        [0., 8., 3., 8.],
    ]);
    assert_eq!(m.transpose(), t);
}

#[test]
fn matrix_identity_transpose() {
    let m = Matrix::identity(4, 4);
    let t = Matrix::from_iter([
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ]);
    assert_eq!(m.transpose(), t);
}

#[test]
fn matrix_determinant() {
    let m = Matrix::from_iter([[1., 5.], [-3., 2.]]);
    assert_eq!(m.determinant(), 17.);
}

#[test]
fn submatrix() {
    let m = Matrix::from_iter([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);
    let sm = Matrix::from_iter([[-3., 2.], [0., 6.]]);
    assert_eq!(m.submatrix(0, 2), sm);

    let m = Matrix::from_iter([
        [-6., 1., 1., 6.],
        [-8., 5., 8., 6.],
        [-1., 0., 8., 2.],
        [-7., 1., -1., 1.],
    ]);
    let sm = Matrix::from_iter([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);
    assert_eq!(m.submatrix(2, 1), sm);
}

#[test]
fn minor() {
    let m = Matrix::from_iter([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
    let sm = m.submatrix(1, 0);

    assert_eq!(sm.determinant(), 25.);
    assert_eq!(m.minor(1, 0), 25.);
}

#[test]
fn cofactor() {
    let m = Matrix::from_iter([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
    assert_eq!(m.minor(0, 0), -12.);
    assert_eq!(m.cofactor(0, 0), -12.);
    assert_eq!(m.minor(1, 0), 25.);
    assert_eq!(m.cofactor(1, 0), -25.);
}

#[test]
fn determinant_large() {
    let m = Matrix::from_iter([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);

    assert_eq!(m.cofactor(0, 0), 56.);
    assert_eq!(m.cofactor(0, 1), 12.);
    assert_eq!(m.cofactor(0, 2), -46.);
    assert_eq!(m.determinant(), -196.);

    let m = Matrix::from_iter([
        [-2., -8., 3., 5.],
        [-3., 1., 7., 3.],
        [1., 2., -9., 6.],
        [-6., 7., 7., -9.],
    ]);

    assert_eq!(m.cofactor(0, 0), 690.);
    assert_eq!(m.cofactor(0, 1), 447.);
    assert_eq!(m.cofactor(0, 2), 210.);
    assert_eq!(m.cofactor(0, 3), 51.);
    assert_eq!(m.determinant(), -4071.);
}
