//! orbit — moon phases, ephemeris, and sun/planet positions for the
//! Fe₂O₃ Rust terminal suite. Shared by `nova` (astronomy panel) and
//! `tock` (calendar). Planet calculations ported from
//! [ruby-ephemeris](https://github.com/isene/ephemeris).

use std::f64::consts::PI;

// ── Structs ──────────────────────────────────────────────────────────

pub struct MoonPhase {
    pub illumination: f64,
    pub phase: f64,
    pub phase_name: &'static str,
    pub symbol: &'static str,
    pub phase_index: usize,
}

pub struct NotablePhase {
    pub day: u32,
    pub phase_name: &'static str,
    pub symbol: &'static str,
}

pub struct VisiblePlanet {
    pub name: &'static str,
    pub symbol: &'static str,
    pub color: &'static str,
    pub rise: String,
    pub set: String,
}

// ── Constants ────────────────────────────────────────────────────────

const SYNODIC_MONTH: f64 = 29.530588853;
const NEW_MOON_EPOCH_JD: f64 = 2451550.1;

const PHASE_NAMES: [&str; 8] = [
    "New Moon", "Waxing Crescent", "First Quarter", "Waxing Gibbous",
    "Full Moon", "Waning Gibbous", "Last Quarter", "Waning Crescent",
];

const PHASE_SYMBOLS: [&str; 8] = [
    "\u{1F311}", "\u{1F312}", "\u{1F313}", "\u{1F314}",
    "\u{1F315}", "\u{1F316}", "\u{1F317}", "\u{1F318}",
];

pub const PLANET_SYMBOLS: &[(&str, &str)] = &[
    ("mercury", "\u{263F}"), ("venus", "\u{2640}"), ("mars", "\u{2642}"),
    ("jupiter", "\u{2643}"), ("saturn", "\u{2644}"),
];

pub const BODY_COLORS: &[(&str, &str)] = &[
    ("sun", "FFD700"), ("moon", "888888"), ("mercury", "8F6E54"),
    ("venus", "E6B07C"), ("mars", "BC2732"), ("jupiter", "C08040"), ("saturn", "E8D9A0"),
];

fn deg(d: f64) -> f64 { d * PI / 180.0 }

// ── Meeus chapter 47 moon tables (verified against pymeeus 0.5.12)──
//
// Each row in MOON_LR is (D, M, M', F, Σl_coef, Σr_coef) where the
// integers select a combination of the moon's fundamental angles
// (D = mean elongation, M = sun's mean anomaly, M' = moon's mean
// anomaly, F = moon's argument of latitude) and Σl is in 10⁻⁶ deg,
// Σr in 10⁻³ km. MOON_B is the same shape for ecliptic latitude.
//
// 60 terms each. Reproduces Meeus's worked example (1992-04-12 0h TT,
// AA 2nd ed page 342) — λ = 133.162655°, β = -3.229126°,
// Δ = 368409.685 km — to within a tenth of a milli-arcsecond. Verified
// further against Skyfield + JPL DE421 to ~1' across 2025-2030.
//
// The eccentricity correction E (= 1 - 0.002516 T - 0.0000074 T²) is
// applied at runtime to terms with |M| > 0.

#[rustfmt::skip]
const MOON_LR: &[(i8, i8, i8, i8, i32, i32)] = &[
    (  0,  0,  1,  0,   6288774,  -20905355),
    (  2,  0, -1,  0,   1274027,   -3699111),
    (  2,  0,  0,  0,    658314,   -2955968),
    (  0,  0,  2,  0,    213618,    -569925),
    (  0,  1,  0,  0,   -185116,      48888),
    (  0,  0,  0,  2,   -114332,      -3149),
    (  2,  0, -2,  0,     58793,     246158),
    (  2, -1, -1,  0,     57066,    -152138),
    (  2,  0,  1,  0,     53322,    -170733),
    (  2, -1,  0,  0,     45758,    -204586),
    (  0,  1, -1,  0,    -40923,    -129620),
    (  1,  0,  0,  0,    -34720,     108743),
    (  0,  1,  1,  0,    -30383,     104755),
    (  2,  0,  0, -2,     15327,      10321),
    (  0,  0,  1,  2,    -12528,          0),
    (  0,  0,  1, -2,     10980,      79661),
    (  4,  0, -1,  0,     10675,     -34782),
    (  0,  0,  3,  0,     10034,     -23210),
    (  4,  0, -2,  0,      8548,     -21636),
    (  2,  1, -1,  0,     -7888,      24208),
    (  2,  1,  0,  0,     -6766,      30824),
    (  1,  0, -1,  0,     -5163,      -8379),
    (  1,  1,  0,  0,      4987,     -16675),
    (  2, -1,  1,  0,      4036,     -12831),
    (  2,  0,  2,  0,      3994,     -10445),
    (  4,  0,  0,  0,      3861,     -11650),
    (  2,  0, -3,  0,      3665,      14403),
    (  0,  1, -2,  0,     -2689,      -7003),
    (  2,  0, -1,  2,     -2602,          0),
    (  2, -1, -2,  0,      2390,      10056),
    (  1,  0,  1,  0,     -2348,       6322),
    (  2, -2,  0,  0,      2236,      -9884),
    (  0,  1,  2,  0,     -2120,       5751),
    (  0,  2,  0,  0,     -2069,          0),
    (  2, -2, -1,  0,      2048,      -4950),
    (  2,  0,  1, -2,     -1773,       4130),
    (  2,  0,  0,  2,     -1595,          0),
    (  4, -1, -1,  0,      1215,      -3958),
    (  0,  0,  2,  2,     -1110,          0),
    (  3,  0, -1,  0,      -892,       3258),
    (  2,  1,  1,  0,      -810,       2616),
    (  4, -1, -2,  0,       759,      -1897),
    (  0,  2, -1,  0,      -713,      -2117),
    (  2,  2, -1,  0,      -700,       2354),
    (  2,  1, -2,  0,       691,          0),
    (  2, -1,  0, -2,       596,          0),
    (  4,  0,  1,  0,       549,      -1423),
    (  0,  0,  4,  0,       537,      -1117),
    (  4, -1,  0,  0,       520,      -1571),
    (  1,  0, -2,  0,      -487,      -1739),
    (  2,  1,  0, -2,      -399,          0),
    (  0,  0,  2, -2,      -381,      -4421),
    (  1,  1,  1,  0,       351,          0),
    (  3,  0, -2,  0,      -340,          0),
    (  4,  0, -3,  0,       330,          0),
    (  2, -1,  2,  0,       327,          0),
    (  0,  2,  1,  0,      -323,       1165),
    (  1,  1, -1,  0,       299,          0),
    (  2,  0,  3,  0,       294,          0),
    (  2,  0, -1, -2,         0,       8752),
];

