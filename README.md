# simple-web-server-rs
A simple web server in Rust.
## Prequisites
- [Rust 2021 or higher](https://www.rust-lang.org/learn/get-started)
- - [rustup](https://rustup.rs/) is the official installer for the Rust language.

## Getting Start
```bash
  git clone https://github.com/MrWoo034/simple-web-server-rs.git
  cd simple-web-server-rs
```

## Build
```bash
  make build
```

## Run
```bash
  make run
```

## Clean
```bash
  make clean
```

## API Methods
  ### POST `/foo`
  Takes a name as a `String` and generated a UUID for `Foo.id`.  Places this `Foo` in the cache.
  Returns the `Foo` object placed in cache.
  Example payload:
  ```jsonlines
    {
      "name": "Leif"
    }
  ```
  
  ### GET `/foo/{id}`
  Takes an `id` as a String and looks for the corresponding `Foo` object in cache.
  Returns the Status `200` and body of `Foo` object if found, `404` otherwise.
  
  ### DELETE `/foo/{id}`
  Takes an `id` as a String and removes that `Foo` entry from the cahce.
  Returns the Status `204` and empty body if object was found and removed, and `404` otherwise.
  
  
