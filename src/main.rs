use rand::Rng;
use std::f32;

fn get_coefficient_count(order: u32) -> usize {
    ((order + 1) * (order + 1)) as usize
}

fn get_idx(l: u32, m: i32) -> usize {
    (l * (l + 1) + m as u32) as usize
}

fn img_y_to_theta(y: u32, h: u32) -> f32 {
    f32::consts::PI * (y as f32 + 0.5) / h as f32
}

fn img_x_to_phi(x: u32, w: u32) -> f32 {
    2.0 * f32::consts::PI * (x as f32 + 0.5) / w as f32
}

fn factorial(x: u32) -> f32 {
    const CACHE: [f32; 16] = [
        1.0,
        1.0,
        2.0,
        6.0,
        24.0,
        120.0,
        720.0,
        5040.0,
        40320.0,
        362880.0,
        3628800.0,
        39916800.0,
        479001600.0,
        6227020800.0,
        87178291200.0,
        1307674368000.0,
    ];

    let idx = x as usize;

    if idx < CACHE.len() {
        CACHE[idx]
    } else {
        let mut s = 1.0;
        for n in 2..=x {
            s *= n as f32;
        }
        s
    }
}

fn eval_legendre_polynomial(l: u32, m: i32, x: f32) -> f32 {
    let mut pmm = 1.0;
    if m > 0 {
        let sign = if m % 2 == 0 { 1.0 } else { -1.0 };
        pmm = sign * factorial((2 * m - 1) as u32) * (1.0 - x * x).powf((m / 2) as f32);
    }

    if l as i32 == m {
        return pmm;
    }

    let mut pmm1 = x * ((2 * m + 1) as f32) * pmm;
    if l as i32 == m + 1 {
        return pmm1;
    }

    for n in m + 2..=l as i32 {
        let pmn = (x * ((2 * n - 1) as f32) * pmm1 - ((n + m - 1) as f32) * pmm) / ((n - m) as f32);
        pmm = pmm1;
        pmm1 = pmn;
    }

    return pmm1;
}

fn eval_sh_slow(l: u32, m: i32, phi: f32, theta: f32) -> f32 {
    let klm = (((2.0 * l as f32 + 1.0) * factorial(l - m.abs() as u32))
        / (4.0 * f32::consts::PI * factorial(l + m.abs() as u32)))
    .sqrt();

    if m > 0 {
        2.0f32.sqrt() * klm * (m as f32 * phi).cos() * eval_legendre_polynomial(l, m, theta.cos())
    } else if m < 0 {
        2.0f32.sqrt() * klm * (-m as f32 * phi).sin() * eval_legendre_polynomial(l, -m, theta.cos())
    } else {
        klm * eval_legendre_polynomial(l, 0, theta.cos())
    }
}

fn eval_sh(l: u32, m: i32, phi: f32, theta: f32) -> f32 {
    eval_sh_slow(l, m, phi, theta)
}

pub fn project_env(order: u32, img: &Image) -> Vec<(f32, f32, f32)> {
    let mut coeffs = vec![(0.0f32, 0.0f32, 0.0f32); get_coefficient_count(order)];

    let pixel_area =
        (2.0 * f32::consts::PI / img.width() as f32) * (f32::consts::PI / img.height() as f32);

    for t in 0..img.height() {
        let theta = img_y_to_theta(t, img.height());
        let weight = pixel_area * theta.sin();

        for p in 0..img.width() {
            let phi = img_x_to_phi(p, img.width());
            let c = img.get_pixel(p, t);

            for l in 0..=order {
                for m in -(l as i32)..=l as i32 {
                    let idx = get_idx(l, m);
                    let sh = eval_sh(l, m, phi, theta);
                    let r = (c.0 * sh * weight, c.1 * sh * weight, c.2 * sh * weight);

                    let ref mut c = &mut coeffs[idx];
                    c.0 += r.0;
                    c.1 += r.1;
                    c.2 += r.2;
                }
            }
        }
    }

    coeffs
}

pub fn project_fn<R: Rng, F: Fn(f32, f32) -> f32>(
    order: u32,
    sample_count: u32,
    rng: &mut R,
    f: F,
) -> Vec<f32> {
    let mut coeffs = vec![0.0f32; get_coefficient_count(order)];

    let sample_side = (sample_count as f32).sqrt().floor() as i32;

    for t in 0..sample_side {
        for p in 0..sample_side {
            let alpha = (t as f32 + rng.gen::<f32>()) / sample_side as f32;
            let beta = (p as f32 + rng.gen::<f32>()) / sample_side as f32;

            let phi = 2.0 * f32::consts::PI * beta;
            let theta = (2.0 * alpha - 1.0).acos();

            let val = f(phi, theta);

            for l in 0..=order {
                for m in -(l as i32)..=l as i32 {
                    let idx = get_idx(l, m);
                    let sh = eval_sh(l, m, phi, theta);
                    coeffs[idx] += val * sh;
                }
            }
        }
    }

    let w = 4.0 * f32::consts::PI / (sample_side * sample_side) as f32;
    for c in &mut coeffs {
        *c *= w;
    }

    coeffs
}

pub fn to_spherical(dir: &(f32, f32, f32)) -> (f32, f32) {
    (dir.1.atan2(dir.0), dir.2.max(0.0).min(1.0).acos())
}

pub fn project_sparse_samples(
    order: u32,
    dirs: &Vec<(f32, f32, f32)>,
    values: &Vec<f32>,
) -> Vec<f32> {
    use nalgebra::*;

    let mut basis_values =
        DMatrix::<f32>::from_element(dirs.len(), get_coefficient_count(order), 0.0);
    let func_values = DVector::<f32>::from_iterator(values.len(), values.iter().cloned());

    for (idx, d) in dirs.iter().enumerate() {
        let (phi, theta) = to_spherical(d);
        for l in 0..=order {
            for m in -(l as i32)..=l as i32 {
                basis_values[(idx, get_idx(l, m))] = eval_sh(l, m, phi, theta);
            }
        }
    }

    let coeffs = basis_values
        .svd(false, false)
        .solve(&func_values, f32::EPSILON)
        .unwrap();

    coeffs.iter().map(|x| *x).collect::<Vec<_>>()
}

pub struct Image {
    w: u32,
    h: u32,
    data: Vec<(f32, f32, f32)>,
}

impl Image {
    fn width(&self) -> u32 {
        self.w
    }

    fn height(&self) -> u32 {
        self.h
    }

    fn get_pixel(&self, x: u32, y: u32) -> (f32, f32, f32) {
        self.data[(x + y * self.w) as usize]
    }
}

fn main() {
    println!("Hello, world!");
}