#[rustfmt::skip]
const MOON_B: &[(i8, i8, i8, i8, i32)] = &[
    (  0,  0,  0,  1,   5128122),
    (  0,  0,  1,  1,    280602),
    (  0,  0,  1, -1,    277693),
    (  2,  0,  0, -1,    173237),
    (  2,  0, -1,  1,     55413),
    (  2,  0, -1, -1,     46271),
    (  2,  0,  0,  1,     32573),
    (  0,  0,  2,  1,     17198),
    (  2,  0,  1, -1,      9266),
    (  0,  0,  2, -1,      8822),
    (  2, -1,  0, -1,      8216),
    (  2,  0, -2, -1,      4324),
    (  2,  0,  1,  1,      4200),
    (  2,  1,  0, -1,     -3359),
    (  2, -1, -1,  1,      2463),
    (  2, -1,  0,  1,      2211),
    (  2, -1, -1, -1,      2065),
    (  0,  1, -1, -1,     -1870),
    (  4,  0, -1, -1,      1828),
    (  0,  1,  0,  1,     -1794),
    (  0,  0,  0,  3,     -1749),
    (  0,  1, -1,  1,     -1565),
    (  1,  0,  0,  1,     -1491),
    (  0,  1,  1,  1,     -1475),
    (  0,  1,  1, -1,     -1410),
    (  0,  1,  0, -1,     -1344),
    (  1,  0,  0, -1,     -1335),
    (  0,  0,  3,  1,      1107),
    (  4,  0,  0, -1,      1021),
    (  4,  0, -1,  1,       833),
    (  0,  0,  1, -3,       777),
    (  4,  0, -2,  1,       671),
    (  2,  0,  0, -3,       607),
    (  2,  0,  2, -1,       596),
    (  2, -1,  1, -1,       491),
    (  2,  0, -2,  1,      -451),
    (  0,  0,  3, -1,       439),
    (  2,  0,  2,  1,       422),
    (  2,  0, -3, -1,       421),
    (  2,  1, -1,  1,      -366),
    (  2,  1,  0,  1,      -351),
    (  4,  0,  0,  1,       331),
    (  2, -1,  1,  1,       315),
    (  2, -2,  0, -1,       302),
    (  0,  0,  1,  3,      -283),
    (  2,  1,  1, -1,      -229),
    (  1,  1,  0, -1,       223),
    (  1,  1,  0,  1,       223),
    (  0,  1, -2, -1,      -220),
    (  2,  1, -1, -1,      -220),
    (  1,  0,  1,  1,      -185),
    (  2, -1, -2, -1,       181),
    (  0,  1,  2,  1,      -177),
    (  4,  0, -2, -1,       176),
    (  4, -1, -1, -1,       166),
    (  1,  0,  1, -1,      -164),
    (  4,  0,  1, -1,       132),
    (  1,  0, -1, -1,      -119),
    (  4, -1,  0, -1,       115),
    (  2, -2,  0,  1,       107),
];

// ── Ephemeris engine (ported from ruby-ephemeris) ───────────────────

struct OrbitalElements {
    n: f64, i: f64, w: f64, a: f64, e: f64, m: f64,
}

struct Ephemeris {
    lat: f64, lon: f64, tz: f64,
    d: f64, ecl: f64, ls: f64,
    xs: f64, ys: f64, sidtime: f64,
    sun_ra: f64, sun_dec: f64,
    bodies: std::collections::HashMap<&'static str, OrbitalElements>,
    jupiter_m: f64, saturn_m: f64, _uranus_m: f64,
}

