# A basic Rust REST backend using ActixWeb framework

Run tests:

```shell
cargo test
```

Compile production version:

```shell
cargo build --release
```

Execute production build:

```shell
./target/release/rust-basic-actix-web
```

A REST request can be sent using *curl*:

```shell
curl -X POST --location "http://localhost:8080/api/activities" \
    -H "Content-Type: application/json" \
    -d '{
          "activity": "partying"
        }'
```

If successful, it should print: *Let's get started with partying!*

To put some heavy load on the server, use a benchmark tool such as [wrk](https://github.com/wg/wrk). A *wrk.lua* script
is provided and can be used as follows:

```shell
wrk -t26 -c600 -d25s http://localhost:8080/api/activities -s wrk.lua
```
