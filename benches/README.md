# Benchmarks

Small benchmarks for various parts of the model codebase. Should be used intermittently to ensure against regression in
performance.

## List of Benchmarks:

| Benchmark  |  Description |
| ---------- | ------------ | 
| Iterations | Tests the effects of alignment and struct sizes on raw iterative loops. Utilised to motivate the decision behind using Option\<NonMaxU64> for smaller sizes. |
| Routing    | Rudimentary insights into direct and transit routing times, as well as cost of selecting nearby transit nodes on the GranularGrid |
| Event Loop | Runs the full event loop and checks the time to simulate a day on various sizes of test Synthetic Environments |

## Usage:

 * `cargo bench --bench $BENCH_NAME` where `$BENCH_NAME` is any of: [`iterations`, `routing`, `event_loop`]