impl Ephemeris {
    fn new(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Self {
        let y = year; let m = month as i32; let dd = day as i32;
        let d = (367 * y - 7 * (y + (m + 9) / 12) / 4 + 275 * m / 9 + dd - 730530) as f64;
        let t = d / 36525.0;
        let ecl = 23.439279444 - 46.8150 / 3600.0 * t - 0.00059 / 3600.0 * t * t + 0.001813 / 3600.0 * t * t * t;

        let mut bodies = std::collections::HashMap::new();
        bodies.insert("sun", OrbitalElements {
            n: 0.0, i: 0.0, w: 282.9404 + 4.70935e-5 * d, a: 1.0,
            e: 0.016709 - 1.151e-9 * d, m: 356.0470 + 0.9856002585 * d,
        });
        bodies.insert("moon", OrbitalElements {
            n: 125.1228 - 0.0529538083 * d, i: 5.1454,
            w: 318.0634 + 0.1643573223 * d, a: 60.2666, e: 0.054900,
            m: 115.3654 + 13.0649929509 * d,
        });
        bodies.insert("mercury", OrbitalElements {
            n: 48.3313 + 3.24587e-5 * d, i: 7.0047 + 5.00e-8 * d,
            w: 29.1241 + 1.01444e-5 * d, a: 0.387098, e: 0.205635 + 5.59e-10 * d,
            m: 168.6562 + 4.0923344368 * d,
        });
        bodies.insert("venus", OrbitalElements {
            n: 76.6799 + 2.46590e-5 * d, i: 3.3946 + 2.75e-8 * d,
            w: 54.8910 + 1.38374e-5 * d, a: 0.723330, e: 0.006773 - 1.302e-9 * d,
            m: 48.0052 + 1.6021302244 * d,
        });
        bodies.insert("mars", OrbitalElements {
            n: 49.5574 + 2.11081e-5 * d, i: 1.8497 - 1.78e-8 * d,
            w: 286.5016 + 2.92961e-5 * d, a: 1.523688, e: 0.093405 + 2.516e-9 * d,
            m: 18.6021 + 0.5240207766 * d,
        });
        bodies.insert("jupiter", OrbitalElements {
            n: 100.4542 + 2.76854e-5 * d, i: 1.3030 - 1.557e-7 * d,
            w: 273.8777 + 1.64505e-5 * d, a: 5.20256, e: 0.048498 + 4.469e-9 * d,
            m: 19.8950 + 0.0830853001 * d,
        });
        bodies.insert("saturn", OrbitalElements {
            n: 113.6634 + 2.38980e-5 * d, i: 2.4886 - 1.081e-7 * d,
            w: 339.3939 + 2.97661e-5 * d, a: 9.55475, e: 0.055546 - 9.499e-9 * d,
            m: 316.9670 + 0.0334442282 * d,
        });
        bodies.insert("uranus", OrbitalElements {
            n: 74.0005 + 1.3978e-5 * d, i: 0.7733 + 1.9e-8 * d,
            w: 96.6612 + 3.0565e-5 * d, a: 19.18171 - 1.55e-8 * d,
            e: 0.047318 + 7.45e-9 * d, m: 142.5905 + 0.011725806 * d,
        });
        bodies.insert("neptune", OrbitalElements {
            n: 131.7806 + 3.0173e-5 * d, i: 1.7700 - 2.55e-7 * d,
            w: 272.8461 - 6.027e-6 * d, a: 30.05826 + 3.313e-8 * d,
            e: 0.008606 + 2.15e-9 * d, m: 260.2471 + 0.005995147 * d,
        });

        let jupiter_m = (19.8950 + 0.0830853001 * d) % 360.0;
        let saturn_m = (316.9670 + 0.0334442282 * d) % 360.0;
        let uranus_m = (142.5905 + 0.011725806 * d) % 360.0;

        // Compute sun position
        let sun = &bodies["sun"];
        let w_s = (sun.w + 360.0) % 360.0;
        let ms = sun.m % 360.0;
        let es = ms + (180.0 / PI) * sun.e * deg(ms).sin() * (1.0 + sun.e * deg(ms).cos());
        let x = deg(es).cos() - sun.e;
        let y = deg(es).sin() * (1.0 - sun.e * sun.e).sqrt();
        let v = y.atan2(x) * 180.0 / PI;
        let r = (x * x + y * y).sqrt();
        let tlon = (v + w_s) % 360.0;
        let xs = r * deg(tlon).cos();
        let ys = r * deg(tlon).sin();
        let xe = xs;
        let ye = ys * deg(ecl).cos();
        let ze = ys * deg(ecl).sin();
        let sun_ra = ((ye.atan2(xe) * 180.0 / PI) + 360.0) % 360.0;
        let sun_dec = ze.atan2((xe * xe + ye * ye).sqrt()) * 180.0 / PI;

        let ls = (w_s + ms) % 360.0;
        let gmst0 = (ls + 180.0) / 15.0 % 24.0;
        let sidtime = gmst0 + lon / 15.0;

        Ephemeris {
            lat, lon, tz, d, ecl, ls, xs, ys, sidtime,
            sun_ra, sun_dec, bodies, jupiter_m, saturn_m, _uranus_m: uranus_m,
        }
    }

    /// Moon position via Meeus chapter 47 (full 60-term Σl + Σr
    /// longitude table, 60-term Σb latitude table, plus the
    /// A1/A2/A3 long-period corrections). Returns
    /// (ra_deg, dec_deg, distance_earth_radii). Distance is in earth
    /// radii so the topocentric parallax math in `body_calc` stays
    /// unit-consistent with the Schlyter path.
    ///
    /// Apparent position in the mean equator/equinox of date. No
    /// nutation in longitude (Δψ ≈ 17" is below the algorithm's
    /// residual error against modern JPL DE441 / IAU 2006/2000A).
    fn moon_radec(&self) -> (f64, f64, f64) {
        // Julian centuries from J2000 (TT). Schlyter's `d` counts
        // days from 2000-01-01 00:00 UT, so JDE − 2451545.0 = d − 1.5.
        let t = (self.d - 1.5) / 36525.0;
        let t2 = t * t;
        let t3 = t2 * t;
        let t4 = t3 * t;

        // Mean elements (Meeus 47.1-47.6, degrees)
        let lp =  218.3164477 + 481267.88123421 * t - 0.0015786 * t2
                  + t3 / 538841.0 - t4 / 65194000.0;
        let d  =  297.8501921 + 445267.1114034  * t - 0.0018819 * t2
                  + t3 / 545868.0 - t4 / 113065000.0;
        let m  =  357.5291092 +  35999.0502909  * t - 0.0001536 * t2
                  + t3 / 24490000.0;
        let mp =  134.9633964 + 477198.8675055  * t + 0.0087414 * t2
                  + t3 / 69699.0  - t4 / 14712000.0;
        let f  =   93.2720950 + 483202.0175233  * t - 0.0036539 * t2
                  - t3 / 3526000.0 + t4 / 863310000.0;

        // Three additional long-period arguments (Meeus 47.7)
        let a1 = 119.75 + 131.849   * t;
        let a2 =  53.09 + 479264.290 * t;
        let a3 = 313.45 + 481266.484 * t;

        // Eccentricity correction for Earth's orbit, applied to
        // any periodic-term coefficient with |M| > 0.
        let e = 1.0 - 0.002516 * t - 0.0000074 * t2;
        let e2 = e * e;

        let dr  = deg(d);
        let mr  = deg(m);
        let mpr = deg(mp);
        let fr  = deg(f);

        let mut sigma_l = 0.0_f64;
        let mut sigma_r = 0.0_f64;
        for &(di, mi, mpi, fi, lc, rc) in MOON_LR.iter() {
            let arg = (di as f64) * dr
                    + (mi as f64) * mr
                    + (mpi as f64) * mpr
                    + (fi as f64) * fr;
            let scale = match mi.abs() { 0 => 1.0, 1 => e, _ => e2 };
            sigma_l += (lc as f64) * scale * arg.sin();
            sigma_r += (rc as f64) * scale * arg.cos();
        }

        let mut sigma_b = 0.0_f64;
        for &(di, mi, mpi, fi, bc) in MOON_B.iter() {
            let arg = (di as f64) * dr
                    + (mi as f64) * mr
                    + (mpi as f64) * mpr
                    + (fi as f64) * fr;
            let scale = match mi.abs() { 0 => 1.0, 1 => e, _ => e2 };
            sigma_b += (bc as f64) * scale * arg.sin();
        }

        // Additional A1/L'-F/A2 longitude terms and A3 + L' + cross
        // terms in latitude (Meeus 47.7).
        sigma_l += 3958.0 * deg(a1).sin();
        sigma_l += 1962.0 * deg(lp - f).sin();
        sigma_l +=  318.0 * deg(a2).sin();

        sigma_b += -2235.0 * deg(lp).sin();
        sigma_b +=   382.0 * deg(a3).sin();
        sigma_b +=   175.0 * deg(a1 - f).sin();
        sigma_b +=   175.0 * deg(a1 + f).sin();
        sigma_b +=   127.0 * deg(lp - mp).sin();
        sigma_b +=  -115.0 * deg(lp + mp).sin();

        // Geocentric ecliptic of date (degrees) and distance (km).
        let lambda = (lp + sigma_l / 1_000_000.0).rem_euclid(360.0);
        let beta = sigma_b / 1_000_000.0;
        let dist_km = 385000.56 + sigma_r / 1000.0;

        // Ecliptic → mean equator of date via the obliquity already
        // computed in Ephemeris::new.
        let lam = deg(lambda);
        let bet = deg(beta);
        let eps = deg(self.ecl);
        let y_eq = lam.sin() * eps.cos() - bet.tan() * eps.sin();
        let x_eq = lam.cos();
        let mut ra = y_eq.atan2(x_eq) * 180.0 / PI;
        if ra < 0.0 { ra += 360.0; }
        let sin_dec = bet.sin() * eps.cos() + bet.cos() * eps.sin() * lam.sin();
        let dec = sin_dec.clamp(-1.0, 1.0).asin() * 180.0 / PI;

        // Distance in earth radii: par = asin(1/r) in radians for the
        // moon-radius parallax in topocentric correction.
        let r_b = dist_km / 6378.14;
        (ra, dec, r_b)
    }

    fn body_calc(&self, name: &str) -> (f64, f64, f64, f64, f64) {
        // Moon: Meeus chapter 47 path. Schlyter's truncated series in
        // the planet branch below caps at ~1° worst-case for the moon;
        // Meeus 47 with the full 60-term tables gets us to ~1'.
        if name == "moon" {
            let (ra, dec_val, r_b) = self.moon_radec();
            // Topocentric correction (moon parallax ~1°, real fix).
            let par = (1.0 / r_b).asin() * 180.0 / PI;
            let gclat = self.lat - 0.1924 * deg(2.0 * self.lat).sin();
            let rho = 0.99833 + 0.00167 * deg(2.0 * self.lat).cos();
            let lst = self.sidtime * 15.0;
            let ha = (lst - ra + 360.0) % 360.0;
            let cos_dec = deg(dec_val).cos();
            let top_ra = if cos_dec.abs() < 1e-9 {
                ra
            } else {
                ra - par * rho * deg(gclat).cos() * deg(ha).sin() / cos_dec
            };
            let top_dec = dec_val - par * rho * (
                deg(gclat).sin() * cos_dec
                - deg(gclat).cos() * deg(ha).cos() * deg(dec_val).sin()
            );
            return (top_ra, top_dec, r_b, ra, dec_val);
        }

        let b = &self.bodies[name];
        let n_b = b.n; let i_b = b.i;
        let w_b = (b.w + 360.0) % 360.0;
        let a_b = b.a; let e_b = b.e;
        let m_b = b.m % 360.0;

        // Solve Kepler's equation iteratively
        let mut e1 = m_b + (180.0 / PI) * e_b * deg(m_b).sin() * (1.0 + e_b * deg(m_b).cos());
        let mut e0;
        loop {
            e0 = e1;
            e1 = e0 - (e0 - (180.0 / PI) * e_b * deg(e0).sin() - m_b) / (1.0 - e_b * deg(e0).cos());
            if (e1 - e0).abs() <= 0.0005 { break; }
        }
        let e = e1;
        let x = a_b * (deg(e).cos() - e_b);
        let y = a_b * (1.0 - e_b * e_b).sqrt() * deg(e).sin();
        let r = (x * x + y * y).sqrt();
        let v = ((y.atan2(x) * 180.0 / PI) + 360.0) % 360.0;

        let xeclip = r * (deg(n_b).cos() * deg(v + w_b).cos() - deg(n_b).sin() * deg(v + w_b).sin() * deg(i_b).cos());
        let yeclip = r * (deg(n_b).sin() * deg(v + w_b).cos() + deg(n_b).cos() * deg(v + w_b).sin() * deg(i_b).cos());
        let zeclip = r * deg(v + w_b).sin() * deg(i_b).sin();

        let mut lon = (yeclip.atan2(xeclip) * 180.0 / PI + 360.0) % 360.0;
        let mut lat = zeclip.atan2((xeclip * xeclip + yeclip * yeclip).sqrt()) * 180.0 / PI;
        let mut r_b = (xeclip * xeclip + yeclip * yeclip + zeclip * zeclip).sqrt();

        // Perturbation corrections
        let (mut plon, mut plat, mut pdist) = (0.0, 0.0, 0.0);
        let m_j = self.jupiter_m;
        let m_s = self.saturn_m;
        match name {
            "jupiter" => {
                plon += -0.332 * deg(2.0 * m_j - 5.0 * m_s - 67.6).sin();
                plon += -0.056 * deg(2.0 * m_j - 2.0 * m_s + 21.0).sin();
                plon += 0.042 * deg(3.0 * m_j - 5.0 * m_s + 21.0).sin();
                plon += -0.036 * deg(m_j - 2.0 * m_s).sin();
                plon += 0.022 * deg(m_j - m_s).cos();
                plon += 0.023 * deg(2.0 * m_j - 3.0 * m_s + 52.0).sin();
                plon += -0.016 * deg(m_j - 5.0 * m_s - 69.0).sin();
            }
            "saturn" => {
                plon += 0.812 * deg(2.0 * m_j - 5.0 * m_s - 67.6).sin();
                plon += -0.229 * deg(2.0 * m_j - 4.0 * m_s - 2.0).cos();
                plon += 0.119 * deg(m_j - 2.0 * m_s - 3.0).sin();
                plon += 0.046 * deg(2.0 * m_j - 6.0 * m_s - 69.0).sin();
                plon += 0.014 * deg(m_j - 3.0 * m_s + 32.0).sin();
                plat += -0.020 * deg(2.0 * m_j - 4.0 * m_s - 2.0).cos();
                plat += 0.018 * deg(2.0 * m_j - 6.0 * m_s - 49.0).sin();
            }
            _ => {}
        }
        lon += plon;
        lat += plat;
        r_b += pdist;

        // Geocentric ecliptic to equatorial. The moon takes the
        // dedicated Meeus 47 path in the early-return above, so by
        // here we're always converting a heliocentric body vector
        // to the geocentric equatorial frame.
        let (xeclip2, yeclip2, zeclip2) =
            (xeclip + self.xs, yeclip + self.ys, zeclip);

        let xequat = xeclip2;
        let yequat = yeclip2 * deg(self.ecl).cos() - zeclip2 * deg(self.ecl).sin();
        let zequat = yeclip2 * deg(self.ecl).sin() + zeclip2 * deg(self.ecl).cos();

        let ra = (yequat.atan2(xequat) * 180.0 / PI + 360.0) % 360.0;
        let dec_val = zequat.atan2((xequat * xequat + yequat * yequat).sqrt()) * 180.0 / PI;
        let dist = (xequat * xequat + yequat * yequat + zequat * zequat).sqrt();

        // Topocentric correction (parallax shift from geocentric to
        // observer-on-surface). Significant for the moon (~1°),
        // tiny for everything else. The earlier form used an
        // auxiliary angle `g = atan(tan(gclat) / cos(ha))` and
        // divided the dec correction by sin(g) — which goes NaN at
        // gclat = 0 (observer on the equator) and at hour angles
        // where cos(ha) flips sign. Replaced with the direct trig
        // (Meeus AA 40.6) that has no removable singularity.
        //
        // Note: positions stay in equator-of-date — Schlyter's
        // linear w/n element terms already account for the secular
        // precession of the orbits, so an explicit J2000→date
        // rotation here would double-count by ~50"/year.
        let par = (8.794 / 3600.0) / r_b;
        let gclat = self.lat - 0.1924 * deg(2.0 * self.lat).sin();
        let rho = 0.99833 + 0.00167 * deg(2.0 * self.lat).cos();
        let lst = self.sidtime * 15.0;
        let ha = (lst - ra + 360.0) % 360.0;
        let cos_dec = deg(dec_val).cos();
        let top_ra = if cos_dec.abs() < 1e-9 {
            ra
        } else {
            ra - par * rho * deg(gclat).cos() * deg(ha).sin() / cos_dec
        };
        let top_dec = dec_val - par * rho * (
            deg(gclat).sin() * cos_dec
            - deg(gclat).cos() * deg(ha).cos() * deg(dec_val).sin()
        );

        (top_ra, top_dec, dist, ra, dec_val)
    }

    fn alt_az(&self, ra: f64, dec: f64, time: f64) -> (f64, f64) {
        let ha = (time - ra / 15.0) * 15.0;
        let x = deg(ha).cos() * deg(dec).cos();
        let y = deg(ha).sin() * deg(dec).cos();
        let z = deg(dec).sin();
        let xhor = x * deg(self.lat).sin() - z * deg(self.lat).cos();
        let yhor = y;
        let zhor = x * deg(self.lat).cos() + z * deg(self.lat).sin();
        let az = (yhor.atan2(xhor) * 180.0 / PI + 180.0) % 360.0;
        let alt = zhor.asin() * 180.0 / PI;
        (alt, az)
    }

    fn body_alt_az(&self, name: &str, hour: f64) -> (f64, f64) {
        let (ra, dec, _, _, _) = self.body_calc(name);
        self.alt_az(ra, dec, hour)
    }

    /// Rise / transit / set times for a body at (ra, dec). `h0` is the
    /// true altitude at which the rise/set event is defined — set
    /// it to:
    ///   -0.833° for the sun (upper-limb + 34' refraction + 16' radius)
    ///   +0.125° for the moon (parallax - refraction - radius)
    ///   -0.5667° for stars / planets (just standard refraction)
    /// or 0° for the geometric horizon.
    ///
    /// Earlier this function ignored `h0` entirely (assumed 0°). At
    /// Oslo's 60° latitude on the summer solstice that omission cost
    /// ~12 minutes per end on sun rise/set — half the half-day-length
    /// inflation that h0 = -0.833° gives via Meeus 15.1.
    fn rts(&self, ra: f64, dec: f64, h0: f64) -> (String, String, String) {
        let transit = (ra - self.ls - self.lon) / 15.0 + 12.0 + self.tz;
        let transit = (transit + 24.0) % 24.0;
        let sin_h0 = deg(h0).sin();
        let phi = deg(self.lat);
        let delta = deg(dec);
        let denom = phi.cos() * delta.cos();
        if denom.abs() < 1e-12 {
            return ("never".into(), format_hhmm(transit), "always".into());
        }
        let cos_lha = (sin_h0 - phi.sin() * delta.sin()) / denom;
        if cos_lha < -1.0 {
            return ("always".into(), format_hhmm(transit), "never".into());
        }
        if cos_lha > 1.0 {
            return ("never".into(), format_hhmm(transit), "always".into());
        }
        let lha_h = cos_lha.acos() * 180.0 / PI / 15.0;
        let rise = (transit - lha_h + 24.0) % 24.0;
        let set = (transit + lha_h + 24.0) % 24.0;
        (format_hhmm(rise), format_hhmm(transit), format_hhmm(set))
    }
}

/// Standard rise/set altitudes for each body, in degrees. The sun and
/// moon get special-case offsets; everything else uses the generic
/// "stars and planets" refraction-only floor.
fn rise_altitude(body: &str) -> f64 {
    match body {
        "sun"  => -0.8333,
        "moon" =>  0.125,
        _      => -0.5667,
    }
}

// ── Public API ───────────────────────────────────────────────────────

fn julian_date(y: i32, m: u32, d: u32) -> f64 {
    let y = y as f64; let m = m as f64; let d = d as f64;
    367.0 * y - ((7.0 * (y + ((m + 9.0) / 12.0).floor())) / 4.0).floor()
        + ((275.0 * m) / 9.0).floor() + d + 1_721_013.5
}

/// Julian Date including fractional day from the current time.
pub fn julian_date_now(y: i32, m: u32, d: u32, hour: u32, minute: u32, sec: u32) -> f64 {
    julian_date(y, m, d) + (hour as f64 * 3600.0 + minute as f64 * 60.0 + sec as f64) / 86400.0
}

// ── Moon phase ──────────────────────────────────────────────────────

pub fn moon_phase(year: i32, month: u32, day: u32) -> MoonPhase {
    let jd = julian_date(year, month, day);
    let days_since = jd - NEW_MOON_EPOCH_JD;
    let mut phase = (days_since / SYNODIC_MONTH) % 1.0;
    if phase < 0.0 { phase += 1.0; }
    let illumination = (1.0 - (phase * 2.0 * PI).cos()) / 2.0;
    let phase_index = ((phase * 8.0).floor() as usize) % 8;
    MoonPhase {
        illumination: (illumination * 10000.0).round() / 10000.0,
        phase: (phase * 10000.0).round() / 10000.0,
        phase_name: PHASE_NAMES[phase_index],
        symbol: PHASE_SYMBOLS[phase_index],
        phase_index,
    }
}

pub fn moon_symbol(year: i32, month: u32, day: u32) -> &'static str {
    moon_phase(year, month, day).symbol
}

