# Worldstate Parser

A worldstate parser for http://api.warframe.com/cdn/worldState.php - in Rust!

## Lib

I just recently transformed it into a lib. If you want to give it a shot, add the git dependency.

## Example

As there's not much documentation right now, you can check [the example](./examples/showcase/), which generates [this file](./worldstate_parsed.json).

To run the example:

```
cargo run --manifest-path examples/showcase/Cargo.toml
```

## Translation Data

Provided by the awesome [warframe-worldstate-data](https://github.com/WFCD/warframe-worldstate-data) project.

## Internationalization (i18n)

Not yet supported. Not planning to do it any time soon, as I feel like it's not needed as much.
