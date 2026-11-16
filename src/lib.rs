//! orbit — moon phases, ephemeris, and sun/planet positions for the
//! Fe₂O₃ Rust terminal suite. Shared by `nova` (astronomy panel) and
//! `tock` (calendar). Planet calculations ported from
//! [ruby-ephemeris](https://github.com/isene/ephemeris).

pub mod astro;

use crust::style;


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

pub const PLANET_SYMBOLS: &[(&str, &str)] = &[
    ("mercury", "\u{263F}"), ("venus", "\u{2640}"), ("mars", "\u{2642}"),
    ("jupiter", "\u{2643}"), ("saturn", "\u{2644}"),
];

pub const BODY_COLORS: &[(&str, &str)] = &[
    ("sun", "FFD700"), ("moon", "888888"), ("mercury", "8F6E54"),
    ("venus", "E6B07C"), ("mars", "BC2732"), ("jupiter", "C08040"), ("saturn", "E8D9A0"),
];

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
    let p = astro::moon_phase(year, month, day);
    // `phase` is how far the moon is round from the sun at 0h UT, as a
    // part of a full turn. Callers add the hours of the day to it.
    let s = astro::sky_jd(astro::jd(year, month, day), 0.0, 0.0, 0.0);
    let turn = (astro::pos(&s, "moon").lon - s.sun_lon).rem_euclid(360.0) / 360.0;
    MoonPhase {
        illumination: (astro::moon_lit(&s) * 10000.0).round() / 10000.0,
        phase: (turn * 10000.0).round() / 10000.0,
        phase_name: p.name,
        symbol: p.symbol,
        phase_index: p.index,
    }
}

pub fn moon_symbol(year: i32, month: u32, day: u32) -> &'static str {
    moon_phase(year, month, day).symbol
}

pub fn notable_phase(year: i32, month: u32, day: u32) -> bool {
    astro::notable_phase(year, month, day)
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
    let seasons = [
        "\u{2600} Vernal Equinox", "\u{2600} Summer Solstice",
        "\u{2600} Autumnal Equinox", "\u{2744} Winter Solstice",
    ];
    for (which, text) in seasons.iter().enumerate() {
        let (_, m, d, _, _) = astro::date_of(astro::season(year, which as u32));
        if (m, d) == (month, day) { events.push(text.to_string()); }
    }
    for sh in astro::showers() {
        if (sh.month, sh.day) == (month, day) {
            events.push(format!("\u{2604} {} peak", sh.name));
        }
    }
    events
}

// ── Sun times (via ephemeris engine) ────────────────────────────────

/// A time that does not happen on the day ("none") shows as "--:--".
fn clock(s: &str) -> String {
    if s == "none" { "--:--".into() } else { s.to_string() }
}

fn rise_and_set(t: astro::Times) -> Option<(String, String)> {
    if t.rise == "never" || t.rise == "always" { return None; }
    Some((clock(&t.rise), clock(&t.set)))
}

