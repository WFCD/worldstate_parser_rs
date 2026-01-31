use heck::ToTitleCase;

use crate::core::{ContextRef, InternalPath, Resolve, resolve_with, split_camel_case};

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

impl Resolve<ContextRef<'_>> for InternalPath<resolve_with::VaultTraderItem> {
    type Output = String;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        match &self.path {
            path if path.contains("Weapons") => prime_part(self.cast()),
            path if path.contains("Powersuits") => {
                InternalPath::<resolve_with::LanguageItems>::resolve(self.cast(), ctx)
            },
            path if path.contains("Upgrades/Skins") => skins(self.cast(), ctx),
            path if path.contains("Projections") => relics(self.cast(), ctx),
            path if path.contains("MegaPrimeVault") => prime_vault_pkg(self.cast()),

            _ => self
                .last_segment()
                .map(|s| s.to_title_case())
                .unwrap_or_default(),
        }
    }
}

pub fn prime_part(path: InternalPath) -> String {
    let Some(mut parts) = path.last_segment().map(split_camel_case) else {
        return path.path;
    };

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
