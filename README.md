# Foris Dumper

Import and export remote and local scenarios from DarwinEd.

## Dependencies

1. [Rust](https://www.rust-lang.org/tools/install).
2. [Foris' Darwin](https://github.com/Foris/darwined).
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

```command
cargo run -- --client <my-client> --action <my-action> <options>
```

### Examples

- Import `tags`:
```command
cargo run -- --client some_client --action dump-tags
```

- Dump remote `scenario` and import it in your local Darwin:
```command
cargo run -- --client some_client --action dump-scenario --scenario some_scenario
```

- Import remote `scenario` without creating the dump:
```command
cargo run -- --client some_client --action dump-scenario --scenario some_scenario --skip_dump_creation
```

### Descriptions

|Actions|Description|
|---|---|
|<div align="center"><kbd>client</kbd></div>|Name of the source client|
|<div align="center"><kbd>action</kbd></div>|Action to execute (options: `dump-scenario`, `dump-tags`)|
|<div align="center"><kbd>scenario</kbd></div>|Scenario to dump. Only used with the `dump-scenario` action|
|<div align="center"><kbd>skip_dump_creation</kbd></div>|Skip the creation of the dump. Only used with the `dump-scenario` action|
