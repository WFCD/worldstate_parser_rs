# Worldstate Parser

A very WIP worldstate parser for http://api.warframe.com/cdn/worldState.php - in Rust!

## Not a lib?

No, not yet. Why? To make development a bit easier/more comfortable.

## Running

```sh
cargo run
```

You need a `worldstate.json`, the content of which you can grab [here](http://api.warframe.com/cdn/worldState.php).

Additionally run the [nushell script](./fetch_data.nu) which will fetch the translation data.

## Translation Data

Provided by the awesome [warframe-worldstate-data](https://github.com/WFCD/warframe-worldstate-data) project.

## Localization

Not yet supported. Can be added if requested.
