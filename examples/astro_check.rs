//! Prints the numbers the Simpler module `astro` prints for the same
//! questions, so the two can be compared line by line.
use orbit::astro::*;

fn f(x: f64) -> String { fixed(x, 4) }

fn times_block() {
    let places = [(59.9139, 10.7522, 1.0), (69.6492, 18.9553, 1.0), (-33.8688, 151.2093, 10.0), (40.7128, -74.006, -5.0), (0.0, 0.0, 0.0), (78.2232, 15.6267, 1.0)];
    let dates = [(2026, 1, 15), (2026, 3, 20), (2026, 5, 18), (2026, 6, 21), (2026, 8, 12), (2026, 11, 16), (2026, 12, 21), (2031, 2, 3), (1995, 7, 9), (2060, 10, 30)];
    for (pi, p) in places.iter().enumerate() {
        for (y, m, d) in dates {
            for n in names() {
                let t = body_times(y, m, d, n, p.0, p.1, p.2);
                println!("{} {} {} {} {} {} {} {}", pi, y, m, d, n, t.rise, t.transit, t.set);
            }
        }
    }
}

fn numbers_block() {
    let (la, lo) = (59.9139, 10.7522);
    let s = sky(2026, 11, 16, la, lo, 1.0);
    for n in names() {
        let p = pos(&s, n);
        println!("{} {} {} {} mag {} el {} lit {} dia {}", n, f(p.ra), f(p.dec), f(p.dist), f(magnitude(&s, n)), f(elongation(&s, n)), f(lit_part(&s, n)), f(diameter(&s, n)));
    }
    let pl = orbit_pos(&s, &pluto());
    println!("pluto {} {} {}", f(pl.ra), f(pl.dec), f(pl.dist));
    for n in names() {
        let t = body_times(2026, 11, 16, n, la, lo, 1.0);
        println!("{} {} {} {}", n, t.rise, t.transit, t.set);
    }
    let tw = twilight(2026, 11, 16, 6.0, la, lo, 1.0);
    println!("civil {} {} len {} tromso {} {}", tw.rise, tw.set, f(day_length(2026, 11, 16, la, lo, 1.0)), f(day_length(2026, 6, 21, 69.6492, 18.9553, 2.0)), f(day_length(2026, 12, 21, 69.6492, 18.9553, 1.0)));
    println!("eot {} sid {} dt {} jdof {}", f(equation_of_time(&s)), f(sidereal(&s)), f(delta_t(2026.0)), f(jd_of(&s)));
    for w in 0..4 { println!("phase {}", jd_text(next_phase(jd(2026, 11, 1), w))); }
    println!("{}", phases_in_month(2026, 11).join(" | "));
    let mp = moon_phase(2026, 11, 16);
    println!("{} {} {} {} age {}", mp.name, mp.index, f(mp.lit), f(mp.age), f(moon_age(jd(2026, 11, 16))));
    for e in eclipses(2026) { println!("{} {} g {} p {} t {}", jd_text(e.when), eclipse_text(&e), f(e.gamma), f(e.partial), f(e.total)); }
    let ne = next_eclipse(jd(2026, 4, 1), "solar").unwrap();
    let lc = local_eclipse(ne.when, la, lo, 2.0).unwrap();
    println!("oslo {} {} {} {} {} {}", jd_text(lc.start), jd_text(lc.peak), jd_text(lc.stop), f(lc.mag), f(lc.cover), f(lc.alt));
    println!("sydney {}", f(local_eclipse(ne.when, -33.8688, 151.2093, 10.0).map_or(0.0, |l| l.mag)));
    for w in 0..4 { println!("season {}", jd_text(season(2026, w))); }
    let a = jd(2026, 1, 1);
    let opp = |n| jd_text(next_opposition(n, a).unwrap());
    println!("opp mars {} jup {} sat {}", opp("mars"), opp("jupiter"), opp("saturn"));
    println!("ge mer {} ven {}", jd_text(next_greatest_elongation("mercury", a).unwrap()), jd_text(next_greatest_elongation("venus", a).unwrap()));
    println!("peri {} apo {} perih {} aph {}", jd_text(next_perigee(a)), jd_text(next_apogee(a)), jd_text(next_perihelion(a)), jd_text(next_aphelion(a)));
    println!("conj {} none {}", jd_text(next_conjunction("venus", "jupiter", a, 365.0).unwrap()), f(next_conjunction("mars", "saturn", jd(2026, 11, 1), 10.0).unwrap_or(0.0)));
    let ev = sky_at(2026, 11, 16, 21.0, la, lo, 1.0);
    for n in ["Sirius", "vega", "Polaris", "Capella"] {
        let st = star(n).unwrap();
        let sp = star_pos(&ev, n).unwrap();
        let h = horizon(&ev, sp.ra, sp.dec);
        println!("{} {} {} {} {} pos {} {} alt {} az {}", st.name, st.con, st.spectral, f(st.mag), f(st.ly), f(sp.ra), f(sp.dec), f(h.alt), f(h.az));
    }
    let up = stars_up(&ev, 30.0);
    println!("stars {} up30 {} first {} nostar [{}]", stars().len(), up.len(), up[0].name, star("xx").map_or("", |s| s.name));
    println!("con {} {} {} {}", constellation(s.sun_ra, s.sun_dec, jd_of(&s)), constellation(83.82, -5.39, 2451545.0), con_short("Ursa Major"), constellations().len());
    let g = galactic(266.405, -28.936);
    let e2 = to_ecliptic(s.sun_ra, s.sun_dec, s.ecl);
    let b2 = to_equatorial(e2.lon, e2.lat, s.ecl);
    let pr = precess(101.287, -16.716, 2451545.0, jd(2026, 11, 16));
    println!("gal {} {} ecl {} {} back {} prec {} {}", f(g.lon), f(g.lat), f(e2.lon), f(e2.lat), f(b2.ra - s.sun_ra), f(pr.ra), f(pr.dec));
    println!("misc {} {} {} {} {} {} refr {} air {} sep {}", weekday(2026, 11, 16), easter(2026), easter(2027), compass(200.0), day_of_year(2026, 11, 16), jd_local(jd_at(2026, 11, 16, 23, 30, 0), 1.0), f(refraction(0.0)), f(airmass(30.0)), f(separation(0.0, 0.0, 90.0, 0.0)));
    let al = almanac(2026);
    println!("almanac {} {} {}", al.len(), jd_text(al[0].when), al[0].text);
    let vn: String = visible_planets(2026, 11, 16, la, lo, 1.0).iter().map(|b| format!("{} ", b.name)).collect();
    println!("vis {}| {} | {}", vn, events(2026, 8, 12).join(" ; "), overhead(11, 59.9).join(","));
    let bs = bodies(2026, 11, 16, la, lo, 1.0);
    let tb = table(&bs);
    println!("{}", tb[0]);
    println!("{}", tb[2]);
    println!("cover {} showers {} {} {}", f(sun_cover(&sky_jd(lc.peak, la, lo, 2.0))), showers().len(), sign("mars"), title("sun"));
}

