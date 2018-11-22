# Writing Parsers and Cutting Corners or:

### How I Learned to Stop Worrying and Love ASCII

## What should I do here?

* Look at the code in the [`src`](src) directory.
* Run `cargo test` to see that the test there compiles but fails.
* Make it pass :).
* Read on for more information.
* If this seems too easy, I'd still recommend reading below before skipping to the next chapter.

## What is *Lexical Analysis*? Do you need it? Let's write a Lexer!

Over the course of those chapters we are going to be building a parser for a simple language that can do some math. The first thing we might want to do is do Lexical Analaysis, or write a *Lexer*, sometimes also called a *Tokenizer*.

Programming languages, despite being used for writing programs that run on computers, are really *human languages*, not that different from English or German, probably more German than English I'd say, *all that recursion and stack unwinding when you finally read a verb at the end of a sentence...* Anyway, I digress...

Lexical Analysis helps us by reducing the scope of what we are trying to achieve (parse a language) to a fairly simple task that is much more manageable: figure out what individual *bits* of the source code are? When you look at a screen full of text or computer code, you immediately see things: words, numbers, punctuation. Your brain really is quite amazing at detecting those things, and it can detect those things before it knows what it means, or if you happen to be looking at some German or Polish (pick the right one for you), it can detect them even if it has no idea what they mean. Congratulations! You now roughly know what Lexical Analysis is.

Your computer, for all it's speed, is really just a dumb bunch of silicon wrapped in plastic and/or aluminum or other metals. It does not have your brain in it. When it reads some source code from memory, it's just a bunch of bits of a given length, it has no idea what it is. Before we try to make the computer what the source code _means_, we can first tell it what it consists of - what are the bits, the words, **the Tokens**.

## Do you need a Lexer?

Having your Parser depend on a Lexer introduces some overhead, that is to say, there is probably a way to make a parser without it, and with all things in programming, *the less your code does, the faster it does it*.

For a lot of things Lexers might be a bit redundant when matching on bytes in Parsers directly does the job. This can be true for things like JSON, where `'{'`, `'['`, `'0'...'9'`, `'n'`, `'t'`, `'f'` and `'"'` can only have one possible outcome: object, array, number, `null`, `true`, `false` or a string, in that order.

Lexers do bring a lot of clarity into your code however. The separation of concerns between Lexer and Parser will make reasoning about the latter much easier, *especially* if the grammar of the code you are trying to parse it is complex (and it is for virtually any modern language that we write interesting code in).

## Cutting corners: `char`s or `u8` bytes?

You might have noticed that I just wrote *"matching on bytes"*, not *"matching on characters"*. We want to parse a `&str`, which is UTF-8 encoded text in a slice of bytes. Calling `.chars()` on a `&str` gives an iterator of `char`s that the string represents, but those `chars` are **not** what is stored inside the string. The `Chars` iterator from the standard library is really fast, but if you have learned anything from last segment, it is that doing some work very fast is still slower than not doing that work *at all*. So it goes with converting bytes to `chars`.

Unicode, the thing that UTF-8 encodes on 8-bit chunks we call bytes, has one amazing property: the first 127 codepoints are exactly equal to ASCII! ASCII is from the 60s, it's so old that some of it's number values are reserved for things like *physically moving a typewriter head*, *ringing a bell* or *delete*. That's right, there is a character in ASCII that deletes the previous character - you can send two bytes that equate to exactly nothing at all. It's kind of amazing. Also, it's not exactly bytes, ASCII encodes on just 7 bits, not 8, which is one property of ASCII that UTF-8 exploits.

Consider having a string "ę". Those are two bytes: 196 and 153. Let's see how they look as binary:

```
+-----------------+-----------------+
| Byte 0          | Byte 1          |
+-----------------+-----------------+
| 1 1 0 0 0 1 0 0 | 1 0 0 1 1 0 0 1 |
+-----------------+-----------------+
```

The first byte begins with two `1`s, the second byte begins with one `1`. Ok, now let's try an emoji: "🦀" is a sequence of 240, 159, 166, and 128. In binary:

```
+-----------------+-----------------+-----------------+-----------------+
| Byte 0          | Byte 1          | Byte 2          | Byte 3          |
+-----------------+-----------------+-----------------+-----------------+
| 1 1 1 1 0 0 0 0 | 1 0 0 1 1 1 1 0 | 1 0 1 0 0 1 1 0 | 1 0 0 0 0 0 0 0 |
+-----------------+-----------------+-----------------+-----------------+
```

The first byte begins with four `1`s, and all consequent bytes begin with one `1`, and in both cases those are always followed by at least one zero. When looking at a byte of UTF-8 encoded text you can therefore immediately tell that:

* If the first bit is set to `0`, that byte is a single byte that contains ASCII codepoint. Remember when I said that ASCII is just 7 bits? Turned out to be useful here!
* If the first bit is set to `1` and is followed by more `1` bits, then this is the first byte of a sequence of bytes that encode a full Unicode codepoint, with it's byte length set to the number of those `1` bits.
* If the first bit is set to `1` and is followed by `0`, that byte is part of a Unicode codepoint, but it did not begin it! This is also how Rust knows when to panic if you slice a `&str` at invalid indexes. Clever, huh?

This also means that if you are writing a Lexer, even if your source code permits UTF-8 (in strings for example), you can **safely** read it as just bytes and ignore any values larger than 127 (that is, any bytes that have the first bit set to `1`). As long as you don't interrupt the unicode sequences, and most if not all of your tokens are happy to be written in ASCII, you can iterate on bytes and save yourself the hassle (and the computer the work).

Also, Rust has nice first class byte literal support, so if `'a'` makes a `char`, `b'a'` makes a `u8`. Analogous if `"foobar"` is a `&str`, `b"foobar"` is a `&[u8]` slice.

Armed with this knowledge, happy hacking!
