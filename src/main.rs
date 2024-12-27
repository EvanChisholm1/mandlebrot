use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

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

fn mandlebrot(c: &Complex, n: u32) -> u32 {
    let mut z_n = Complex { re: c.re, im: c.im };

    for i in 0..n {
        if modulus(&z_n) > 2.0 {
            return i 
        }

        let z_square = mul(&z_n, &z_n);
        z_n = add(&z_square, c);
    }

    n
}

fn mandlebrot2(c: &Complex, n: u32) -> u32 {
    let mut  a = c.re;
    let mut b = c.im;

    let mut a_n = a;
    let mut b_n = b;

    let c_re = a;
    let c_im = b;

    for i in 0..n {
        if a * a + b * b > 4.0 {
            return i
        }

        a_n = a * a - b * b;
        b_n = 2.0 * a * b;
        a = a_n + c_re;
        b = b_n + c_im;
    }

    n
}

fn map_to_color(iter: u32, max_iters: u32) -> (u8, u8, u8) {
    if iter == max_iters {
        return (0, 0, 0);
    }

    let t = iter as f64 / max_iters as f64;
    let r = (t * 255.0) as u8;
    let g = ((1.0 - t) * 255.0) as u8;
    let b = 255 - r;

    (r, g, b)
}

fn main() {    
    let mut scope = 1.0;
    let mut center_x = -0.5;
    let mut center_y  = 0.0;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let max_max_iters = 2000;
    let min_max_iters = 150;

    let mut cur_max_iters = min_max_iters;

    let mut frames_since_move = 0;



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
        let change_val = scope * 0.05;
        if window.is_key_down(Key::J) {
            scope += change_val;
            frames_since_move = 0;
        }
        if window.is_key_down(Key::K) {
            scope -= change_val;
            frames_since_move = 0;
        }

        if window.is_key_down(Key::W) {
            center_y += change_val;
            frames_since_move = 0;
        }
        if window.is_key_down(Key::S) {
            center_y -= change_val;
            frames_since_move = 0;
        }

        if window.is_key_down(Key::D) {
            center_x += change_val;
            frames_since_move = 0;
        }
        if window.is_key_down(Key::A) {
            center_x -= change_val;
            frames_since_move = 0;
        }

        if frames_since_move == 0 {
            cur_max_iters = min_max_iters;
        } else if frames_since_move % 5 == 0 {
            cur_max_iters = (cur_max_iters + 100).clamp(min_max_iters, max_max_iters);
        }


        for y in 0..HEIGHT {
            for x in 0..WIDTH {
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
                // let iter = mandlebrot(&c, cur_max_iters);
                let iter = mandlebrot2(&c, cur_max_iters);
                let (r, g, b) = map_to_color(iter, cur_max_iters);
                
                let color = (r as u32) << 16 | (g as u32) << 8 | b as u32;

                buffer[y * WIDTH + x] = color;
            }
            frames_since_move += 1;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