fn moons_block() {
    for dd in [0.0, 0.3, 1.7, 5.25, 40.1, 200.6, 900.9, -3000.2] {
        let j = jd(2026, 11, 16) + dd;
        for m in jupiter_moons(j) {
            println!("M {} {} {} {} {}.", f(j), m.name, fixed(m.x, 3), fixed(m.y, 3), m.state);
        }
        let r = saturn_rings(j);
        println!("R {} {} {} {} {} {}", f(j), fixed(r.tilt, 3), fixed(r.sun_tilt, 3), fixed(r.wide, 2), fixed(r.high, 2), fixed(r.angle, 2));
    }
    println!("[{}]", moons_line(jd(2026, 11, 16), 61));
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("times") => times_block(),
        Some("numbers") => numbers_block(),
        Some("moons") => moons_block(),
        Some("almanac") => for e in almanac(2026) { println!("{} {}", jd_text(e.when), e.text); },
        Some("tonight") => {
            for l in tonight(2026, 11, 16, 59.9139, 10.7522, 1.0) { println!("{}", l); }
            for l in table(&bodies(2026, 11, 16, 59.9139, 10.7522, 1.0)) { println!("{}", l); }
        }
        _ => eprintln!("usage: astro_check times|numbers|moons|almanac|tonight"),
    }
}
