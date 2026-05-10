Here is a learning roadmap based on the Rust code we just analyzed. Each row focuses on one essential Rust concept that appeared in the monitor. For every concept, you get a practical task and a hint to help you build muscle memory.

---

Roadmap: Rust Concepts → Practice Tasks

# Concept Practice Task Hint
1 Result and ? operator Write a function that reads a file and returns the number of lines as Result<usize, io::Error>. Use ? to propagate errors. std::fs::read_to_string returns a Result. Call it with ?.
2 Pattern matching (if let, match) Take a Vec<Option<i32>>. Print only the Some values, doubling them. Use if let. Loop over the vector; if let Some(x) = item { println!("{}", x*2); }
3 Ownership and moves Create a struct Book with title: String. Write a function that takes ownership of the Book and prints it. Try to use the book again after the call – see compiler error. Move happens when you pass without &. The compiler will tell you “value used here after move”.
4 Borrowing (& and &mut) Write a function add_to_vec(vec: &mut Vec<i32>, val: i32) that pushes a value. Call it from main and show the vector changed. &mut allows mutation. You need to pass &mut my_vec.
5 Lifetimes (elision) Write a function longest that takes two string slices (&str) and returns the longer one. Do not write explicit lifetimes – let elision work. The signature is fn longest<'a>(x: &'a str, y: &'a str) -> &'a str, but you can omit <'a> because the compiler adds it. Try writing it anyway to see.
6 Iterators and closures Given a vector of integers, produce a new vector containing only even numbers, each squared. Use iter(), filter(), map(), and collect(). `vec.iter().filter(
7 Traits (custom trait) Define a trait Sound with a method make_noise(&self) -> String. Implement it for Dog and Cat structs. Call the method on instances. trait Sound { fn make_noise(&self) -> String; } then impl Sound for Dog { ... }
8 Trait objects (&dyn Trait) Create a vector Vec<Box<dyn Sound>> that can hold both Dog and Cat. Push instances and iterate, calling make_noise. Use Box::new(Dog{}) and cast to Box<dyn Sound> automatically.
9 Structs and impl blocks Define a Counter struct with a field count: u32. Implement new(), increment(&mut self), and value(&self). impl Counter { fn new() -> Self { Counter { count: 0 } } ... }
10 Option<T> and unwrap() vs match Write a function that divides two numbers safely: fn safe_div(a: f64, b: f64) -> Option<f64>. Return None if b == 0. Use match to handle the result. if b == 0.0 { None } else { Some(a / b) }
11 Macros (println!, format!, vec!) Create a vec! of three strings, then use format! to combine them with commas, and println! to output. let v = vec!["a".to_string(), "b".to_string(), "c".to_string()]; let s = format!("{}, {}, {}", v[0], v[1], v[2]);
12 External crates (Cargo.toml) Add the rand crate to a new project. Generate a random number between 1 and 100 and print it. Add rand = "0.8" to Cargo.toml; then use rand::Rng; let n = rand::thread_rng().gen_range(1..=100);
13 SQLite with rusqlite Create an in‑memory database, create a table users (id INTEGER PRIMARY KEY, name TEXT), insert two rows, and query them back. let conn = Connection::open_in_memory()?; conn.execute("CREATE TABLE ...", [])?;
14 Raw terminal mode (crossterm) Write a program that switches to raw mode, waits for a single key press, prints its code, then restores the terminal. Use enable_raw_mode(), read(), then disable_raw_mode(). Don’t forget execute!(stdout, LeaveAlternateScreen) if you use alternate screen.
15 Event loop with timeout Write a loop that every 2 seconds prints “tick” unless the user presses the spacebar, in which case it prints “boop” immediately. Use event::poll(Duration::from_secs(2)). if event::poll(timeout)? { if let Event::Key(key) = event::read()? { if key.code == KeyCode::Char(' ') { ... } } } else { println!("tick"); }
16 TUI with ratatui (basic) Build a simple UI with two blocks: left side a list of numbers (1 to 5), right side a paragraph showing the selected number. Use List and Paragraph. See ratatui examples: create a List from ListItem, use render_stateful_widget with a selection index.
17 Error handling without unwrap Take any of the above tasks and replace all .unwrap() with proper error handling using match or ? and main() -> Result<(), Box<dyn Error>>. Change fn main() to fn main() -> Result<(), Box<dyn std::error::Error>> and use ?.
18 Modules and file splitting Split a simple program (e.g., the counter struct) into three files: main.rs, counter.rs, and counter/mod.rs (or counter.rs as sibling). Use mod counter; in main.rs. Create counter.rs with pub struct Counter and pub impl. In main.rs, write mod counter; use counter::Counter;

---

Suggested Learning Order

1. Start with ownership, borrowing, lifetimes (1–5) – they are unique to Rust.
2. Then practice structs, impl, traits (7–9).
3. Move to error handling with Result and Option (1, 10, 17).
4. Learn iterators and closures (6) – very common.
5. Dive into external crates (12, 13, 14, 15, 16) – that’s where real projects begin.
6. Finally, master macros (11) and modules (18) – these are about organisation and syntactic sugar.

For each task, try to write the code without looking at the hint first. Then use the hint if you get stuck. Compile the code – let the compiler be your teacher. Once it works, experiment: change ownership, add errors, split into modules.

If you complete all 18 tasks, you will have a solid grasp of every Rust feature used in the process monitor. You’ll then be ready to extend that monitor (e.g., add CPU usage, export history, handle more edge cases).