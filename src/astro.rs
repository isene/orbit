//! A full almanac: the sun, the moon, the planets, Pluto, 105 bright stars
//! and the 88 constellations. Where they are, when they rise and set, how
//! bright they are, moon phases to the minute, eclipses, Jupiter's moons,
//! Saturn's rings, seasons, twilight, oppositions and a year's almanac.
//!
//! Rules for all of it: angles are in degrees. `lat` is north-positive,
//! `lon` is east-positive, `tz` is hours ahead of UT (Oslo in winter: 1.0).
//! A moment is a Julian date (an `f64`, UT): make one with [`jd_at`] or
//! [`jd_of_unix`], print one with [`jd_text`] or [`jd_local`]. A body is
//! named "sun", "moon", "mercury", "venus", "mars", "jupiter", "saturn",
//! "uranus" or "neptune". Good to a minute of arc for the sun and the
//! moon, a few for the planets, between 1900 and 2100.
//!
//! This module is the same almanac as the `astro` module of the Simpler
//! language (<https://github.com/isene/simpler>, `lib/astro.smplr`): the
//! same names, the same numbers. Its results are checked against PyEphem,
//! which shares no code with it.

use std::f64::consts::PI;

include!("astro_tables.rs");

/// One place at one moment. Make it with [`sky_at`], [`sky_jd`] or
/// [`sky`]; hand it to the rest.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sky {
    pub lat: f64,
    pub lon: f64,
    pub tz: f64,
    /// Days since 1999-12-31 0h UT.
    pub d: f64,
    /// The tilt of the earth.
    pub ecl: f64,
    pub ls: f64,
    pub xs: f64,
    pub ys: f64,
    /// Local sidereal time in hours (not wrapped; see [`sidereal`]).
    pub sidtime: f64,
    pub sun_ra: f64,
    pub sun_dec: f64,
    pub sun_lon: f64,
    pub sun_dist: f64,
    pub mj: f64,
    pub ms: f64,
}

/// A place in the sky. `ra`, `dec`: as seen by the observer. `gra`,
/// `gdec`: as seen from the centre of the earth. `lon`, `lat`: along and
/// above the sun's path (ecliptic). `dist`: distance in AU (earth radii
/// for the moon). `sun_dist`: distance from the sun in AU. `tdist`: the
/// distance from the observer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub ra: f64,
    pub dec: f64,
    pub dist: f64,
    pub gra: f64,
    pub gdec: f64,
    pub lon: f64,
    pub lat: f64,
    pub sun_dist: f64,
    pub tdist: f64,
}

/// Height above the horizon and compass direction (0 north, 90 east).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hor {
    pub alt: f64,
    pub az: f64,
}

/// Rise, transit and set as "HH:MM" local time. Up all day: rise
/// "always", set "never". Down all day: rise "never", set "always".
/// "none": it does not happen on this day.
#[derive(Debug, Clone, PartialEq)]
pub struct Times {
    pub rise: String,
    pub transit: String,
    pub set: String,
}

/// One body on one day: place (at 0h UT), times, brightness (`mag`, lower
/// is brighter), the lit part (0 to 1), width in seconds of arc, and the
/// constellation it stands in.
#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub name: String,
    pub ra: f64,
    pub dec: f64,
    pub dist: f64,
    pub rise: String,
    pub transit: String,
    pub set: String,
    pub mag: f64,
    pub lit: f64,
    pub size: f64,
    pub con: String,
}

/// The moon's phase: `lit` is the lit part (0 to 1), `age` the part of the
/// month gone (0 to 1), `index` 0 new, 2 first quarter, 4 full, 6 last
/// quarter (odd numbers are in between).
#[derive(Debug, Clone, PartialEq)]
pub struct Phase {
    pub lit: f64,
    pub age: f64,
    pub index: usize,
    pub name: &'static str,
    pub symbol: &'static str,
}

/// Two angles: ra and dec.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spot {
    pub ra: f64,
    pub dec: f64,
}

/// Two angles: along (`lon`) and above (`lat`) the ecliptic or the Milky Way.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LonLat {
    pub lon: f64,
    pub lat: f64,
}

/// A star: `ra` and `dec` for the year 2000, `mag` (lower is brighter),
/// `ly` the distance in light-years (0.0 when not known), `con` the
/// constellation, `spectral` its class ("A1").
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Star {
    pub name: &'static str,
    pub con: &'static str,
    pub spectral: &'static str,
    pub ra: f64,
    pub dec: f64,
    pub mag: f64,
    pub ly: f64,
}

/// A meteor shower: the day it peaks, meteors an hour at best, and where
/// they come from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shower {
    pub name: &'static str,
    pub month: u32,
    pub day: u32,
    pub rate: u32,
    pub ra: f64,
    pub dec: f64,
}

/// Something that happens at a moment (a Julian date).
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub when: f64,
    pub text: String,
}

/// An eclipse at its greatest (`when`). `kind`: "solar" or "lunar".
/// `form`: "total", "annular", "hybrid", "partial" or "penumbral". `mag`:
/// the part of the width covered (over 1 is total). `gamma`: how far from
/// the centre the shadow passes. `partial` and `total`: minutes that
/// phase lasts (lunar only). `form` is "" when there is no eclipse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eclipse {
    pub when: f64,
    pub kind: &'static str,
    pub form: &'static str,
    pub mag: f64,
    pub gamma: f64,
    pub partial: f64,
    pub total: f64,
}

/// A solar eclipse seen from one place: `start`, `peak` and `stop` as
/// Julian dates, `mag` the part of the sun's width covered at the peak,
/// `cover` the part of its face, `alt` the sun's height then. `mag` is
/// 0.0 when the place sees nothing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Local {
    pub start: f64,
    pub peak: f64,
    pub stop: f64,
    pub mag: f64,
    pub cover: f64,
    pub alt: f64,
}

/// A path round the sun, for a comet, an asteroid or a dwarf planet.
/// `node`, `incl` and `peri` are the usual three angles (for the year
/// 2000), `a` the half width in AU, `e` the stretch (under 1), `m0` the
/// mean anomaly at the moment `epoch` (a Julian date), `daily` the
/// degrees it moves a day (0.0: worked out from `a`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orbit {
    pub name: &'static str,
    pub node: f64,
    pub incl: f64,
    pub peri: f64,
    pub a: f64,
    pub e: f64,
    pub m0: f64,
    pub epoch: f64,
    pub daily: f64,
}

/// One of Jupiter's four big moons as seen from the earth. `x` and `y`
/// are counted in widths of half the planet (Jupiter's edge is at 1.0):
/// `x` runs along the planet's belts, positive to the west, and `y`
/// towards its north pole. `state` is "" (seen beside the planet),
/// "transit" (in front of it) or "hidden" (behind it).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JMoon {
    pub name: &'static str,
    pub x: f64,
    pub y: f64,
    pub state: &'static str,
}

/// Saturn's rings as seen from the earth. `tilt`: how far they are opened
/// towards us, in degrees (positive: we see their north side, 0.0:
/// edge-on and gone). `sun_tilt`: the same for the sun. `wide` and
/// `high`: the size of the outer ring in seconds of arc. `angle`: the
/// direction the planet's north pole points, from north towards east.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rings {
    pub tilt: f64,
    pub sun_tilt: f64,
    pub wide: f64,
    pub high: f64,
    pub angle: f64,
}

fn rad(x: f64) -> f64 { x * PI / 180.0 }
fn to_deg(r: f64) -> f64 { r * 180.0 / PI }
fn sn(x: f64) -> f64 { rad(x).sin() }
fn cs(x: f64) -> f64 { rad(x).cos() }

/// An angle brought into 0 to 360.
fn turn(x: f64) -> f64 {
    let r = x % 360.0;
    if r < 0.0 { r + 360.0 } else { r }
}

/// An angle brought into -180 to 180.
fn swing(x: f64) -> f64 {
    let r = turn(x);
    if r > 180.0 { r - 360.0 } else { r }
}

fn clamp1(x: f64) -> f64 { x.clamp(-1.0, 1.0) }

/// A number with a fixed count of decimals: `fixed(3.14159, 2)` is "3.14".
pub fn fixed(x: f64, places: usize) -> String {
    let scale = 10f64.powi(places as i32);
    let n = (x.abs() * scale).round();
    let whole = (n / scale).floor();
    let mut out = format!("{}", whole as i64);
    if places > 0 {
        let frac = (n - whole * scale).round() as i64;
        out = format!("{}.{:0width$}", out, frac, width = places);
    }
    if x < 0.0 && n > 0.0 {
        out.insert(0, '-');
    }
    out
}

/// Hours as "HH:MM".
pub fn hhmm(hours: f64) -> String {
    let mut h = hours % 24.0;
    if h < 0.0 { h += 24.0; }
    let mut hh = h.floor() as i64;
    let mut mm = ((h - hh as f64) * 60.0).round() as i64;
    if mm >= 60 { hh = (hh + 1) % 24; mm = 0; }
    format!("{:02}:{:02}", hh, mm)
}

/// "HH:MM" as hours, or -1.0 for "always", "never" and "none".
pub fn hours_of(s: &str) -> f64 {
    let b = s.as_bytes();
    if b.len() < 5 || !b[0].is_ascii_digit() || !b[1].is_ascii_digit() {
        return -1.0;
    }
    let h: f64 = s[0..2].parse().unwrap_or(0.0);
    let m: f64 = s[3..5].parse().unwrap_or(0.0);
    h + m / 60.0
}

/// A right ascension as "05h 12m".
pub fn ra_text(ra: f64) -> String {
    let h = turn(ra) / 15.0;
    let mut hh = h.floor() as i64;
    let mut mm = ((h - hh as f64) * 60.0).round() as i64;
    if mm >= 60 { hh = (hh + 1) % 24; mm = 0; }
    format!("{:02}h {:02}m", hh, mm)
}

/// A declination as "+19° 05′".
pub fn dec_text(dec: f64) -> String {
    let mark = if dec < 0.0 { "-" } else { "+" };
    let a = dec.abs();
    let mut dd = a.floor() as i64;
    let mut mm = ((a - dd as f64) * 60.0).round() as i64;
    if mm >= 60 { dd += 1; mm = 0; }
    format!("{}{:02}° {:02}′", mark, dd, mm)
}

/// A compass direction as one of 16 words: `compass(200.0)` is "SSW".
pub fn compass(az: f64) -> &'static str {
    let i = ((turn(az) / 22.5 + 0.5).floor() as usize) % 16;
    ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"][i]
}

fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// The number of days in a month.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(year) { 29 } else { 28 },
        _ => 31,
    }
}

/// The number of a day in its year: 1 for 1 January.
pub fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    day + (1..month).map(|m| days_in_month(year, m)).sum::<u32>()
}

/// The Julian date at 0h UT of a day.
pub fn jd(year: i32, month: u32, day: u32) -> f64 {
    let y = year as f64;
    let m = month as f64;
    (367.0 * y) - ((7.0 * (y + ((m + 9.0) / 12.0).floor())) / 4.0).floor()
        + ((275.0 * m) / 9.0).floor() + day as f64 + 1721013.5
}

/// The Julian date of a day and a time of day (UT).
pub fn jd_at(year: i32, month: u32, day: u32, hour: u32, minute: u32, sec: u32) -> f64 {
    jd(year, month, day) + (hour * 3600 + minute * 60 + sec) as f64 / 86400.0
}

/// The Julian date of a Unix clock value (seconds since 1970).
pub fn jd_of_unix(secs: i64) -> f64 {
    secs as f64 / 86400.0 + 2440587.5
}

/// A Julian date as five numbers: year, month, day, hour, minute (UT, to
/// the nearest minute).
pub fn date_of(j: f64) -> (i32, u32, u32, u32, u32) {
    let mins = ((j + 0.5) * 1440.0).round();
    let z = (mins / 1440.0).floor();
    let left = (mins - z * 1440.0).round() as u32;
    let mut a = z;
    if z >= 2299161.0 {
        let al = ((z - 1867216.25) / 36524.25).floor();
        a = z + 1.0 + al - (al / 4.0).floor();
    }
    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let dd = (365.25 * c).floor();
    let e = ((b - dd) / 30.6001).floor();
    let day = (b - dd - (30.6001 * e).floor()) as u32;
    let mut month = e as i32 - 1;
    if month > 12 { month -= 12; }
    let mut year = c as i32 - 4716;
    if month <= 2 { year += 1; }
    (year, month as u32, day, left / 60, left % 60)
}

/// A Julian date as "2026-03-20 14:46" (UT).
pub fn jd_text(j: f64) -> String {
    let (y, m, d, h, mi) = date_of(j);
    format!("{}-{:02}-{:02} {:02}:{:02}", y, m, d, h, mi)
}

