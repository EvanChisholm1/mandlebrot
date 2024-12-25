use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

const COMP_WIDTH: f64 = 2.0;
const COMP_HEIGHT: f64 = 2.0;

struct Complex {
    re: f64,
    im: f64,
}

fn add(a: &Complex, b: &Complex) -> Complex {
    Complex {
        re: a.re + b.re,
        im: a.im + b.im,
    }
}

fn mul(a: &Complex, b: &Complex) -> Complex {
    Complex {
        re: a.re * b.re - a.im * b.im,
        im: a.re * b.im + a.im * b.re,
    }
}
fn window_to_complex_plane(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    real_min: f64,
    real_max: f64,
    imag_min: f64,
    imag_max: f64,
) -> Complex {
    let re = real_min + (x / width) * (real_max - real_min);
    let im = imag_max - (y / height) * (imag_max - imag_min); // Flipping y-axis
    Complex { re, im }
}

fn modulus(z: &Complex) -> f64 {
    (z.im * z.im + z.re * z.re).sqrt()
}

fn diverges(c: &Complex, n: u32) -> bool {
    let mut z_n = Complex { re: c.re, im: c.im };

    for _ in 0..n {
        if modulus(&z_n) > 2.0 {
            return true
        }

        let z_square = mul(&z_n, &z_n);
        z_n = add(&z_square, c);
    }

    modulus(&z_n) > 2.0
}

fn main() {
    let mut scope = 1.0;
    let mut center_x = -0.5;
    let mut center_y  = 0.0;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // let c = window_to_complex(x, y);
            let c = window_to_complex_plane(
                x as f64,
                y as f64,
                WIDTH as f64,
                HEIGHT as f64,
                center_x - scope,
                center_x + scope,
                center_y -scope,
                center_y + scope,
            );
            let point_diverges = diverges(&c, 100);
            if point_diverges {
                buffer[x + y * WIDTH] = 0xFF0000;
            } else {
                buffer[x + y * WIDTH] = 0;
            }
        }
    }

    // Create a window
    let mut window = Window::new(
        "Basic Window - ESC to Exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open Window: {}", e);
    });


    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::J) {
            scope += scope * 0.05;
        }
        if window.is_key_down(Key::K) {
            scope -= scope * 0.05;
        }

        if window.is_key_down(Key::W) {
            center_y += scope * 0.05;
        }
        if window.is_key_down(Key::S) {
            center_y -= scope * 0.05;
        }

        if window.is_key_down(Key::D) {
            center_x += scope * 0.05;
        }
        if window.is_key_down(Key::A) {
            center_x -= scope * 0.05;
        }


        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // let c = window_to_complex(x, y);
                // let c = window_to_complex_plane(
                //     x as f64,
                //     y as f64,
                //     WIDTH as f64,
                //     HEIGHT as f64,
                //     -0.5 -scope,
                //     -0.5 + scope,
                //     -scope,
                //     scope,
                // );
                let c = window_to_complex_plane(
                    x as f64,
                    y as f64,
                    WIDTH as f64,
                    HEIGHT as f64,
                    center_x - scope,
                    center_x + scope,
                    center_y -scope,
                    center_y + scope,
                );
                let point_diverges = diverges(&c, 100);
                if point_diverges {
                    buffer[x + y * WIDTH] = 0xFF0000;
                } else {
                    buffer[x + y * WIDTH] = 0;
                }
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
