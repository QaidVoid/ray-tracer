use ray_tracer::tuple::{Point, Tuple, Vector};

#[test]
fn tuple_is_point() {
    let (x, y, z, w) = (4.3, -4.2, 3.1, 1.);
    let a = Tuple::new(x, y, z, w);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.);
    assert_eq!(a, Point::new(x, y, z));
    assert_ne!(a, Vector::new(x, y, z));
}

#[test]
fn tuple_is_vector() {
    let (x, y, z, w) = (4.3, -4.2, 3.1, 0.);
    let a = Tuple::new(x, y, z, w);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.);
    assert_eq!(a, Vector::new(x, y, z));
    assert_ne!(a, Point::new(x, y, z));
}

#[test]
fn tuple_point() {
    let p = Point::new(4.3, -4.2, 3.1);
    assert_eq!(p, Tuple::new(4.3, -4.2, 3.1, 1.));
}

#[test]
fn tuple_vector() {
    let p = Vector::new(4.3, -4.2, 3.1);
    assert_eq!(p, Tuple::new(4.3, -4.2, 3.1, 0.));
}

#[test]
fn tuple_add() {
    let a1 = Tuple::new(3., -2., 5., 1.);
    let a2 = Tuple::new(-2., 3., 1., 0.);
    assert_eq!(a1 + a2, Tuple::new(1., 1., 6., 1.));
}

#[test]
fn sub_points() {
    let p1 = Point::new(3., 2., 1.);
    let p2 = Point::new(5., 6., 7.);
    assert_eq!(p1 - p2, Vector::new(-2., -4., -6.));
}

#[test]
fn sub_vector_from_point() {
    let p = Point::new(3., 2., 1.);
    let v = Vector::new(5., 6., 7.);
    assert_eq!(p - v, Point::new(-2., -4., -6.));
}

#[test]
fn sub_vectors() {
    let v1 = Vector::new(3., 2., 1.);
    let v2 = Vector::new(5., 6., 7.);
    assert_eq!(v1 - v2, Vector::new(-2., -4., -6.));
}

#[test]
fn sub_vector_from_zero() {
    // it's essentially the same as negating the vector
    let zero = Vector::new(0., 0., 0.);
    let v = Vector::new(1., -2., 3.);
    assert_eq!(zero - v, Vector::new(-1., 2., -3.));
}

#[test]
fn tuple_neg() {
    let a = Tuple::new(1., -2., 3., -4.);
    assert_eq!(-a, Tuple::new(-1., 2., -3., 4.));
}

#[test]
fn tuple_mul() {
    let a = Tuple::new(1., -2., 3., -4.);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
    assert_eq!(a * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
}

#[test]
fn tuple_div() {
    let a = Tuple::new(1., -2., 3., -4.);
    assert_eq!(a / 2., Tuple::new(0.5, -1., 1.5, -2.));
}

#[test]
fn vec_magnitude() {
    let v = Vector::new(1., 0., 0.);
    assert_eq!(v.magnitude(), 1.);

    let v = Vector::new(0., 1., 0.);
    assert_eq!(v.magnitude(), 1.);

    let v = Vector::new(0., 0., 1.);
    assert_eq!(v.magnitude(), 1.);

    let v = Vector::new(1., 2., 3.);
    assert_eq!(v.magnitude(), 14_f32.sqrt());

    let v = Vector::new(-1., -2., -3.);
    assert_eq!(v.magnitude(), 14_f32.sqrt());
}

#[test]
fn vec_normalize() {
    let v = Vector::new(4., 0., 0.);
    assert_eq!(v.normalize(), Vector::new(1., 0., 0.));

    let v = Vector::new(1., 2., 3.);
    assert_eq!(v.normalize(), Vector::new(0.26726, 0.53452, 0.80178));
}

#[test]
fn vec_dot() {
    let a = Vector::new(1., 2., 3.);
    let b = Vector::new(2., 3., 4.);
    assert_eq!(a.dot(&b), 20.);
}

#[test]
fn vec_cross() {
    let a = Vector::new(1., 2., 3.);
    let b = Vector::new(2., 3., 4.);
    assert_eq!(a.cross(&b), Vector::new(-1., 2., -1.));
    assert_eq!(b.cross(&a), Vector::new(1., -2., 1.));
}
