use serde::{Deserialize, Serialize};

use crate::{
    core::{InternalPath, Resolve, resolve_with},
    target_types::relic::{Relic, RelicReward, RelicRewardRarity},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum RelicArcane {
    Relic(ManifestRelic),
    Arcane(Arcane),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Arcane {
    pub unique_name: String,

    pub name: String,

    pub codex_secret: bool,

    #[serde(default)]
    pub level_stats: Vec<LevelStat>,

    #[serde(default)]
    pub exclude_from_codex: bool,

    #[serde(default)]
    pub rarity: ArcaneRarity,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LevelStat {
    pub stats: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ArcaneRarity {
    Legendary,

    Rare,

    Uncommon,

    #[default]
    Common,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestRelic {
    pub unique_name: String,

    pub name: String,

    pub codex_secret: bool,

    pub description: String,

    pub relic_rewards: Vec<ManifestRelicReward>,
}

impl Resolve<()> for ManifestRelic {
    type Output = Relic;

    fn resolve(self, _ctx: ()) -> Self::Output {
        Relic {
            unique_name: self.unique_name,
            name: self.name,
            codex_secret: self.codex_secret,
            description: self.description,
            relic_rewards: self.relic_rewards.resolve(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestRelicReward {
    pub reward_name: InternalPath<resolve_with::PrimePart>,

    pub rarity: ManifestRelicRewardRarity,

    pub tier: i64,

    pub item_count: i64,
}

impl Resolve<()> for ManifestRelicReward {
    type Output = RelicReward;

    fn resolve(self, _ctx: ()) -> Self::Output {
        RelicReward {
            reward_name: self.reward_name.resolve(()),
            rarity: self.rarity.resolve(()),
            tier: self.tier,
            item_count: self.item_count,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ManifestRelicRewardRarity {
    #[serde(rename = "COMMON")]
    Common,

    #[serde(rename = "RARE")]
    Rare,

    #[serde(rename = "UNCOMMON")]
    Uncommon,
}

impl Resolve<()> for ManifestRelicRewardRarity {
    type Output = RelicRewardRarity;

    fn resolve(self, _ctx: ()) -> Self::Output {
        match self {
            ManifestRelicRewardRarity::Common => RelicRewardRarity::Common,
            ManifestRelicRewardRarity::Rare => RelicRewardRarity::Rare,
            ManifestRelicRewardRarity::Uncommon => RelicRewardRarity::Uncommon,
        }
    }
}
