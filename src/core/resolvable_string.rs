use std::marker::PhantomData;

use heck::ToTitleCase;
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, Resolve, resolve_with},
    wfcd_data::sortie_data::Boss,
};

#[derive(derive_more::Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(from = "String")]
pub(crate) struct ResolvableString<Resolver = ()>(pub String, PhantomData<Resolver>);

impl<T> From<String> for ResolvableString<T> {
    fn from(value: String) -> Self {
        Self(value, PhantomData)
    }
}

impl Resolve<ContextRef<'_>> for ResolvableString<resolve_with::sortie::Modifier> {
    type Output = String;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ctx.worldstate_data
            .sortie_data
            .modifier_types
            .get(&self.0)
            .cloned()
            .unwrap_or_else(|| self.0.to_title_case())
    }
}

impl<'a> Resolve<ContextRef<'a>> for ResolvableString<resolve_with::sortie::Boss> {
    type Output = Option<&'a Boss>;

    fn resolve(self, ctx: ContextRef<'a>) -> Self::Output {
        ctx.worldstate_data.sortie_data.bosses.get(&self.0)
    }
}

impl Resolve<ContextRef<'_>> for ResolvableString<resolve_with::Hubs> {
    type Output = String;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        let hubs = &ctx.worldstate_data.hubs;

        hubs.get(&self.0).cloned().unwrap_or(self.0)
    }
}
