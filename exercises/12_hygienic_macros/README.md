# Hygeine

By default, all identifiers referred to in a macro are expanded as-is, and are
looked up at the macro's invocation site. This can lead to issues if a macro
refers to an item or macro which isn't in scope at the invocation site. To
alleviate this, the `$crate` metavariable can be used at the start of a path to
force lookup to occur inside the crate defining the macro.

## Exercise 12

Exercise 12 consists of a file containing multiple modules. Fix the code so
that the macro works correctly in all invocations.

Note that you will need to use the `$crate` metavariable.
