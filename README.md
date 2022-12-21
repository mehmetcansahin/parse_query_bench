# HTML Parse and Query Title Benchmark

Run

```bash
$ cargo bench
```

- [tl](https://github.com/y21/tl)
- [scraper](https://github.com/causal-agent/scraper)
- [kuchiki](https://github.com/kuchiki-rs/kuchiki)
- [visdom](https://github.com/fefit/visdom)
- [html_editor](https://github.com/lomirus/html_editor)
- [lol-html](https://github.com/cloudflare/lol-html)

## tl

```
time: [1.4151 µs 1.4357 µs 1.4607 µs]
change: [-28.242% -26.711% -25.128%] (p = 0.00 < 0.05)
Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
5 (5.00%) high mild
9 (9.00%) high severe
```

## scraper

```
time: [10.070 µs 10.197 µs 10.368 µs]
change: [-29.054% -27.464% -25.843%] (p = 0.00 < 0.05)
Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
7 (7.00%) high mild
6 (6.00%) high severe
```

## kuchiki

```
time: [11.097 µs 11.170 µs 11.257 µs]
change: [-3.4731% -2.5815% -1.6003%] (p = 0.00 < 0.05)
Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
2 (2.00%) high mild
8 (8.00%) high severe
```

## visdom

```
time: [8.1937 µs 8.2668 µs 8.3544 µs]
change: [-0.5969% +0.4835% +1.5364%] (p = 0.38 > 0.05)
No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
3 (3.00%) high mild
5 (5.00%) high severe
```

## html_editor

```
time: [19.550 µs 19.685 µs 19.858 µs]
change: [-5.8012% -4.8899% -4.0324%] (p = 0.00 < 0.05)
Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
8 (8.00%) high mild
8 (8.00%) high severe
```

## lol_html

```
time: [5.6688 µs 5.7066 µs 5.7501 µs]
change: [-0.9402% +1.0476% +3.4096%] (p = 0.40 > 0.05)
No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
3 (3.00%) high mild
10 (10.00%) high severe
```
