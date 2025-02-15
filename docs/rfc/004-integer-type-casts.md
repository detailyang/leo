# Leo RFC 004: Integer Type Casts

## Authors

- Max Bruce
- Collin Chin
- Alessandro Coglio
- Eric McCarthy
- Jon Pavlik
- Damir Shamanaev
- Damon Sicore
- Howard Wu

## Status

DRAFT

# Summary

This proposal provides support for casts among integer types in Leo.
The syntax is similar to Rust.
Two possible semantics are discussed:
_value-preserving casts_,
which just serve to change types
but cause errors when values are not representable in the new types;
and _values-changing casts_,
which never cause errors but may change the mathematical values.

# Motivation

Currently the Leo integer types are "siloed":
arithmetic integer operations require operands of the same type
and return results of the same type.
There are no implicit or explicit ways to turn, for example,
a `u8` into a `u16`, even though
every non-negative integers that fits in 8 bits also fits in 16 bits.
However, the ability to convert values between different (integer) types
is a useful feature that is normally found in programming languages.

# Design

## Background

Recall that Leo supports the following _integer types_:
```
u8 u16 u32 u64 u128
i8 i16 i32 i64 i128
```

## Scope

This RFC proposes type casts between any two integer types,
but not between two non-integer types
or between an integer type and a non-integer type.

This RFC does not propose any implicit cast,
even widening casts (i.e. upcasts)
from a type to another type with the same signedness
and with the same or larger size
(e.g. from `u8` to `u16`).
All the type casts must be explicit.

## Syntax and Static Semantics

The proposed syntax is
```
<expression> as <integer-type>
```
where `<expression>` must have an integer type.

The ABNF grammar is modified as follows:
```
; add this rule:
cast-expression = unary-expression
                / cast-expression %s"as" integer-type

; modify this rule:
exponential-expression = cast-expression
                       / cast-expression "**" exponential-expression
```
There is no need to modify the `keyword` rule
because it already includes `as` as one of the keywords.
Note the use of `integer-type` in the `cast-expression` rule;
an alternative is to use `type` there
and check post-parsing that the type is in fact an integer one.

The above grammar rules imply that casts bind
tighter than binary operators and looser than unary operators.
For instance,
```
x + - y as u8
```
is like
```
x + ((- y) as u8)
```
This precedence is the same as in Rust:
see [here](https://doc.rust-lang.org/stable/reference/expressions.html#expression-precedence).

## Dynamic Semantics

When the mathematical integer value of the expression
is representable in the type that the expression is cast to,
there is no question that the cast must succeed
and merely change the type of the Leo value,
but not its mathematical integer value.
This is always the case when the cast is to a type
with the same signedness and with the same or larger size.
This is also the case when
the cast is to a type whose range does not cover the range of the source type
but the value in question is in the intersection of the two ranges.

When the mathematical integer value of the expression
is not representable in the type that the expression is cast to,
there are two possible approaches, discussed below.

### Value-Preserving Casts

The first approach is to deem that situation erroneous.
That is, to require casts to always preserve the mathematical integer values.

In this approach, casts only serve to change types, never values.
When values are to be changed, separate (built-in) functions can be used,
e.g. to mask bits and achieve the same effect as
the value-changing casts discussed below.

From a point of view, this approach seems to match Leo's
treatment of potentially erroneous situations like integer overflows:
the principle is that developers should explicitly use
operations that may overflow if that is their intention,
rather than having those situation possibly occur unexpectedly.

A value-preserving cast to a type
whose range does not cover the original type's range
implicitly expresses a developer expectation that the value
is actually in the intersection of the two types' ranges,
in the same way that the use of integer addition
implicitly expresses the expectation that the addition does not overflow.

Consider this somewhat abstract example:
```
... // some computations on u32 values, which could not be done with u16
let r: u32 = ...; // this is the final result of the u32 operations above
let s: u16 = r as u16; // but r is expected to fit in u16, so we cast it here
```
With value-preserving casts, the expectation mentioned above
is checked by the Leo compiler during proof generation,
in the same way as with integer overflow.

In the example above,
if instead the variable `s` is meant to contain the low 16 bits of `r`,
e.g. in a cryptographic computation,
then the value-preserving cast should be preceded by
an explicit operation to obtain the low 16 bits, making the intent clear:
```
... // some computations on u32 values, which could not be done with u16
let r: u32 = ...; // this is the final result of the u32 operations above
let r_low16: u32 = r & 0xFFFF; // assuming we have bitwise ops and hex literals
let s: u16 = r_low16 as u16; // no value change here
```

### Value-Changing Casts

The second approach is the following:
 1. `uN` to `uM` with `N < M`: just change type of value.
 2. `uN` to `uM` with `N > M`: take low `M` bits of value.
 3. `iN` to `iM` with `N < M`: just change type of value.
 4. `iN` to `iM` with `N > M`: take low `M` bits of value.
 5. `uN` to `iM` with `N < M`: zero-extend to `M` bits and re-interpret as signed.
 6. `uN` to `iM` with `N > M`: take low `M` bits and re-interpret as signed.
 7. `uN` to `iN`: re-interpret as signed
 8. `iN` to `uM` with `N < M`: sign-extend to `M` bits and re-interpret as unsigned.
 9. `iN` to `uM` with `N > M`: take low `M` bits and re-interpret as unsigned.
10. `iN` to `uN`: re-interpret as unsigned
Except for the 1st and 3rd cases, the value may change.

This approach is common in other programming languages.
However, it should be noted that other programming languages
typically do not check for overflow in integer operations either
(at least, not for production code).
Presumably, the behavior of type casts in those programming languages
is motivated by efficiency of execution, at least in part.
Since in Leo the input data is available at compile time,
considerations that apply to typical programming languages
do not necessarily apply to Leo.

Back to the somewhat abstract example in the section on value-preserving casts,
with value-changing casts, the expectation that the final result fits in `u16`
would have to be checked with explicit code:
```
... // some computations on u32 values, which could not be done with u16
let r: u32 = ...; // this is the final result of the u32 operations above
if (r > 0xFFFF) {
    ... // error
}
let s: u16 = r as u16; // could change value in principle, but does not here
```
However, it would be easy for a developer to neglect to add the checking code,
and thus have the Leo code silently produce an unexpected result.

## Compilation to R1CS

It should be possible to compile Leo casts to the same R1CS constraints
whether we choose the value-preserving or value-changing semantics.
If the R1CS constraints represent Leo integers as bits,
the bits of the new value can be determined from the bits of the old value,
with additional zero or sign extension bits when needed.
This is clearly the value-changing behavior.
With the value-preserving behavior,
all casts for the known inputs are checked,
and thus we value-changing behavior coincides
with the value-preserving behavior if the checks succeed.
Thus, if he behavior of the R1CS constraints is "don't care"
for Leo program inputs that cause errors (such as cast errors),
the compilation strategy for value-changing casts
should be also adequate for value-preserving casts.

# Drawbacks

This proposal does not appear to bring any drawbacks,
other than making the language and compiler inevitably more complex.
But the benefits to support type casts justifies the extra complexity.

# Effect on Ecosystem

This proposal does not appear to have any direct effects on the ecosystem.

# Alternatives

The 'Design' section above already discusses two alternative semantics.
After we settle on one, the other one could be mentioned in this section.
