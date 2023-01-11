# Advent of Code 2022 Solutions

This directory contains my solutions for [Advent of Code 2022](https://adventofcode.com/2022).

## Thoughts

As always, my top priorities were correctness and clarity, though for the first
time I set myself a goal of getting sub-1s all-parts runtime on my computer.

I leaned heavily into Rust's type system this year, sometimes going a little
overboard with the parsing, but it's really satisfying to have data types that
encode the constraints of your problem.

## Performance

I've made no real attempt to micro-optimize, but aimed to use efficient
algorithms and data structures. I probably used bit sets a little more often
than strictly justified by my goals. I really like bit sets.

As an exception to my normal priority of algorithmic efficiency over
micro-optimization, performance for day 20 is O(n^2) in the size of the input.
I originally wrote an O(n) solution, but on the actual data it was much slower
and in the name of total runtime I decided to switch to the less efficient
solution.

I'm confident there are meaningful speed-ups to be had on the slowest days
(particularly 23 and 24), but I'm happy with where I've landed for now!

I measured performance using [hyperfine](https://github.com/sharkdp/hyperfine).
See the `bench` [script](rust/bench) for details.

| Day | Runtime |
| --- | ------: |
| 1   |   0.5ms |
| 2   |   0.5ms |
| 3   |   0.5ms |
| 4   |   0.5ms |
| 5   |   0.5ms |
| 6   |   0.4ms |
| 7   |   0.9ms |
| 8   |   1.5ms |
| 9   |   1.1ms |
| 10  |   0.4ms |
| 11  |   4.2ms |
| 12  |   0.6ms |
| 13  |   1.0ms |
| 14  |   1.1ms |
| 15  |   1.8ms |
| 16  |  41.1ms |
| 17  |   1.4ms |
| 18  |   2.2ms |
| 19  |  53.4ms |
| 20  |  59.7ms |
| 21  |   1.1ms |
| 22  |   1.1ms |
| 23  | 160.1ms |
| 24  |  98.9ms |
| 25  |   0.4ms |

Total Runtime: 435.5ms
