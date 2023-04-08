# To panic! or not to panic!

## Prototype code and tests

When in early stage of a project `unwrap` and `expect` are handy

Use them were **not sure** how to handle errors

## When being sure that code works

If we are sure that our code will **not break**, but function used returns `Result<T, E>` we can use **expect**

## More handling

When something can **break** and do something **unexpected**:

- wrong types
- typos in command input
- relying on code not being in bad state

Then we should `panic!`

When we know that something **can break** we should *return a `Result`*

When working on **invalid data**: `panic!`
