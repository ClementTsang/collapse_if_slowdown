# collapse_if_slowdown

Run the benchmarks using something like (at least on Linux):

```bash
cargo bench 2> /dev/null
```

We dump stderr as the test functions write to there, so you might get spammed with messages otherwise.

## Benchmark results

Results on my machine (Ryzen 5700x, 32GiB of RAM):

```sh
slow                    time:   [7.7474 ms 7.8033 ms 7.8992 ms]
fast                    time:   [363.49 µs 368.44 µs 374.35 µs]
```
