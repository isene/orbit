//! Accuracy probe: compares orbit's RA/Dec output against Skyfield-
//! generated truth in the **equator-of-date** frame (the frame orbit
//! produces — Schlyter's element terms include the secular precession
//! of the orbital plane). Run with `cargo run --example verify --release`.

use orbit::*;

// Truth from Skyfield with `.radec(epoch='date')` — see
// /tmp/orbit-truth/gen_truth_date.py. JPL DE421 + IAU 2006/2000A
// precession-nutation; accuracy at the milliarcsec level.
const TRUTH: &[(i32, u32, u32, &str, f64, f64)] = &[
    (2025, 1, 1, "sun",     281.7601, -22.9982),
    (2025, 1, 1, "moon",    296.6803, -25.8605),
    (2025, 1, 1, "mercury", 259.0716, -21.9414),
    (2025, 1, 1, "venus",   330.3948, -13.5864),
    (2025, 1, 1, "mars",    125.1258,  23.5452),
    (2025, 1, 1, "jupiter",  71.8822,  21.7874),
    (2025, 1, 1, "saturn",  346.5164,  -7.9168),
    (2025, 1, 1, "uranus",   51.3188,  18.4365),
    (2025, 1, 1, "neptune", 358.0317,  -2.2540),

    (2025, 6, 21, "sun",     89.8828,  23.4383),
    (2025, 6, 21, "moon",    25.7305,  13.8820),
    (2025, 6, 21, "mars",   154.4616,  11.8223),
    (2025, 6, 21, "jupiter", 92.7575,  23.2688),
    (2025, 6, 21, "saturn",   2.2696,  -1.4043),

    (2026, 1, 1, "sun",     281.4947, -23.0172),
    (2026, 1, 1, "moon",     63.9203,  26.4037),
    (2026, 1, 1, "mercury", 268.5241, -24.0013),
    (2026, 1, 1, "venus",   280.0565, -23.6224),
    (2026, 1, 1, "mars",    283.8796, -23.7200),
    (2026, 1, 1, "jupiter", 113.1243,  21.9791),
    (2026, 1, 1, "saturn",  357.3810,  -3.5964),

    (2026, 6, 21, "sun",     89.6355,  23.4375),
    (2026, 6, 21, "moon",   169.0315,   3.1162),
    (2026, 6, 21, "mars",    52.1363,  18.4654),
    (2026, 6, 21, "jupiter",120.2369,  20.9711),
    (2026, 6, 21, "saturn",  13.4983,   3.2418),

    (2026, 12, 21, "sun",   269.0367, -23.4345),
    (2026, 12, 21, "moon",   42.2325,  21.5655),

    (2030, 1, 1, "sun",     281.5310, -23.0117),
    (2030, 1, 1, "mercury", 280.0892, -20.4593),
    (2030, 1, 1, "venus",   290.6761, -18.8869),
    (2030, 1, 1, "mars",    317.5219, -17.5346),
    (2030, 1, 1, "jupiter", 228.4149, -16.9267),
    (2030, 1, 1, "saturn",   46.6066,  15.0608),
];

fn angle_diff_deg(a: f64, b: f64) -> f64 {
    let mut d = (a - b).abs();
    if d > 180.0 { d = 360.0 - d; }
    d
}

fn main() {
    // Geocentric-ish vantage. lat=0 also exercises the topocentric
    // dec correction's old NaN path.
    let lat = 0.0; let lon = 0.0; let tz = 0.0;
    println!("date          body      ra_truth   ra_orbit    Δra'   dec_truth  dec_orbit  Δdec'");
    println!("────────────────────────────────────────────────────────────────────────────────────");

    let mut worst_ra: (f64, &str, i32, u32, u32) = (0.0, "", 0,0,0);
    let mut worst_dec: (f64, &str, i32, u32, u32) = (0.0, "", 0,0,0);

    for &(y, m, d, body, ra_truth, dec_truth) in TRUTH {
        let bodies = all_bodies(y, m, d, lat, lon, tz);
        let b = bodies.iter().find(|x| x.name == body).expect("body");
        let dra = angle_diff_deg(b.ra_deg, ra_truth) * 60.0;
        let ddec = (b.dec_deg - dec_truth).abs() * 60.0;
        if dra > worst_ra.0  { worst_ra  = (dra, body, y, m, d); }
        if ddec > worst_dec.0 { worst_dec = (ddec, body, y, m, d); }
        println!("{y:04}-{m:02}-{d:02}   {body:<8} {ra_truth:>9.4}  {:>9.4}  {dra:>6.2}   {dec_truth:>+9.4}  {:>+9.4}  {ddec:>6.2}",
                 b.ra_deg, b.dec_deg);
    }
    println!();
    println!("Worst RA error : {:.2}'  ({} on {}-{:02}-{:02})", worst_ra.0, worst_ra.1, worst_ra.2, worst_ra.3, worst_ra.4);
    println!("Worst Dec error: {:.2}'  ({} on {}-{:02}-{:02})", worst_dec.0, worst_dec.1, worst_dec.2, worst_dec.3, worst_dec.4);
}
