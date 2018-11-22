# Writing Parsers and Cutting Corners or:

### How I Learned to Stop Worrying and Love ASCII

## What is *Lexical Analysis*? Do you need it? Let's write a Lexer!

Over the course of those chapters we are going to be building a parser for a simple language that can do some math. The first thing we might want to do is do Lexical Analaysis, or write a *Lexer*, sometimes also called a *Tokenizer*.

Programming languages, despite being used for writing programs that run on computers, are really *human languages*, not that different from English or German, probably more German than English I'd say, *all that recursion and stack unwinding when you finally read a verb at the end of a sentence...* Anyway, I digress...

Lexical Analysis helps us by reducing the scope of what we are trying to achieve (parse a language) to a fairly simple task that is much more manageable: figure out what individual *bits* of the source code are? When you look at a screen full of text or computer code, you immediately see things: words, numbers, punctuation. Your brain really is quite amazing at detecting those things, and it can detect those things before it knows what it means, or if you happen to be looking at some German or Polish (pick the right one for you), it can detect them even if it has no idea what they mean. Congratulations! You now roughly know what Lexical Analysis is.

Your computer, for all it's speed, is really just a dumb bunch of silicon wrapped in plastic and/or aluminum or other metals. It does not have your brain in it. When it reads some source code from memory, it's just a bunch of bits of a given length, it has no idea what it is. Before we try to make the computer what the source code _means_, we can first tell it what it consists of - what are the bits, the words, **the Tokens**.
