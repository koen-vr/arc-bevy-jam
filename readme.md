# Arc - Bevy Demo

Test project for story arc using the bevy engine.

## WASM Setup Info

[Bevy Cheatbook](https://bevy-cheatbook.github.io/platforms/wasm.html)

~~~
rustup target install wasm32-unknown-unknown
cargo run --target wasm32-unknown-unknown
~~~

Requires a change to `cargo.toml`, WASM can not be linked dynamicly. A better way would be ot setup cargo targets but for now it is good to be reminded of the faq.
~~~
...

[dependencies]
bevy = { version = "0.8.0", features = ["dynamic"] }

...
~~~

## Game State

Game States - Push Pop
~~~
-> Pick Drop <> Actor Create <> Camera Zoom
=> Base Grid -> Explore Grid -> Event Grid
<= Base Grid <- Explore Grid <- Event Grid
~~~

~~~
GUI -> Gamehud -> Base
GUI -> Gamehud -> Event
GUI -> Gamehud -> Explore

World -> Grid   -> Static Entities
World -> Mode   -> Dynamic Entities
World -> Player -> Player Driven Entities
~~~

## Web Build

~~~
npm install http-server -g
http-server ./wasm
~~~

~~~
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name arc-demo --out-dir wasm --target web target/wasm32-unknown-unknown/release/arc-demo.wasm
~~~

## Tag GIT from release trough CLI

Read more: [Managing releases in a repository](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)

~~~
git tag -a "arc-proto-1.0" -m "Arc-Proto Release: v1.0"
git push --tags
~~~