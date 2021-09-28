# JsonRs

```
clark@doomguy:~/code/personal/json_rs$ MIX_ENV=bench mix run bench.exs
Operating System: Linux
CPU Information: AMD Ryzen 7 5800X 8-Core Processor
Number of Available Cores: 16
Available memory: 50.17 GB
Elixir 1.12.3
Erlang 24.0.6

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 5 s
memory time: 0 ns
parallel: 1
inputs: 100kB, 33kB, issue-90, very small
Estimated total run time: 56 s

Benchmarking Jason encode/1 with input 100kB...
Benchmarking Jason encode/1 with input 33kB...
Benchmarking Jason encode/1 with input issue-90...
Benchmarking Jason encode/1 with input very small...
Benchmarking JsonRs encode/1 with input 100kB...
Benchmarking JsonRs encode/1 with input 33kB...
Benchmarking JsonRs encode/1 with input issue-90...
Benchmarking JsonRs encode/1 with input very small...

##### With input 100kB #####
Name                      ips        average  deviation         median         99th %
JsonRs encode/1        6.36 K      157.31 μs    ±48.05%      154.20 μs      193.45 μs
Jason encode/1         1.83 K      546.37 μs    ±19.15%      581.45 μs      767.11 μs

Comparison:
JsonRs encode/1        6.36 K
Jason encode/1         1.83 K - 3.47x slower +389.06 μs

##### With input 33kB #####
Name                      ips        average  deviation         median         99th %
JsonRs encode/1       19.05 K       52.48 μs    ±64.62%       50.96 μs       67.86 μs
Jason encode/1         7.03 K      142.34 μs    ±13.15%      143.30 μs      168.15 μs

Comparison:
JsonRs encode/1       19.05 K
Jason encode/1         7.03 K - 2.71x slower +89.85 μs

##### With input issue-90 #####
Name                      ips        average  deviation         median         99th %
JsonRs encode/1        183.49        5.45 ms     ±6.10%        5.43 ms        6.70 ms
Jason encode/1          43.70       22.88 ms     ±4.56%       22.71 ms       25.67 ms

Comparison:
JsonRs encode/1        183.49
Jason encode/1          43.70 - 4.20x slower +17.43 ms

##### With input very small #####
Name                      ips        average  deviation         median         99th %
JsonRs encode/1        1.97 M        0.51 μs  ±6787.00%        0.37 μs        0.88 μs
Jason encode/1         0.78 M        1.28 μs  ±5769.83%        0.42 μs        1.37 μs

Comparison:
JsonRs encode/1        1.97 M
Jason encode/1         0.78 M - 2.53x slower +0.77 μs
Operating System: Linux
CPU Information: AMD Ryzen 7 5800X 8-Core Processor
Number of Available Cores: 16
Available memory: 50.17 GB
Elixir 1.12.3
Erlang 24.0.6

Benchmark suite executing with the following configuration:
warmup: 2 s
time: 5 s
memory time: 0 ns
parallel: 1
inputs: 100kB, 33kB, issue-90, very small
Estimated total run time: 56 s

Benchmarking Jason decode/1 with input 100kB...
Benchmarking Jason decode/1 with input 33kB...
Benchmarking Jason decode/1 with input issue-90...
Benchmarking Jason decode/1 with input very small...
Benchmarking JsonRs decode/1 with input 100kB...
Benchmarking JsonRs decode/1 with input 33kB...
Benchmarking JsonRs decode/1 with input issue-90...
Benchmarking JsonRs decode/1 with input very small...

##### With input 100kB #####
Name                      ips        average  deviation         median         99th %
JsonRs decode/1        1.76 K      566.72 μs    ±25.12%      505.89 μs      949.96 μs
Jason decode/1         1.58 K      631.26 μs     ±6.73%      624.19 μs      809.38 μs

Comparison:
JsonRs decode/1        1.76 K
Jason decode/1         1.58 K - 1.11x slower +64.53 μs

##### With input 33kB #####
Name                      ips        average  deviation         median         99th %
JsonRs decode/1        5.58 K      179.12 μs    ±28.72%      150.61 μs      294.18 μs
Jason decode/1         4.64 K      215.44 μs    ±16.67%      212.85 μs      285.19 μs

Comparison:
JsonRs decode/1        5.58 K
Jason decode/1         4.64 K - 1.20x slower +36.32 μs

##### With input issue-90 #####
Name                      ips        average  deviation         median         99th %
JsonRs decode/1        143.04        6.99 ms     ±6.24%        6.86 ms        9.02 ms
Jason decode/1          42.39       23.59 ms     ±2.57%       23.60 ms       25.30 ms

Comparison:
JsonRs decode/1        143.04
Jason decode/1          42.39 - 3.37x slower +16.60 ms

##### With input very small #####
Name                      ips        average  deviation         median         99th %
JsonRs decode/1      986.99 K        1.01 μs  ±4671.45%        0.64 μs        1.57 μs
Jason decode/1       796.58 K        1.26 μs  ±5605.52%        0.42 μs        1.33 μs

Comparison:
JsonRs decode/1      986.99 K
Jason decode/1       796.58 K - 1.24x slower +0.24 μs
```
