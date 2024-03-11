# rust-basics
Error handling in Rust is crucial for writing robust and reliable applications. Unlike some other languages that use exceptions, Rust uses a combination of the Result enum and the panic! macro to deal with errors.

Here's a breakdown of error handling in Rust with an example:

1. The Result Enum:

The Result enum is a fundamental type in Rust for handling errors. It has two variants:

Ok(T): This variant represents a successful outcome and holds the value of type T (the type of data the function was supposed to return).
Err(E): This variant represents an error and holds a value of type E (typically an error type you define).
2. Example Function with Result:

Rust
```
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Division by zero"));
    }
    Ok(x / y)
}
```
Use code with caution.
This divide function takes two integers and attempts to divide them.

If y is zero, it returns Err(String::from("Division by zero")), indicating an error.
If the division is successful, it returns Ok(x / y), containing the result.
3. Handling Results:

You can handle the Result using pattern matching in the match expression:

Rust
```
fn main() {
    let result = divide(10, 2);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(err_msg) => println!("Error: {}", err_msg),
    }
}
```
Use code with caution.
The main function calls divide and stores the result in a variable.
The match expression checks the variant of the result.
If it's Ok(result), it prints the division result.
If it's Err(err_msg), it prints the error message.
4. Propagating Errors:

Functions can propagate errors by returning a Result. If a function encounters an error, it can return Err(error_value) to signal the error to the calling function. The calling function can then handle the error appropriately.

5. The panic! Macro:

For unrecoverable errors (like invalid memory access), Rust uses the panic! macro. It immediately halts program execution and prints an error message.

Example with Propagating Error:

Rust
```
fn read_file(filename: &str) -> Result<String, String> {
    let contents = std::fs::read_to_string(filename).map_err(|err| err.to_string())?;
    Ok(contents)
}

fn main() {
    let result = read_file("non-existent-file.txt");
    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err_msg) => println!("Error reading file: {}", err_msg),
    }
}
```
Use code with caution.
The read_file function attempts to read a file.
If the file operation fails, it uses map_err to convert the standard library error to a String and returns Err(err_msg).
In main, the error is propagated to the user if the file cannot be read.
This is a basic example of error handling in Rust. You can define custom error types for more specific errors and use libraries like thiserror to simplify error message generation.

Key Points:

Result helps manage success and error scenarios.
Pattern matching with match allows handling different outcomes.
Errors can be propagated through the function call chain.
panic! is for unrecoverable errors that halt program execution.
By understanding and using these concepts, you can write robust Rust programs that gracefully handle errors and provide informative feedback to the user.
