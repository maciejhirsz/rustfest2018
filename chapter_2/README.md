# Writing Parsers and Cutting Corners or:

### How I Learned to Stop Worrying and Love ASCII

## Chapter 2.

#### What should I do here?

* Look at the code in the [`src`](src) directory.
* Run `cargo test` to see that the test there compiles but fails.
* Make the tests pass pass :).
* Read on for more information.
* If this seems too easy, I'd still recommend reading below before skipping to the next chapter.

#### Plant a tree! What is an *Abstract Syntax Tree*? Who is *Vaughan Pratt*? Let's write a Parser!

Let's deal with the terminology first. Are now going to attempt to build a *Pratt Parser*, a variant of a *Recursive Descent Parser* that produces an *Abstract Syntax Tree*. All of that is quite a mouthful, but it isn't really that scary.

**_Recursive Descent_**, turns out, is both the easiest and most intuitive way to write a Parser for something that allows nested structures, whether that's JSON (in which objects can contain other objects) or a programming language in which expressions can contain other expressions. A common understanding of *recursion* in programming is a function calling itself to do something, but that is not the entire picture. In fact, recursion does not need to have loop-like characteristics at all (although more often than not it will). Instead, recursion is about *descending* from one frame of reference to the next, and then the next, to an arbitrary depth, and then having the ability to come back.

It's like having a painting of a scene that includes another painting, which in turn represents another scene (not necessarily the same) that contains another painting and so on. If you can imagine yourself stepping into the first painting, and then the next, and the next, would be doing a *recursive descent*.

In programming it is fairly straight forward - a function responsible for parsing the entire program would invoke a function responsible for parsing a statement, which can invoke a function responsible for parsing an expression, which can invoke itself again, hitting a *function expression*, which in turn invokes a function to parse that function's statements and so on. At every step of the descent we rely on the ability of your Parser's *stack* to keep track at which level of depth it is, so we don't have to think about it.

There are disadvantages to doing it this way. Most notably, if some mischievous gremlin were to write a sample that your Parser is supposed to parse, said gremlin could include nothing but a series of nested expressions in it, leading to an all too real *stack overflow* and crashing your Parser. If your Parser is running on a server, allowing user input to crash your server is probably not something you want! For that reason *Recursive Descent Parsers* can also increment some number to keep track of how deep in the stack they go, and gracefully error instead of crashing should that number become too high. For our workshop right now we aren't going to worry about it, but it is something to keep in mind.

#### The mysterious AST!

The *Abstract Syntax Tree*, or **AST**, is a representation of the program that the Parser is going to produce. If you look through the `src` for this chapter, you will see that in the `ast.rs` I took the liberty to prepare `struct`s and `enum`s that should be sufficient to create our AST.

 Since AST is, well, a Tree, it makes it reasonably easy to represent complex syntax of a language. Consider something quite simple like: `a = b * 2 + c`, in a tree we would represent it like this:

```
        Expression
            |
            |
     BinaryExpression
         /  |  \
        /  '='  `--------------.
       /                        \
  Identifier              BinaryExpression
      |                       /  |  \
     'a'            .--------'  '+'  \
                   /                  \
           BinaryExpression       Identifier
               /  |  \                 |
              /  '*'  \               'c'
             /         \
        Identifier    Number
            |           |
           'b'          2
```

A `BinaryExpression` is just an expression that combines exactly two other expressions with an operator, like doing addition or assigning values to variables. There are also *unary expressions* - an expression that can be either a *prefix* (e.g. `-5`) or *postfix* (`i++`) variant and combines a single other expression with an operator. In a lot of languages you will also see what people tend to call a *ternary expression* - `a ? b : c` - which is an accurate but weird way to call it, since all that "ternary" means is that it has 3 sub-expressions, not what it *does* with them.

Okay. Let's modify our input slightly to `a = b + 2 * c`, the tree is now going to look like this:

```
        Expression
            |
            |
     BinaryExpression
         /  |  \
        /  '='  `--------.
       /                  \
  Identifier        BinaryExpression
      |                 /  |  \
     'a'               /  '+'  `--------.
                      /                  \
                 Identifier        BinaryExpression
                     |                 /  |  \
                    'b'               /  '*'  \
                                     /         \
                                  Number    Identifier
                                    |           |
                                    2          'c'
```

Notice something? We only swapped the two operators - '+' and '\*' - and our tree looks different! That is because of *operator precedence* or *operator binding power*. You can most likely Google operator precedence tables for mostly any programming languages that aren't Lisps. Operator precedence tells us which operations should be performed first, in this case multiplication should be done before addition, and so the Parser we are going to write needs to be able to recognize that and *balance the tree* accordingly.

This is where our buddy **Vaughan Pratt** comes in! In a 1973 paper Pratt described a way to build a Recursive Descent Parser that handles operator precedence in a way that is simple, easy to understand and really quite *elegant*. Some Rustic-pseudocode would look like this:

```
fn parse_nested_expression(parser: Parser, mut left: Expression, lbp: u8) -> Expression {
    loop {
        let operator = match parser.get_next_operator() {
            Some(operator) => operator,
            None           => break,
        }

        if lbp < operator.binding_power() {
            break;
        }

        let right = parser.parse_simple_expression();
        let right = parser.parse_nested_expression(right, operator.binding_power());

        left = BinaryExpression {
            left,
            operator,
            right,
        };
    }

    left
}
```

So we loop and read operators for the next binary expression and compares the binding powers of the expression on the side with the binding power of the upcoming operator. Let's say we have a multiplication followed by addition, such as `a * b +`. Our stack will be aware of something like this:

* Parsing an expression produced `a`, we check if an operator follows, it does. Since `a` is an atomic expression that cannot be broken down, the `lbp` (left binding power) passed in should have the highest possible value, so we use `'*'` as an operator and attempt to construct the right hand side of the binary expression.
* We jump, or *descend* one stack frame down when calling `parse_simple_expression`. This will first give us an expression for the next token without bothering with operators, in our case we get `b`, we pop back and return to our loop.
* Ok, so now that we have `a * b` as a potential binary expression, should the right side be something more? We don't know, so we *descend* into another `parse_nested_expression`, passing on the binding power of the multiplication.
