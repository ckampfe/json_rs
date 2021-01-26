# RegexRs

```
(base) clark@doomguy:~/code/personal/json_rs$ MIX_ENV=bench mix run bench.exs
Compiling NIF crate :json_rs (native/json_rs)...
warning: unused variable: `s`
  --> src/lib.rs:23:11
   |
23 | fn decode(s: &str) -> Result<Term, Error> {
   |           ^ help: if this is intentional, prefix it with an underscore: `_s`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

    Finished release [optimized] target(s) in 0.00s
Operating System: Linux
CPU Information: AMD Ryzen 7 5800X 8-Core Processor
Number of Available Cores: 16
Available memory: 50.17 GB
Elixir 1.11.2
Erlang 23.2

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 5 s
memory time: 0 ns
parallel: 1
inputs: 100kB, 33kB
Estimated total run time: 28 s

Benchmarking Jason encode/1 with input 100kB...
Benchmarking Jason encode/1 with input 33kB...
Benchmarking JsonRs encode/1 with input 100kB...
Benchmarking JsonRs encode/1 with input 33kB...

##### With input 100kB #####
Name                      ips        average  deviation         median         99th %
JsonRs encode/1        5.79 K       0.173 ms    ±13.49%       0.173 ms       0.193 ms
Jason encode/1         0.93 K        1.08 ms     ±6.93%        1.06 ms        1.37 ms

Comparison:
JsonRs encode/1        5.79 K
Jason encode/1         0.93 K - 6.26x slower +0.91 ms

##### With input 33kB #####
Name                      ips        average  deviation         median         99th %
JsonRs encode/1       18.36 K       54.47 μs     ±8.37%       52.90 μs       65.10 μs
Jason encode/1         2.77 K      360.61 μs     ±4.11%      358.60 μs      395.40 μs

Comparison:
JsonRs encode/1       18.36 K
Jason encode/1         2.77 K - 6.62x slower +306.14 μs
```
