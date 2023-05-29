# Exercise 11: Macro Recursion

This exercise is a sort of culmination of everything you've learned so far about macros.

To complete it, you'll need to note one important fact: macros can recurse into themselves.

This allows very powerful expansions. As a simple example:

``` rust

enum LinkedList {
    Node(i32, Box<LinkedList>),
    Empty
}

macro_rules! linked_list {
    () => {
        LinkedList::Empty
    };
    ($expr:expr $(, $exprs:expr)*) => {
        LinkedList::Node($expr, Box::new(linked_list!($($exprs),*)))
    }
}

fn main() {
    let my_list = linked_list!(3, 4, 5);
}
```

The above example is very typical. The first rule is the "base case": an empty
list of tokens implies an empty linked list.

The second rule always matches one expression first (`expr`). This allows us
to refer to it on its own, in this case to create the `Node`. The rest of
the expressions (`exprs`) are stored in a repetition, and all we'll do with
them is recurse into `linked_list!()`. If there's no expressions left,
that call to `linked_list!()` will give back `Empty`, otherwise it'll
repeat the same process.

While macro recursion is incredibly powerful, it is also slow. As a result,
there is a limit to the amount of recursion you are allowed to do.
In rustc, the limit is `128`, but you can configure it with
`#![recursion_limit = "256"]` as a crate-level attribute.


## Exercise 11: Currying

Before you complete the exercise, let's briefly discuss a concept called "currying".
If you're already familiar with the concept, perhaps from your own experience of
functional programming, you can skip the next two paragraphs.

In most imperative languages, the syntax to call a function with multiple arguments
is `function(arg1, arg2, arg3)`. If you do not provide all the arguments, that is
an error. In many functional languages, however, the syntax for function calls is
more akin to `function(arg1)(arg2)(arg3)`. The advantage of this notation is that
if you specify less than the required number of arguments, it's not an error:
you get back a function that takes the rest of the arguments. A function that behaves
this way is said to be "curried" (named after Haskell Curry, a famous mathematician).

A good example of this is a curried `add` function. In regular Rust, we'd say `add` is
`move |a, b| a + b`. If we curried that function, we'd instead have `move |a| move |b| a + b`.
What this means is that we can write `let add_1 = add(1);`, and we now have a function
which will add 1 to anything.

In this exercise, you will build a macro which helps you understand currying,
and build a curried function in Rust. The syntax for this macro will be
`curry!((a: i32) => (b: i32) => _, {a + b})`. Each pair of `ident: ty` is an
argument, and the last `_` indicates that the compiler will infer the return
type. The block provided last is, of course, the computation we want to do after
receiving all the arguments.

Each step of the currying process, you should call the macro `print_curried_argument`.
This takes in a value (which, for the purposes of the exercise, you can assume will 
always be `Copy`). It will print out the value that you have been provided as an argument.

