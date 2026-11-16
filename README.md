# orbit

![Rust](https://img.shields.io/badge/language-Rust-f74c00) ![License](https://img.shields.io/badge/license-Unlicense-green)

An almanac for the [Fe₂O₃ Rust terminal suite](https://github.com/isene/fe2o3):
sun, moon, planets, stars and eclipses. Where they are, when they rise,
how bright they are. Shared by [astro](https://github.com/isene/astro),
[tock](https://github.com/isene/tock) and [moon](https://github.com/isene/moon).

## Install

```toml
[dependencies]
orbit = { version = "0.2", package = "fe2o3-orbit" }
```

## The almanac: `orbit::astro`

```rust
use orbit::astro;

let oslo = astro::sky(2026, 11, 16, 59.9139, 10.7522, 1.0);
let moon = astro::pos(&oslo, "moon");                       // ra, dec, distance
let t = astro::moon_times(2026, 11, 16, 59.9139, 10.7522, 1.0);
println!("{} {} {}", t.rise, t.transit, t.set);             // 13:55 17:47 21:53
```

- Places: `pos`, `where_is` (height and compass direction), `orbit_pos`
  for a comet or a dwarf planet.
- Times: `body_times`, `sun_times`, `moon_times`, `twilight`, `day_length`.
- Brightness: `magnitude`, `lit_part`, `diameter`, `elongation`.
- The moon: `moon_phase`, `next_phase`, `phases_in_month`, `next_perigee`.
- Eclipses: `eclipses(year)`, `next_eclipse`, `local_eclipse` for one place.
- The year: `season`, `next_opposition`, `next_conjunction`, `almanac(year)`.
- Stars: 105 named stars, the 88 constellations, `stars_up`, `precess`.
- Jupiter's four moons (`jupiter_moons`, `moons_line`) and Saturn's
  rings (`saturn_rings`).
- A day at a glance: `bodies`, `table`, `tonight`, `events`.

`cargo doc --open` shows every function with a line on what it gives.

The module is the same almanac as the `astro` module of
[Simpler](https://github.com/isene/simpler), function for function. The
two print the same 758 test lines.

## How good the numbers are

Checked against PyEphem 4.2 (the XEphem library), which shares no code
with this one:

- places within 0.01 degrees;
- all of 1,620 rise, transit and set times within 1.1 minutes, from the
  equator to Svalbard;
- Saturn's ring tilt within 0.02 degrees.

`cargo run --release --example astro_check times` prints the lines that
were compared.

## The first functions

The functions at the top of the crate (`moon_phase`, `moon_times`,
`sun_times`, `all_bodies`, `sky_at`, `visible_planets`,
`astro_events_for_year`, `tonight_summary`, `ephemeris_table` and the
colour and symbol helpers) keep their shape and take their numbers from
`orbit::astro`.

## License

Public domain (Unlicense).