pub fn notable_phase(year: i32, month: u32, day: u32) -> bool {
    let today = moon_phase(year, month, day);
    if !matches!(today.phase_index, 0 | 2 | 4 | 6) { return false; }
    let (py, pm, pd) = prev_day(year, month, day);
    moon_phase(py, pm, pd).phase_index != today.phase_index
}

pub fn notable_phases_in_month(year: i32, month: u32) -> Vec<NotablePhase> {
    let last = days_in_month(year, month);
    let mut result = Vec::new();
    for d in 1..=last {
        if notable_phase(year, month, d) {
            let p = moon_phase(year, month, d);
            result.push(NotablePhase { day: d, phase_name: p.phase_name, symbol: p.symbol });
        }
    }
    result
}

// ── Astronomical events ─────────────────────────────────────────────

pub fn astro_events(month: u32, day: u32) -> Vec<String> {
    astro_events_for_year(2025, month, day)
}

pub fn astro_events_for_year(year: i32, month: u32, day: u32) -> Vec<String> {
    let mut events = Vec::new();
    if notable_phase(year, month, day) {
        let p = moon_phase(year, month, day);
        events.push(format!("{} {}", p.symbol, p.phase_name));
    }
    match (month, day) {
        (6, 21)  => events.push("\u{2600} Summer Solstice".into()),
        (12, 21) => events.push("\u{2744} Winter Solstice".into()),
        (3, 20)  => events.push("\u{2600} Vernal Equinox".into()),
        (9, 22)  => events.push("\u{2600} Autumnal Equinox".into()),
        _ => {}
    }
    match (month, day) {
        (1, 3)   => events.push("\u{2604} Quadrantids peak".into()),
        (4, 22)  => events.push("\u{2604} Lyrids peak".into()),
        (5, 6)   => events.push("\u{2604} Eta Aquariids peak".into()),
        (7, 30)  => events.push("\u{2604} Delta Aquariids peak".into()),
        (8, 12)  => events.push("\u{2604} Perseids peak".into()),
        (10, 8)  => events.push("\u{2604} Draconids peak".into()),
        (10, 21) => events.push("\u{2604} Orionids peak".into()),
        (11, 5)  => events.push("\u{2604} Taurids peak".into()),
        (11, 17) => events.push("\u{2604} Leonids peak".into()),
        (12, 14) => events.push("\u{2604} Geminids peak".into()),
        (12, 22) => events.push("\u{2604} Ursids peak".into()),
        _ => {}
    }
    events
}

