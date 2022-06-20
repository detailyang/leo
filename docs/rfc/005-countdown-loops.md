# Leo RFC 005: Countdown Loops

## Authors

The Aleo Team.

## Status

IMPLEMENTED

# Summary

This proposal suggests adding countdown loops and inclusive loop ranges into the Leo language.

# Motivation

In the current design of the language only incremental ranges are allowed. Though
in some cases there's a need for loops going in the reverse direction. These examples
demonstrate the shaker sort and bubble sort algorithms where countdown loops are mocked:

```ts
function shaker_sort(a: [u32; 10], const rounds: u32) -> [u32; 10] {
    for k in 0..rounds {
        for i in 0..9 {
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }

        }
        for j in 0..9 { // j goes from 0 to 8
            let i = 8 - j; // j is flipped
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }
        }
    }
    return a;
}
```

```ts
function bubble_sort(a: [u32; 10]) -> [u32; 10] {
    for i in 0..9 { // i counts up
        for j in 0..9-i { // i is flipped
            if (a[j] > a[j+1]) {
                let tmp = a[j];
                a[j] = a[j+1];
                a[j+1] = tmp;
            }
        }
    }
    return a
}
```

Having a countdown loop in the examples above could improve readability and
usability of the language by making it more natural to the developer.

However, if we imagined this example using a countdown loop, we would see that
it wouldn't be possible to count to 0; because the first bound of the range is
inclusive and the second is exclusive, and loops ranges must use only unsigned integers.

```ts
// loop goes 0,1,2,3,4,5,6,7,8
for i in 0..9 { /* ... */ }

// loop goes 9,8,7,6,5,4,3,2,1
for i in 9..0 { /* ... */ }
```

Hence direct implementation of the coundown loop ranges would create asymmetry (1)
and would not allow loops to count down to 0 (2). To implement coundown loops and
solve these two problems we suggest adding an inclusive range bounds.

# Design

## Coundown loops

Countdown ranges do not need any changes to the existing syntax. However their
functionality needs to be implemented in the compiler.

```ts
for i in 5..0 {}
```

## Inclusive ranges

To solve loop asymmetry and to improve loop ranges in general we suggest adding
inclusive range operator to Leo. Inclusive range would extend the second bound
of the loop making it inclusive (instead of default - exclusive)
therefore allowing countdown loops to reach 0 value.

```ts
// default loop: 0,1,2,3,4
for i in 0..5 {}

// inclusive range: 0,1,2,3,4,5
for i in 0..=5 {}
```

## Step and Direction

We remark that the step of both counting-up and counting-down loops is implicitly 1;
that is, the loop variable is incremented or decremented by 1.

Whether the loop counts up or down is determined by how the starting and ending bounds compare.
Note that the bounds are not necessarily literals;
they may be more complex `const` expressions, and thus in general their values are resolved at code flattening time.
Because of the type restrictions on bounds, their values are always non-negative integers.
If `S` is the integer value of the starting bound and `E` is the integer value of the ending bound,
there are several cases to consider:
1. If `S == E` and the ending bound is exclusive, there is no actual loop; the range is empty.
2. If `S == E` and the ending bound is inclusive, the loop consists of just one iteration; the loop counts neither up nor down.
3. If `S < E` and the ending bound is exclusive, the loop counts up, from `S` to `E-1`.
4. If `S < E` and the ending bound is inclusive, the loop counts up, from `S` to `E`.
5. If `S > E` and the ending bound is exclusive, the loop counts down, from `S` to `E+1`.
6. If `S > E` and the ending bound is inclusive, the loop counts down, from `S` to `E`.

Cases 3 and 5 consist of one or more iterations; cases 4 and 6 consist of two or more iterations.

## Examples

The code examples demostrated in the Motivation part of this document
could be extended (or simplified) with the suggested syntax:

```ts
function shaker_sort(a: [u32; 10], const rounds: u32) -> [u32; 10] {
    for k in 0..rounds {
        for i in 0..9 { // i goes from 0 to 8
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }

        }
        for i in 8..=0 { // i goes from 8 to 0
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }
        }
    }
    return a;
}
```

```ts
function bubble_sort(a: [u32; 10]) -> [u32; 10] {
    for i in 9..0 { // counts down
        for j in 0..i { // no flipping
            if (a[j] > a[j+1]) {
                let tmp = a[j];
                a[j] = a[j+1];
                a[j+1] = tmp;
            }
        }
    }
    return a
}
```

# Drawbacks

No obvious drawback.

# Effect on Ecosystem

Suggested change should have no effect on ecosystem because of its backward compatibility.

# Alternatives

## Mocking

Coundown loops can be mocked manually.

## Exclusive Starting Bounds

