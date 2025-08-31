Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?

Uh oh - the Accounting-Elves have realized that they double-counted everything red.

Ignore any object (and all of its children) which has any property with the value "red". Do this only for objects ({...}), not arrays ([...]).

The example-input.txt input should give the answer 14.

### Usage
```bash
cargo run -- "<input file>" [--debug]
```