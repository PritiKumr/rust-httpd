## A WebServer in Rust for fun & learning

Just to see what it takes to build a HTTP web server and to learn Rust along the way.

### How to run?

Make sure you have `cargo` [installed](https://www.rust-lang.org/en-US/install.html) and run the command `cargo run` to boot up the server. 

Visit `locahost:8888` from your browser.

### What can it do now?

1. **Say hello world** - visit `localhost:8888/hello`

2. **Serve static files** - visit `localhost:8888/files/index.html`. Place any other file inside `www` and they can be served

### TODO

* CGI Scripting
* Reverse Proxy
* Server Configuration with TOML

### Follow the project

We ([Steve](https://github.com/steverob) & [Preethi](https://github.com/PritiKumr)) will be posting updates about the project and will try to write stuff on Rust as we learn more about the language at our blog - [Adventures in Rust](https://medium.com/adventures-in-rust). Do follow :purple_heart:
