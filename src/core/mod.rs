pub mod resolvable_string;
pub mod sol_node;

use std::{fs, marker::PhantomData, path::Path};

use derive_more::Display;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum TranslationLanguage {
    Czech,
    German,
    Spanish,
    French,
    Italian,
    Korean,
    Polish,
    Portuguese,
    Russian,
    Serbian,
    Turkish,
    Ukrainian,
    Chinese,
    English,
}

impl TranslationLanguage {
    /// Returns the 2-letter ISO folder code associated with the language.
    ///
    /// Returns [`None`] for english, as it makes it easier f
    pub fn as_code(&self) -> Option<&'static str> {
        Some(match self {
            TranslationLanguage::Czech => "cs",
            TranslationLanguage::German => "de",
            TranslationLanguage::Spanish => "es",
            TranslationLanguage::French => "fr",
            TranslationLanguage::Italian => "it",
            TranslationLanguage::Korean => "ko",
            TranslationLanguage::Polish => "pl",
            TranslationLanguage::Portuguese => "pt",
            TranslationLanguage::Russian => "ru",
            TranslationLanguage::Serbian => "sr",
            TranslationLanguage::Turkish => "tr",
            TranslationLanguage::Ukrainian => "uk",
            TranslationLanguage::Chinese => "zh",
            TranslationLanguage::English => return None,
        })
    }
}

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
    pub fn new(language: TranslationLanguage) -> Result<Self, BoxDynError> {
        let exports = get_export()?;
        let custom_maps = CustomMaps::new(&exports);
        let worldstate_data = WorldstateData::new(language, "data/", "drops/", "assets/")?;

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

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash, derive_more::FromStr)]
pub enum InternalPathTag {
    Levels,
    Types,
    Language,
    Upgrades,
    Interface,

    Unknown,
}

pub mod resolve_with {
    pub struct LanguageItems;
    pub struct SolNodes;
    pub struct LastSegment;
    pub struct RotationalReward;
    pub struct Hubs;

    pub mod sortie {
        pub struct Modifier;

        pub struct Boss;
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
    pub tag: InternalPathTag,

    #[serde(skip)]
    #[debug(skip)]
    _p: PhantomData<Resolver>,
}

impl<Resolver> From<String> for InternalPath<Resolver> {
    fn from(path: String) -> Self {
        let tag = path
            .split('/')
            .filter(|segment| !segment.is_empty())
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(InternalPathTag::Unknown);

        Self {
            path,
            tag,
            _p: PhantomData,
        }
    }
}

impl<T> InternalPath<T> {
    pub fn to_title_case(&self) -> Option<String> {
        self.path.split('/').next_back().map(|s| s.to_title_case())
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

#[cfg(test)]
mod tests {
    use crate::{
        BoxDynError,
        core::{InternalPath, InternalPathTag},
    };

    #[test]
    fn test_from_internal_path() -> Result<(), BoxDynError> {
        let internal_path: InternalPath =
            serde_json::from_str("\"/Lotus/Levels/Proc/Orokin/OrokinTowerMobileDefense\"")?;

        assert_eq!(internal_path.tag, InternalPathTag::Levels);

        assert_eq!(
            internal_path.to_title_case().unwrap(),
            "Orokin Tower Mobile Defense"
        );

        Ok(())
    }
}
