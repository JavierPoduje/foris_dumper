# Migrator

Import and export remote and local scenarios from DarwinEd.

## Dependencies

1. [Rust](https://www.rust-lang.org/tools/install).
2. [Darwined](https://github.com/Foris/darwined).
3. ssh connection to DarwinEd.

## Initial setup

Copy and fill the .env file:
```sh
cp env.example .env
```

Copy and fill the hosts.json file:
```sh
cp hosts.example.json hosts.json
```

## Use

Bring remote `tags`:
```sh
cargo run -- --client <client>
```

Example (assuming `hyades` is a client defined in the hosts.json file):
```sh
cargo run -- --client hyades
```

## Todo

1. dump scenario.
2. import scenario to local.
3. import scenario to remote.
2. export scenarios from local to remote.
