# Pangui Benchmark Results

## Environment

- Commit: `2c09be2`
- Platform: windows x86_64
- Rust: rustc 1.98.0 (88d9e12ae 2026-08-18)
- Command: `cargo pangui-report`

Each fixture is parsed once, a fresh ECS tree is constructed outside the timed routine, and a full layout is measured. Times are Criterion slope point estimates with 95% confidence intervals.

## Results

| Benchmark | Nodes | Time | 95% confidence interval | Throughput |
| --- | ---: | ---: | ---: | ---: |
| `expand_with_max_constraint` | 3,001 | 1.7960 ms | 1.7661-1.8337 ms | 1.6709 Melem/s |
| `expand_with_min_constraint` | 3,001 | 1.7751 ms | 1.7446-1.8061 ms | 1.6906 Melem/s |
| `fit_nesting` | 101,111 | 152.90 ms | 145.53-158.75 ms | 0.6613 Melem/s |
| `flex_expand_equal_weights` | 15,001 | 3.1262 ms | 2.8702-3.4279 ms | 4.7985 Melem/s |
| `flex_expand_weights` | 15,001 | 3.6375 ms | 3.5161-3.8169 ms | 4.1240 Melem/s |
| `nested_vertical_stack` | 10,001 | 1.8387 ms | 1.7556-1.9101 ms | 5.4392 Melem/s |
| `percentage_and_ratio` | 10,001 | 2.1196 ms | 1.9980-2.2405 ms | 4.7183 Melem/s |
| `perpendicular_expand_with_wrap` | 12,001 | 4.5562 ms | 4.1417-4.9030 ms | 2.6340 Melem/s |
| `pixels_with_min_expand_constraint` | 30,001 | 15.2344 ms | 13.2337-17.2765 ms | 1.9693 Melem/s |
| `wide_no_wrap_simple_few` | 1,001 | 203.31 us | 196.08-215.46 us | 4.9236 Melem/s |
| `wide_no_wrap_simple_many` | 100,001 | 19.2622 ms | 18.0302-20.8308 ms | 5.1916 Melem/s |
| `wide_wrapping` | 10,001 | 1.8406 ms | 1.7557-1.9704 ms | 5.4336 Melem/s |

## Skipped Fixtures

- `padding_and_margin.txt` (unsupported or produced no Criterion result)
