# Advent of Code 2020 - Rust edition

By Michael Walsh.

## Intro

I have already done the AOC2020 challenges last year in Python. However I am very strong
in Python, and have been trying to learn Rust recently. I thought that these simple
but slowly escalating challenges are a good practice run for learning to use Rust better.

## Days

### Day 1

Have added the calculator and followed along with some of the things fasterthanli.me has done
on this one. The actual problem is so simple that most of this was struggling with Rust syntax.

### Day 2.

Another simple one, but I tried to do some slightly more advanced enum based error handling
using the TryFrom method in the parser.

### Day 3.

Instead of error handling here I just panic! (if the input data is bad there's not a lot to do).

Have actually added tests for the parser and using the advent of code examples.

### Day 4.

A little bit of error handling (but mostly panic around the input data). Lots of tests.

This is where I've first delved into using enums more instead of trying Pythonic ways of doing things.

### Day 5.

A bit of an easy one this time. Just have to convert some chars to binary values and find a missing number
in the middle. Nothing particularly novel to learn here. Still has tests for the individual parsing. Was
a good chance to practice using vim-sneak and quickscope which I just installed.

### Day 13. b.

Saw a novel approach to calculating this that I just wanted to try, even though I hadn't done the in betweens.

Similar to Day 3, this just panics if things are no good, but has unit tests for the parser and the given website examples.