/// A Julian date as "2026-03-20 15:46" on the clock of a time zone.
pub fn jd_local(j: f64, tz: f64) -> String {
    jd_text(j + tz / 24.0)
}

/// The day of the week: "Monday".
pub fn weekday(year: i32, month: u32, day: u32) -> &'static str {
    let i = ((jd(year, month, day) + 1.5).floor() as i64).rem_euclid(7) as usize;
    ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"][i]
}

/// Easter Sunday of a year, as "2026-04-05".
pub fn easter(year: i32) -> String {
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let h = (19 * a + b - b / 4 - (b - (b + 8) / 25 + 1) / 3 + 15) % 30;
    let l = (32 + 2 * (b % 4) + 2 * (c / 4) - h - c % 4) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let n = h + l - 7 * m + 114;
    format!("{}-{:02}-{:02}", year, n / 31, n % 31 + 1)
}

/// Seconds the even time of the planets (TT) runs ahead of clock time
/// (UT) in a year.
pub fn delta_t(year: f64) -> f64 {
    if (2005.0..2050.0).contains(&year) {
        let t = year - 2000.0;
        return 62.92 + 0.32217 * t + 0.005589 * t * t;
    }
    if (1986.0..2005.0).contains(&year) {
        let t = year - 2000.0;
        return 63.86 + 0.3345 * t - 0.060374 * t * t + 0.0017275 * t * t * t
            + 0.000651814 * t * t * t * t + 0.00002373599 * t * t * t * t * t;
    }
    if (1961.0..1986.0).contains(&year) {
        let t = year - 1975.0;
        return 45.45 + 1.067 * t - t * t / 260.0 - t * t * t / 718.0;
    }
    if (1941.0..1961.0).contains(&year) {
        let t = year - 1950.0;
        return 29.07 + 0.407 * t - t * t / 233.0 + t * t * t / 2547.0;
    }
    if (1920.0..1941.0).contains(&year) {
        let t = year - 1920.0;
        return 21.2 + 0.84493 * t - 0.0761 * t * t + 0.0020936 * t * t * t;
    }
    if (1900.0..1920.0).contains(&year) {
        let t = year - 1900.0;
        return -2.79 + 1.494119 * t - 0.0598939 * t * t + 0.0061966 * t * t * t
            - 0.000197 * t * t * t * t;
    }
    let u = (year - 1820.0) / 100.0;
    -20.0 + 32.0 * u * u
}

/// From the even time of the planets to clock time.
fn to_ut(jde: f64) -> f64 {
    jde - delta_t(2000.0 + (jde - 2451545.0) / 365.25) / 86400.0
}

/// `d`: days since 1999-12-31 0h UT, with the part of the day. `ut`: hours.
fn sky_d(d: f64, ut: f64, lat: f64, lon: f64, tz: f64) -> Sky {
    let t = d / 36525.0;
    let ecl = 23.439279444 - 46.815 / 3600.0 * t - 0.00059 / 3600.0 * t * t
        + 0.001813 / 3600.0 * t * t * t;
    let sw = 282.9404 + 4.70935e-5 * d;
    let se = 0.016709 - 1.151e-9 * d;
    let sm = 356.047 + 0.9856002585 * d;
    let ws = (sw + 360.0) % 360.0;
    let ms = sm % 360.0;
    let es = ms + (180.0 / PI) * se * rad(ms).sin() * (1.0 + se * rad(ms).cos());
    let x = rad(es).cos() - se;
    let y = rad(es).sin() * (1.0 - se * se).sqrt();
    let v = to_deg(y.atan2(x));
    let r = (x * x + y * y).sqrt();
    let tlon = (v + ws) % 360.0;
    let xs = r * rad(tlon).cos();
    let ys = r * rad(tlon).sin();
    let ye = ys * rad(ecl).cos();
    let ze = ys * rad(ecl).sin();
    let sun_ra = (to_deg(ye.atan2(xs)) + 360.0) % 360.0;
    let sun_dec = to_deg(ze.atan2((xs * xs + ye * ye).sqrt()));
    let ls = (ws + ms) % 360.0;
    let gmst0 = (ls + 180.0) / 15.0 % 24.0;
    Sky {
        lat, lon, tz, d, ecl, ls, xs, ys,
        sidtime: gmst0 + ut + lon / 15.0,
        sun_ra, sun_dec,
        sun_lon: turn(tlon),
        sun_dist: r,
        mj: (19.895 + 0.0830853001 * d) % 360.0,
        ms: (316.967 + 0.0334442282 * d) % 360.0,
    }
}

/// The sky over a place at an hour of Universal Time on a day.
pub fn sky_ut(year: i32, month: u32, day: u32, ut: f64, lat: f64, lon: f64, tz: f64) -> Sky {
    let (y, m, dd) = (year as i64, month as i64, day as i64);
    let d = (367 * y - 7 * (y + (m + 9) / 12) / 4 + 275 * m / 9 + dd - 730530) as f64 + ut / 24.0;
    sky_d(d, ut, lat, lon, tz)
}

/// The sky over a place on a day, at 0h UT.
pub fn sky(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Sky {
    sky_ut(year, month, day, 0.0, lat, lon, tz)
}

/// The sky over a place at a local clock hour (17.5 is half past five;
/// 25.0 is 01:00 the next day).
pub fn sky_at(year: i32, month: u32, day: u32, hour: f64, lat: f64, lon: f64, tz: f64) -> Sky {
    sky_ut(year, month, day, hour - tz, lat, lon, tz)
}

/// The sky over a place at a moment (a Julian date). For a place-free
/// question (a phase, a conjunction) give 0.0 for lat, lon and tz.
pub fn sky_jd(j: f64, lat: f64, lon: f64, tz: f64) -> Sky {
    let d = j - 2451543.5;
    sky_d(d, (d - d.floor()) * 24.0, lat, lon, tz)
}

/// The moment of a sky, as a Julian date.
pub fn jd_of(s: &Sky) -> f64 { s.d + 2451543.5 }

/// The local sidereal time of a sky, in degrees.
pub fn sidereal(s: &Sky) -> f64 {
    ((s.sidtime % 24.0 + 24.0) % 24.0) * 15.0
}

/// Minutes a sundial runs ahead of the clock (the equation of time).
pub fn equation_of_time(s: &Sky) -> f64 {
    swing(s.ls - s.sun_ra) * 4.0
}

/// The six orbit numbers of a body: n, i, w, a, e, m.
fn elements(name: &str, d: f64) -> [f64; 6] {
    match name {
        "mercury" => [48.3313 + 3.24587e-5 * d, 7.0047 + 5.0e-8 * d, 29.1241 + 1.01444e-5 * d, 0.387098, 0.205635 + 5.59e-10 * d, 168.6562 + 4.0923344368 * d],
        "venus" => [76.6799 + 2.4659e-5 * d, 3.3946 + 2.75e-8 * d, 54.891 + 1.38374e-5 * d, 0.72333, 0.006773 - 1.302e-9 * d, 48.0052 + 1.6021302244 * d],
        "mars" => [49.5574 + 2.11081e-5 * d, 1.8497 - 1.78e-8 * d, 286.5016 + 2.92961e-5 * d, 1.523688, 0.093405 + 2.516e-9 * d, 18.6021 + 0.5240207766 * d],
        "jupiter" => [100.4542 + 2.76854e-5 * d, 1.303 - 1.557e-7 * d, 273.8777 + 1.64505e-5 * d, 5.20256, 0.048498 + 4.469e-9 * d, 19.895 + 0.0830853001 * d],
        "saturn" => [113.6634 + 2.3898e-5 * d, 2.4886 - 1.081e-7 * d, 339.3939 + 2.97661e-5 * d, 9.55475, 0.055546 - 9.499e-9 * d, 316.967 + 0.0334442282 * d],
        "uranus" => [74.0005 + 1.3978e-5 * d, 0.7733 + 1.9e-8 * d, 96.6612 + 3.0565e-5 * d, 19.18171 - 1.55e-8 * d, 0.047318 + 7.45e-9 * d, 142.5905 + 0.011725806 * d],
        "neptune" => [131.7806 + 3.0173e-5 * d, 1.77 - 2.55e-7 * d, 272.8461 - 6.027e-6 * d, 30.05826 + 3.313e-8 * d, 0.008606 + 2.15e-9 * d, 260.2471 + 0.005995147 * d],
        _ => [0.0, 0.0, 282.9404 + 4.70935e-5 * d, 1.0, 0.016709 - 1.151e-9 * d, 356.047 + 0.9856002585 * d],
    }
}

/// The moon seen from the centre of the earth: ra, dec, distance in earth
/// radii, and its place along and above the ecliptic.
fn moon_centre(s: &Sky) -> [f64; 5] {
    let t = (s.d - 1.5) / 36525.0;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let lp = 218.3164477 + 481267.88123421 * t - 0.0015786 * t2 + t3 / 538841.0 - t4 / 65194000.0;
    let dm = 297.8501921 + 445267.1114034 * t - 0.0018819 * t2 + t3 / 545868.0 - t4 / 113065000.0;
    let m = 357.5291092 + 35999.0502909 * t - 0.0001536 * t2 + t3 / 24490000.0;
    let mp = 134.9633964 + 477198.8675055 * t + 0.0087414 * t2 + t3 / 69699.0 - t4 / 14712000.0;
    let f = 93.272095 + 483202.0175233 * t - 0.0036539 * t2 - t3 / 3526000.0 + t4 / 863310000.0;
    let a1 = 119.75 + 131.849 * t;
    let a2 = 53.09 + 479264.29 * t;
    let a3 = 313.45 + 481266.484 * t;
    let e = 1.0 - 0.002516 * t - 0.0000074 * t2;
    let (dr, mr, mpr, fr) = (rad(dm), rad(m), rad(mp), rad(f));
    let scale_of = |mi: i8| match mi.abs() { 0 => 1.0, 1 => e, _ => e * e };
    let mut sl = 0.0;
    let mut sr = 0.0;
    for &(cd, cm, cmp, cf, l, r) in super::MOON_LR {
        let arg = cd as f64 * dr + cm as f64 * mr + cmp as f64 * mpr + cf as f64 * fr;
        sl += l as f64 * scale_of(cm) * arg.sin();
        sr += r as f64 * scale_of(cm) * arg.cos();
    }
    let mut sb = 0.0;
    for &(cd, cm, cmp, cf, b) in super::MOON_B {
        let arg = cd as f64 * dr + cm as f64 * mr + cmp as f64 * mpr + cf as f64 * fr;
        sb += b as f64 * scale_of(cm) * arg.sin();
    }
    sl += 3958.0 * rad(a1).sin() + 1962.0 * rad(lp - f).sin() + 318.0 * rad(a2).sin();
    sb += -2235.0 * rad(lp).sin() + 382.0 * rad(a3).sin() + 175.0 * rad(a1 - f).sin()
        + 175.0 * rad(a1 + f).sin() + 127.0 * rad(lp - mp).sin() - 115.0 * rad(lp + mp).sin();
    let lam = rad(turn(lp + sl / 1000000.0));
    let bet = rad(sb / 1000000.0);
    let eps = rad(s.ecl);
    let mut ra = to_deg((lam.sin() * eps.cos() - bet.tan() * eps.sin()).atan2(lam.cos()));
    if ra < 0.0 { ra += 360.0; }
    let sd = clamp1(bet.sin() * eps.cos() + bet.cos() * eps.sin() * lam.sin());
    [ra, to_deg(sd.asin()), (385000.56 + sr / 1000.0) / 6378.14, to_deg(lam), to_deg(bet)]
}

/// From the centre of the earth to the observer. `c`: ra, dec, dist,
/// sun_dist, lon, lat, and the parallax in degrees.
fn observed(s: &Sky, c: [f64; 7]) -> Pos {
    let (ra, dec) = (c[0], c[1]);
    let u = (0.99664719 * rad(s.lat).tan()).atan();
    let rs = 0.99664719 * u.sin();
    let rc = u.cos();
    let sp = rad(c[6]).sin();
    let ha = rad(s.sidtime * 15.0 - ra);
    let ax = rad(dec).cos() * ha.sin();
    let bx = rad(dec).cos() * ha.cos() - rc * sp;
    let cx = rad(dec).sin() - rs * sp;
    let q = (ax * ax + bx * bx + cx * cx).sqrt();
    Pos {
        ra: turn(s.sidtime * 15.0 - to_deg(ax.atan2(bx))),
        dec: to_deg(clamp1(cx / q).asin()),
        dist: c[2], gra: ra, gdec: dec, lon: c[4], lat: c[5], sun_dist: c[3],
        tdist: c[2] * q,
    }
}

/// The eccentric anomaly, in degrees.
fn kepler(m: f64, e: f64) -> f64 {
    let mut e1 = m + (180.0 / PI) * e * rad(m).sin() * (1.0 + e * rad(m).cos());
    if e > 0.8 { e1 = 180.0; }
    for _ in 0..80 {
        let e0 = e1;
        e1 = e0 - (e0 - (180.0 / PI) * e * rad(e0).sin() - m) / (1.0 - e * rad(e0).cos());
        if (e1 - e0).abs() <= 0.00005 { break; }
    }
    e1
}

/// Round the sun: along the ecliptic, above it, and the distance.
fn helio(nb: f64, ib: f64, wb: f64, ab: f64, eb: f64, mb: f64) -> [f64; 3] {
    let e1 = kepler(turn(mb), eb);
    let x = ab * (rad(e1).cos() - eb);
    let y = ab * (1.0 - eb * eb).sqrt() * rad(e1).sin();
    let r = (x * x + y * y).sqrt();
    let v = (to_deg(y.atan2(x)) + 360.0) % 360.0;
    let xe = r * (rad(nb).cos() * rad(v + wb).cos() - rad(nb).sin() * rad(v + wb).sin() * rad(ib).cos());
    let ye = r * (rad(nb).sin() * rad(v + wb).cos() + rad(nb).cos() * rad(v + wb).sin() * rad(ib).cos());
    let ze = r * rad(v + wb).sin() * rad(ib).sin();
    [turn(to_deg(ye.atan2(xe))), to_deg(ze.atan2((xe * xe + ye * ye).sqrt())), (xe * xe + ye * ye + ze * ze).sqrt()]
}

/// From round the sun to round the earth: ra, dec, dist, sun_dist, lon,
/// lat, parallax.
fn from_earth(s: &Sky, lon: f64, lat: f64, rb: f64, is_sun: bool) -> [f64; 7] {
    let mut xh = rb * rad(lon).cos() * rad(lat).cos();
    let mut yh = rb * rad(lon).sin() * rad(lat).cos();
    let zh = rb * rad(lat).sin();
    if !is_sun {
        xh += s.xs;
        yh += s.ys;
    }
    let yq = yh * rad(s.ecl).cos() - zh * rad(s.ecl).sin();
    let zq = yh * rad(s.ecl).sin() + zh * rad(s.ecl).cos();
    let dist = (xh * xh + yq * yq + zq * zq).sqrt();
    let sd = if is_sun { 0.0 } else { rb };
    [turn(to_deg(yq.atan2(xh))), to_deg(zq.atan2((xh * xh + yq * yq).sqrt())), dist, sd,
     turn(to_deg(yh.atan2(xh))), to_deg(zh.atan2((xh * xh + yh * yh).sqrt())), (8.794 / 3600.0) / dist]
}

/// The nine body names, in the order [`bodies`] gives them.
pub const NAMES: [&str; 9] = ["sun", "moon", "mercury", "venus", "mars", "jupiter", "saturn", "uranus", "neptune"];

/// The nine body names, in the order [`bodies`] gives them.
pub fn names() -> &'static [&'static str] { &NAMES }

