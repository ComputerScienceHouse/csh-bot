# CSH Bot

The fulfillment service for a DialogFlow agent built to control CSH projects.

DialogFlow is a service (now run by Google) that performs natural-language
processing on English string inputs, performs actions based on the _intent_ of
the user, and returns an English output that describes the actions taken or the
status requested. To achieve the middle step - performing actions - DialogFlow
will send an HTTP request to an address of our choosing. At the address we
provide, we must host an API server that consumes the request as JSON, then
performs actions however we like. We then return an HTTP response that contains
the string that the agent will return to the user.

The documentation on how to parse requests and how to construct responses is
given on the [DialogFlow docs] site. This project uses [actix-web] to implement
the API server, and makes use of [serde] to deserialize the requests and to
serialize the responses.

[DialogFlow docs]: https://dialogflow.com/docs/fulfillment/how-it-works
[actix-web]: https://github.com/actix/actix-web
[serde]: https://github.com/serde-rs/serde

## Getting Started

This project is written in Rust, so make sure you have Rust installed.

### Linux/MacOS

Just go to [rustup.rs](https://rustup.rs) and copy/paste the curl command into
a terminal. After installation is complete, you should be able to run
`cargo`.

Note: You may need to add the following line to your `~/.bashrc` file in order
for `cargo` to show up as a command:

```
export PATH="${HOME}/.cargo/bin:${PATH}"
```

### Windows

You'll need to download the [Build Tools for Visual Studio], which includes
some general development tools, namely the compiler. If you already have
Visual Studio installed, you should be able to skip this step.

[Build Tools for Visual Studio]: https://visualstudio.microsoft.com/downloads/

Then, go to [rustup.rs](https://rustup.rs) and download the `rustup-init.exe`
and run it, which should install the latest version of Rust on your system.
You can test that it installed successfully by opening `cmd.exe`, you should
be able to execute the command `cargo`.

### All Platforms

For development purposes, we'll need to download a tool called [ngrok], which
will create a public URL that will forward all of its traffic to `localhost`.
After downloading and extracting the zip, you should be able to just run the
executable `ngrok` file. Each time you want to open a "development session",
just run

```
ngrok http 8000
```

You'll get a status screen with two public addresses that it created that now
point to your machine. Copy the `https` address and give it to DialogFlow
under the "fulfillment" tab, and hit save. Every time you execute ngrok, you'll
get a new address, so you'll have to go and change the fulfillment URL and
save it again.

[ngrok]: https://ngrok.com/download

## Building and Running

Assuming you got Rust set up successfully, all you need to do to run the
project is to run the command:

```
cargo run
```

This will download all of the dependencies, build, and execute the project.
When it starts running, it won't give you any output, but it will seem to just
hang. That means it's working. It's waiting for DialogFlow to send it a request.
If you type in a sentence to the test input, you should see a printout of the
request it received.
