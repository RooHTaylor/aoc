Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?

The example-input.txt input should give the answer 27.

### Usage
```bash
cargo run -- "<input file>" [--debug]
```