fn index_of(name: &str) -> Option<usize> {
    NAMES.iter().position(|n| *n == name)
}

/// Where a body is in a sky. A name that is no body gives the sun.
pub fn pos(s: &Sky, name: &str) -> Pos {
    if name == "moon" {
        let mc = moon_centre(s);
        return observed(s, [mc[0], mc[1], mc[2], s.sun_dist, mc[3], mc[4], to_deg((1.0 / mc[2]).asin())]);
    }
    let el = elements(name, s.d);
    let h = helio(el[0], el[1], (el[2] + 360.0) % 360.0, el[3], el[4], el[5]);
    let mut lon = h[0];
    let mut lat = h[1];
    let (mj, ms) = (s.mj, s.ms);
    if name == "jupiter" {
        lon = lon - 0.332 * rad(2.0 * mj - 5.0 * ms - 67.6).sin() - 0.056 * rad(2.0 * mj - 2.0 * ms + 21.0).sin()
            + 0.042 * rad(3.0 * mj - 5.0 * ms + 21.0).sin() - 0.036 * rad(mj - 2.0 * ms).sin()
            + 0.022 * rad(mj - ms).cos() + 0.023 * rad(2.0 * mj - 3.0 * ms + 52.0).sin()
            - 0.016 * rad(mj - 5.0 * ms - 69.0).sin();
    }
    if name == "saturn" {
        lon = lon + 0.812 * rad(2.0 * mj - 5.0 * ms - 67.6).sin() - 0.229 * rad(2.0 * mj - 4.0 * ms - 2.0).cos()
            + 0.119 * rad(mj - 2.0 * ms - 3.0).sin() + 0.046 * rad(2.0 * mj - 6.0 * ms - 69.0).sin()
            + 0.014 * rad(mj - 3.0 * ms + 32.0).sin();
        lat = lat - 0.02 * rad(2.0 * mj - 4.0 * ms - 2.0).cos() + 0.018 * rad(2.0 * mj - 6.0 * ms - 49.0).sin();
    }
    let is_sun = !matches!(index_of(name), Some(i) if i >= 1);
    observed(s, from_earth(s, lon, lat, h[2], is_sun))
}

/// Pluto's path, good to a few minutes of arc from 1800 to 2050.
pub fn pluto() -> Orbit {
    Orbit { name: "pluto", node: 110.30393684, incl: 17.14001206, peri: 113.76497945, a: 39.48211675,
            e: 0.2488273, m0: 14.86012204, epoch: 2451545.0, daily: 0.003975577 }
}

/// Where a body on a path round the sun is in a sky: `orbit_pos(&s,
/// &pluto())`, or a comet or an asteroid with numbers from a catalogue.
pub fn orbit_pos(s: &Sky, o: &Orbit) -> Pos {
    let n = if o.daily == 0.0 { 0.9856076686 / o.a.powf(1.5) } else { o.daily };
    let h = helio(o.node, o.incl, o.peri, o.a, o.e, o.m0 + n * (jd_of(s) - o.epoch));
    // The numbers are for the year 2000; the sky is for its own day.
    observed(s, from_earth(s, h[0] + 3.82394e-5 * (s.d - 1.5), h[1], h[2], false))
}

/// Height and direction of a place in the sky (ra, dec).
pub fn horizon(s: &Sky, ra: f64, dec: f64) -> Hor {
    let ha = rad(s.sidtime * 15.0 - ra);
    let x = ha.cos() * rad(dec).cos();
    let y = ha.sin() * rad(dec).cos();
    let z = rad(dec).sin();
    let xh = x * rad(s.lat).sin() - z * rad(s.lat).cos();
    let zh = x * rad(s.lat).cos() + z * rad(s.lat).sin();
    Hor { alt: to_deg(clamp1(zh).asin()), az: (to_deg(y.atan2(xh)) + 180.0) % 360.0 }
}

/// Height and direction of a body. The height is the true one; add
/// [`refraction`] for the height the eye sees.
pub fn where_is(s: &Sky, name: &str) -> Hor {
    let p = pos(s, name);
    horizon(s, p.ra, p.dec)
}

/// How much the air lifts a thing at a true height, in degrees (0.48 at
/// the horizon).
pub fn refraction(alt: f64) -> f64 {
    if alt < -1.0 { return 0.0; }
    1.02 / rad(alt + 10.3 / (alt + 5.11)).tan() / 60.0
}

/// How much air the light passes at a height: 1.0 straight up, 38 at the
/// horizon.
pub fn airmass(alt: f64) -> f64 {
    if alt < 0.0 { return 38.0; }
    1.0 / (rad(alt).sin() + 0.50572 * (alt + 6.07995).powf(-1.6364))
}

/// The height a body's centre has when it is seen to rise or set.
pub fn rise_height(name: &str) -> f64 {
    if name == "sun" || name == "moon" { -0.8333 } else { -0.5667 }
}

/// Rise, transit and set of a fixed place in the sky (a star), for the
/// day of a sky made with [`sky`]. `h0` is the height that counts as
/// rising, as a rule -0.5667.
pub fn times(s: &Sky, ra: f64, dec: f64, h0: f64) -> Times {
    let tr = ((ra - s.ls - s.lon) / 15.0 + 12.0 + s.tz + 24.0) % 24.0;
    let phi = rad(s.lat);
    let del = rad(dec);
    let den = phi.cos() * del.cos();
    let fixed_state = |rise: &str, set: &str| Times { rise: rise.into(), transit: hhmm(tr), set: set.into() };
    if den.abs() < 1.0e-12 {
        return fixed_state("never", "always");
    }
    let c = (rad(h0).sin() - phi.sin() * del.sin()) / den;
    if c < -1.0 { return fixed_state("always", "never"); }
    if c > 1.0 { return fixed_state("never", "always"); }
    let lha = to_deg(c.acos()) / 15.04107;
    Times { rise: hhmm((tr - lha + 24.0) % 24.0), transit: hhmm(tr), set: hhmm((tr + lha + 24.0) % 24.0) }
}

/// A body at a local hour: its height, and the sine and cosine of its
/// hour angle.
#[allow(clippy::too_many_arguments)]
fn probe(year: i32, month: u32, day: u32, hour: f64, name: &str, lat: f64, lon: f64, tz: f64) -> [f64; 3] {
    let s = sky_at(year, month, day, hour, lat, lon, tz);
    let p = pos(&s, name);
    let ha = rad(s.sidtime * 15.0 - p.ra);
    [horizon(&s, p.ra, p.dec).alt, ha.sin(), ha.cos()]
}

/// Rise, transit and set of a body on a local day, found by following the
/// body through the day. `h0` is the height that counts as rising:
/// [`rise_height`].
#[allow(clippy::too_many_arguments)]
pub fn day_times(year: i32, month: u32, day: u32, name: &str, h0: f64, lat: f64, lon: f64, tz: f64) -> Times {
    let at = |hour: f64| probe(year, month, day, hour, name, lat, lon, tz);
    // The hour between lo and hi where number k of probe passes off.
    let root = |lo: f64, hi: f64, k: usize, off: f64| -> f64 {
        let (mut a, mut b) = (lo, hi);
        let mut fa = at(a)[k] - off;
        for _ in 0..13 {
            let mid = (a + b) / 2.0;
            let fm = at(mid)[k] - off;
            if fa * fm > 0.0 { a = mid; fa = fm; } else { b = mid; }
        }
        (a + b) / 2.0
    };
    let mut rise: Option<String> = None;
    let mut set: Option<String> = None;
    let mut transit: Option<String> = None;
    let mut ups = 0;
    let mut last = at(0.0);
    if last[0] > h0 { ups = 1; }
    for i in 0..24 {
        let lo = i as f64;
        let now = at(lo + 1.0);
        if now[0] > h0 { ups += 1; }
        if rise.is_none() && last[0] <= h0 && now[0] > h0 {
            rise = Some(hhmm(root(lo, lo + 1.0, 0, h0)));
        }
        if set.is_none() && last[0] > h0 && now[0] <= h0 {
            set = Some(hhmm(root(lo, lo + 1.0, 0, h0)));
        }
        if transit.is_none() && last[1] < 0.0 && now[1] >= 0.0 && now[2] > 0.0 {
            let top = root(lo, lo + 1.0, 1, 0.0);
            transit = Some(hhmm(top));
            // A body that is up for less than an hour: both ends of the hour are below.
            if last[0] <= h0 && now[0] <= h0 && at(top)[0] > h0 {
                ups += 1;
                if rise.is_none() { rise = Some(hhmm(root(lo, top, 0, h0))); }
                if set.is_none() { set = Some(hhmm(root(top, lo + 1.0, 0, h0))); }
            }
        }
        last = now;
    }
    let state = |up: &str, down: &str| match ups { 25 => up, 0 => down, _ => "none" }.to_string();
    Times {
        rise: rise.unwrap_or_else(|| state("always", "never")),
        transit: transit.unwrap_or_else(|| "none".to_string()),
        set: set.unwrap_or_else(|| state("never", "always")),
    }
}

/// Rise, transit and set of a body on a day.
pub fn body_times(year: i32, month: u32, day: u32, name: &str, lat: f64, lon: f64, tz: f64) -> Times {
    day_times(year, month, day, name, rise_height(name), lat, lon, tz)
}

