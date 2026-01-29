pub mod resolvable_string;
pub(crate) mod sol_node;

use std::{fs, marker::PhantomData, path::Path};

use heck::ToTitleCase;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    BoxDynError,
    CACHE_DIR,
    custom_maps::CustomMaps,
    manifests::{self, Exports},
    wfcd_data::WorldstateData,
};

fn get_from_cache_or_fetch<T: DeserializeOwned>(manifest: &str) -> Result<T, BoxDynError> {
    let path = Path::new(CACHE_DIR)
        .join(manifest)
        .with_added_extension("json");

    if let Ok(cached) = fs::read_to_string(&path) {
        println!("Using cache at {}", path.to_str().unwrap());
        return Ok(serde_json::from_str(&cached)?);
    }

    let item_json = get(format!(
        "http://content.warframe.com/PublicExport/Manifest/{}",
        manifest
    ))?
    .text()?;

    for file in fs::read_dir(CACHE_DIR)? {
        let file_name = file?.file_name().into_string().unwrap();

        if file_name.starts_with(
            manifest
                .split_once('!')
                .expect("Manifest should be valid")
                .0,
        ) {
            fs::remove_file(file_name)?;
        }
    }

    fs::write(path, &item_json)?;

    Ok(serde_json::from_str(&item_json)?)
}

fn get_export() -> Result<Exports, BoxDynError> {
    let file = get("https://origin.warframe.com/PublicExport/index_en.txt.lzma")?
        .bytes()?
        .to_vec();

    let mut buffer: Vec<u8> = Vec::new();

    lzma_rs::lzma_decompress(&mut file.as_slice(), &mut buffer).unwrap();

    let data = String::from_utf8(buffer)?;

    let export: manifests::PublicExportIndex = data.parse()?;

    let exports = Exports {
        export_regions: get_from_cache_or_fetch(&export.regions)?,
        export_relic_arcane: get_from_cache_or_fetch(&export.relic_arcane)?,
        export_customs: get_from_cache_or_fetch(&export.customs)?,
    };

    Ok(exports)
}
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub exports: Exports,
    pub custom_maps: CustomMaps,
    pub worldstate_data: WorldstateData,
}

impl Context {
    pub fn new() -> Result<Self, BoxDynError> {
        let exports = get_export()?;
        let custom_maps = CustomMaps::new(&exports);
        let worldstate_data = WorldstateData::new("data/", "drops/", "assets/", "assets_manual/")?;

        Ok(Context {
            custom_maps,
            exports,
            worldstate_data,
        })
    }

    pub fn as_ref(&self) -> ContextRef<'_> {
        ContextRef {
            exports: &self.exports,
            custom_maps: &self.custom_maps,
            worldstate_data: &self.worldstate_data,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextRef<'a> {
    pub exports: &'a Exports,
    pub custom_maps: &'a CustomMaps,
    pub worldstate_data: &'a WorldstateData,
}

pub trait Resolve<Ctx> {
    type Output;

    fn resolve(self, ctx: Ctx) -> Self::Output;
}

impl<T, Ctx> Resolve<Ctx> for Option<T>
where
    T: Resolve<Ctx>,
{
    type Output = Option<T::Output>;

    fn resolve(self, ctx: Ctx) -> Self::Output {
        self.map(|value| value.resolve(ctx))
    }
}

impl<T, Ctx> Resolve<Ctx> for Vec<T>
where
    Ctx: Copy,
    T: Resolve<Ctx>,
{
    type Output = Vec<T::Output>;

    fn resolve(self, ctx: Ctx) -> Self::Output {
        self.into_iter().map(|item| item.resolve(ctx)).collect()
    }
}

pub mod resolve_with {
    macro_rules! define_resolvers {
        (
            $mod_name:ident { $( $inner:tt )* }
            $( ; $( $rest:tt )* )?
        ) => {
            pub mod $mod_name {
                define_resolvers!( $( $inner )* );
            }
            $( define_resolvers!( $( $rest )* ); )?
        };

        (
            $ident:ident
            $( ; $( $rest:tt )* )?
        ) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            pub struct $ident;

            $( define_resolvers!( $( $rest )* ); )?
        };

        () => {};
    }

    define_resolvers! {
        LanguageItems;
        SolNodes;
        LastSegment;
        RotationalReward;
        Hubs;
        VaultTraderItem;
        PrimePart;
        sortie {
            Modifier;
            Boss;
        };
    }
}

/// Deserializes an internal path like `/Lotus/Levels/Proc/Orokin/OrokinTowerMobileDefense`.
///
/// Yields additional info about the tag via the [`InternalPath::tag`] field.
#[derive(
    derive_more::Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, derive_more::Display,
)]
#[serde(from = "String")]
#[display("{path}")]
pub struct InternalPath<Resolver = ()> {
    pub path: String,

    #[serde(skip)]
    #[debug(skip)]
    _p: PhantomData<Resolver>,
}

impl<Resolver> From<String> for InternalPath<Resolver> {
    fn from(path: String) -> Self {
        Self {
            path,
            _p: PhantomData,
        }
    }
}

impl<T> InternalPath<T> {
    pub fn new<S>(s: String) -> InternalPath<S> {
        InternalPath {
            path: s,
            _p: PhantomData,
        }
    }

    pub fn cast<S>(self) -> InternalPath<S> {
        InternalPath {
            path: self.path,
            _p: PhantomData,
        }
    }

    pub fn last_segment(&self) -> Option<&str> {
        self.path.split('/').next_back()
    }