// ── Sun times (via ephemeris engine) ────────────────────────────────

pub fn sun_times(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Option<(String, String)> {
    let eph = Ephemeris::new(year, month, day, lat, lon, tz);
    let (rise, _, set) = eph.rts(eph.sun_ra, eph.sun_dec, rise_altitude("sun"));
    if rise == "never" || rise == "always" { return None; }
    Some((truncate_hms(&rise), truncate_hms(&set)))
}

pub fn sun_times_oslo(year: i32, month: u32, day: u32) -> Option<(String, String)> {
    sun_times(year, month, day, 59.9139, 10.7522, 1.0)
}

/// Moon rise and set times via ephemeris.
pub fn moon_times(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Option<(String, String)> {
    let eph = Ephemeris::new(year, month, day, lat, lon, tz);
    let (ra, dec, _, _, _) = eph.body_calc("moon");
    let (rise, _, set) = eph.rts(ra, dec, rise_altitude("moon"));
    if rise == "never" || rise == "always" { return None; }
    Some((truncate_hms(&rise), truncate_hms(&set)))
}

// ── Visible planets (via ephemeris engine) ──────────────────────────

/// Returns planets visible at night (altitude > 5 degrees at any hour 20:00-04:00).
/// Uses the same algorithm as Timely's ruby-ephemeris integration.
// ── Body constants (astropanel-compatible) ──────────────────────────

pub const BODY_ORDER: &[&str] = &[
    "sun", "moon", "mercury", "venus", "mars",
    "jupiter", "saturn", "uranus", "neptune",
];

pub fn body_symbol(name: &str) -> &'static str {
    // VS-15 (`U+FE0E`) appended so emoji-presentation terminals
    // keep these as monochrome text glyphs, matching the size /
    // weight of the surrounding text rather than scaling them up
    // into 2-cell colour icons. Same rationale as
    // `visible_planets` — see the comment there.
    match name {
        "sun"     => "\u{2600}\u{FE0E}",
        "moon"    => "\u{263E}\u{FE0E}",
        "mercury" => "\u{263F}\u{FE0E}",
        "venus"   => "\u{2640}\u{FE0E}",
        "mars"    => "\u{2642}\u{FE0E}",
        "jupiter" => "\u{2643}\u{FE0E}",
        "saturn"  => "\u{2644}\u{FE0E}",
        "uranus"  => "\u{2645}\u{FE0E}",
        "neptune" => "\u{2646}\u{FE0E}",
        _ => "?",
    }
}

pub fn body_color_hex(name: &str) -> &'static str {
    match name {
        "sun" => "FFD700", "moon" => "888888", "mercury" => "8F6E54",
        "venus" => "E6B07C", "mars" => "BC2732", "jupiter" => "C08040",
        "saturn" => "E8D9A0", "uranus" => "80DFFF", "neptune" => "1E90FF",
        _ => "FFFFFF",
    }
}

pub fn body_color_256(name: &str) -> u8 {
    match name {
        "sun" => 220, "moon" => 248, "mercury" => 137,
        "venus" => 216, "mars" => 196, "jupiter" => 208,
        "saturn" => 229, "uranus" => 117, "neptune" => 33,
        _ => 255,
    }
}

pub fn body_display(name: &str) -> &'static str {
    match name {
        "sun" => "Sun", "moon" => "Moon", "mercury" => "Mercury",
        "venus" => "Venus", "mars" => "Mars", "jupiter" => "Jupiter",
        "saturn" => "Saturn", "uranus" => "Uranus", "neptune" => "Neptune",
        _ => "?",
    }
}

/// All-body ephemeris data for a given date and location.
#[derive(Clone, Debug)]
pub struct BodyObs {
    pub name: &'static str,
    pub ra_deg: f64,
    pub dec_deg: f64,
    pub distance: f64,
    pub rise: String,
    pub transit: String,
    pub set: String,
    /// rise hour in fractional hours (None if "always" or "never")
    pub rise_h: Option<f64>,
    pub set_h: Option<f64>,
    pub always_up: bool,
    pub never_up: bool,
}

fn parse_hhmm(s: &str) -> Option<f64> {
    if s.len() < 5 { return None; }
    let h: f64 = s[..2].parse().ok()?;
    let m: f64 = s[3..5].parse().ok()?;
    Some(h + m / 60.0)
}

