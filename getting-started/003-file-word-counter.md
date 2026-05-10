
# Rust Beginner Subject 3: File Word Counter

## What You Will Learn
- Reading files efficiently with `BufReader`
- Using `HashMap` to count occurrences
- Working with iterators and closures
- Splitting strings into words
- Sorting collections (by value or key)
- Writing results to a file or stdout

---

## Explanation

A word counter reads a text file, splits it into words, and counts how many times each word appears. This teaches you file I/O, the powerful `HashMap` type, and how to handle string processing in Rust.

### Complete Example: Basic Word Counter

```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get filename from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Open and read file line by line
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let mut word_count: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        // Split line into words: split by whitespace and punctuation (simplified)
        for word in line.split_whitespace() {
            // Convert to lowercase to count "The" and "the" as same
            let word = word.to_lowercase();
            // Remove common punctuation at start/end (.,!?;:)
            let word = word.trim_matches(&['.', ',', '!', '?', ';', ':', '"', '\''][..]);
            if !word.is_empty() {
                *word_count.entry(word).or_insert(0) += 1;
            }
        }
    }

    // Print results sorted by frequency (most common first)
    let mut counts: Vec<(String, u32)> = word_count.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1)); // descending by count

    println!("Top 10 words:");
    for (i, (word, count)) in counts.iter().take(10).enumerate() {
        println!("{}. {}: {}", i + 1, word, count);
    }
    println!("Total unique words: {}", counts.len());

    Ok(())
}
```

Key Points

· File reading: BufReader provides efficient line‑by‑line reading.
· Splitting words: line.split_whitespace() gives an iterator over whitespace‑separated substrings.
· Lowercasing: .to_lowercase() ensures case‑insensitive counting.
· String cleanup: .trim_matches(&[...][..]) removes punctuation from the beginning and end of each word.
· HashMap: entry(word).or_insert(0) returns a mutable reference to the value for that key, initialising to 0 if missing. Then *ref += 1 increments.
· Sorting: into_iter().collect() gives a Vec<(String, u32)>. sort_by with b.1.cmp(&a.1) sorts descending by count.
· Iterating top N: .iter().take(10) only takes the first 10 items.

Handling Large Files

This code reads and processes the file line by line, so it can handle very large files without loading everything into memory.

Including Only Alphabetic Words (Optional)

If you want to ignore words with numbers or punctuation inside (like "don't" or "123"), you can filter:

```rust
if word.chars().all(|c| c.is_alphabetic()) {
    // count it
}
```

---

Mini Task

Extend the word counter to include the following features:

1. Command line flags – allow the user to specify:
   · --top N to show the top N words (default 10).
   · --min-length L to ignore words shorter than L characters.
   · --case-sensitive to treat "The" and "the" as different.
2. Output formatting – if the output is longer than 20 lines, ask the user if they want to pipe to a pager (e.g., less). (Bonus: implement a simple pager.)
3. Stop words – load a list of common stop words (e.g., "the", "and", "of", "to") from a separate file stopwords.txt and exclude them from counting.

Example usage:

```
$ ./wordcounter --top 20 --min-length 3 --case-sensitive alice.txt
Loading stopwords from stopwords.txt...
Top 20 words (min length 3, case sensitive):
1. Alice: 187
2. said: 156
3. little: 98
...
```

Hints:

· Parse command line flags manually using std::env::args() or better, use the clap crate (add clap = { version = "4.0", features = ["derive"] } to Cargo.toml).
· For --min-length, filter words before inserting: if word.len() >= min_len { ... }.
· For stop words: read stopwords.txt into a HashSet<String> and check if !stopwords.contains(word) { ... }.
· To ask for user input, use std::io::stdin().read_line(&mut answer).

After finishing this task, you will be proficient with HashMap, iterators, file I/O, and basic argument parsing – all essential for data‑processing tools in Rust.

---
