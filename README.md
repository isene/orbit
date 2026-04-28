# orbit

![Rust](https://img.shields.io/badge/language-Rust-f74c00) ![License](https://img.shields.io/badge/license-Unlicense-green)

Moon phases, ephemeris, and sun/planet positions for the
[Fe₂O₃ Rust terminal suite](https://github.com/isene/fe2o3). Shared by
[nova](https://github.com/isene/nova) (astronomy panel) and
[tock](https://github.com/isene/tock) (calendar).

Planet calculations are ported from
[ruby-ephemeris](https://github.com/isene/ephemeris).

## Install

```toml
[dependencies]
orbit = { version = "0.1", package = "fe2o3-orbit" }
```

## Highlights

- `moon_phase(y, m, d)` → illumination, phase name, symbol, phase index
- `moon_times(y, m, d, lat, lon, tz)` → rise / set
- `sun_times(y, m, d, lat, lon, tz)` → sunrise / sunset
- `visible_planets(y, m, d, lat, lon, tz)` → planets above 5° between 20:00–04:00
- `all_bodies(y, m, d, lat, lon, tz)` → ephemeris rows for sun/moon/planets
- `astro_events(m, d)`, `astro_events_for_year(y, m, d)` → meteor showers, equinoxes, etc.
- `tonight_summary(y, m, d, lat, lon, tz, bortle)` → human-readable nightly summary
- `notable_phase(y, m, d)`, `notable_phases_in_month(y, m)` → quarter / new / full markers
- Display helpers: `body_symbol`, `body_color_hex`, `body_color_256`, `moon_phase_gray`

## License

Public domain (Unlicense).