pub fn all_bodies(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Vec<BodyObs> {
    let eph = Ephemeris::new(year, month, day, lat, lon, tz);
    let mut out = Vec::with_capacity(BODY_ORDER.len());
    for &name in BODY_ORDER {
        let (ra, dec, dist, _, _) = eph.body_calc(name);
        let (rise, transit, set) = eph.rts(ra, dec, rise_altitude(name));
        let always_up = rise == "always";
        let never_up = rise == "never";
        let rise_h = if always_up || never_up { None } else { parse_hhmm(&rise) };
        let set_h = if always_up || never_up { None } else { parse_hhmm(&set) };
        let name_s: &'static str = match name {
            "sun" => "sun", "moon" => "moon", "mercury" => "mercury",
            "venus" => "venus", "mars" => "mars", "jupiter" => "jupiter",
            "saturn" => "saturn", "uranus" => "uranus", "neptune" => "neptune",
            _ => "?",
        };
        out.push(BodyObs {
            name: name_s, ra_deg: ra, dec_deg: dec, distance: dist,
            rise, transit, set, rise_h, set_h, always_up, never_up,
        });
    }
    out
}

/// Is a body above the horizon at the given local hour?
/// Uses rise/set hours from all_bodies result.
pub fn is_above(rise_h: Option<f64>, set_h: Option<f64>, always_up: bool, never_up: bool, hour: f64) -> bool {
    if always_up { return true; }
    if never_up { return false; }
    match (rise_h, set_h) {
        (Some(r), Some(s)) => {
            if r > s { hour >= r || hour <= s }
            else { hour >= r && hour <= s }
        }
        _ => false,
    }
}

fn ra_to_hm(ra_deg: f64) -> String {
    let mut h = ra_deg / 15.0;
    if h < 0.0 { h += 24.0; }
    let hh = h.floor() as i32;
    let mm = ((h - hh as f64) * 60.0).round() as i32;
    let (hh, mm) = if mm >= 60 { ((hh + 1) % 24, 0) } else { (hh, mm) };
    format!("{:02}h {:02}m", hh, mm)
}

fn dec_to_dm(dec_deg: f64) -> String {
    let sign = if dec_deg < 0.0 { "-" } else { "+" };
    let d = dec_deg.abs();
    let dd = d.floor() as i32;
    let mm = ((d - dd as f64) * 60.0).round() as i32;
    let (dd, mm) = if mm >= 60 { (dd + 1, 0) } else { (dd, mm) };
    format!("{}{:02}\u{00B0} {:02}\u{2032}", sign, dd, mm)
}

fn format_distance(d: f64) -> String {
    // 4 decimals, two-digit integer part where possible
    format!("{:7.4}", d)
}

/// Formatted ephemeris table (matches astropanel's layout).
/// Returns a multi-line string with ANSI colors for each body.
pub fn ephemeris_table(bodies: &[BodyObs]) -> String {
    let mut out = String::new();
    out.push_str("Planet      \u{2502} RA       \u{2502} Dec      \u{2502} d=AU   \u{2502} Rise  \u{2502} Trans \u{2502} Set\n");
    out.push_str("\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\n");
    for b in bodies {
        let color = body_color_256(b.name);
        let name = format!("{} {}", body_symbol(b.name), capitalize_short(b.name));
        let name = format!("{:<11}", name);
        let ra_s = ra_to_hm(b.ra_deg);
        let dec_s = dec_to_dm(b.dec_deg);
        let d_s = format_distance(b.distance);
        let hhmm = |s: &str| -> String {
            if s.len() >= 5 { s[..5].to_string() } else { s.to_string() }
        };
        let rise = hhmm(&b.rise);
        let trans = hhmm(&b.transit);
        let set = hhmm(&b.set);
        let colored = |s: &str| -> String {
            format!("\x1b[38;5;{}m{}\x1b[0m", color, s)
        };
        out.push_str(&format!(
            "{} \u{2502} {} \u{2502} {} \u{2502} {} \u{2502} {} \u{2502} {} \u{2502} {}\n",
            colored(&name),
            colored(&ra_s),
            colored(&format!("{:<8}", dec_s)),
            colored(&d_s),
            colored(&format!("{:<5}", rise)),
            colored(&format!("{:<5}", trans)),
            colored(&format!("{:<5}", set)),
        ));
    }
    out
}

fn capitalize_short(name: &str) -> &'static str {
    // Match Ruby astropanel's 8-char-max labels
    match name {
        "sun" => "Sun", "moon" => "Moon", "mercury" => "Mercury",
        "venus" => "Venus", "mars" => "Mars", "jupiter" => "Jupiter",
        "saturn" => "Saturn", "uranus" => "Uranus", "neptune" => "Neptune",
        _ => "?",
    }
}

/// Moon phase percent (0..100), useful for coloring visibility bars.
pub fn moon_phase_pct(year: i32, month: u32, day: u32) -> u8 {
    let mp = moon_phase(year, month, day);
    (mp.illumination * 100.0).round() as u8
}

/// Gray hex code for moon visibility block, based on illumination pct (0-100).
pub fn moon_phase_gray(pct: u8) -> String {
    let min: u8 = 0x22;
    let v = (min as u16 + ((0xFF - min as u16) * pct as u16) / 100) as u8;
    format!("{:02x}{:02x}{:02x}", v, v, v)
}

/// Map hex color to nearest 256-color index for terminal output. Quick/dirty.
pub fn hex_to_256(hex: &str) -> u8 {
    if hex.len() < 6 { return 255; }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    // Grayscale detection
    if (r as i32 - g as i32).abs() < 8 && (g as i32 - b as i32).abs() < 8 {
        let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
        if gray < 8 { return 16; }
        if gray > 248 { return 231; }
        return 232 + (gray - 8) / 10;
    }
    let conv = |c: u8| -> u8 {
        if c < 48 { 0 } else if c < 115 { 1 } else { ((c - 35) / 40).min(5) }
    };
    16 + 36 * conv(r) + 6 * conv(g) + conv(b)
}

