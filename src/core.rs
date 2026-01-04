use std::marker::PhantomData;

use heck::ToTitleCase;
use serde::{Deserialize, Serialize};

use crate::{
    custom_maps::{CustomMaps, solnode_to_region::Region},
    manifests::Exports,
    worldstate_data::WorldstateData,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context<'a> {
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

impl Resolve<Context<'_>> for InternalPath<resolve_with::LanguageItems> {
    type Output = String;

    fn resolve(self, ctx: Context) -> Self::Output {
        ctx.worldstate_data
            .language_items
            .get(&self.path)
            .map(|item| &item.value)
            .cloned()
            .unwrap_or_else(|| self.to_title_case().unwrap_or(self.path))
    }
}

impl Resolve<Context<'_>> for InternalPath<resolve_with::LastSegment> {
    type Output = String;

    fn resolve(self, _ctx: Context<'_>) -> Self::Output {
        self.into_title_case_or_path()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct SolNode(pub String);

impl<'a> Resolve<Context<'a>> for SolNode {
    type Output = Option<&'a Region>;

    fn resolve(self, ctx: Context<'a>) -> Self::Output {
        ctx.custom_maps.solnode_to_region.get(&self.0)
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
