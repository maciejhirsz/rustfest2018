# Writing Parsers and Cutting Corners or:

### How I Learned to Stop Worrying and Love ASCII

## Foreword

Hey there 👋!

This workshop is going to take you through the art of writing parsers. The chapters are designed to follow each other, with one building on the learnings from before, but if you feel brave don't let me stand in your way and look through this repo to figure out what it is YOU really want to try and learn. All chapters come with a README like this one, which should make this a pretty straight forward process, as well as an exercise in writing Rust code that should help you figure this stuff out.

The READMEs also contain a bunch of different tips and tricks that might help you and explain things to you. I'll be also presenting some of this stuff at the beginning and throughout the workshop, so feel free to just jump around and come back to read on things you might find interesting. Also, feel free to just ask questions, either to me or to your fellow Rustaceans 🦀!

## What is all this fuzz about anyway?

Parsers are incredibly useful! They power our compilers, APIs, Web Browsers, Terminals, you name it. There are specialized Parsers for different tasks and formats, some are text-based (either ASCII, Unicode or any other encoding), others just read binary data. The Rust community has already created a bunch of crates for writing parsers like [pest](https://crates.io/crates/pest), [nom](https://crates.io/crates/nom) and [combine](https://crates.io/crates/combine), as well a plethora of parsers for different things, with the entire [Serde](https://crates.io/crates/serde) ecosystem being a great start for anything pure-data related.

This workshop is not about learning how to use those tools, although they are very useful and you might find yourself using 3rd party crates for some exercises to make your life easier. This workshop is about **understanding** how parsing works from the ground up, and I strongly believe that the best way to understand a system is to try to build it. The second best way would be to find one that's already working and trying to break it ;).

We will also have somewhat of a focus on parsing something resembling a programming language. That is not the easiest thing to do, which is exactly why it's the most interesting thing to! If that sounds intimidating to you, don't worry, it really isn't *that* hard. When I first had to explain not just parsers, but *compilers* at a JavaScript meetup to a crowd of programmers of very varying degree of proficiency in 5 minutes, the thought that put that presentation on track (and should put you on track) was _"well, it's not rocket science... or is it?"_. You see, rocket science (and it really is not a thing, I've been told) is actually really simple - to paraphrase myself from the past: you all know how a rocket works - some fuel gets burnt and it pushes the thing up - it's actually building the thing and sending it to the moon that's hard.

## Chapters

* [**Chapter 1**](chapter_1): What is *Lexical Analysis*? What is a *Token*? Do you need it? Let's write a Lexer!
* [**Chapter 2**](chapter_2): Plant a tree! What is an *Abstract Syntax Tree*? Who is *Vaughan Pratt*? Let's write a Parser!
* [**Chapter 3**](chapter_3): Unboxing the `Box`es. Memory layout and how to make things tighter and faster.
* [**Chapter 3**](chapter_4): Jumping around *Lookup Tables*. Why does **C** have strings that end with **`nul`**?
