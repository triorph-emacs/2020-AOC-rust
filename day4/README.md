### Day 4

Added unit tests from the example as I went.

At first I did this using a HashMap, as that's the kind of quick way I would do this in Python.
However I think that is limiting the way I think from how it should be done in Rust. Instead
I have gone with an enum with match method for dispatching the correct validate call for the
enum.

Was a bit fiddly getting all the syntax right, but once I did the actual code just fell into
place and mostly just worked.

##### One downside:

HashMap gave an easy way of checking that all 7 values provided were not the same. Enums
(especially when we add a value into them) are more difficult to compare this way.
Alternatively we could have made a tuple type of enum + val and could have made a HashSet
of the enum type (now limited to 7) and checked the length. My "hacky" approach was to
just check the length. This works because all the input data only provides each field once,
so we don't have to check for that anyway. Notably, a HashMap wouldn't necessarily work
better at failing if we have 2 entries of the same name, so the tuple approach could be
best.
