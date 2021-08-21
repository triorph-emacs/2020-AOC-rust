## Advent of Code 2020 - Day 13

Just part b done, but also some tests.
I found a way to "brute-force" the answer without having to know about the
chinese remainder theorem. I was not particularly fond of that answer at
the time as it felt like I was just having to look up someone else's
algorithm and apply it without super understanding why it worked.

Instead, I found an algorithm that only needs to brute-force the address space
of each modulo. So if the we need to find:
`x mod a = b`
`x mod c = d`
`x mod e = f`
then at most we have to do `a` + `c` + `e` searches.

For `x mod a`, we increase by 1 each time until we find an `x` (in theory we just
set our first `x` to be `b` but I didn't bother special casing the first one)

FOr `x mod c = d`, we increase by `a` every time, so that we know the previous rule
stays true and don't need to recalculate it.

For `x mod e = f` we increase by `a*c` every time, as both previous rules are true
and we don't need to recalculate them.

### Tests

Tested both the parser and the time calculator using the examples on the advent of code site.

Note: The parser still has the "start time" from day a, although I have not written the code
for day a here. I vaguely remember there was not too much to that one when I did it in python.
