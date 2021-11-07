## Day 7

Rules for which bags can hold which bags.
By following the lines of logic, we should be able to determine
a) Which bag colours can hold a shiny gold bag (4 in test data a)
b) How many individual bags can be held inside a shiny gold bag (126 in test data b)

Parsing the input data is getting to be more and more of a pain,
there's a library "Parse Expression Grammar" in rust that's worth checking out
for this explicit purpose

# Retrospective

Took me a long time to figure out how to get the PEG stuff working, especially
with "look-ahead" on the " bag" for the colour match.

I worked on a memoization of the bag calculation, which should help speed
it up, but actually timing it didn't seem to change anything.

I have basic unit tests for the main tests, as well as unit tests for all the PEG
parseing.

The main "learn something new" from this one was trying to get the PEG working.
The next step is to separate the core logic from the code into a library file.
