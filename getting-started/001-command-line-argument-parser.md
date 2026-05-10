
# Rust Beginner Subject 1: Command‑Line Argument Parser

## What You Will Learn
- Reading command line arguments with `std::env::args`
- Using `Result` and `?` for error handling
- Basic pattern matching with `match` and `if let`
- Converting strings to other types (`parse`)
- Exiting with error messages

---

## Explanation

Rust programs can read the arguments passed to them on the command line. This is essential for building tools that accept input files, flags, or configuration.

The standard library provides `std::env::args()`, which returns an iterator over the arguments. The first argument is always the program’s own name.

### Basic Example: Echo the arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("You passed {} arguments:", args.len() - 1);
    for (i, arg) in args.iter().enumerate() {
        println!("args[{}] = {}", i, arg);
    }
}
```

If you run this program as ./myprogram hello world, it will print:

```
You passed 2 arguments:
args[0] = ./myprogram
args[1] = hello
args[2] = world
```

Realistic Example: Grep‑lite

Let’s build a simple version of grep that takes a pattern and a filename, then prints all lines containing the pattern.

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    
    // We expect exactly two arguments besides the program name
    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <filename>", args[0]);
        std::process::exit(1);
    }
    
    let pattern = &args[1];
    let filename = &args[2];
    
    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    // Search line by line
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;  // Handle possible I/O error
        if line.contains(pattern) {
            println!("{}: {}", line_num + 1, line);
        }
    }
    
    Ok(())
}
```

Key Points

· env::args().collect() gives a Vec<String> – easy to index.
· We check the number of arguments and print a usage message if incorrect.
· eprintln! prints to stderr (better for error messages).
· std::process::exit(1) terminates the program with an error code.
· File::open(filename)? uses ? to propagate any I/O error.
· BufReader makes reading lines efficient.
· The Result return type of main lets us use ? and automatically prints errors.

Handling Numbers (Optional Extension)

If you need a numeric argument, use parse:

```rust
let number: u32 = args[1].parse()?;  // returns Result, use ?
```

---

Mini Task

Write a Rust program called math_cli that does the following:

· Takes three command line arguments: num1, operator, num2 (e.g., ./math_cli 5 + 3).
· Supported operators: +, -, *, / (integer division).
· Prints the result or an appropriate error if:
  · Wrong number of arguments.
  · Invalid operator.
  · Division by zero.
· Use Result and ? where possible.
· Use match on the operator string.

Example runs:

```
$ ./math_cli 10 + 2
Result: 12

$ ./math_cli 8 / 0
Error: division by zero

$ ./math_cli 3.5 + 2
Error: invalid integer (only whole numbers)

$ ./math_cli 5 ^ 2
Error: unsupported operator '^'
```

Hint:

· Parse num1 and num2 as i32 using .parse::<i32>().
· Match the operator string literal: match op { "+" => a + b, ... }.
· To exit with an error message, you can return Err(Box::from("your message")) or use eprintln! + std::process::exit(1).

After finishing this task, you will be comfortable with command line arguments, error handling, and basic pattern matching – all essential for any Rust utility.

---

