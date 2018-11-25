# Writing Parsers and Cutting Corners or:

### Unboxing the `Box`es. Memory layout and how to make things tighter and faster.

## Chapter 3.

#### What should I do here?

* Try to migrate the code from previous chapter to this one and make it compile and run.
* Look over docs for [`toolshed`](https://crates.io/crates/toolshed). Find another crate for an `Arena` allocator if you'd like, or try to write your own if you aren't afraid of using `unsafe`!

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

Ok, so what is happening here? The `struct`s in Rust, unlike something like objects in JavaScript, are fairly low level and by default they live on the *stack*. The practical implication of that is that Rust needs to know how many bytes on stack said `struct` is going to take, and should you introduce some recursion Rust is actually clever enough to detect that and inform us about it. So, instead of putting a `struct` inside a `struct`, what we can do is put a *pointer* to the child struct instead. Here too the Rust compiler gives us some suggestions, and the first suggestion is what our choice was: `Box`. `Box<T>` will allocate memory for `T` on the heap and put its value there, while it itself is just a pointer to that location in memory, which is to say that it also has a fixed size: 4 bytes if you compile Rust for a 32-bit target, and more commonly today 8 bytes if you compile for a 64-bit target. This breaks our type recursion and our code compiles. But is this the best choice?

Truth is, it depends. `Box` is one of the most universal pointers in Rust and you can't really go wrong with it in terms of ergonomics, but it does incur the cost of doing a *heap allocation*, that is - your program has to ask for a bit of free memory somewhere where it can fit `T`. This takes some time on the scale of nanoseconds, so for most purposes it really is just fine. That said, if you are parsing a really large source, or if you want to target a platform where heap allocation either isn't available or would be more of a performance hiccup, we can look for alternatives.

#### Enter the `Arena`

There is one assumption we can make about the AST our Parser produces is that it's likely that it's all going to be put in memory in one go. Some transformations can be done on it, it might be printed out to source code again, or translated to an AST of some other language, and then discarded all at once. That might not always be true, particularly if your Parser is part of something like the [Rust Language Server](https://github.com/rust-lang/rls), but for a lot of cases it will be true. If that assumption holds, then we can do something clever and use an **Arena**.

The general principle here is that you do a heap allocation once, and then put all your stuff on that big array of memory. The whole process of the allocator looking for some place in memory where it can fit it turns into a single increment of a pointer. Implementing such an Arena might require some *unsafe* magic, but it is a problem that is easy to encapsulate. Rust lifetimes also become very handy here to ensure that our tree does not outlive the Arena it was allocated on. This is making something that would be a nightmare in C, especially with multiple people involved, actually very pleasant to work with (assuming the underlying primitives are implemented correctly).

So with that, let's try to rewrite the Parser from Chapter 2 using Arena allocations and benchmark the difference!

#### Bonus material

If you want to learn more about *heap* and *stack* memory in Rust, and which types use what, take a look at the excellent [Rust container cheat sheet](https://docs.google.com/presentation/d/1q-c7UAyrUlM-eZyTo1pd8SZ0qwA_wYxmPZVOQkoDmH4/edit#slide=id.p).
