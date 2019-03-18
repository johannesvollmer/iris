pub use num::clamp;

pub fn lerp<T: num::Float>(param: T, min: T, max: T) -> T {
    min * (T::one() - param) + max * param
}

pub fn solve_quadratic<T: num::Float + num::FromPrimitive>(a: T, b: T, c: T) -> Option<(T, T)> {
    let (a, b, c) = (a.to_f64().unwrap(), b.to_f64().unwrap(), c.to_f64().unwrap());

    let discrim = b * b - 4.0 * a * c;
    if discrim < 0.0 {
        return None;
    }

    let root_discrim = discrim.sqrt();

    let q = {
        if b < 0.0 {
            -0.5 * (b - root_discrim)
        } else {
            -0.5 * (b + root_discrim)
        }
    };

    let mut t0 = q / a;
    let mut t1 = c / q;
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    Some((T::from_f64(t0).unwrap(), T::from_f64(t1).unwrap()))
}
