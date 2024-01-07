# Advent of Code 2023 Solutions

This directory contains my solutions for [Advent of Code 2022](https://adventofcode.com/2022).

## Goals

Like last year, I've aimed for sub 1s total runtime. Correctness, algorithmic
efficiency and readability are my priorities, in that order. I have generally
avoided optimizations that hurt readability without substantially improving
runtime, but didn't avoid added complexity if the performance gain was
substantial.

I've avoided optimizations that could lead to incorrect results for any inputs.
There are a few cases where I've elected to use faster less general algorithm
to solve a problem, but in those cases I've tried to at least ensure that if
the assumptions of the faster algorithm are violated the solution will return
an error rather than failing silently.

Where possible I've preferred algorithms that will handle all possible inputs
efficiently, but in a few cases I've chosen algorithms that perform well on
real inputs even though the solutions may not be optimal for adversarially
crafted inputs.

Many of these runtimes could be further improved by using parallelism, but my
goal here is to measure how efficient the solution is, not how many cores my
computer has. I'm not really interested in seeing how much faster rayon
`par_iter()` makes things. There are a couple problems where parallelization
could be somewhat challenging and interesting - day 23 in particular jumps out
- but for the sake of uniformity, I've avoided parallelization altogether.

## Performance

| Day | Runtime |
| --- | ------: |
| 1   |   0.6ms |
| 2   |   0.5ms |
| 3   |   0.7ms |
| 4   |   0.5ms |
| 5   |   0.5ms |
| 6   |   0.5ms |
| 7   |   0.9ms |
| 8   |   5.0ms |
| 9   |   0.7ms |
| 10  |   1.5ms |
| 11  |   1.1ms |
| 12  |   9.1ms |
| 13  |   0.6ms |
| 14  |   7.5ms |
| 15  |   0.9ms |
| 16  |  20.8ms |
| 17  |  32.6ms |
| 18  |   0.5ms |
| 19  |   1.3ms |
| 20  |   2.8ms |
| 21  |  11.4ms |
| 22  |   1.7ms |
| 23  | 150.4ms |
| 24  |   3.1ms |
| 25  |   6.9ms |

Total Runtime: 262.1ms
