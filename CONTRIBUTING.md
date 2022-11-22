# Contributing to Macrokata

Thank you for your interest in helping with MacroKata.

These instructions are probably incomplete (feel free to PR to add to them)!

## What are `solution.diff` files?

Since we have to maintain two separate files (the starter code, and the solution code),
it's very easy to make a change in one that affects the other.

Therefore, `solution.diff` files describe what the correct diff is between a starter
code and solution code. If you intentionally change this, use the `macrokata update-diff`
command to update the file. Part of any PR will be checking that this diff is sane.
