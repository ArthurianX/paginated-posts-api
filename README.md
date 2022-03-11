# Paginated CRUD API with Rust, Rocket and Diesel on SQLite3

https://user-images.githubusercontent.com/2720451/157906631-c306a950-1c6d-4752-85f4-63ac390694d0.mov


## Rust
**Performance**

Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

**Reliability**

Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

**Productivity**

Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

https://www.rust-lang.org/


## Rocket
Rocket is a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety.

https://rocket.rs/master/overview/

## Diesel
Diesel is a Safe, Extensible ORM and Query Builder for Rust
**Preventing Runtime Errors**

We don’t want to waste time tracking down runtime errors. We achieve this by having Diesel eliminate the possibility of incorrect database interactions at compile time.

**Built for Performance**

Diesel offers a high level query builder and lets you think about your problems in Rust, not SQL. Our focus on zero-cost abstractions allows Diesel to run your query and load your data even faster than C.

**Productive and Extensible**

Unlike Active Record and other ORMs, Diesel is designed to be abstracted over. Diesel enables you to write reusable code and think in terms of your problem domain and not SQL.

## SQLite
No need to present this.

-----------

## To get this to work:
- Clone this repo
- Have SQLite3 installed on your machine with `brew install sqlite3`
- Install Rust toolchain with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Go into the cloned repo folder and switch to Rust nighthly `rustup override set nightly`
- `cargo run` to install the initial packages, it will fail at first because we don't have the db
- Install Diesel with `cargo install diesel_cli --no-default-features --features sqlite`
- Run Diesel db migration `diesel migration run`, it will create `db.db` in the root folder
- Running `cargo run` again should start the Rocket server, like below:
<img width="641" alt="image" src="https://user-images.githubusercontent.com/2720451/157904380-2001ecb0-4c5d-4ec4-ae47-3d7ab4148f2d.png">


