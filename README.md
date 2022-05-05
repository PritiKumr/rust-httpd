## A WebServer in Rust for fun & learning

Just to see what it takes to build a HTTP web server and to learn Rust along the way.

### How to run?

Make sure you have `cargo` [installed](https://www.rust-lang.org/en-US/install.html) and run the command `cargo run` to boot up the server. 

Visit `locahost:8888` from your browser.

### What can it do now?

1. **Say hello world** - visit `localhost:8888/hello`

2. **Serve static files** - visit `localhost:8888/files/index.html` - this will serve the `index.html` file from the `www` folder in the repo root. Place any other file inside `www` and they can be served similarly (using the `/files` prefix - this will be configurable by the user in future, just like in Apache and Nginx).

3. **Execute CGI Scripts** - place any script inside the `cgi` folder and they can be executed by visiting `localhost:8888/cgi/script`. This is a very simplistic implementation. Planning to support `SCGI`. Maybe FastCGI in future.

### TODO

* CGI Scripting
* Reverse Proxy
* Server Configuration with TOML