    pub fn to_title_case(&self) -> Option<String> {
        self.last_segment().map(|s| s.to_title_case())
    }

    pub fn into_title_case_or_path(self) -> String {
        self.to_title_case().unwrap_or(self.path)
    }
}

impl Resolve<ContextRef<'_>> for InternalPath<resolve_with::LanguageItems> {
    type Output = String;

    fn resolve(self, ctx: ContextRef) -> Self::Output {
        let items = &ctx.worldstate_data.language_items;

        items
            .get(&self.path)
            .or_else(|| items.get(&self.path.to_lowercase()))
            .map(|item| item.value.clone())
            .unwrap_or_else(|| self.to_title_case().unwrap_or(self.path))
    }
}

impl Resolve<()> for InternalPath<resolve_with::LastSegment> {
    type Output = String;

    fn resolve(self, _ctx: ()) -> Self::Output {
        self.into_title_case_or_path()
    }
}

fn split_camel_case(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut last_index = 0;

    for (i, c) in s.char_indices() {
        if c.is_uppercase() && i > 0 {
            parts.push(&s[last_index..i]);
            last_index = i;
        }
    }

    if last_index < s.len() {
        parts.push(&s[last_index..]);
    }

    parts
}

impl Resolve<()> for InternalPath<resolve_with::PrimePart> {
    type Output = String;

    fn resolve(self, _ctx: ()) -> Self::Output {
        resolve::prime_part(self.cast())
    }
}

impl Resolve<ContextRef<'_>> for InternalPath<resolve_with::VaultTraderItem> {
    type Output = String;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        match &self.path {
            path if path.contains("Weapons") => resolve::prime_part(self.cast()),
            path if path.contains("Powersuits") => {
                InternalPath::<resolve_with::LanguageItems>::resolve(self.cast(), ctx)
            },
            path if path.contains("Upgrades/Skins") => resolve::skins(self.cast(), ctx),
            path if path.contains("Projections") => resolve::relics(self.cast(), ctx),
            path if path.contains("MegaPrimeVault") => resolve::prime_vault_pkg(self.cast()),

            _ => self
                .last_segment()
                .map(|s| s.to_title_case())
                .unwrap_or_default(),
        }
    }
}

mod resolve {
    use heck::ToTitleCase;

    use crate::core::{ContextRef, InternalPath, split_camel_case};

    const WEAPON_ARCHTYPE_REMOVAL_LIST: &[&str] = &[
        "Dagger",
        "Weapon",
        "Sniper",
        "Bow",
        "Launcher",
        "Sword",
        "Dual Daggers",
        "Claws",
        "Nikana",
    ];

    pub fn prime_part(path: InternalPath) -> String {
        let mut parts = split_camel_case(match path.last_segment() {
            Some(segment) => segment,
            None => return "".to_string(),
        });

        // swap leading prime, e.g. `Prime Knell` -> `Knell Prime`
        if parts.len() >= 2 && parts[0] == "Prime" {
            // for stuff like `Prime Dual Keres` -> `Dual Keres Prime`
            parts.rotate_left(1);
        }

        // strip trailing weapon archtype info
        if let Some(index) = parts
            .iter()
            .position(|x| WEAPON_ARCHTYPE_REMOVAL_LIST.contains(x))
        {
            parts.remove(index);
        }

        parts.join(" ")
    }

    pub fn skins(path: InternalPath, ctx: ContextRef<'_>) -> String {
        if let Some(entry) = ctx
            .custom_maps
            .unique_to_customs_entry
            .get(&path.path.replace("/Lotus/StoreItems/", "/Lotus/Upgrades/"))
        {
            entry.name.clone()
        } else if let Some(entry) = ctx.worldstate_data.language_items.get(&path.path) {
            entry.value.clone()
        } else {
            path.last_segment().unwrap_or_default().to_title_case()
        }
    }

    pub fn relics(path: InternalPath, ctx: ContextRef<'_>) -> String {
        ctx.custom_maps
            .relic_uniq_to_relic
            .get(&path.path.replace("/StoreItems", ""))
            .map(|relic| relic.name.clone())
            .unwrap_or_default()
    }

    pub fn prime_vault_pkg(path: InternalPath) -> String {
        let mut last_segment = match path.last_segment() {
            Some(segment) => segment,
            None => return path.path,
        };

        if last_segment.starts_with("MPV") {
            last_segment = &last_segment[3..];
        }

        let mut split = split_camel_case(last_segment);

        if last_segment.ends_with("DualPack") {
            split.insert(1, "&");
        }

        split.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BoxDynError,
        core::{Context, InternalPath, Resolve, resolve_with},
    };

    #[test]
    fn test_from_internal_path() -> Result<(), BoxDynError> {
        let internal_path: InternalPath =
            serde_json::from_str("\"/Lotus/Levels/Proc/Orokin/OrokinTowerMobileDefense\"")?;

        assert_eq!(
            internal_path.to_title_case().unwrap(),
            "Orokin Tower Mobile Defense"
        );

        Ok(())
    }

    #[test]
    fn test_language_items() -> Result<(), BoxDynError> {
        let internal_path: InternalPath<resolve_with::LanguageItems> = serde_json::from_str(
            "\"/Lotus/StoreItems/Types/Recipes/WarframeRecipes/DagathChassisComponent\"",
        )?;

        let context = Context::new().unwrap();

        dbg!(internal_path.resolve(context.as_ref()));

        Ok(())
    }
}