/// Sunrise, noon and sunset on a day.
pub fn sun_times(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Times {
    day_times(year, month, day, "sun", -0.8333, lat, lon, tz)
}

/// Moonrise and moonset on a day.
pub fn moon_times(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Times {
    day_times(year, month, day, "moon", -0.8333, lat, lon, tz)
}

/// Dawn and dusk: rise is when the sun climbs past `depth` degrees below
/// the horizon, set when it sinks past it. `depth` is 6.0 (civil), 12.0
/// (nautical) or 18.0 (astronomical).
pub fn twilight(year: i32, month: u32, day: u32, depth: f64, lat: f64, lon: f64, tz: f64) -> Times {
    day_times(year, month, day, "sun", -depth, lat, lon, tz)
}

/// Hours of daylight on a day: 24.0 in the midnight sun, 0.0 in the polar
/// night.
pub fn day_length(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> f64 {
    let t = sun_times(year, month, day, lat, lon, tz);
    match (t.rise.as_str(), t.set.as_str()) {
        ("always", _) => 24.0,
        ("never", _) => 0.0,
        ("none", set) => hours_of(set),
        (rise, "none") => 24.0 - hours_of(rise),
        (rise, set) => (hours_of(set) - hours_of(rise) + 24.0) % 24.0,
    }
}

/// The angle between two places in the sky.
pub fn separation(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> f64 {
    to_deg(clamp1(rad(dec1).sin() * rad(dec2).sin() + rad(dec1).cos() * rad(dec2).cos() * rad(ra1 - ra2).cos()).acos())
}

/// How far a body stands from the sun in the sky.
pub fn elongation(s: &Sky, name: &str) -> f64 {
    let p = pos(s, name);
    separation(p.gra, p.gdec, s.sun_ra, s.sun_dec)
}

/// Is a body east of the sun, so that it is seen in the evening?
/// (Otherwise: the morning.)
pub fn evening(s: &Sky, name: &str) -> bool {
    swing(pos(s, name).lon - s.sun_lon) > 0.0
}

/// The angle sun, body, earth: 0 when the body is fully lit, 180 when it
/// is dark.
pub fn phase_angle(s: &Sky, name: &str) -> f64 {
    if name == "sun" { return 0.0; }
    let p = pos(s, name);
    if name == "moon" {
        let el = rad(separation(p.gra, p.gdec, s.sun_ra, s.sun_dec));
        let md = p.dist * 6378.14 / 149597870.7;
        return to_deg((s.sun_dist * el.sin()).atan2(md - s.sun_dist * el.cos()));
    }
    to_deg(clamp1((p.sun_dist * p.sun_dist + p.dist * p.dist - s.sun_dist * s.sun_dist) / (2.0 * p.sun_dist * p.dist)).acos())
}

/// The lit part of a body's disc, 0 to 1.
pub fn lit_part(s: &Sky, name: &str) -> f64 {
    (1.0 + rad(phase_angle(s, name)).cos()) / 2.0
}

/// The lit part of the moon, 0 to 1.
pub fn moon_lit(s: &Sky) -> f64 { lit_part(s, "moon") }

/// How bright a body is, as a magnitude: lower is brighter (Venus -4,
/// Saturn about 1, the eye sees down to 6).
pub fn magnitude(s: &Sky, name: &str) -> f64 {
    if name == "sun" { return -26.74; }
    let p = pos(s, name);
    let fv = phase_angle(s, name);
    if name == "moon" {
        return 0.23 + 5.0 * (s.sun_dist * p.dist * 6378.14 / 149597870.7).log10() + 0.026 * fv + 4.0e-9 * fv.powf(4.0);
    }
    let base = 5.0 * (p.sun_dist * p.dist).log10();
    match name {
        "mercury" => -0.36 + base + 0.027 * fv + 2.2e-13 * fv.powf(6.0),
        "venus" => -4.34 + base + 0.013 * fv + 4.2e-7 * fv.powf(3.0),
        "mars" => -1.51 + base + 0.016 * fv,
        "jupiter" => -9.25 + base + 0.014 * fv,
        "saturn" => {
            let tilt = clamp1(sn(p.lat) * cs(28.06) - cs(p.lat) * sn(28.06) * sn(p.lon - 169.51 - 3.82e-5 * s.d)).asin();
            -9.0 + base + 0.044 * fv - 2.6 * tilt.sin().abs() + 1.2 * tilt.sin() * tilt.sin()
        }
        "uranus" => -7.15 + base + 0.001 * fv,
        _ => -6.9 + base + 0.001 * fv,
    }
}

/// How wide a body looks, in seconds of arc (the sun and the moon: about
/// 1900).
pub fn diameter(s: &Sky, name: &str) -> f64 {
    let p = pos(s, name);
    if name == "moon" {
        return 2.0 * to_deg((0.2725076 / p.tdist).asin()) * 3600.0;
    }
    match index_of(name) {
        Some(i) => [1919.26, 0.0, 6.74, 16.92, 9.36, 196.94, 165.6, 65.8, 62.2][i] / p.dist,
        None => 0.0,
    }
}

/// The moment (UT) of moon phase number `k`: a whole number is a new moon
/// (0.0 is the one of 2000-01-06), .25 a first quarter, .5 a full moon,
/// .75 a last quarter.
fn phase_time(k: f64) -> f64 {
    let w = ((k - k.floor()) * 4.0).round() as i64;
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let j = 2451550.09766 + 29.530588861 * k + 0.00015437 * t2 - 0.00000015 * t3 + 0.00000000073 * t4;
    let e = 1.0 - 0.002516 * t - 0.0000074 * t2;
    let m = 2.5534 + 29.1053567 * k - 0.0000014 * t2 - 0.00000011 * t3;
    let mp = 201.5643 + 385.81693528 * k + 0.0107582 * t2 + 0.00001238 * t3 - 0.000000058 * t4;
    let f = 160.7108 + 390.67050284 * k - 0.0016118 * t2 - 0.00000227 * t3 + 0.000000011 * t4;
    let om = 124.7746 - 1.56375588 * k + 0.0020672 * t2 + 0.00000215 * t3;
    let mut c;
    if w == 0 || w == 2 {
        let q = if w == 2 {
            [0.40614, 0.17302, 0.01614, 0.01043, 0.00734, 0.00515, 0.00209]
        } else {
            [0.4072, 0.17241, 0.01608, 0.01039, 0.00739, 0.00514, 0.00208]
        };
        c = 0.0 - q[0] * sn(mp) + q[1] * e * sn(m) + q[2] * sn(2.0 * mp) + q[3] * sn(2.0 * f) + q[4] * e * sn(mp - m) - q[5] * e * sn(mp + m) + q[6] * e * e * sn(2.0 * m);
        c = c - 0.00111 * sn(mp - 2.0 * f) - 0.00057 * sn(mp + 2.0 * f) + 0.00056 * e * sn(2.0 * mp + m) - 0.00042 * sn(3.0 * mp) + 0.00042 * e * sn(m + 2.0 * f) + 0.00038 * e * sn(m - 2.0 * f) - 0.00024 * e * sn(2.0 * mp - m) - 0.00017 * sn(om) - 0.00007 * sn(mp + 2.0 * m);
        c = c + 0.00004 * sn(2.0 * mp - 2.0 * f) + 0.00004 * sn(3.0 * m) + 0.00003 * sn(mp + m - 2.0 * f) + 0.00003 * sn(2.0 * mp + 2.0 * f) - 0.00003 * sn(mp + m + 2.0 * f) + 0.00003 * sn(mp - m + 2.0 * f) - 0.00002 * sn(mp - m - 2.0 * f) - 0.00002 * sn(3.0 * mp + m) + 0.00002 * sn(4.0 * mp);
    } else {
        c = 0.0 - 0.62801 * sn(mp) + 0.17172 * e * sn(m) - 0.01183 * e * sn(mp + m) + 0.00862 * sn(2.0 * mp) + 0.00804 * sn(2.0 * f) + 0.00454 * e * sn(mp - m) + 0.00204 * e * e * sn(2.0 * m) - 0.0018 * sn(mp - 2.0 * f) - 0.0007 * sn(mp + 2.0 * f);
        c = c - 0.0004 * sn(3.0 * mp) - 0.00034 * e * sn(2.0 * mp - m) + 0.00032 * e * sn(m + 2.0 * f) + 0.00032 * e * sn(m - 2.0 * f) - 0.00028 * e * e * sn(mp + 2.0 * m) + 0.00027 * e * sn(2.0 * mp + m) - 0.00017 * sn(om) - 0.00005 * sn(mp - m - 2.0 * f);
        c = c + 0.00004 * sn(2.0 * mp + 2.0 * f) - 0.00004 * sn(mp + m + 2.0 * f) + 0.00004 * sn(mp - 2.0 * m) + 0.00003 * sn(mp + m - 2.0 * f) + 0.00003 * sn(3.0 * m) + 0.00002 * sn(2.0 * mp - 2.0 * f) + 0.00002 * sn(mp - m + 2.0 * f) - 0.00002 * sn(3.0 * mp + m);
        let wq = 0.00306 - 0.00038 * e * cs(m) + 0.00026 * cs(mp) - 0.00002 * cs(mp - m) + 0.00002 * cs(mp + m) + 0.00002 * cs(2.0 * f);
        if w == 1 { c += wq; } else { c -= wq; }
    }
    let a0 = [299.77, 251.88, 251.83, 349.42, 84.66, 141.74, 207.14, 154.84, 34.52, 207.19, 291.34, 161.72, 239.56, 331.55];
    let a1 = [0.107408, 0.016321, 26.651886, 36.412478, 18.206239, 53.303771, 2.453732, 7.30686, 27.261239, 0.121824, 1.844379, 24.198154, 25.513099, 3.592518];
    let a2 = [325.0, 165.0, 164.0, 126.0, 110.0, 62.0, 60.0, 56.0, 47.0, 42.0, 40.0, 37.0, 35.0, 23.0];
    c += 0.000325 * sn(299.77 + 0.107408 * k - 0.009173 * t2);
    for i in 1..14 {
        c += a2[i] / 1000000.0 * sn(a0[i] + a1[i] * k);
    }
    to_ut(j + c)
}

/// The next moment (a Julian date, UT) the moon is new (`which` 0), at
/// first quarter (1), full (2) or at last quarter (3), at or after a
/// moment. Good to a minute.
pub fn next_phase(from: f64, which: u32) -> f64 {
    let w = which as f64 / 4.0;
    let mut k = ((from - 2451550.09766) / 29.530588861 - w).floor() - 1.0 + w;
    let mut j = phase_time(k);
    while j < from {
        k += 1.0;
        j = phase_time(k);
    }
    j
}

/// Days since the last new moon, at a moment.
pub fn moon_age(j: f64) -> f64 {
    let mut k = ((j - 2451550.09766) / 29.530588861).floor() + 1.0;
    let mut t = phase_time(k);
    while t > j {
        k -= 1.0;
        t = phase_time(k);
    }
    j - t
}

const PHASE_NAMES: [&str; 8] = ["New Moon", "Waxing Crescent", "First Quarter", "Waxing Gibbous", "Full Moon", "Waning Gibbous", "Last Quarter", "Waning Crescent"];
const PHASE_SYMBOLS: [&str; 8] = ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"];

/// The moon's phase on a day (UT). The day a new moon, a quarter or a
/// full moon falls on gets that name; the days between get the name of
/// what the moon is on its way to.
pub fn moon_phase(year: i32, month: u32, day: u32) -> Phase {
    let j0 = jd(year, month, day);
    let mut idx: Option<usize> = None;
    let q = ((j0 - 2451550.09766) / 29.530588861 * 4.0).floor() - 3.0;
    for i in 0..8 {
        let kq = q + i as f64;
        let t = phase_time(kq / 4.0);
        if t >= j0 && t < j0 + 1.0 {
            idx = Some(((kq % 4.0 + 4.0) % 4.0).round() as usize * 2);
        }
    }
    let s = sky_jd(j0 + 0.5, 0.0, 0.0, 0.0);
    let el = turn(pos(&s, "moon").lon - s.sun_lon);
    let idx = idx.unwrap_or(1 + 2 * (el / 90.0).floor() as usize).min(7);
    Phase {
        lit: (moon_lit(&s) * 10000.0).round() / 10000.0,
        age: (el / 360.0 * 10000.0).round() / 10000.0,
        index: idx,
        name: PHASE_NAMES[idx],
        symbol: PHASE_SYMBOLS[idx],
    }
}

/// Is this the day (UT) the moon turns new, first quarter, full or last
/// quarter?
pub fn notable_phase(year: i32, month: u32, day: u32) -> bool {
    moon_phase(year, month, day).index % 2 == 0
}

/// The days of a month with a notable phase, each as
/// "14 🌕 Full Moon 14:53" (UT).
pub fn phases_in_month(year: i32, month: u32) -> Vec<String> {
    let mut out = Vec::new();
    let j = jd(year, month, 1);
    let stop = j + days_in_month(year, month) as f64;
    let q = ((j - 2451550.09766) / 29.530588861 * 4.0).floor() - 3.0;
    for i in 0..12 {
        let kq = q + i as f64;
        let t = phase_time(kq / 4.0);
        if t >= j && t < stop {
            let idx = ((kq % 4.0 + 4.0) % 4.0).round() as usize * 2;
            out.push(format!("{} {} {} {}", date_of(t).2, PHASE_SYMBOLS[idx], PHASE_NAMES[idx], &jd_text(t)[11..16]));
        }
    }
    out
}

const NO_ECLIPSE: Eclipse = Eclipse { when: 0.0, kind: "", form: "", mag: 0.0, gamma: 0.0, partial: 0.0, total: 0.0 };

/// The eclipse at moon phase number `k` (a whole number: of the sun; .5:
/// of the moon), or one with form "" when the moon misses.
fn eclipse_of(k: f64) -> Eclipse {
    let lunar = k - k.floor() > 0.25;
    let t = k / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let f = 160.7108 + 390.67050284 * k - 0.0016118 * t2 - 0.00000227 * t3 + 0.000000011 * t4;
    if sn(f).abs() > 0.36 {
        return NO_ECLIPSE;
    }
    let j = 2451550.09766 + 29.530588861 * k + 0.00015437 * t2 - 0.00000015 * t3 + 0.00000000073 * t4;
    let e = 1.0 - 0.002516 * t - 0.0000074 * t2;
    let m = 2.5534 + 29.1053567 * k - 0.0000014 * t2 - 0.00000011 * t3;
    let mp = 201.5643 + 385.81693528 * k + 0.0107582 * t2 + 0.00001238 * t3 - 0.000000058 * t4;
    let om = 124.7746 - 1.56375588 * k + 0.0020672 * t2 + 0.00000215 * t3;
    let f1 = f - 0.02665 * sn(om);
    let a1 = 299.77 + 0.107408 * k - 0.009173 * t2;
    let mut c = if lunar {
        0.0 - 0.4065 * sn(mp) + 0.1727 * e * sn(m)
    } else {
        0.0 - 0.4075 * sn(mp) + 0.1721 * e * sn(m)
    };
    c = c + 0.0161 * sn(2.0 * mp) - 0.0097 * sn(2.0 * f1) + 0.0073 * e * sn(mp - m) - 0.005 * e * sn(mp + m) - 0.0023 * sn(mp - 2.0 * f1) + 0.0021 * e * sn(2.0 * m) + 0.0012 * sn(mp + 2.0 * f1) + 0.0006 * e * sn(2.0 * mp + m);
    c = c - 0.0004 * sn(3.0 * mp) - 0.0003 * e * sn(m + 2.0 * f1) + 0.0003 * sn(a1) - 0.0002 * e * sn(m - 2.0 * f1) - 0.0002 * e * sn(2.0 * mp - m) - 0.0002 * sn(om);
    let p = 0.207 * e * sn(m) + 0.0024 * e * sn(2.0 * m) - 0.0392 * sn(mp) + 0.0116 * sn(2.0 * mp) - 0.0073 * e * sn(mp + m) + 0.0067 * e * sn(mp - m) + 0.0118 * sn(2.0 * f1);
    let q = 5.2207 - 0.0048 * e * cs(m) + 0.002 * e * cs(2.0 * m) - 0.3299 * cs(mp) - 0.006 * e * cs(mp + m) + 0.0041 * e * cs(mp - m);
    let g = (p * cs(f1) + q * sn(f1)) * (1.0 - 0.0048 * cs(f1).abs());
    let u = 0.0059 + 0.0046 * e * cs(m) - 0.0182 * cs(mp) + 0.0004 * cs(2.0 * mp) - 0.0005 * cs(m + mp);
    let when = to_ut(j + c);
    let ga = g.abs();
    if lunar {
        let pen = (1.5573 + u - ga) / 0.545;
        let umb = (1.0128 - u - ga) / 0.545;
        if pen <= 0.0 {
            return NO_ECLIPSE;
        }
        if umb <= 0.0 {
            return Eclipse { when, kind: "lunar", form: "penumbral", mag: pen, gamma: g, partial: 0.0, total: 0.0 };
        }
        let n = 0.5458 + 0.04 * cs(mp);
        let pp = 1.0128 - u;
        let tt = 0.4678 - u;
        let part = 120.0 / n * (pp * pp - g * g).sqrt();
        if tt * tt - g * g <= 0.0 {
            return Eclipse { when, kind: "lunar", form: "partial", mag: umb, gamma: g, partial: part, total: 0.0 };
        }
        return Eclipse { when, kind: "lunar", form: "total", mag: umb, gamma: g, partial: part, total: 120.0 / n * (tt * tt - g * g).sqrt() };
    }
    if ga > 1.5433 + u {
        return NO_ECLIPSE;
    }
    let mut form = "partial";
    let mut size = (1.5433 + u - ga) / (0.5461 + 2.0 * u);
    if ga < 0.9972 + u.abs() {
        size = 0.5461 / (0.5461 + 2.0 * u);
        form = if u < 0.0 { "total" } else { "annular" };
        if ga < 0.9972 && (0.0..=0.0047).contains(&u) && u < 0.00464 * (1.0 - g * g).sqrt() {
            form = "hybrid";
        }
    }
    Eclipse { when, kind: "solar", form, mag: size, gamma: g, partial: 0.0, total: 0.0 }
}

/// The eclipses of a year (UT), of the sun and of the moon, in order.
pub fn eclipses(year: i32) -> Vec<Eclipse> {
    let k = ((year - 2000) as f64 * 12.3685).floor() - 1.0;
    (0..30)
        .map(|i| eclipse_of(k + i as f64 / 2.0))
        .filter(|e| !e.form.is_empty() && date_of(e.when).0 == year)
        .collect()
}

/// The next eclipse at or after a moment. `kind`: "solar", "lunar", or ""
/// for any. None when 40 months hold no such eclipse (never the case).
pub fn next_eclipse(from: f64, kind: &str) -> Option<Eclipse> {
    let k = ((from - 2451550.09766) / 29.530588861).floor() - 1.0;
    (0..80)
        .map(|i| eclipse_of(k + i as f64 / 2.0))
        .find(|e| !e.form.is_empty() && e.when >= from && (kind.is_empty() || kind == e.kind))
}

/// An eclipse in words: "Total lunar eclipse, magnitude 1.15".
pub fn eclipse_text(e: &Eclipse) -> String {
    if e.form.is_empty() {
        return "no eclipse".to_string();
    }
    format!("{}{} {} eclipse, magnitude {}", e.form[0..1].to_uppercase(), &e.form[1..], e.kind, fixed(e.mag, 2))
}

/// The angle between sun and moon as the observer sees them, and the half
/// width of each.
fn sun_moon(s: &Sky) -> [f64; 3] {
    let a = pos(s, "sun");
    let b = pos(s, "moon");
    [separation(a.ra, a.dec, b.ra, b.dec), 959.63 / 3600.0 / a.dist, to_deg((0.2725076 / b.tdist).asin())]
}

/// The part of the sun's width the moon covers right now at the place of
/// a sky: 0.0 when there is no eclipse there, 1.0 or more when it is
/// total.
pub fn sun_cover(s: &Sky) -> f64 {
    let v = sun_moon(s);
    ((v[1] + v[2] - v[0]) / (2.0 * v[1])).max(0.0)
}

/// The part of a disc of half width `r` that a disc of half width `b`
/// hides, centres `d` apart.
fn face_cover(d: f64, r: f64, b: f64) -> f64 {
    if d >= r + b {
        return 0.0;
    }
    if d <= (b - r).abs() {
        return if b >= r { 1.0 } else { b * b / (r * r) };
    }
    let a1 = r * r * clamp1((d * d + r * r - b * b) / (2.0 * d * r)).acos();
    let a2 = b * b * clamp1((d * d + b * b - r * r) / (2.0 * d * b)).acos();
    let a3 = 0.5 * ((r + b - d) * (d + r - b) * (d - r + b) * (d + r + b)).sqrt();
    (a1 + a2 - a3) / (PI * r * r)
}

/// A solar eclipse seen from a place: give the moment of the eclipse
/// (`Eclipse.when`) and the place. Looks three hours to each side; good
/// to a minute or two. None when the place sees nothing.
pub fn local_eclipse(when: f64, lat: f64, lon: f64, tz: f64) -> Option<Local> {
    let mut best = 0.0;
    let mut peak = when;
    let mut start = 0.0;
    let mut stop = 0.0;
    for i in 0..361 {
        let j = when + (i - 180) as f64 / 1440.0;
        let m = sun_cover(&sky_jd(j, lat, lon, tz));
        if m > 0.0 {
            if start == 0.0 { start = j; }
            stop = j;
        }
        if m > best {
            best = m;
            peak = j;
        }
    }
    if best == 0.0 {
        return None;
    }
    let s = sky_jd(peak, lat, lon, tz);
    let v = sun_moon(&s);
    Some(Local { start, peak, stop, mag: best, cover: face_cover(v[0], v[1], v[2]), alt: where_is(&s, "sun").alt })
}

#[derive(Clone, Copy, PartialEq)]
enum What { SunDist, Dist, Elong, Opp, Lon }

/// A number that changes with time, seen from the centre of the earth.
fn measure(what: What, j: f64, a: &str, b: &str) -> f64 {
    let s = sky_jd(j, 0.0, 0.0, 0.0);
    if what == What::SunDist {
        return s.sun_dist;
    }
    let p = pos(&s, a);
    match what {
        What::Dist => p.dist,
        What::Elong => separation(p.gra, p.gdec, s.sun_ra, s.sun_dec),
        What::Opp => swing(p.lon - s.sun_lon - 180.0),
        _ if b == "sun" => swing(p.lon - s.sun_lon),
        _ => swing(p.lon - pos(&s, b).lon),
    }
}

/// The first moment the number passes zero.
fn find_zero(what: What, a: &str, b: &str, from: f64, days: f64, step: f64) -> Option<f64> {
    let mut f0 = measure(what, from, a, b);
    let n = (days / step).ceil() as i64;
    for i in 0..n {
        let t1 = from + (i + 1) as f64 * step;
        let f1 = measure(what, t1, a, b);
        if f0 * f1 <= 0.0 && (f0 - f1).abs() < 180.0 {
            let (mut lo, mut hi, mut flo) = (t1 - step, t1, f0);
            for _ in 0..24 {
                let mid = (lo + hi) / 2.0;
                let fm = measure(what, mid, a, b);
                if flo * fm > 0.0 { lo = mid; flo = fm; } else { hi = mid; }
            }
            return Some((lo + hi) / 2.0);
        }
        f0 = f1;
    }
    None
}

/// The first moment the number is at its highest (`up`) or lowest.
fn find_peak(what: What, a: &str, from: f64, days: f64, step: f64, up: bool) -> Option<f64> {
    let sg = if up { 1.0 } else { -1.0 };
    let mut f0 = sg * measure(what, from, a, "");
    let mut f1 = sg * measure(what, from + step, a, "");
    let n = (days / step).ceil() as i64;
    for i in 1..n {
        let t2 = from + (i + 1) as f64 * step;
        let f2 = sg * measure(what, t2, a, "");
        if f1 > f0 && f1 >= f2 {
            let (mut lo, mut hi) = (t2 - 2.0 * step, t2);
            for _ in 0..40 {
                let m1 = lo + (hi - lo) / 3.0;
                let m2 = hi - (hi - lo) / 3.0;
                if sg * measure(what, m1, a, "") < sg * measure(what, m2, a, "") { lo = m1; } else { hi = m2; }
            }
            return Some((lo + hi) / 2.0);
        }
        f0 = f1;
        f1 = f2;
    }
    None
}

/// The next moment two bodies stand at the same place along the ecliptic,
/// within `days` from a moment; None when they do not. `b` may be "sun".
pub fn next_conjunction(a: &str, b: &str, from: f64, days: f64) -> Option<f64> {
    let step = if a == "moon" || b == "moon" { 0.25 } else { 1.0 };
    find_zero(What::Lon, a, b, from, days, step)
}

/// The next moment a planet stands opposite the sun (Mars to Neptune).
pub fn next_opposition(name: &str, from: f64) -> Option<f64> {
    find_zero(What::Opp, name, "", from, 800.0, 1.0)
}

/// The next moment Mercury or Venus stands farthest from the sun in the sky.
pub fn next_greatest_elongation(name: &str, from: f64) -> Option<f64> {
    find_peak(What::Elong, name, from, 620.0, 1.0, true)
}

/// The next moment the moon is nearest the earth.
pub fn next_perigee(from: f64) -> f64 {
    find_peak(What::Dist, "moon", from, 40.0, 0.5, false).unwrap_or(0.0)
}

/// The next moment the moon is farthest from the earth.
pub fn next_apogee(from: f64) -> f64 {
    find_peak(What::Dist, "moon", from, 40.0, 0.5, true).unwrap_or(0.0)
}

/// The next moment the earth is nearest the sun (early January).
pub fn next_perihelion(from: f64) -> f64 {
    find_peak(What::SunDist, "sun", from, 380.0, 1.0, false).unwrap_or(0.0)
}

/// The next moment the earth is farthest from the sun (early July).
pub fn next_aphelion(from: f64) -> f64 {
    find_peak(What::SunDist, "sun", from, 380.0, 1.0, true).unwrap_or(0.0)
}

/// When a season starts, as a Julian date (UT, good to a minute). `which`
/// is 0 for the March equinox, 1 the June solstice, 2 the September
/// equinox, 3 the December solstice.
pub fn season(year: i32, which: u32) -> f64 {
    let y = (year - 2000) as f64 / 1000.0;
    let j = match which {
        1 => 2451716.56767 + 365241.62603 * y + 0.00325 * y * y + 0.00888 * y * y * y - 0.0003 * y * y * y * y,
        2 => 2451810.21715 + 365242.01767 * y - 0.11575 * y * y + 0.00337 * y * y * y + 0.00078 * y * y * y * y,
        3 => 2451900.05952 + 365242.74049 * y - 0.06223 * y * y - 0.00823 * y * y * y + 0.00032 * y * y * y * y,
        _ => 2451623.80984 + 365242.37404 * y + 0.05169 * y * y - 0.00411 * y * y * y - 0.00057 * y * y * y * y,
    };
    let t = (j - 2451545.0) / 36525.0;
    let w = 35999.373 * t - 2.47;
    let dl = 1.0 + 0.0334 * rad(w).cos() + 0.0007 * rad(2.0 * w).cos();
    let a = [485.0, 203.0, 199.0, 182.0, 156.0, 136.0, 77.0, 74.0, 70.0, 58.0, 52.0, 50.0, 45.0, 44.0, 29.0, 18.0, 17.0, 16.0, 14.0, 12.0, 12.0, 12.0, 9.0, 8.0];
    let b = [324.96, 337.23, 342.08, 27.85, 73.14, 171.52, 222.54, 296.72, 243.58, 119.81, 297.17, 21.02, 247.54, 325.15, 60.93, 155.12, 288.79, 198.04, 199.76, 95.39, 287.11, 320.81, 227.73, 15.45];
    let c = [1934.136, 32964.467, 20.186, 445267.112, 45036.886, 22518.443, 65928.934, 3034.906, 9037.513, 33718.147, 150.678, 2281.226, 29929.562, 31555.956, 4443.417, 67555.328, 4562.452, 62894.029, 31436.921, 14577.848, 31931.756, 34777.259, 1222.114, 16859.074];
    let s: f64 = (0..24).map(|i| a[i] * rad(b[i] + c[i] * t).cos()).sum();
    to_ut(j + 0.00001 * s / dl)
}

/// A place moved from the grid of one moment to the grid of another (both
/// Julian dates), since the earth's axis slowly turns. The year 2000 is
/// 2451545.0.
pub fn precess(ra: f64, dec: f64, from: f64, to: f64) -> Spot {
    let tt = (from - 2451545.0) / 36525.0;
    let t = (to - from) / 36525.0;
    let zeta = ((2306.2181 + 1.39656 * tt - 0.000139 * tt * tt) * t + (0.30188 - 0.000344 * tt) * t * t + 0.017998 * t * t * t) / 3600.0;
    let z = ((2306.2181 + 1.39656 * tt - 0.000139 * tt * tt) * t + (1.09468 + 0.000066 * tt) * t * t + 0.018203 * t * t * t) / 3600.0;
    let th = ((2004.3109 - 0.8533 * tt - 0.000217 * tt * tt) * t - (0.42665 + 0.000217 * tt) * t * t - 0.041833 * t * t * t) / 3600.0;
    let a = cs(dec) * sn(ra + zeta);
    let b = cs(th) * cs(dec) * cs(ra + zeta) - sn(th) * sn(dec);
    let c = sn(th) * cs(dec) * cs(ra + zeta) + cs(th) * sn(dec);
    Spot { ra: turn(to_deg(a.atan2(b)) + z), dec: to_deg(clamp1(c).asin()) }
}

/// From ra and dec to along and above the ecliptic. `ecl` is the tilt of
/// the earth: `s.ecl`.
pub fn to_ecliptic(ra: f64, dec: f64, ecl: f64) -> LonLat {
    LonLat {
        lon: turn(to_deg((sn(ra) * cs(ecl) + rad(dec).tan() * sn(ecl)).atan2(cs(ra)))),
        lat: to_deg(clamp1(sn(dec) * cs(ecl) - cs(dec) * sn(ecl) * sn(ra)).asin()),
    }
}

/// From along and above the ecliptic to ra and dec.
pub fn to_equatorial(lon: f64, lat: f64, ecl: f64) -> Spot {
    Spot {
        ra: turn(to_deg((sn(lon) * cs(ecl) - rad(lat).tan() * sn(ecl)).atan2(cs(lon)))),
        dec: to_deg(clamp1(sn(lat) * cs(ecl) + cs(lat) * sn(ecl) * sn(lon)).asin()),
    }
}

/// From ra and dec (for the year 2000) to along and above the Milky Way.
pub fn galactic(ra: f64, dec: f64) -> LonLat {
    let b = clamp1(sn(dec) * sn(27.12825) + cs(dec) * cs(27.12825) * cs(ra - 192.85948)).asin();
    let l = 122.93192 - to_deg((cs(dec) * sn(ra - 192.85948)).atan2(sn(dec) * cs(27.12825) - cs(dec) * sn(27.12825) * cs(ra - 192.85948)));
    LonLat { lon: turn(l), lat: to_deg(b) }
}

/// The names of the 88 constellations.
pub fn constellations() -> &'static [&'static str] { &CON_NAMES }

/// The short form of a constellation's name: "Ori" for "Orion". No such
/// constellation: "".
pub fn con_short(name: &str) -> &'static str {
    CON_NAMES.iter().position(|n| *n == name).map_or("", |i| CON_ABBRS[i])
}

/// The constellation a place in the sky is in, by the official borders.
/// `ra` and `dec` are for the moment `j` (for a [`Pos`]: `p.gra`,
/// `p.gdec` and `jd_of(&s)`; for a [`Star`]: 2451545.0).
pub fn constellation(ra: f64, dec: f64, j: f64) -> &'static str {
    let p = precess(ra, dec, j, 2405889.25855);
    let h = p.ra / 15.0;
    BOUNDS.iter()
        .find(|b| p.dec >= b.2 && h >= b.0 && h < b.1)
        .map_or("", |b| CON_NAMES[b.3 as usize])
}

fn star_no(i: usize) -> Star {
    let t = &STARS[i];
    Star { name: t.0, con: t.1, spectral: t.2, ra: t.3, dec: t.4, mag: t.5, ly: t.6 }
}

fn star_index(name: &str) -> Option<usize> {
    let want = name.to_lowercase();
    STARS.iter().position(|t| t.0.to_lowercase() == want)
}

/// The bright and named stars, brightest first.
pub fn stars() -> Vec<Star> {
    (0..STARS.len()).map(star_no).collect()
}

/// One star by name ("Sirius", any case).
pub fn star(name: &str) -> Option<Star> {
    star_index(name).map(star_no)
}

fn star_place(s: &Sky, i: usize) -> Pos {
    let t = &STARS[i];
    let yrs = (jd_of(s) - 2451545.0) / 365.25;
    let dec0 = t.4 + t.8 / 3600000.0 * yrs;
    let ra0 = t.3 + t.7 / 3600000.0 / cs(t.4) * yrs;
    let p = precess(ra0, dec0, 2451545.0, jd_of(s));
    let e = to_ecliptic(p.ra, p.dec, s.ecl);
    Pos { ra: p.ra, dec: p.dec, dist: 0.0, gra: p.ra, gdec: p.dec, lon: e.lon, lat: e.lat, sun_dist: 0.0, tdist: 0.0 }
}

/// Where a star is in a sky, moved to the sky's own day. Its height:
/// `horizon(&s, p.ra, p.dec)`. Its rise and set: `times(&s, p.ra, p.dec,
/// -0.5667)`.
pub fn star_pos(s: &Sky, name: &str) -> Option<Pos> {
    star_index(name).map(|i| star_place(s, i))
}

/// The stars that stand higher than `min_alt` degrees in a sky, brightest
/// first.
pub fn stars_up(s: &Sky, min_alt: f64) -> Vec<Star> {
    (0..STARS.len())
        .filter(|&i| {
            let p = star_place(s, i);
            horizon(s, p.ra, p.dec).alt > min_alt
        })
        .map(star_no)
        .collect()
}

/// The eleven big meteor showers of the year.
pub const SHOWERS: [Shower; 11] = [
    Shower { name: "Quadrantids", month: 1, day: 3, rate: 110, ra: 230.0, dec: 49.0 },
    Shower { name: "Lyrids", month: 4, day: 22, rate: 18, ra: 271.0, dec: 34.0 },
    Shower { name: "Eta Aquariids", month: 5, day: 6, rate: 50, ra: 338.0, dec: -1.0 },
    Shower { name: "Delta Aquariids", month: 7, day: 30, rate: 25, ra: 340.0, dec: -16.0 },
    Shower { name: "Perseids", month: 8, day: 12, rate: 100, ra: 48.0, dec: 58.0 },
    Shower { name: "Draconids", month: 10, day: 8, rate: 10, ra: 262.0, dec: 54.0 },
    Shower { name: "Orionids", month: 10, day: 21, rate: 20, ra: 95.0, dec: 16.0 },
    Shower { name: "Taurids", month: 11, day: 5, rate: 5, ra: 52.0, dec: 15.0 },
    Shower { name: "Leonids", month: 11, day: 17, rate: 15, ra: 152.0, dec: 22.0 },
    Shower { name: "Geminids", month: 12, day: 14, rate: 150, ra: 112.0, dec: 33.0 },
    Shower { name: "Ursids", month: 12, day: 22, rate: 10, ra: 217.0, dec: 76.0 },
];

/// The eleven big meteor showers of the year.
pub fn showers() -> &'static [Shower] { &SHOWERS }

/// The sign of a body, as text and not as a wide picture.
pub fn sign(name: &str) -> String {
    match index_of(name) {
        Some(i) => format!("{}\u{fe0e}", ["☀", "☾", "☿", "♀", "♂", "♃", "♄", "♅", "♆"][i]),
        None => "?".to_string(),
    }
}

/// The colour number (0 to 255) a body is drawn in on a terminal.
pub fn colour(name: &str) -> u8 {
    index_of(name).map_or(255, |i| [220, 248, 137, 216, 196, 208, 229, 117, 33][i])
}

/// "sun" as "Sun".
pub fn title(name: &str) -> String {
    let mut c = name.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn body_of(year: i32, month: u32, day: u32, n: &str, lat: f64, lon: f64, tz: f64) -> Body {
    let s = sky(year, month, day, lat, lon, tz);
    let p = pos(&s, n);
    let t = body_times(year, month, day, n, lat, lon, tz);
    Body {
        name: n.to_string(), ra: p.ra, dec: p.dec, dist: p.dist,
        rise: t.rise, transit: t.transit, set: t.set,
        mag: magnitude(&s, n), lit: lit_part(&s, n), size: diameter(&s, n),
        con: constellation(p.gra, p.gdec, jd_of(&s)).to_string(),
    }
}

/// Every body for one day: where it is, when it rises, transits and sets,
/// how bright.
pub fn bodies(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Vec<Body> {
    NAMES.iter().map(|n| body_of(year, month, day, n, lat, lon, tz)).collect()
}

/// Is a body up at a local clock hour, going by its rise and set?
pub fn is_up(b: &Body, hour: f64) -> bool {
    match (b.rise.as_str(), b.set.as_str()) {
        ("always", _) => true,
        ("never", _) => false,
        ("none", set) => hour <= hours_of(set),
        (rise, "none") => hour >= hours_of(rise),
        (rise, set) => {
            let (r, st) = (hours_of(rise), hours_of(set));
            if r > st { hour >= r || hour <= st } else { hour >= r && hour <= st }
        }
    }
}

/// The planets (Mercury to Saturn) to see in the night that starts on a
/// day: more than 5 degrees up at some full hour while the sun is more
/// than 6 degrees down.
pub fn visible_planets(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Vec<Body> {
    let dark: Vec<Sky> = (12..37)
        .map(|i| sky_at(year, month, day, i as f64, lat, lon, tz))
        .filter(|s| horizon(s, s.sun_ra, s.sun_dec).alt < -6.0)
        .collect();
    ["mercury", "venus", "mars", "jupiter", "saturn"].iter()
        .filter(|n| dark.iter().any(|s| where_is(s, n).alt > 5.0))
        .map(|n| body_of(year, month, day, n, lat, lon, tz))
        .collect()
}

/// A table of bodies as lines of text: name, RA, Dec, distance,
/// magnitude, rise, transit, set, constellation.
pub fn table(list: &[Body]) -> Vec<String> {
    let five = |s: &str| format!("{:<5}", s.chars().take(5).collect::<String>());
    let mut out = vec![
        "Body      │ RA      │ Dec      │ Dist    │ Mag   │ Rise  │ Trans │ Set   │ In".to_string(),
        "──────────┼─────────┼──────────┼─────────┼───────┼───────┼───────┼───────┼─────────────".to_string(),
    ];
    for b in list {
        out.push(format!("{:<9} │ {} │ {} │ {:>7} │ {:>5} │ {} │ {} │ {} │ {}",
            title(&b.name), ra_text(b.ra), dec_text(b.dec), fixed(b.dist, 4), fixed(b.mag, 1),
            five(&b.rise), five(&b.transit), five(&b.set), b.con));
    }
    out
}

/// The constellations high in the evening sky in a month (the south gets
/// the other half-year).
pub fn overhead(month: u32, lat: f64) -> Vec<&'static str> {
    let m = if lat < 0.0 { (month + 5) % 12 + 1 } else { month };
    let all = ["Orion, Taurus, Auriga, Gemini", "Orion, Canis Major, Gemini, Auriga", "Leo, Cancer, Gemini, Hydra", "Leo, Virgo, Ursa Major", "Virgo, Boötes, Coma Berenices", "Boötes, Hercules, Corona Borealis", "Lyra, Cygnus, Hercules, Scorpius", "Cygnus, Lyra, Aquila, Sagittarius", "Pegasus, Andromeda, Cygnus, Capricornus", "Pegasus, Andromeda, Pisces, Aquarius", "Andromeda, Triangulum, Aries, Perseus", "Taurus, Orion, Perseus, Auriga"];
    if !(1..=12).contains(&m) {
        return Vec::new();
    }
    all[m as usize - 1].split(", ").collect()
}

const SEASON_NAMES: [&str; 4] = ["☀ March equinox", "☀ June solstice", "☀ September equinox", "☀ December solstice"];

/// What a day (UT) is known for: a notable moon phase, an eclipse, a
/// season's start, a meteor shower's peak.
pub fn events(year: i32, month: u32, day: u32) -> Vec<String> {
    let mut out = Vec::new();
    let stamp = format!("{}-{:02}-{:02}", year, month, day);
    for l in phases_in_month(year, month) {
        if let Some((d, rest)) = l.split_once(' ') {
            if d == day.to_string() {
                out.push(format!("{} UT", rest));
            }
        }
    }
    if let Some(e) = next_eclipse(jd(year, month, day), "") {
        let when = jd_text(e.when);
        if when[0..10] == stamp {
            out.push(format!("{} {} UT", eclipse_text(&e), &when[11..16]));
        }
    }
    for w in 0..4 {
        let when = jd_text(season(year, w));
        if when[0..10] == stamp {
            out.push(format!("{} {} UT", SEASON_NAMES[w as usize], &when[11..16]));
        }
    }
    for sh in SHOWERS.iter().filter(|sh| sh.month == month && sh.day == day) {
        out.push(format!("☄ {} peak, up to {} an hour", sh.name, sh.rate));
    }
    out
}

/// Tonight in a few lines: the moon, the planets to see, what is
/// overhead, the day's events.
pub fn tonight(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Vec<String> {
    let mut out = Vec::new();
    let mp = moon_phase(year, month, day);
    let mt = moon_times(year, month, day, lat, lon, tz);
    let mut line = format!("Moon: {} {} {}%", mp.symbol, mp.name, (mp.lit * 100.0).round() as i64);
    if mt.rise.len() == 5 { line += &format!(", rises {}", mt.rise); }
    if mt.set.len() == 5 { line += &format!(", sets {}", mt.set); }
    out.push(line);
    let parts: Vec<String> = visible_planets(year, month, day, lat, lon, tz).iter()
        .map(|p| format!("{} {} {} in {}", sign(&p.name), title(&p.name), fixed(p.mag, 1), p.con))
        .collect();
    if parts.is_empty() {
        out.push("Planets: none in the dark sky".to_string());
    } else {
        out.push(format!("Planets: {}", parts.join(", ")));
    }
    out.push(format!("Overhead: {}", overhead(month, lat).join(", ")));
    for e in events(year, month, day) {
        out.push(format!("Today: {}", e));
    }
    out
}

/// All that happens in the sky in a year, in order (UT): moon phases,
/// eclipses, seasons, meteor showers, oppositions, Mercury and Venus
/// farthest from the sun, the moon nearest and farthest, the earth
/// nearest and farthest from the sun. Print a line as
/// `format!("{} {}", jd_text(e.when), e.text)`.
pub fn almanac(year: i32) -> Vec<Event> {
    let mut raw: Vec<Event> = Vec::new();
    let a = jd(year, 1, 1);
    let b = jd(year + 1, 1, 1);
    for w in 0..4 {
        raw.push(Event { when: season(year, w), text: SEASON_NAMES[w as usize].to_string() });
    }
    let q = ((a - 2451550.09766) / 29.530588861 * 4.0).floor() - 3.0;
    for i in 0..60 {
        let kq = q + i as f64;
        let t = phase_time(kq / 4.0);
        if t >= a && t < b {
            let idx = ((kq % 4.0 + 4.0) % 4.0).round() as usize * 2;
            raw.push(Event { when: t, text: format!("{} {}", PHASE_SYMBOLS[idx], PHASE_NAMES[idx]) });
        }
    }
    for e in eclipses(year) {
        raw.push(Event { when: e.when, text: eclipse_text(&e) });
    }
    for sh in &SHOWERS {
        raw.push(Event { when: jd(year, sh.month, sh.day) + 0.5, text: format!("☄ {} peak, up to {} an hour", sh.name, sh.rate) });
    }
    for n in ["mars", "jupiter", "saturn", "uranus", "neptune"] {
        if let Some(t) = find_zero(What::Opp, n, "", a, b - a, 1.0).filter(|t| *t < b) {
            raw.push(Event { when: t, text: format!("{} {} opposite the sun, magnitude {}", sign(n), title(n), fixed(magnitude(&sky_jd(t, 0.0, 0.0, 0.0), n), 1)) });
        }
    }
    for n in ["mercury", "venus"] {
        let mut t = a;
        while b - t > 3.0 {
            match find_peak(What::Elong, n, t, b - t, 1.0, true).filter(|g| *g < b) {
                Some(g) => {
                    let s = sky_jd(g, 0.0, 0.0, 0.0);
                    let side = if evening(&s, n) { "evening" } else { "morning" };
                    raw.push(Event { when: g, text: format!("{} {} farthest from the sun, {}° in the {} sky", sign(n), title(n), fixed(elongation(&s, n), 0), side) });
                    t = g + 20.0;
                }
                None => break,
            }
        }
    }
    for (up, word) in [(false, "nearest"), (true, "farthest")] {
        let mut t = a;
        while t < b {
            match find_peak(What::Dist, "moon", t, 40.0, 0.5, up).filter(|g| *g < b) {
                Some(g) => {
                    raw.push(Event { when: g, text: format!("☾ Moon {}, {} km", word, fixed(measure(What::Dist, g, "moon", "") * 6378.14, 0)) });
                    t = g + 20.0;
                }
                None => break,
            }
        }
    }
    let ph = next_perihelion(a - 5.0);
    if ph >= a && ph < b {
        raw.push(Event { when: ph, text: "Earth nearest the sun".to_string() });
    }
    let ah = next_aphelion(a);
    if ah > 0.0 && ah < b {
        raw.push(Event { when: ah, text: "Earth farthest from the sun".to_string() });
    }
    raw.sort_by(|x, y| x.when.total_cmp(&y.when));
    raw
}

/// Jupiter's four big moons at a moment (a Julian date): Io, Europa,
/// Ganymede, Callisto. Good to a tenth of the planet's width.
pub fn jupiter_moons(j: f64) -> [JMoon; 4] {
    let d = j + delta_t(2000.0 + (j - 2451545.0) / 365.25) / 86400.0 - 2451545.0;
    let v = 172.74 + 0.00111588 * d;
    let m = 357.529 + 0.9856003 * d;
    let n = 20.02 + 0.0830853 * d + 0.329 * sn(v);
    let jj = 66.115 + 0.9025179 * d - 0.329 * sn(v);
    let a = 1.915 * sn(m) + 0.02 * sn(2.0 * m);
    let b = 5.555 * sn(n) + 0.168 * sn(2.0 * n);
    let k = jj + a - b;
    let re = 1.00014 - 0.01671 * cs(m) - 0.00014 * cs(2.0 * m);
    let r = 5.20872 - 0.25208 * cs(n) - 0.00611 * cs(2.0 * n);
    let dist = (r * r + re * re - 2.0 * r * re * cs(k)).sqrt();
    let psi = to_deg(clamp1(re / dist * sn(k)).asin());
    let lam = 34.35 + 0.083091 * d + 0.329 * sn(v) + b;
    let ds = 3.12 * sn(lam + 42.8);
    let de = ds - 2.22 * sn(psi) * cs(lam + 22.0) - 1.3 * (r - dist) / dist * sn(lam - 100.5);
    let t = d - dist / 173.0;
    let u1 = 163.8069 + 203.4058646 * t + psi - b;
    let u2 = 358.414 + 101.2916335 * t + psi - b;
    let u3 = 5.7176 + 50.234518 * t + psi - b;
    let u4 = 224.8092 + 21.48798 * t + psi - b;
    let g = 331.18 + 50.310482 * t;
    let h = 87.45 + 21.569231 * t;
    let us = [u1 + 0.473 * sn(2.0 * (u1 - u2)), u2 + 1.065 * sn(2.0 * (u2 - u3)), u3 + 0.165 * sn(g), u4 + 0.843 * sn(h)];
    let rs = [5.9057 - 0.0244 * cs(2.0 * (u1 - u2)), 9.3966 - 0.0882 * cs(2.0 * (u2 - u3)), 14.9883 - 0.0216 * cs(g), 26.3627 - 0.1939 * cs(h)];
    let names = ["Io", "Europa", "Ganymede", "Callisto"];
    std::array::from_fn(|i| {
        let x = rs[i] * sn(us[i]);
        let y = -rs[i] * cs(us[i]) * sn(de);
        let state = if x * x + y * y * 1.148 >= 1.0 { "" } else if cs(us[i]) > 0.0 { "transit" } else { "hidden" };
        JMoon { name: names[i], x, y, state }
    })
}

/// Jupiter and its moons as one line of text, as a telescope with north
/// up shows them: east to the left. O is the planet; I, E, G and C are
/// the moons. A moon in front of the planet or behind it is not drawn.
/// `width` is the number of characters (61 is a good one).
pub fn moons_line(j: f64, width: usize) -> String {
    let mid = (width / 2) as i64;
    let mut cells = vec![' '; width];
    for mo in jupiter_moons(j) {
        let col = mid + (mo.x / 27.0 * mid as f64).round() as i64;
        if mo.state.is_empty() && col >= 0 && (col as usize) < width {
            cells[col as usize] = mo.name.chars().next().unwrap_or(' ');
        }
    }
    if width > 0 {
        cells[mid as usize] = 'O';
    }
    cells.into_iter().collect()
}

/// Saturn's rings at a moment (a Julian date).
pub fn saturn_rings(j: f64) -> Rings {
    let s = sky_jd(j, 0.0, 0.0, 0.0);
    let p = pos(&s, "saturn");
    let e = to_ecliptic(p.gra, p.gdec, s.ecl);
    let t = (j - 2451545.0) / 36525.0;
    let i = 28.075216 - 0.012998 * t + 0.000004 * t * t;
    let om = 169.50847 + 1.394681 * t + 0.000412 * t * t;
    let tilt = to_deg(clamp1(sn(i) * cs(e.lat) * sn(e.lon - om) - cs(i) * sn(e.lat)).asin());
    let x = p.dist * cs(e.lat) * cs(e.lon) - s.xs;
    let y = p.dist * cs(e.lat) * sn(e.lon) - s.ys;
    let z = p.dist * sn(e.lat);
    let hl = to_deg(y.atan2(x));
    let hb = to_deg(z.atan2((x * x + y * y).sqrt()));
    let sun_tilt = to_deg(clamp1(sn(i) * cs(hb) * sn(hl - om) - cs(i) * sn(hb)).asin());
    let pole = to_equatorial(om - 90.0, 90.0 - i, s.ecl);
    let angle = to_deg((cs(pole.dec) * sn(pole.ra - p.gra)).atan2(sn(pole.dec) * cs(p.gdec) - cs(pole.dec) * sn(p.gdec) * cs(pole.ra - p.gra)));
    let wide = 375.35 / p.dist;
    Rings { tilt, sun_tilt, wide, high: wide * sn(tilt).abs(), angle }
}

// The same ten tests as Simpler's lib/astro.smplr. The numbers were
// checked against the XEphem library (PyEphem 4.2), which shares no code
// with this file.
#[cfg(test)]
mod tests {
    use super::*;

    const OSLO: (f64, f64) = (59.9139, 10.7522);
    const TROMSO: (f64, f64) = (69.6492, 18.9553);

    fn near(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() <= tol }
    fn oslo(year: i32, month: u32, day: u32) -> Sky { sky(year, month, day, OSLO.0, OSLO.1, 1.0) }
    fn day(j: Option<f64>) -> String { jd_text(j.unwrap())[..10].to_string() }

    #[test]
    fn dates() {
        assert_eq!(jd(2000, 1, 1), 2451544.5);
        assert_eq!(jd_text(jd_at(2026, 11, 16, 9, 30, 0)), "2026-11-16 09:30");
        assert_eq!(jd_local(jd_at(2026, 11, 16, 23, 30, 0), 1.0), "2026-11-17 00:30");
        assert_eq!(date_of(jd_at(2026, 11, 16, 9, 30, 0)).3, 9);
        assert!(days_in_month(2024, 2) == 29 && days_in_month(2100, 2) == 28);
        assert!(day_of_year(2026, 11, 16) == 320 && weekday(2026, 11, 16) == "Monday");
        assert!(easter(2026) == "2026-04-05" && easter(2027) == "2027-03-28");
        assert!(hhmm(7.999) == "08:00" && hhmm(-0.5) == "23:30");
        assert!(hours_of("06:30") == 6.5 && hours_of("never") == -1.0);
        assert!(ra_text(77.9) == "05h 12m" && dec_text(-19.08) == "-19° 05′");
        assert!(compass(200.0) == "SSW" && fixed(3.14159, 2) == "3.14");
    }

    #[test]
    fn positions() {
        let s = oslo(2026, 11, 16);
        let sun = pos(&s, "sun");
        assert!(near(sun.ra, 231.2561, 0.001) && near(sun.dec, -18.6816, 0.001) && near(sun.dist, 0.9891, 0.0001));
        let moon = pos(&s, "moon");
        assert!(near(moon.ra, 309.5851, 0.001) && near(moon.dec, -20.6358, 0.001) && near(moon.dist, 63.1437, 0.001));
        let sat = pos(&s, "saturn");
        assert!(near(sat.ra, 8.8257, 0.001) && near(sat.dec, 0.9333, 0.001) && near(sat.dist, 8.7202, 0.001));
        let nep = pos(&s, "neptune");
        assert!(near(nep.ra, 2.2173, 0.001) && near(nep.dec, -0.5729, 0.001));
        let far = orbit_pos(&s, &pluto());
        assert!(near(far.ra, 306.6589, 0.001) && near(far.dec, -23.5575, 0.001) && near(far.dist, 35.9755, 0.001));
        assert!(near(equation_of_time(&s), 15.33, 0.01) && near(sidereal(&s), 65.84, 0.01));
        assert!(jd_of(&s) == 2461360.5 && near(delta_t(2026.0), 75.0, 1.0));
    }

    #[test]
    fn rise_and_set() {
        let t = sun_times(2026, 11, 16, OSLO.0, OSLO.1, 1.0);
        assert!(t.rise == "08:17" && t.transit == "12:02" && t.set == "15:46");
        let m = moon_times(2026, 11, 16, OSLO.0, OSLO.1, 1.0);
        assert!(m.rise == "13:55" && m.transit == "17:47" && m.set == "21:53");
        let u = body_times(2026, 11, 16, "uranus", OSLO.0, OSLO.1, 1.0);
        assert!(u.rise == "15:52" && u.transit == "00:45" && u.set == "09:34");
        let dusk = twilight(2026, 11, 16, 6.0, OSLO.0, OSLO.1, 1.0);
        assert!(dusk.rise == "07:28" && dusk.set == "16:35");
        assert!(near(day_length(2026, 11, 16, OSLO.0, OSLO.1, 1.0), 7.48, 0.02));
        // Tromsø: midnight sun and polar night.
        assert_eq!(day_length(2026, 6, 21, TROMSO.0, TROMSO.1, 2.0), 24.0);
        assert_eq!(day_length(2026, 12, 21, TROMSO.0, TROMSO.1, 1.0), 0.0);
        assert_eq!(sun_times(2026, 6, 21, TROMSO.0, TROMSO.1, 2.0).rise, "always");
        // Uranus over Tromsø on 1995-07-09 was up for ten minutes: no full hour sees it.
        let brief = body_times(1995, 7, 9, "uranus", TROMSO.0, TROMSO.1, 1.0);
        assert!(hours_of(&brief.rise) > 0.5 && hours_of(&brief.set) - hours_of(&brief.rise) < 0.3);
        // Svalbard: the moon rises and does not set that day.
        assert_eq!(moon_times(2026, 3, 20, 78.2232, 15.6267, 1.0).set, "none");
    }

    #[test]
    fn brightness() {
        let s = oslo(2026, 11, 16);
        assert!(near(magnitude(&s, "venus"), -4.61, 0.02) && near(magnitude(&s, "saturn"), 0.50, 0.02));
        assert!(near(lit_part(&s, "venus"), 0.143, 0.002) && near(moon_lit(&s), 0.360, 0.002));
        assert!(near(diameter(&s, "jupiter"), 37.39, 0.05) && near(diameter(&s, "sun"), 1940.5, 1.0));
        assert!(near(elongation(&s, "uranus"), 169.54, 0.02) && !evening(&s, "venus"));
        assert!(near(refraction(0.0), 0.483, 0.001) && near(airmass(30.0), 1.994, 0.001));
        assert!(near(separation(0.0, 0.0, 90.0, 0.0), 90.0, 0.0001));
    }

    #[test]
    fn moon() {
        let from = jd(2026, 11, 1);
        assert_eq!(jd_text(next_phase(from, 0)), "2026-11-09 07:02");
        assert_eq!(jd_text(next_phase(from, 1)), "2026-11-17 11:48");
        assert_eq!(jd_text(next_phase(from, 2)), "2026-11-24 14:53");
        assert_eq!(jd_text(next_phase(from, 3)), "2026-11-01 20:28");
        let p = moon_phase(2026, 11, 16);
        assert!(p.name == "Waxing Crescent" && p.index == 1);
        assert_eq!(phases_in_month(2026, 11)[3], "24 🌕 Full Moon 14:53");
        assert!(notable_phase(2026, 11, 24) && !notable_phase(2026, 11, 26));
        assert_eq!(jd_text(next_perigee(jd(2026, 1, 1))), "2026-01-01 21:46");
        assert_eq!(jd_text(next_apogee(jd(2026, 1, 1))), "2026-01-13 20:48");
    }

    #[test]
    fn eclipses_of_2026() {
        let all = eclipses(2026);
        assert_eq!(all.len(), 4);
        assert!(jd_text(all[0].when) == "2026-02-17 12:13" && eclipse_text(&all[0]) == "Annular solar eclipse, magnitude 0.96");
        assert!(jd_text(all[1].when) == "2026-03-03 11:34" && eclipse_text(&all[1]) == "Total lunar eclipse, magnitude 1.15");
        assert!(near(all[1].total, 57.5, 1.0) && all[3].form == "partial");
        let e = next_eclipse(jd(2026, 4, 1), "solar").unwrap();
        assert!(jd_text(e.when) == "2026-08-12 17:46" && e.form == "total");
        let seen = local_eclipse(e.when, OSLO.0, OSLO.1, 2.0).unwrap();
        assert!(jd_text(seen.peak) == "2026-08-12 17:58" && near(seen.mag, 0.858, 0.005) && near(seen.alt, 8.3, 0.2));
        assert!(near(sun_cover(&sky_jd(seen.peak, OSLO.0, OSLO.1, 2.0)), seen.mag, 0.001));
        assert_eq!(local_eclipse(e.when, -33.8688, 151.2093, 10.0).map_or(0.0, |l| l.mag), 0.0);
    }

    #[test]
    fn year() {
        assert_eq!(jd_text(season(2026, 0)), "2026-03-20 14:45");
        assert_eq!(jd_text(season(2026, 1)), "2026-06-21 08:25");
        assert_eq!(jd_text(season(2026, 2)), "2026-09-23 00:05");
        assert_eq!(jd_text(season(2026, 3)), "2026-12-21 20:50");
        let start = jd(2026, 1, 1);
        assert_eq!(day(next_opposition("mars", start)), "2027-02-19");
        assert_eq!(day(next_opposition("jupiter", start)), "2026-01-10");
        assert_eq!(day(next_opposition("saturn", start)), "2026-10-04");
        assert_eq!(day(next_greatest_elongation("mercury", start)), "2026-02-19");
        assert_eq!(day(next_greatest_elongation("venus", start)), "2026-08-15");
        assert_eq!(day(next_conjunction("venus", "jupiter", start, 365.0)), "2026-06-09");
        assert_eq!(next_conjunction("mars", "saturn", jd(2026, 11, 1), 10.0), None);
        assert_eq!(day(Some(next_perihelion(start))), "2026-01-03");
        assert_eq!(day(Some(next_aphelion(start))), "2026-07-05");
        let year = almanac(2026);
        assert!(year.len() == 109 && year[0].text.starts_with("☾ Moon nearest"));
        assert!(year.windows(2).all(|w| w[1].when >= w[0].when));
    }

    #[test]
    fn star_list() {
        assert!(stars().len() == 105 && stars()[0].name == "Sirius");
        let v = star("vega").unwrap();
        assert!(v.name == "Vega" && v.con == "Lyra" && v.spectral == "A0" && near(v.ly, 25.0, 0.5));
        assert!(star("Sirius").unwrap().con == "Canis Major" && star("nothing").is_none());
        let night = sky_at(2026, 11, 16, 21.0, OSLO.0, OSLO.1, 1.0);
        let p = star_pos(&night, "Capella").unwrap();
        let h = horizon(&night, p.ra, p.dec);
        assert!(near(h.alt, 46.42, 0.02) && near(h.az, 74.42, 0.02));
        let up = stars_up(&night, 30.0);
        assert!(up.len() == 31 && up[0].name == "Vega");
        let s = oslo(2026, 11, 16);
        assert_eq!(constellation(s.sun_ra, s.sun_dec, jd_of(&s)), "Libra");
        assert_eq!(constellation(83.82, -5.39, 2451545.0), "Orion");
        assert!(constellations().len() == 88 && con_short("Ursa Major") == "UMa");
        let centre = galactic(266.405, -28.936);
        assert!(near(centre.lon, 0.0, 0.01) && near(centre.lat, 0.0, 0.01));
        let e = to_ecliptic(s.sun_ra, s.sun_dec, s.ecl);
        let back = to_equatorial(e.lon, e.lat, s.ecl);
        assert!(near(e.lat, 0.0, 0.001) && near(back.ra, s.sun_ra, 0.0001) && near(back.dec, s.sun_dec, 0.0001));
        let moved = precess(101.287, -16.716, 2451545.0, jd(2026, 11, 16));
        assert!(near(moved.ra, 101.587, 0.002) && near(moved.dec, -16.746, 0.002));
    }

    #[test]
    fn tonight_and_table() {
        let list = bodies(2026, 11, 16, OSLO.0, OSLO.1, 1.0);
        assert!(list.len() == 9 && list[0].con == "Libra" && list[6].name == "saturn");
        assert!(table(&list).len() == 11 && table(&list)[2].starts_with("Sun"));
        let seen = visible_planets(2026, 11, 16, OSLO.0, OSLO.1, 1.0);
        assert!(seen.len() == 5 && seen[0].name == "mercury");
        let day = events(2026, 8, 12);
        assert!(day.len() == 3 && day[1] == "Total solar eclipse, magnitude 1.03 17:46 UT");
        assert!(overhead(11, 59.9)[0] == "Andromeda" && showers().len() == 11);
        assert!(tonight(2026, 11, 16, OSLO.0, OSLO.1, 1.0).len() > 2);
        assert!(title("sun") == "Sun" && names().len() == 9);
    }

    #[test]
    fn moons_and_rings() {
        let at = jd(2026, 11, 16);
        let four = jupiter_moons(at);
        assert!(four[0].name == "Io" && four[3].name == "Callisto");
        assert!(near(four[0].x, 5.44, 0.05) && near(four[2].x, -10.69, 0.05) && four[1].state.is_empty());
        assert!(jupiter_moons(at + 0.3)[0].state == "hidden" && jupiter_moons(at + 40.1)[0].state == "transit");
        let line = moons_line(at, 61);
        assert!(line.chars().count() == 61 && line.chars().nth(30) == Some('O'));
        assert!(line.find('G') < line.find('E') && line.find('I') < line.find('C'));
        let r = saturn_rings(at);
        assert!(near(r.tilt, -6.26, 0.03) && near(r.sun_tilt, -8.33, 0.03) && near(r.wide, 43.04, 0.05) && near(r.high, 4.70, 0.05));
        assert!(near(saturn_rings(at - 3000.2).tilt, 26.57, 0.05));
    }
}
