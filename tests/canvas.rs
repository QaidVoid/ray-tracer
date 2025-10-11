use ray_tracer::{canvas::Canvas, color::Color};

#[test]
fn canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    assert!(c.pixels.into_iter().all(|p| p == Color::default()));
}

#[test]
fn write_pixels_to_canvas() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1., 0., 0.);
    c.write_pixel(2, 3, red);
    assert_eq!(c.pixel_at(2, 3), red);
}

#[test]
fn canvas_to_ppm() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    assert_eq!(
        ppm.lines().take(3).collect::<Vec<_>>(),
        vec!["P3", "5 3", "255"]
    );
}

#[test]
fn construct_ppm_from_canvas() {
    let mut c = Canvas::new(5, 3);
    let c1 = Color::new(1.5, 0., 0.);
    let c2 = Color::new(0., 0.5, 0.);
    let c3 = Color::new(-0.5, 0., 1.);

    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);

    let ppm = c.to_ppm();

    assert_eq!(
        ppm.lines().skip(3).take(3).collect::<Vec<_>>(),
        vec![
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
        ]
    );
}

#[test]
fn split_long_lines_in_ppm() {
    let mut c = Canvas::new(10, 2);
    c.pixels
        .iter_mut()
        .for_each(|p| *p = Color::new(1., 0.8, 0.6));
    let ppm = c.to_ppm();
    assert_eq!(
        ppm.lines().skip(3).take(4).collect::<Vec<_>>(),
        vec![
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        ]
    );
}

#[test]
fn ppm_terminates_with_newline() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    assert_eq!(ppm.lines().last().unwrap(), "");
}
