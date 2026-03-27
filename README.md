# `assert-order`
A Rust crate for asserting the definition order of enum Variants.

## Motivation
For serialization formats that aren't self-describing, the typical
way for discriminating enum variants is to use an integer
discriminant. Unfortunately, this means that reordering variants
will cause deserialization of previously serialized data to fail
(potentially silently).

In general, you _can_ just document strict ordering and ensure that
changes aren't made to the variant ordering. However, it is more
comforting to have this documented requirement as an assertion in a
test.

## License
Licensed under the MIT license.
