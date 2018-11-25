# Writing Parsers and Cutting Corners or:

### How I Learned to Stop Worrying and Love ASCII

## Chapter 4.

#### What should I do here?

* Read on and see if you can apply any of this to the work you've done in previous Chapters.

#### Jumping around *Lookup Tables*. Why does **C** have strings that end with **`nul`**?

If you've managed to get this far, congratulations! This last chapter doesn't have a source code on it's own, but is instead just a couple suggestions and ramblings on how to make things faster. In particular I want to touch on two, rather old, ways of doing things: *Lookup Tables*, sometimes called *LUT*s, and *nul-terminated* strings, that tend to go hand in hand. Those also are more relevant for the Lexer than the Parser itself, but you can use Lookup Tables for many problems.

The most common Lookup Table will be an array that has 256 elements exactly. Let's say you want to check that a byte is a letter, an upper case letter, a digit, `'_'` or `'$'`. You could write a `match` expression such as:

```rust
match byte {
    b'a'...b'z' |
    b'A'...b'Z' |
    b'0'...b'9' |
    b'_' | b'$' => true, // We have a match!
    _           => false, // bummer!
}
```

This is pretty nice and definitely readable! The compiler could do some optimizations here, Rust is usually pretty good at optimizing `match` expressions and can produce either Lookup Tables (for values, like here) or *Jump Tables* for branching. Unfortunately, occasionally it will get confused or decide some optimization is not worth it and fall back to a more direct approach.

That direct approach here would be to do couple of checks: is my byte greater or equal or `b'a'` and lesser or equal to `b'z'`, and so on... Instead, we could create an array of 256 `bool`s, with each index in that array corresponding to a different value of the byte. We can then quickly index into that array by casting `TABLE_NAME[byte as usize]`. Since the compiler knows that the array is 256 elements long exactly, and that is as high as the byte value can go, it is clever enough to know that it doesn't need to do bounds checking! If instead of `[bool; 256]` we used a slice `&[bool]`, that might not have held true and we could have to resort to `unsafe` Rust to get that optimization, risking *Undefined Behavior* if we can't guarantee that indexing really is always in bounds!

Here is just such a table in `serde_json`:

```rust
const CT: bool = true; // control character \x00...\x1F
const QU: bool = true; // quote \x22
const BS: bool = true; // backslash \x5C
const O: bool = false; // allow unescaped

// Lookup table of bytes that must be escaped. A value of true at index i means
// that byte i requires an escape sequence in the input.
#[cfg_attr(rustfmt, rustfmt_skip)]
static ESCAPE: [bool; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, // 0
    CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, CT, // 1
     O,  O, QU,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 2
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 3
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 4
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, BS,  O,  O,  O, // 5
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 6
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 7
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 8
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // 9
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // A
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // B
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // C
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // D
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // E
     O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O,  O, // F
];
```

This one checks if a byte in a JSON string literal needs to be escaped. It is also `static`, which means that we don't need to create it on stack every time we want to use it. I particularly like the formatting here - Lookup Tables are not very pretty, and even though this is just booleans, the particular values for different things are designated with their own `const`s that are all `true`. We can pull up an ASCII table from the Internet and see that byte `0x5C` is indeed a backslash, while byte `0x22` is a quote, neat!

#### Engage!

Another common thing to do with Lookup Tables is to put *function pointers* in them. This is a bit more of memory pet table, 2 kilobytes for such a table matching bytes on a 64-bit architecture, but still nothing we should worry about on modern hardware. Rust signature for such a table would look something like `[fn(); 256]` or, if we want to pass a reference to some mutable object (like a Lexer!): `[for<'a> fn(foo: &'a mut Foo; 256]`. Notice that we are using lowercase `fn`, not the upper case `Fn` trait! Regular functions that aren't closures and don't capture anything from the environment are static values just like `static FOO: usize = 42`, except instead of plain numbers the compiler understands them to be function pointers. If we want to do something for some bytes, but skip others (like whitespace), another common pattern would be to wrap our `fn` pointer in an `Option`: `[Option<for<'a> fn(foo: &'a mut Foo>; 256]`. Can you guess how much memory will such a table take? If you looked at the [bonus cheat sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p) from the last Chapter you might think it is still 2 kilobytes, and you would indeed be correct! That is because being a pointer means that Rust can reserve some specific value that definitely is not a valid pointer (a *null pointer* would be a natural pick here, but doesn't have to be) and use that value as the `None` variant.

Okay, this is all great, but there are some drawbacks of this approach that we need to be aware of. While using a Lookup Table to function pointers that we then call for different bytes is *O(1)* - that is, it has a constant, predictable performance - being a function call that the compiler can't predict does come at a cost. For one, there is no way for the compiler to *inline* any of the functions, so the result will be always creating a new stack frame and doing a jump in memory. Those are very fast operations, but they are operations that wouldn't have to be done for *inlined* functions! This *indirection* can also invalidate the instruction cache on the CPU, and that has it's own implications to performance, but we aren't going to go that deep here.

For our little toy Lexer/Parser here, using a Lookup Table like this might be an overkill, but for any sufficiently complex language that has tokens for every bracket type known to humanity, as well as a bunch of keywords, doing that O(1) indirect call can be substantially faster than doing a whole bunch of checks for each byte value.

#### `nul` termination is sometimes useful!

All strings in Rust are *fat pointers*, that is to say they aren't just pointers to some bytes somewhere in memory, but there is some extra information that gets copied around the stack along with the pointer: for `&str` it's pointer + length, for `String` it's pointer + length + capacity. Having the length known at all times is great, and being able to do slices by offsetting the pointer and reducing the length without having to do anything to the underlying memory is a brilliant idea, so it's definitely not something we are willing to give up on.

In the olden days, or today if you are still stuck writing C like it's the 80s, a string was just a pointer to some bytes without any information about the length. How did the computer know when to stop reading the string you might ask? The answer is really simple, all strings *had to* end with an extra byte, the `0` byte, designated `NUL` in ASCII. There is one advantage to doing things this way: it means you can keep on reading one byte at a time without having to do bounds checking, but you have to keep checking for `NUL` instead.

Now let's think about what we've just learned about Lookup Tables: instead of doing a whole bunch of checks for byte values, we can do a single indexing operation into an array. If the end of the string is not an extra check on length, but yet another value, we can just account for that in our Lookup Table with no extra cost, removing bounds checking altogether.

`toolshed`, the crate that I gave as a default in previous chapter, has a [`NulTermString`](https://docs.rs/toolshed/0.6.3/toolshed/struct.NulTermStr.html) just for those occasions. It still actually stores the length, so you can always cast it to a `&str`, but it also gives you the guarantee that the underlying memory will terminate with `NUL` should you go one byte over bounds. Just one, no more!

So, can you work out some optimizations with any of this? Also, if you have any other suggestions, please let me know :).
