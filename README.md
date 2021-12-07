# Advent of Code 2021

A feeble attempt to finish Advent of Code for once. Last year I bailed at day
~20, this year, I might get further, but we'll see.

Went with Rust as I know it well enough to use it, but don't use it day to day.
Trying to balance between learning a new language with wanting to be able to
solve the problems.

A rough `src/main.rs.template` file is the starting point for each day. Each
day should have a test for `part1` and `part2`, which verifies the real answer
when found.

Github Actions have been set up to test, lint, check code style and everything
of course. Might set it up to cross compile binaries for maximum time wasting,
instead of focusing on the problems.

Side thought: Would a versioning scheme of DD.PP where DD is the day and PP is
the part make sense? When Day 3, Part 2 is solved, automatically build and
release AdventOfCode v03.02? Possibly have a patch version as well, because who
knows, I might find a bug and have to release a new solution (I'm only half
serious).
