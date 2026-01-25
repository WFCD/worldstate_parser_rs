pub mod core;
pub mod custom_maps;
pub mod manifest_entries;
pub mod manifests;
pub mod target_types;
pub(crate) mod wfcd_data;
pub mod world_state;
pub(crate) mod worldstate_model;

use std::{error::Error, fs};

use serde::Serialize;
use serde_json::Serializer;

use crate::{core::Context, world_state::WorldStateUnmapped};

type BoxDynError = Box<dyn Error>;

pub const CACHE_DIR: &str = "./cache";

fn main() -> Result<(), BoxDynError> {
    let context = Context::new()?;

    let world_state =
        serde_json::from_str::<WorldStateUnmapped>(&fs::read_to_string("worldstate.json")?)?
            .map_worldstate(context.as_ref())
            .ok_or("Failed to map worldstate")?;

    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");

    let mut buf = Vec::new();

    let mut ser = Serializer::with_formatter(&mut buf, formatter);

    world_state.serialize(&mut ser).unwrap();

    let world_state_json = String::from_utf8(buf).unwrap();

    fs::write("worldstate_parsed.json", world_state_json)?;

    Ok(())
}