pub fn visible_planets(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Vec<VisiblePlanet> {
    let eph = Ephemeris::new(year, month, day, lat, lon, tz);

    // Variation selector 15 (U+FE0E) forces emoji-presentation
    // terminals to render the astronomical/astrological symbols as
    // text-style monochrome glyphs rather than 2-cell colour icons.
    // Without it, kitty / wezterm / Konsole blow `☿♀♂♃♄` up to emoji
    // size next to small-pt time strings; with it, they sit cleanly
    // inline with the rest of the row.
    let planet_info: &[(&str, &str, &str)] = &[
        ("mercury", "\u{263F}\u{FE0E}", "8F6E54"),
        ("venus",   "\u{2640}\u{FE0E}", "E6B07C"),
        ("mars",    "\u{2642}\u{FE0E}", "BC2732"),
        ("jupiter", "\u{2643}\u{FE0E}", "C08040"),
        ("saturn",  "\u{2644}\u{FE0E}", "E8D9A0"),
    ];

    let check_hours = [20.0, 21.0, 22.0, 23.0, 0.0, 1.0, 2.0, 3.0, 4.0];
    let mut visible = Vec::new();

    for &(name, symbol, color) in planet_info {
        let is_visible = check_hours.iter().any(|&h| {
            let (alt, _) = eph.body_alt_az(name, h);
            alt > 5.0
        });
        if is_visible {
            let (ra, dec, _, _, _) = eph.body_calc(name);
            let (rise, _, set) = eph.rts(ra, dec, rise_altitude(name));
            visible.push(VisiblePlanet {
                name: match name {
                    "mercury" => "Mercury", "venus" => "Venus", "mars" => "Mars",
                    "jupiter" => "Jupiter", "saturn" => "Saturn", _ => name,
                },
                symbol, color,
                rise: truncate_hms(&rise), set: truncate_hms(&set),
            });
        }
    }
    visible
}

/// Locally-computed "tonight" summary for days with no notable
/// astronomical event. Shows moon phase + rise/set, visible planets
/// with rise/set times, constellations near the zenith for the date
/// + hemisphere, and a one-line Bortle hint.
pub fn tonight_summary(
    year: i32, month: u32, day: u32,
    lat: f64, lon: f64, tz: f64, bortle: f64,
) -> String {
    let mut out = String::new();
    let mp = moon_phase(year, month, day);
    let illum_pct = (mp.illumination * 100.0).round() as u32;
    let (mrise, mset) = moon_times(year, month, day, lat, lon, tz)
        .unwrap_or_else(|| ("--:--".into(), "--:--".into()));
    out.push_str(&format!(
        "Tonight: {} {} {}%, rises {} sets {}\n",
        mp.symbol, mp.phase_name, illum_pct, mrise, mset,
    ));

    let planets = visible_planets(year, month, day, lat, lon, tz);
    if planets.is_empty() {
        out.push_str("Planets: none above 5° between 20:00 and 04:00\n");
    } else {
        out.push_str("Planets: ");
        let parts: Vec<String> = planets.iter()
            .map(|p| format!("{} {} (rises {} sets {})",
                p.symbol, p.name, p.rise, p.set))
            .collect();
        out.push_str(&parts.join(", "));
        out.push('\n');
    }

    let constellations = constellations_near_zenith(month, lat);
    if !constellations.is_empty() {
        out.push_str(&format!("Near zenith: {}\n", constellations.join(", ")));
    }

    out.push_str(&bortle_hint(bortle));
    out
}

/// Northern-hemisphere zenith constellations by month (mid-northern
/// latitudes). For southern hemisphere we shift by six months as a
/// rough approximation; at low latitudes this is less meaningful but
/// the names are still recognisable to the user.
fn constellations_near_zenith(month: u32, lat: f64) -> Vec<&'static str> {
    let northern = lat >= 0.0;
    let m = if northern {
        month
    } else {
        ((month + 5) % 12) + 1
    };
    match m {
        1  => vec!["Orion", "Taurus", "Auriga", "Gemini"],
        2  => vec!["Orion", "Canis Major", "Gemini", "Auriga"],
        3  => vec!["Leo", "Cancer", "Gemini", "Hydra"],
        4  => vec!["Leo", "Virgo", "Ursa Major"],
        5  => vec!["Virgo", "Boötes", "Coma Berenices"],
        6  => vec!["Boötes", "Hercules", "Corona Borealis"],
        7  => vec!["Lyra", "Cygnus", "Hercules", "Scorpius"],
        8  => vec!["Cygnus", "Lyra", "Aquila", "Sagittarius"],
        9  => vec!["Pegasus", "Andromeda", "Cygnus", "Capricornus"],
        10 => vec!["Pegasus", "Andromeda", "Pisces", "Aquarius"],
        11 => vec!["Andromeda", "Triangulum", "Aries", "Perseus"],
        12 => vec!["Taurus", "Orion", "Perseus", "Auriga"],
        _  => vec![],
    }
}

/// One-line Bortle-class hint about what's visible from the observer's
/// site. Matches the user-configured Bortle rating in nova's config.
fn bortle_hint(bortle: f64) -> &'static str {
    let b = bortle.round() as i32;
    match b {
        1 => "Bortle 1 — pristine; zodiacal light, gegenschein, M33 naked-eye.\n",
        2 => "Bortle 2 — truly dark; Milky Way structure obvious overhead.\n",
        3 => "Bortle 3 — rural; Milky Way clear, M31 naked-eye.\n",
        4 => "Bortle 4 — rural/suburban transition; Milky Way visible overhead.\n",
        5 => "Bortle 5 — suburban; Milky Way faint near zenith only.\n",
        6 => "Bortle 6 — bright suburban; Milky Way invisible, M31 with effort.\n",
        7 => "Bortle 7 — suburban/urban transition; only the brightest stars.\n",
        8 => "Bortle 8 — city; bright stars only, planets and Moon dominate.\n",
        9 => "Bortle 9 — inner city; only Moon, planets, brightest stars.\n",
        _ => "",
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(year) { 29 } else { 28 },
        _ => 30,
    }
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn prev_day(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    if day > 1 { (year, month, day - 1) }
    else if month > 1 { let pm = month - 1; (year, pm, days_in_month(year, pm)) }
    else { (year - 1, 12, 31) }
}

fn format_hhmm(hours: f64) -> String {
    let mut h = hours % 24.0;
    if h < 0.0 { h += 24.0; }
    let hh = h.floor() as u32;
    let mm = ((h - hh as f64) * 60.0).round() as u32;
    if mm >= 60 { format!("{:02}:{:02}", (hh + 1) % 24, 0) }
    else { format!("{:02}:{:02}", hh, mm) }
}

