
# Rust Beginner Subject 2: Number Guessing Game

## What You Will Learn
- Generating random numbers using the `rand` crate
- Reading user input from `stdin`
- Looping with `loop` and `while`
- Pattern matching with `match` on comparison results
- Basic error handling (ignoring or propagating)
- Type conversion (string to integer)

---

## Explanation

The number guessing game is a classic first project. The program generates a secret number, then repeatedly asks the user to guess it, giving hints like “too high” or “too low” until the guess is correct.

### Setting Up the `rand` Crate

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

Then in your code:

```rust
use rand::Rng;
```

Complete Example

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // Generate a random number between 1 and 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess:");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Convert guess to a number, handling invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Key Points

· Random number: rand::thread_rng().gen_range(1..=100) – note the inclusive range ..=.
· Reading input: stdin().read_line(&mut guess) stores the line (including newline) into a String.
· Trimming: .trim() removes whitespace, necessary before parsing.
· Parsing: .parse::<u32>() returns a Result. We use match to handle errors gracefully.
· Comparison: guess.cmp(&secret_number) returns an Ordering enum (Less, Greater, Equal).
· Loop control: loop creates an infinite loop; break exits when the guess is correct.

Handling Errors Without panic

Notice we used .expect() for read_line. This is okay for a small game, but a better way:

```rust
io::stdin().read_line(&mut guess)?;  // would need main() to return Result
```

Or handle it similarly to parsing:

```rust
if io::stdin().read_line(&mut guess).is_err() {
    println!("Error reading input");
    continue;
}
```

---

Mini Task

Extend the number guessing game with the following features:

1. Limit the number of attempts – allow only 5 guesses. If the user fails, reveal the secret number and exit.
2. Give a hint after 3 wrong guesses – e.g., “Hint: it's an even number” or “The number is divisible by 5”.
3. Ask to play again after win or loss, without restarting the program.

Example interaction:

```
Guess the number! (1-100, you have 5 attempts)
Attempt 1: 50
Too high!
Attempt 2: 25
Too low!
Attempt 3: 37
Too low!
Hint: The number is odd.
Attempt 4: 39
Too high!
Attempt 5: 38
Too low!
You lost! The secret number was 42.
Play again? (y/n): y
New game! ...
```

Hints:

· Use a for loop with a range (e.g., for attempt in 1..=5) instead of loop.
· To generate a hint, you can check properties of the secret number: secret % 2 == 0, secret % 3 == 0, etc.
· After the game ends, ask for input: let mut answer = String::new(); io::stdin().read_line(&mut answer).unwrap(); if answer.trim() == "y" { /* reset and restart */ } else { break; }

Once you finish this task, you will be comfortable with loops, input, random numbers, and the Ordering type – all essential for interactive Rust programs.

---

