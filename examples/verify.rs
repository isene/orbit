//! Accuracy probe: compares orbit's RA/Dec output against
//! Skyfield-generated truth in the **topocentric equator-of-date**
//! frame for an observer at lat=lon=elev=0 — i.e. the frame
//! `all_bodies(0.0, 0.0, 0.0)` produces. Run with `cargo run
//! --example verify --release`.

use orbit::*;

// Truth from Skyfield 1.54 + JPL DE421 at site (lat=0, lon=0, h=0),
// apparent + light-time + aberration, projected onto the true
// equator/equinox of date. Accuracy at the milli-arcsec level.
const TRUTH: &[(i32, u32, u32, &str, f64, f64)] = &[
    (2025, 1, 1, "sun",     281.7600, -22.9972),
    (2025, 1, 1, "moon",    296.3960, -25.4641),
    (2025, 1, 1, "mercury", 259.0724, -21.9407),
    (2025, 1, 1, "venus",   330.3922, -13.5859),
    (2025, 1, 1, "mars",    125.1275,  23.5466),
    (2025, 1, 1, "jupiter",  71.8820,  21.7876),
    (2025, 1, 1, "saturn",  346.5161,  -7.9167),
    (2025, 1, 1, "uranus",   51.3187,  18.4366),
    (2025, 1, 1, "neptune", 358.0316,  -2.2540),

    (2025, 6, 21, "sun",     89.8826,  23.4373),
    (2025, 6, 21, "moon",    26.6448,  13.7751),
    (2025, 6, 21, "mars",   154.4604,  11.8222),
    (2025, 6, 21, "jupiter", 92.7574,  23.2687),
    (2025, 6, 21, "saturn",   2.2698,  -1.4043),

    (2026, 1, 1, "sun",     281.4946, -23.0163),
    (2026, 1, 1, "moon",     63.2335,  26.7674),
    (2026, 1, 1, "mercury", 268.5244, -24.0006),
    (2026, 1, 1, "venus",   280.0564, -23.6218),
    (2026, 1, 1, "mars",    283.8795, -23.7196),
    (2026, 1, 1, "jupiter", 113.1246,  21.9793),
    (2026, 1, 1, "saturn",  357.3807,  -3.5964),

    (2026, 6, 21, "sun",     89.6354,  23.4366),
    (2026, 6, 21, "moon",   168.0941,   3.1067),
    (2026, 6, 21, "mars",    52.1370,  18.4651),
    (2026, 6, 21, "jupiter",120.2366,  20.9710),
    (2026, 6, 21, "saturn",  13.4985,   3.2418),

    (2026, 12, 21, "sun",   269.0367, -23.4335),
    (2026, 12, 21, "moon",   41.4350,  21.8143),

    (2030, 1, 1, "sun",     281.5309, -23.0108),
    (2030, 1, 1, "moon",    236.3607, -21.8877),
    (2030, 1, 1, "mercury", 280.0892, -20.4580),
    (2030, 1, 1, "venus",   290.6743, -18.8841),
    (2030, 1, 1, "mars",    317.5211, -17.5343),
    (2030, 1, 1, "jupiter", 228.4152, -16.9266),
    (2030, 1, 1, "saturn",   46.6064,  15.0609),
];

fn angle_diff_deg(a: f64, b: f64) -> f64 {
    let mut d = (a - b).abs();
    if d > 180.0 { d = 360.0 - d; }
    d
}

fn main() {
    let lat = 0.0; let lon = 0.0; let tz = 0.0;
    println!("date          body      ra_truth   ra_orbit    Δra'   dec_truth  dec_orbit  Δdec'");
    println!("────────────────────────────────────────────────────────────────────────────────────");

    let mut worst_ra: (f64, &str, i32, u32, u32) = (0.0, "", 0,0,0);
    let mut worst_dec: (f64, &str, i32, u32, u32) = (0.0, "", 0,0,0);

    for &(y, m, d, body, ra_t, dec_t) in TRUTH {
        let bodies = all_bodies(y, m, d, lat, lon, tz);
        let b = bodies.iter().find(|x| x.name == body).expect("body");
        let dra = angle_diff_deg(b.ra_deg, ra_t) * 60.0;
        let ddec = (b.dec_deg - dec_t).abs() * 60.0;
        if dra > worst_ra.0  { worst_ra  = (dra, body, y, m, d); }
        if ddec > worst_dec.0 { worst_dec = (ddec, body, y, m, d); }
        println!("{y:04}-{m:02}-{d:02}   {body:<8} {ra_t:>9.4}  {:>9.4}  {dra:>6.2}   {dec_t:>+9.4}  {:>+9.4}  {ddec:>6.2}",
                 b.ra_deg, b.dec_deg);
    }
    println!();
    println!("Worst RA error : {:.2}'  ({} on {}-{:02}-{:02})", worst_ra.0, worst_ra.1, worst_ra.2, worst_ra.3, worst_ra.4);
    println!("Worst Dec error: {:.2}'  ({} on {}-{:02}-{:02})", worst_dec.0, worst_dec.1, worst_dec.2, worst_dec.3, worst_dec.4);
}