/// Truncate "HH:MM:SS" to "HH:MM"
fn truncate_hms(s: &str) -> String {
    if s.len() >= 5 { s[..5].to_string() } else { s.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Sanity tests ──────────────────────────────────────────────

    #[test]
    fn test_moon_phase_range() {
        let p = moon_phase(2026, 4, 7);
        assert!(p.illumination >= 0.0 && p.illumination <= 1.0);
        assert!(p.phase_index < 8);
    }

    #[test]
    fn test_ephemeris_body_calc_no_panic() {
        let eph = Ephemeris::new(2026, 4, 7, 59.9139, 10.7522, 1.0);
        for name in &["mercury", "venus", "mars", "jupiter", "saturn"] {
            let (ra, dec, _, _, _) = eph.body_calc(name);
            assert!(ra >= 0.0 && ra < 360.0, "{} RA out of range: {}", name, ra);
            assert!(dec >= -90.0 && dec <= 90.0, "{} Dec out of range: {}", name, dec);
        }
    }

    // ── Robustness regressions ────────────────────────────────────

    /// Previously body_calc divided the topocentric Dec correction
    /// by sin(g) where g = atan(tan(gclat)/cos(ha)) — which went NaN
    /// for an observer on the equator (gclat = 0) and at hour angles
    /// where cos(ha) flips sign. Guard against the regression.
    #[test]
    fn topocentric_dec_is_finite_at_equator() {
        let bodies = all_bodies(2025, 1, 1, 0.0, 0.0, 0.0);
        for b in &bodies {
            assert!(b.ra_deg.is_finite(),  "{} RA non-finite at lat=0",  b.name);
            assert!(b.dec_deg.is_finite(), "{} Dec non-finite at lat=0", b.name);
        }
    }

    #[test]
    fn topocentric_dec_is_finite_at_poles() {
        let bodies = all_bodies(2025, 1, 1, 89.9, 0.0, 0.0);
        for b in &bodies {
            assert!(b.ra_deg.is_finite(),  "{} RA non-finite at high lat",  b.name);
            assert!(b.dec_deg.is_finite(), "{} Dec non-finite at high lat", b.name);
        }
        let bodies = all_bodies(2025, 1, 1, -89.9, 0.0, 0.0);
        for b in &bodies {
            assert!(b.ra_deg.is_finite(),  "{} RA non-finite at low lat",  b.name);
            assert!(b.dec_deg.is_finite(), "{} Dec non-finite at low lat", b.name);
        }
    }

    // ── Accuracy floor against Skyfield (JPL DE421) ──────────────
    // Truth values from skyfield 1.54 with `apparent().radec(epoch='date')`
    // — i.e. mean equator/equinox of date, geocentric, with light-time
    // and aberration. That's the frame Schlyter's element model
    // approximates, so it's the right benchmark.
    //
    // Tolerances chosen at ~2x the worst observed error per body across
    // 2025-2030 so the test catches a real degradation, not random
    // perturbation-term drift.

    fn angle_diff_deg(a: f64, b: f64) -> f64 {
        let mut d = (a - b).abs();
        if d > 180.0 { d = 360.0 - d; }
        d
    }

    /// Truth values: Skyfield + JPL DE421 at lat=lon=elev=0, true
    /// equator/equinox of date. Matches `all_bodies(_,_,_,0.0,0.0,0.0)`
    /// frame to milli-arcsec precision.
    /// (year, month, day, body, ra_truth_deg, dec_truth_deg)
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

        (2025, 6, 21, "moon",    26.6448,  13.7751),

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

        (2026, 12, 21, "moon",   41.4350,  21.8143),

        (2030, 1, 1, "sun",     281.5309, -23.0108),
        (2030, 1, 1, "moon",    236.3607, -21.8877),
        (2030, 1, 1, "mercury", 280.0892, -20.4580),
        (2030, 1, 1, "venus",   290.6743, -18.8841),
        (2030, 1, 1, "mars",    317.5211, -17.5343),
        (2030, 1, 1, "jupiter", 228.4152, -16.9266),
    ];

    /// Per-body tolerance in arcminutes for both RA and Dec — ~2× the
    /// worst observed delta across 2025-2030 so genuine regressions
    /// surface immediately. See CLAUDE.md for the verified accuracy
    /// floor table.
    fn tolerance_arcmin(body: &str) -> f64 {
        match body {
            "sun"     => 2.0,
            "moon"    => 3.0,    // Meeus 47 full series — 60 terms
            "mercury" => 3.0,
            "venus"   => 2.0,
            "mars"    => 3.0,
            "jupiter" => 3.0,
            "saturn"  => 10.0,   // Schlyter's 5-term Saturn series
            "uranus"  => 2.0,
            "neptune" => 2.0,
            _         => 60.0,
        }
    }

    #[test]
    fn ra_dec_within_tolerance_against_skyfield() {
        // Pure geocentric vantage (lat/lon = 0, tz = 0) so the truth
        // generator's `obs = earth.at(t)` and our orbit lookup are in
        // the same observer frame (topocentric parallax negligible for
        // everything but the moon, which has its own large tolerance).
        let lat = 0.0; let lon = 0.0; let tz = 0.0;
        for &(y, m, d, body, ra_t, dec_t) in TRUTH {
            let bodies = all_bodies(y, m, d, lat, lon, tz);
            let b = bodies.iter().find(|x| x.name == body)
                .unwrap_or_else(|| panic!("body {} not produced by all_bodies", body));
            let dra_min = angle_diff_deg(b.ra_deg, ra_t) * 60.0;
            let ddec_min = (b.dec_deg - dec_t).abs() * 60.0;
            let tol = tolerance_arcmin(body);
            assert!(dra_min <= tol,
                "{} {}-{:02}-{:02} RA: {} vs truth {} (Δ={:.2}' > {:.1}' tolerance)",
                body, y, m, d, b.ra_deg, ra_t, dra_min, tol);
            assert!(ddec_min <= tol,
                "{} {}-{:02}-{:02} Dec: {} vs truth {} (Δ={:.2}' > {:.1}' tolerance)",
                body, y, m, d, b.dec_deg, dec_t, ddec_min, tol);
        }
    }

    // ── Sun rise / set against Oslo almanac data ──────────────────

    /// Convert "HH:MM" to minutes-since-midnight. Panics on bad input —
    /// the caller's test will report the panic location.
    fn to_min(s: &str) -> i32 {
        let h: i32 = s[..2].parse().unwrap();
        let m: i32 = s[3..5].parse().unwrap();
        h * 60 + m
    }

    fn assert_time_close(actual: &str, expected_minutes: i32, tol_min: i32, ctx: &str) {
        let a = to_min(actual);
        let mut diff = (a - expected_minutes).abs();
        if diff > 720 { diff = 1440 - diff; } // wrap around midnight
        assert!(diff <= tol_min,
            "{}: expected {:02}:{:02}, got {} (Δ={} min, tol {})",
            ctx, expected_minutes / 60, expected_minutes % 60, actual, diff, tol_min);
    }

    /// Oslo, lat 59.9139°N lon 10.7522°E. Reference times from
    /// Skyfield (UTC), then shifted by the named tz offset.
    /// Tolerance: 3 minutes, which is well within Schlyter-class
    /// accuracy for sun rise/set at Norwegian latitudes.
    #[test]
    fn sun_rise_set_oslo() {
        let cases = &[
            // (y, m, d, tz_used, expected_rise_local, expected_set_local)
            // Skyfield UTC values shifted by tz:
            //   2025-06-21 UTC: rise 01:53:46, set 20:43:52  → +2h DST: 03:54, 22:44
            //   2025-12-21 UTC: rise 08:18:14, set 14:12:03  → +1h:     09:18, 15:12
            //   2026-06-21 UTC: rise 01:53:44, set 20:43:51  → +2h DST: 03:54, 22:44
            //   2026-12-21 UTC: rise 08:18:06, set 14:11:57  → +1h:     09:18, 15:12
            (2025, 6, 21, 2.0,  3*60 + 54, 22*60 + 44),
            (2025,12, 21, 1.0,  9*60 + 18, 15*60 + 12),
            (2026, 6, 21, 2.0,  3*60 + 54, 22*60 + 44),
            (2026,12, 21, 1.0,  9*60 + 18, 15*60 + 12),
        ];
        for &(y, m, d, tz, exp_rise, exp_set) in cases {
            let (rise, set) = sun_times(y, m, d, 59.9139, 10.7522, tz)
                .unwrap_or_else(|| panic!("sun_times returned None for {}-{}-{}", y, m, d));
            assert_time_close(&rise, exp_rise, 3, &format!("Oslo {}-{:02}-{:02} sunrise", y, m, d));
            assert_time_close(&set,  exp_set,  3, &format!("Oslo {}-{:02}-{:02} sunset",  y, m, d));
        }
    }

    /// Equatorial site (lat 0) — the sun is up ~12h regardless of
    /// season. The day length should sit within a few minutes of 12h
    /// at the equinoxes, with sunrise near 06:00 local.
    #[test]
    fn sun_rise_set_equator() {
        let (rise, set) = sun_times(2026, 3, 20, 0.0, 0.0, 0.0).expect("equator sun");
        let rh = to_min(&rise);
        let sh = to_min(&set);
        let day_len = sh - rh;
        assert!((day_len - 720).abs() <= 10,
            "Equator equinox day length ≈ 12h, got {} → {} ({} min)",
            rise, set, day_len);
    }

    // ── Moon phase against known new/full moons ───────────────────
    // Reference dates from NASA Goddard / timeanddate.com.

    #[test]
    fn moon_phase_at_known_full() {
        // 2025-01-13 22:27 UTC was full moon.
        let p = moon_phase(2025, 1, 13);
        assert!(p.illumination > 0.97,
            "Full moon 2025-01-13: illum {:.4} (expected > 0.97)", p.illumination);
    }

    #[test]
    fn moon_phase_at_known_new() {
        // 2025-01-29 12:36 UTC was new moon.
        let p = moon_phase(2025, 1, 29);
        assert!(p.illumination < 0.03,
            "New moon 2025-01-29: illum {:.4} (expected < 0.03)", p.illumination);
    }

    // ── Sanity: visible_planets returns reasonable shape ──────────

    #[test]
    fn visible_planets_smoke() {
        let v = visible_planets(2026, 4, 7, 59.9139, 10.7522, 1.0);
        assert!(v.len() <= 5);
        for p in &v {
            assert_eq!(p.rise.len(), 5, "rise should be HH:MM, got {}", p.rise);
            assert_eq!(p.set.len(),  5, "set should be HH:MM, got {}",  p.set);
        }
    }
}
