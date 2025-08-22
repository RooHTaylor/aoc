# Advent of Code (AoC)

[Advent of Code](https://adventofcode.com/)

Collection of advent of code challenges written in various programming languages.
I started this in 2025, going all the way back to the initial 2015 AoC year. In 
the beginning I was learning Rust, so the inital challanges are all written in 
Rust. However, I may choose to use other languages as time goes on.

In late 2025 I decided to wipe the slate clean and begin the challenges from 
scratch.

### Completion

|Year|Languages|Completion|
|----|---------|----------|
|2015|Rust|40%|
|2016|||
|2017|||
|2018|||
|2019|||
|2020|||
|2021|||
|2022|||
|2023|||
|2024|||

### Structure

In general the file structure will follow the following structure. Expections may 
apply, depending on the requirements of the challange. The real-input.txt files 
will always be omitted. Any processed data (for example caches written to disk) 
will also be omitted. The code should generate this data anyways.

```
./<year>/<day>/aoc-<year>-<day>-part[1|2]/
./<year>/<day>/example-input.txt
```

There is also a shared library with common functions to read in files etc in 
`./shared`.