pub fn sun_times(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Option<(String, String)> {
    rise_and_set(astro::sun_times(year, month, day, lat, lon, tz))
}

pub fn sun_times_oslo(year: i32, month: u32, day: u32) -> Option<(String, String)> {
    sun_times(year, month, day, 59.9139, 10.7522, 1.0)
}

/// Moon rise and set times via ephemeris.
pub fn moon_times(year: i32, month: u32, day: u32, lat: f64, lon: f64, tz: f64) -> Option<(String, String)> {
    rise_and_set(astro::moon_times(year, month, day, lat, lon, tz))
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
    let s = astro::sky(year, month, day, lat, lon, tz);
    let mut out = Vec::with_capacity(BODY_ORDER.len());
    for &name in BODY_ORDER {
        let p = astro::pos(&s, name);
        let t = astro::body_times(year, month, day, name, lat, lon, tz);
        let always_up = t.rise == "always";
        let never_up = t.rise == "never";
        // No rise on the day: it was up from midnight. No set: up to midnight.
        let hour = |text: &str, missing: f64| {
            if always_up || never_up { None }
            else if text == "none" { Some(missing) }
            else { parse_hhmm(text) }
        };
        out.push(BodyObs {
            name, ra_deg: p.ra, dec_deg: p.dec, distance: p.dist,
            rise_h: hour(&t.rise, 0.0), set_h: hour(&t.set, 24.0),
            rise: clock(&t.rise), transit: clock(&t.transit), set: clock(&t.set),
            always_up, never_up,
        });
    }
    out
}

/// Where the sun, moon and planets sit on the sky at one instant, and
/// the local sidereal time that turns any right ascension into an hour
/// angle. `hour` is local clock time, the same clock `tz` describes.
///
/// This is the chart-drawing counterpart to `all_bodies`: no rise / set
/// strings, but positions good for the hour rather than for the day.
pub fn sky_at(
    year: i32, month: u32, day: u32, hour: f64, lat: f64, lon: f64, tz: f64,
) -> (f64, Vec<(&'static str, f64, f64)>) {
    let s = astro::sky_at(year, month, day, hour, lat, lon, tz);
    let bodies = BODY_ORDER.iter()
        .map(|&name| { let p = astro::pos(&s, name); (name, p.ra, p.dec) })
        .collect();
    (astro::sidereal(&s), bodies)
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
    // Header, rule and rows all have to put their separators in the same
    // columns. They did not: the header padded RA one wide and d=AU one
    // narrow, and the rule's fourth segment was a dash short, so the
    // three disagreed and the table came out skewed. Field widths here
    // are the ones the row format below writes: 11, 7, 8, 7, 5, 5, 5.
    out.push_str("Planet      \u{2502} RA      \u{2502} Dec      \u{2502} d=AU    \u{2502} Rise  \u{2502} Trans \u{2502} Set\n");
    // Same columns as the header: 12, 22, 33, 43, 51, 59.
    let rule: String = [12usize, 9, 10, 9, 7, 7, 6].iter()
        .map(|n| "\u{2500}".repeat(*n))
        .collect::<Vec<_>>()
        .join("\u{253C}");
    out.push_str(&rule);
    out.push('\n');
    for b in bodies {
        let color = body_color_256(b.name);
        let name = format!("{} {}", body_symbol(b.name), capitalize_short(b.name));
        // Pad by CELLS, not chars. Every symbol carries a VS-15, so
        // `{:<11}` counted two chars for one cell and every row came out
        // a column short of the header.
        let name = format!("{}{}", name,
            " ".repeat(11usize.saturating_sub(crust::display_width(&name))));
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
            style::styled(s, Some(color), None, "")
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
            astro::where_is(&astro::sky_at(year, month, day, h, lat, lon, tz), name).alt > 5.0
        });
        if is_visible {
            let t = astro::body_times(year, month, day, name, lat, lon, tz);
            visible.push(VisiblePlanet {
                name: match name {
                    "mercury" => "Mercury", "venus" => "Venus", "mars" => "Mars",
                    "jupiter" => "Jupiter", "saturn" => "Saturn", _ => name,
                },
                symbol, color,
                rise: clock(&t.rise), set: clock(&t.set),
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

#[cfg(test)]
mod tests {
    use super::*;

    // ── Sanity tests ──────────────────────────────────────────────

    /// The sun sits near RA 8h30m, Dec +19° in late July. If the hour
    /// term were wrong by a day the RA would be a degree off.
    #[test]
    fn test_sky_at_sun_position() {
        let (_, bodies) = sky_at(2026, 7, 27, 12.0, 59.9, 10.7, 2.0);
        let (_, ra, dec) = bodies.iter().find(|b| b.0 == "sun").copied().unwrap();
        assert!((ra - 127.0).abs() < 2.0, "sun RA {ra}");
        assert!((dec - 19.0).abs() < 1.5, "sun Dec {dec}");
    }

    /// The moon covers about half a degree an hour, so the same day at
    /// 0h and at 23h must not give the same place.
    #[test]
    fn test_sky_at_moon_moves_within_the_day() {
        let pick = |h: f64| {
            let (_, b) = sky_at(2026, 7, 27, h, 59.9, 10.7, 2.0);
            b.iter().find(|x| x.0 == "moon").copied().unwrap()
        };
        let (_, ra0, _) = pick(0.0);
        let (_, ra23, _) = pick(23.0);
        let moved = (ra23 - ra0 + 360.0) % 360.0;
        assert!((8.0..18.0).contains(&moved), "moon moved {moved}° in 23h");
    }

    /// Sidereal time gains about a degree an hour on the clock, and
    /// stays inside one turn of the sky.
    #[test]
    fn test_sky_at_sidereal_time_advances() {
        let (lst0, _) = sky_at(2026, 7, 27, 0.0, 59.9, 10.7, 2.0);
        let (lst6, _) = sky_at(2026, 7, 27, 6.0, 59.9, 10.7, 2.0);
        assert!((0.0..360.0).contains(&lst0), "lst {lst0}");
        let gained = (lst6 - lst0 + 360.0) % 360.0;
        assert!((gained - 90.25).abs() < 1.0, "6h gained {gained}°");
    }

    #[test]
    fn test_moon_phase_range() {
        let p = moon_phase(2026, 4, 7);
        assert!(p.illumination >= 0.0 && p.illumination <= 1.0);
        assert!(p.phase_index < 8);
    }

    /// The moon moves 13° a day, so its rise and set must come from where
    /// it is at that hour. Taken from the 0h place they were up to 82
    /// minutes off. PyEphem gives 13:55 and 21:53 for Oslo this day.
    #[test]
    fn the_moon_rises_and_sets_on_time() {
        let (rise, set) = moon_times(2026, 11, 16, 59.9139, 10.7522, 1.0).unwrap();
        assert_eq!((rise.as_str(), set.as_str()), ("13:55", "21:53"));
        let moon = all_bodies(2026, 11, 16, 59.9139, 10.7522, 1.0).into_iter().find(|b| b.name == "moon").unwrap();
        assert_eq!((moon.rise.as_str(), moon.set.as_str()), ("13:55", "21:53"));
    }

    /// The parallax of a planet depends on its distance from the earth.
    /// It was taken from the distance to the sun, which moved Mars by up
    /// to 20 seconds of arc. PyEphem, Oslo, 2026-11-16 0h UT.
    #[test]
    fn a_planet_is_seen_from_the_earth() {
        let saturn = all_bodies(2026, 11, 16, 59.9139, 10.7522, 1.0).into_iter().find(|b| b.name == "saturn").unwrap();
        assert!((saturn.ra_deg - 8.8257).abs() < 0.001 && (saturn.dec_deg - 0.9333).abs() < 0.001);
    }

    /// A day with no rise shows "--:--" and counts as up from midnight.
    #[test]
    fn a_missing_time_reads_as_dashes() {
        let moon = all_bodies(2026, 3, 20, 78.2232, 15.6267, 1.0).into_iter().find(|b| b.name == "moon").unwrap();
        assert_eq!(moon.set, "--:--");
        assert_eq!(moon.set_h, Some(24.0));
        assert!(is_above(moon.rise_h, moon.set_h, false, false, 23.5));
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
    fn the_sun_is_one_au_away() {
        let sun = all_bodies(2026, 9, 27, 59.9, 10.7, 2.0).into_iter().find(|b| b.name == "sun").unwrap();
        assert!((0.98..1.02).contains(&sun.distance), "the sun at {} AU", sun.distance);
        assert!((sun.ra_deg - 183.59).abs() < 0.05, "and where it was: RA {}", sun.ra_deg);
    }

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

    /// The header, the rule and every row have to break in the same
    /// columns. They did not, and the table came out visibly skewed.
    #[test]
    fn the_ephemeris_table_lines_up() {
        fn strip_ansi(s: &str) -> String {
            let mut out = String::new();
            let mut chars = s.chars();
            while let Some(c) = chars.next() {
                if c == '\u{1b}' {
                    for c2 in chars.by_ref() {
                        if c2.is_ascii_alphabetic() { break; }
                    }
                } else {
                    out.push(c);
                }
            }
            out
        }
        // Columns are CELLS, not chars: every symbol carries a VS-15, so
        // counting chars is exactly the mistake this test exists to catch.
        fn bars(line: &str) -> Vec<usize> {
            let mut out = Vec::new();
            let mut walker = crust::WidthWalker::new();
            let mut col = 0usize;
            for c in line.chars() {
                if c == '\u{2502}' || c == '\u{253C}' { out.push(col); }
                col += walker.push(c);
            }
            out
        }
        let body = |name: &'static str, ra: f64, dec: f64, dist: f64| BodyObs {
            name, ra_deg: ra, dec_deg: dec, distance: dist,
            rise: "04:10".into(), transit: "12:23".into(), set: "20:36".into(),
            rise_h: None, set_h: None, always_up: false, never_up: false,
        };
        let table = ephemeris_table(&[
            body("sun", 136.0, 16.75, 2.0287),
            body("moon", 38.9, 19.55, 58.5935),   // two-digit distance
            body("neptune", 4.5, 0.38, 29.2115),
            body("mercury", 116.7, 20.3, 0.9738), // leading-zero distance
        ]);
        let lines: Vec<String> = table.lines().map(strip_ansi).collect();
        let want = bars(&lines[0]);
        assert_eq!(want, vec![12, 22, 33, 43, 51, 59], "header columns");
        for (i, l) in lines.iter().enumerate() {
            assert_eq!(bars(l), want, "line {} breaks elsewhere:\n{}\n{}", i, lines[0], l);
        }
    }
}
