use std::marker::PhantomData;

use heck::ToTitleCase;
use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, Resolve, resolve_with},
    wfcd_worldstate_data::sortie_data::Boss,
};

#[derive(derive_more::Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(from = "String")]
pub(crate) struct ResolvableString<Resolver = ()>(pub String, PhantomData<Resolver>);

impl<T> From<String> for ResolvableString<T> {
    fn from(value: String) -> Self {
        Self(value, PhantomData)
    }
}

impl Resolve<Context<'_>> for ResolvableString<resolve_with::sortie::Modifier> {
    type Output = String;

    fn resolve(self, ctx: Context<'_>) -> Self::Output {
        ctx.worldstate_data
            .sortie_data
            .modifier_types
            .get(&self.0)
            .cloned()
            .unwrap_or_else(|| self.0.to_title_case())
    }
}

impl<'a> Resolve<Context<'a>> for ResolvableString<resolve_with::sortie::Boss> {
    type Output = Option<&'a Boss>;

    fn resolve(self, ctx: Context<'a>) -> Self::Output {
        ctx.worldstate_data.sortie_data.bosses.get(&self.0)
    }
}
