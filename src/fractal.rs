
// Z = Z^2 + C
pub fn iterations(opts: &Opts, mut zr: f64, mut zi: f64, cr: f64, ci: f64) -> f64 {

    let er = opts.escape_radius;

    let mut zr2 = zr * zr;
    let mut zi2 = zi * zi;
    let mut i = 0.0;

    while i < opts.max_iterations && zr2 + zi2 < er {
        zi = 2.0 * zr * zi + ci;
        zr = zr2 - zi2 + cr;
        zr2 = zr * zr;
        zi2 = zi * zi;
        i += 1.0;
    }

    // This smoothes things out at the cost of more work:
    let frac = i + 1.0 - ((zr.abs() + zi.abs()).ln() / 2f64.ln()).ln();

    if frac.is_nan() {
        i
    } else {
        frac
    }

}

pub struct Opts {
    pub max_iterations: f64,
    pub escape_radius: f64
}