While the ability to designate the ending bound of a loop as either exclusive or inclusive is critical as discussed below,
we could also consider adding the ability to designate the starting bound of a loop as either exclusive or inclusive.
If we do that, we run into a sort of asymmetry in the defaults for starting and ending bounds:
the default for the starting bound is inclusive, while the default for ending bounds is exclusive.

The most symmetric but verbose approach is exemplified as follows:
* `0=..=5` for `0 1 2 3 4 5`
* `0<..=5` for `1 2 3 4 5`
* `0=..<5` for `0 1 2 3 4`
* `0<..<5` for `1 2 3 4`
* `5=..=0` for `5 4 3 2 1 0`
* `5>..=0` for `4 3 2 1 0`
* `5=..>0` for `5 4 3 2 1`
* `5>..>0` for `4 3 2 1`
That is, this approach makes exclusivensss and inclusiveness implicit.
The use of `<` vs. `>` also indicates a loop direction, which can be inferred anyhow when the `const` bounds are resolved,
so that would entail an additional consistency check,
namely that the inequality sign/signs is/are consistent with the inferred loop direction.

Within the symmetric approach above, there are different options for defaults.
The most symmetric default would be perhaps `=` for both bounds,
but that would be a different behavior from current Leo.
We could instead go for different defaults for starting and ending bounds,
i.e. `=` for the starting bound and `<` or `>` (depending on direction) for the ending bound.

A drawback of this approach is that it is somewhat verbose.
Furthermore, some of the authors of this RFC do not find it very readable.

## Flipping Bound Defaults for Countdown

In the proposed design, there is an asymmetry between the treatment of loops that count up vs. down.
This can be seen clearly by thinking how to iterate through an array of size `N`:
```ts
for i in 0..n { ... a[i] ... } // count up -- 0 1 2 ... n-1
for i in n-1..=0 { ... a[i] ... } // count down -- n-1 ... 2 1 0
```
While the loop that counts up has nice and simple bounds `0` and `n`,
the loop that counts down needs `n-1` and `=0`.

So a possible idea is to use different defaults depending on the loop direction:
* For a loop that counts up:
  * The starting (i.e. lower) bound is always inclusive.
  * The ending (i.e. upper) bound is exclusive by default, inclusive with `=`.
* For loop that counts down:
  * The ending (i.e. lower) bound is always inclusive.
  * The starting (i.e. upper) bound is exclusive by default, inclusive with `=`.

That is, different defaults apply to lower vs. upper bound, rather than to starting and ending bounds.

Things become more symmetric in a way:
```ts
for i in 0..n { ... a[i] ... } // count up -- 0 1 2 ... n-1
for i in n..0 { ... a[i] ... } // count down -- n-1 ... 2 1 0
```

This is also consistent with Rust in a way,
where countdown loops are obtained by reversing the increasing range into a decreasing range, which flips the bounds.

However, if we consider a possible extension in which the step may be larger than 1, we run into some awkwardness.
Imagine an extension in which `step` is specified:
```ts
for i in 10..0 step 2 ... // i = 8 6 4 2 0 -- starts at 10-2 = 8
for i in 10..0 step 3 ... // i = 9 6 3 0 -- starts at 10-1 = 9
```

Note how the actual starting index does not depend on starting/upper bound and step,
but rather on ending/lower bound and step, and must be calculated explicitly;
it doesn't "jump" at the reader.

## Explicit Indication of Loop Direction

Another idea that was brought up is to always write the range as `<lower>..<upper>`,
but include an explicit indication when the loop must count down, e.g.
```ts
for i in 0..n down { ... array[i] ... } // where 'down' indicates count down
```

The advantages are that
we retain the default that the first/lower bound is inclusive and the second/upper bound is exclusive,
and the direction is explicit and does not have to be inferred.
The direction matches starting/ending bound to lower/upper bound or upper/lower bound.

But the awkwardness with larger steps than 1 remains:
```ts
for i in 0..10 down step 2 ... // i = 8 6 4 2 0 -- starts at 10-2 = 8
for i in 0..10 down step 3 ... // i = 9 6 3 0 -- starts at 10-1 = 9
```

## Variable in the Middle of Range with Equalities or Inequalities

Another approach is to put the variable in the middle of the range,
along with equality or inequality signs around the variable, e.g.
```ts
  for 0 <= i < 5   // 0 1 2 3 4
  for 0 <= i <= 5   // 0 1 2 3 4 5
  for 5 > i >= 0    // 4 3 2 1 0
```

This maximizes explicitness, but it may need tweaking to avoid parsing ambiguities or difficulties
(recall that the bounds may be complex `const` expressions).

This could be a future addition to consider, but it seems that it would not replace the current Rust-like syntax.
