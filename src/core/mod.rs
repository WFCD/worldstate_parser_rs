mod calendar_reward_resolve;
pub mod resolvable_string;
pub(crate) mod sol_node;
pub mod vault_trader_resolve;

use std::marker::PhantomData;

use heck::ToTitleCase;
use serde::{Deserialize, Serialize};

use crate::{
    custom_maps::CustomMaps,
    manifests::Exports,
    target_types::display_info::DisplayInfo,
    wfcd_data::WorldstateData,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub exports: Exports,
    pub custom_maps: CustomMaps,
    pub worldstate_data: WorldstateData,
}

impl Context {
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
        LanguageItemWithDesc;
        LastSegment;
        RotationalReward;
        Hubs;
        VaultTraderItem;
        PrimePart;
        TitleCase;
        CalendarReward;
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

impl Resolve<ContextRef<'_>> for InternalPath<resolve_with::LanguageItemWithDesc> {
    type Output = DisplayInfo;

    fn resolve(self, ctx: ContextRef) -> Self::Output {
        let items = &ctx.worldstate_data.language_items;

        items
            .get(&self.path)
            .or_else(|| items.get(&self.path.to_lowercase()))
            .cloned()
            .map(|language_item| language_item.resolve(()))
            .unwrap_or_else(|| DisplayInfo {
                title: self.into_title_case_or_path(),
                description: None,
            })
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
        vault_trader_resolve::prime_part(self.cast())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::InternalPath;

    #[test]
    fn test_from_internal_path() -> Result<(), Box<dyn std::error::Error>> {
        let internal_path: InternalPath =
            serde_json::from_str("\"/Lotus/Levels/Proc/Orokin/OrokinTowerMobileDefense\"")?;

        assert_eq!(
            internal_path.to_title_case().unwrap(),
            "Orokin Tower Mobile Defense"
        );

        Ok(())
    }
}
