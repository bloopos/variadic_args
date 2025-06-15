# variadic_arguments

<!-- [crates.io](https://crates.io/crates/variadic_arguments)  -->

A crate that implements variadic arguments into Rust.

&nbsp;

## Features
- Each argument item supports storing any generic, so long as it implements `Any + Clone`.
	- `OwnedArgument` stores an owned variant. For smaller types, it uses inline storage instead.
	- `Argument` follows Copy-on-Write behavior, which enables borrowing variants.
- In addition, this crate allows for creating sets of known arguments.
	- `ArgumentsBuilder` is meant for building `Arguments` safely. This is done by setting a strict limit to the amount of arguments in the builder itself.
	- `Arguments` allows for parsing each argument. While the inner argument count is set, this allows for parsing each item with mutable access.

&nbsp;

## Known Issues
- Big endian has not been tested yet.
- The documentation is not finished.

&nbsp;

## Todo List
- Improve documentation.
- Keyword arguments.
- Send-sync support.