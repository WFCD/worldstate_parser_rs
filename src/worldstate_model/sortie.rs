use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{mission_type::MissionType, region::Region};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Sortie {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub reward: String,

    pub seed: u64,

    pub boss: Option<String>,

    pub extra_drops: Vec<serde_json::Value>,

    pub variants: Vec<SortieVariant>,

    pub twitter: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SortieVariant {
    pub mission_type: MissionType,

    pub modifier_type: String,

    pub region: Option<Region>,

    pub tileset: String,
}

pub(crate) mod unmapped {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    use crate::{
        core::{
            Context,
            InternalPath,
            Resolve,
            resolvable_string::ResolvableString,
            resolve_with,
            sol_node::SolNode,
        },
        worldstate_model::{
            Id,
            WorldstateMissionType,
            deserialize_mongo_date,
            sortie::{Sortie, SortieVariant},
        },
    };

    impl Resolve<Context<'_>> for SortieUnmapped {
        type Output = Sortie;

        fn resolve(self, ctx: Context<'_>) -> Self::Output {
            Sortie {
                activation: self.activation,
                expiry: self.expiry,
                boss: self.boss.resolve(ctx).cloned().map(|boss| boss.name),
                extra_drops: self.extra_drops,
                id: self.id.oid,
                reward: self.reward.resolve(()),
                seed: self.seed,
                twitter: self.twitter,
                variants: self.variants.resolve(ctx),
            }
        }
    }

    impl Resolve<Context<'_>> for SortieVariantUnmapped {
        type Output = SortieVariant;

        fn resolve(self, ctx: Context<'_>) -> Self::Output {
            SortieVariant {
                mission_type: self.mission_type.into(),
                modifier_type: self.modifier_type.resolve(ctx),
                region: self.node.resolve(ctx).cloned(),
                tileset: self.tileset,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SortieUnmapped {
        #[serde(rename = "_id")]
        pub id: Id,

        #[serde(deserialize_with = "deserialize_mongo_date")]
        pub activation: DateTime<Utc>,

        #[serde(deserialize_with = "deserialize_mongo_date")]
        pub expiry: DateTime<Utc>,

        pub reward: InternalPath<resolve_with::LastSegment>,

        pub seed: u64,

        pub boss: ResolvableString<resolve_with::sortie::Boss>,

        #[serde(default)]
        pub extra_drops: Vec<serde_json::Value>,

        pub variants: Vec<SortieVariantUnmapped>,

        pub twitter: bool,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SortieVariantUnmapped {
        pub mission_type: WorldstateMissionType,

        pub modifier_type: ResolvableString<resolve_with::sortie::Modifier>,

        pub node: SolNode,

        pub tileset: String,
    }
}
