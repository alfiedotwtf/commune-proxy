# CommuneRS - A Commune Reverse Proxy

There are a few optimisations that can be done to make it faster, but first
let's make sure it's working properly...

## How to Build

Apart from installing Rust via `rustup`:

    cargo build

## How to Run

The `commune-proxy` binary is what clients connect to, which shuffles
requests and responses to and from the upstream server:

    ./commune-proxy --listen <ip:port> --upstream <ip:port>

### How to Test

The `example-upstream-server` binary is just an example server to test against,
and should be replaced with the Commune Python backend.

Currently, it's fixed to listen at `127.0.0.1:1337`. So to test `commune-proxy`:

    ./example-upstream-server &
    ./commune-proxy --listen localhost:8000 --upstream localhost:1337