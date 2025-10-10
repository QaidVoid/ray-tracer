#[cfg(test)]
mod tests {
    use ray_tracer::tuple::Tuple;

    #[test]
    fn tuple_is_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_is_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.);
        assert!(a.is_vector());
        assert!(!a.is_point());
    }

    #[test]
    fn tuple_point() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(p, Tuple::new(4.3, -4.2, 3.1, 1.));
    }

    #[test]
    fn tuple_vector() {
        let p = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(p, Tuple::new(4.3, -4.2, 3.1, 0.));
    }

    #[test]
    fn tuple_add() {
        let a1 = Tuple::new(3., -2., 5., 1.);
        let a2 = Tuple::new(-2., 3., 1., 0.);
        assert_eq!(a1 + a2, Tuple::new(1., 1., 6., 1.));
    }

    #[test]
    fn tuple_sub_points() {
        let p1 = Tuple::point(3., 2., 1.);
        let p2 = Tuple::point(5., 6., 7.);
        assert_eq!(p1 - p2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn tuple_sub_vector_from_point() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);
        assert_eq!(p - v, Tuple::point(-2., -4., -6.));
    }

    #[test]
    fn tuple_sub_vector() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);
        assert_eq!(v1 - v2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn tuple_sub_vector_from_zero() {
        // it's essentially the same as negating the vector
        let zero = Tuple::vector(0., 0., 0.);
        let v = Tuple::vector(1., -2., 3.);
        assert_eq!(zero - v, Tuple::vector(-1., 2., -3.));
    }

    #[test]
    fn tuple_neg() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(-a, Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    fn tuple_mul() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a.clone() * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
        assert_eq!(a * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn tuple_div() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a / 2., Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn vec_magnitude() {
        let v = Tuple::vector(1., 0., 0.);
        assert_eq!(v.magnitude(), 1.);

        let v = Tuple::vector(0., 1., 0.);
        assert_eq!(v.magnitude(), 1.);

        let v = Tuple::vector(0., 0., 1.);
        assert_eq!(v.magnitude(), 1.);

        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.magnitude(), 14_f32.sqrt());

        let v = Tuple::vector(-1., -2., -3.);
        assert_eq!(v.magnitude(), 14_f32.sqrt());
    }

    #[test]
    fn vec_normalize() {
        let v = Tuple::vector(4., 0., 0.);
        assert_eq!(v.normalize(), Tuple::vector(1., 0., 0.));

        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn vec_dot() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.dot(&b), 20.);
    }

    #[test]
    fn vec_cross() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.cross(&b), Tuple::vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), Tuple::vector(1., -2., 1.));
    }
}
