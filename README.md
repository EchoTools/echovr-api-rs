# echovr-api-rs

Rust bindings for the [Echo VR HTTP API](https://github.com/Ajedi32/echovr_api_docs).

## Requirements

- Echo VR must be running
- The API must be enabled in Echo VR's settings
- Nothing else can be bound to port 6721

If you're having port conflicts, run `net stop HTTP` in an admin prompt.

## Usage

```toml
[dependencies]
echovr = "0.2"
```

```rust
use echovr::Client;

fn main() {
    let client = Client::new();

    match client.fetch_session() {
        Ok(session) => {
            println!("Blue {} | Orange {}", session.blue_points, session.orange_points);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## License

GPL-3.0