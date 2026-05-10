
# Rust Beginner Subject 4: Simple HTTP Client

## What You Will Learn
- Making HTTP requests with the `reqwest` crate
- Parsing JSON responses with `serde`
- Basic asynchronous programming (async/await)
- Handling different HTTP methods (GET, POST)
- Error handling in async functions
- Working with external APIs (e.g., weather, jokes, cats)

---

## Explanation

An HTTP client allows your program to fetch data from web services. Most modern APIs return JSON, so you’ll learn how to deserialize that into Rust structs. You’ll also get a gentle introduction to async programming, which Rust handles elegantly.

### Setting Up Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

· reqwest – HTTP client (with JSON support)
· tokio – async runtime (needed for async/await)
· serde – serialization/deserialization

Complete Example: Fetch a Random Cat Fact

Let’s call a free API that returns random cat facts (https://catfact.ninja/fact).

```rust
use serde::Deserialize;
use reqwest;

// Define a struct matching the JSON response
#[derive(Debug, Deserialize)]
struct CatFact {
    fact: String,
    length: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching a random cat fact...");
    
    let url = "https://catfact.ninja/fact";
    let response = reqwest::get(url).await?;
    
    // Parse JSON directly into our struct
    let fact: CatFact = response.json().await?;
    
    println!("🐱 Fact: {}", fact.fact);
    println!("📏 Length: {} characters", fact.length);
    
    Ok(())
}
```

Output:

```
Fetching a random cat fact...
🐱 Fact: Cats have over 20 muscles in their ears.
📏 Length: 48 characters
```

Key Points

· Async main: #[tokio::main] lets you use async fn main().
· await: Every HTTP call is asynchronous – we must .await the result.
· Deserialization: #[derive(Deserialize)] automatically creates a struct from JSON.
· .json() method parses the response body into the type you specify.
· Error propagation: ? works in async functions too.

More Advanced: Fetch Weather from Open‑Meteo API

This free API returns weather forecast for any coordinates.

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    weathercode: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lat = 52.52;   // Berlin
    let lon = 13.405;
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        lat, lon
    );
    
    let response = reqwest::get(&url).await?;
    let weather: WeatherResponse = response.json().await?;
    
    println!("🌡️ Temperature: {:.1}°C", weather.current_weather.temperature);
    println!("💨 Wind speed: {:.1} km/h", weather.current_weather.windspeed);
    
    Ok(())
}
```

POST Request Example

To send data (e.g., to a mock API like https://jsonplaceholder.typicode.com/posts):

```rust
use serde_json::json;

#[derive(Debug, Deserialize)]
struct PostResponse {
    id: u32,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let new_post = json!({
        "title": "My Rust Post",
        "body": "This is the body",
        "userId": 1
    });
    
    let response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?;
    
    let created: PostResponse = response.json().await?;
    println!("Created post with ID: {}", created.id);
    
    Ok(())
}
```

---

Mini Task

Build a command‑line tool that fetches information from a public API of your choice and displays it nicely.

Choose one of these (or any free API you like):

Option A: Chuck Norris Jokes

Use the API https://api.chucknorris.io/jokes/random which returns:

```json
{ "value": "Chuck Norris can divide by zero." }
```

Task: Display a random joke each time you run the program.

Option B: Bitcoin Price

Use https://api.coindesk.com/v1/bpi/currentprice.json and extract the current USD price.

Task: Print "1 BTC = $xx,xxx.xx USD" and update automatically every 10 seconds (use a loop with tokio::time::sleep).

Option C: Country Information

Use https://restcountries.com/v3.1/name/{country} (e.g., /name/france). 
Parse the capital, population, and flag emoji.

Task: Accept a country name as a command‑line argument and show its details.

Requirements for all options:

1. Handle errors gracefully (e.g., no internet, invalid JSON, country not found).
2. Print pretty output with emojis or formatting.
3. Optionally, add a --help flag explaining usage.

Example run (Option C):

```
$ ./countryinfo france
🇫🇷 France
Capital: Paris
Population: 67,397,000
```

Hints:

· Use reqwest::get(url).await? and .json::<YourStruct>().await?.
· Create a struct that matches only the fields you need. Use #[serde(rename_all = "camelCase")] if the JSON uses camelCase.
· For command‑line arguments, reuse what you learned in Subject 1.
· To handle missing fields gracefully, make struct fields Option<T> (e.g., capital: Option<String>).

After finishing this task, you will be comfortable with async Rust, JSON parsing, and integrating web APIs – a powerful skill for building real‑world tools.

---
