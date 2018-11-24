# Writing Parsers and Cutting Corners or:

### Unboxing the `Box`es. Memory layout and how to make things tighter and faster.

## Chapter 3.

#### What should I do here?

* Look at the code in the [`src`](src) directory, those are all analogous to previous chapter.
* Try to migrate the code from previous chapter to this one and make it compile and run.
* Read on `toolshed`, find another crate or build an arena allocator yourself if you feel brave!

#### Unboxing the `Box`es. Memory layout and how to make things tighter and faster.

So now that our Parser is working, we are done, we can pack our laptops and go home!

That, or we could sit down and see if we can make things faster, all while trying to understand what Rust is doing to our memory. We've been using a `Vec` for our `Program` body, as well as `Box`es within the `BinaryExpression` and `UnaryExpression`. If you feel like you are fairly familiar with both of those types, feel free to skip to the next headline.

Let's start with a `Box`, because it's both simpler and less intuitive (especially if you come from a Garbage Collected language) why we need them. If you replaced the `Box<Expression>` with just `Expression` the Rust compiler would give us an interesting error:

```
error[E0072]: recursive type `ast::BinaryExpression` has infinite size
  --> src/ast.rs:58:1
   |
58 | pub struct BinaryExpression {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ recursive type has infinite size
59 |     pub left: Expression,
   |     -------------------- recursive without indirection
   |
   = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `ast::BinaryExpression` representable
```

Ok, so what is happening here? The `struct`s in Rust, unlike something like objects in JavaScript, are fairly low level and by default they live on the *stack*. The practical implication of that is that Rust needs to know how many bytes on stack said `struct` is going to take, and if introduce some recursion Rust is actually clever enough to detect that and inform us. So, instead of putting a `struct` inside a `struct` what we can do is put a *pointer* to the child. Here too the Rust compiler gives us some suggestions, and the first suggestion is what our choice was: `Box`. `Box<T>` will allocate memory for `T` on the heap and put its value there, while it itself is just a pointer to that location in memory, which is to say that it also has a fixed size: 4 bytes if you compile Rust for a 32-bit target, and more commonly today 8 bytes if you compile for a 64-bit target. This breaks our type recursion and our compiles, but is this the best choice?

Truth is, it depends. `Box` is one of the most universal pointers in Rust and you can really go wrong with it in terms of ergonomics, but it does incur the cost of doing a *heap allocation*, that is - your program has to ask for a bit of free memory somewhere where it can fit T, which takes some time on the scale of nanoseconds, so for most purposes it really is just fine. That said, if you are parsing a really large source, or if you want to target a platform where heap allocation either isn't available or would be more of a performance hickup, we can look for alternatives.

#### Enter `Arena`

