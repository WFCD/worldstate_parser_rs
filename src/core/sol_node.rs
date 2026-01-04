use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, Resolve},
    target_types::region::Region,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct SolNode(pub String);

impl<'a> Resolve<Context<'a>> for SolNode {
    type Output = Option<&'a Region>;

    fn resolve(self, ctx: Context<'a>) -> Self::Output {
        ctx.custom_maps.solnode_to_region.get(&self.0)
    }
}
