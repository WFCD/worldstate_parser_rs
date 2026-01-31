use heck::ToTitleCase;

use crate::core::{
    ContextRef,
    InternalPath,
    Resolve,
    resolve_with::{self, LanguageItems},
};

impl Resolve<ContextRef<'_>> for InternalPath<resolve_with::CalendarReward> {
    type Output = String;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        match &self.path {
            path if path.contains("ArchonCrystal") => ctx
                .worldstate_data
                .archon_shards_store_item
                .get(path)
                .cloned()
                .unwrap_or_else(|| self.into_title_case_or_path()),

            path if path.contains("ArcaneUnlocker") => self
                .last_segment()
                .and_then(|s| s.strip_prefix("Weapon"))
                .map(|s| s.replace("Unlocker", "Adapter").to_title_case())
                .unwrap_or_else(|| self.into_title_case_or_path()),

            _ => InternalPath::<LanguageItems>::resolve(self.cast(), ctx),
        }
    }
}
