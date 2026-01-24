use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, Resolve},
    target_types::node::Node,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) struct SolNode(pub String);

impl<'a> Resolve<ContextRef<'a>> for SolNode {
    type Output = Option<&'a Node>;

    fn resolve(self, ctx: ContextRef<'a>) -> Self::Output {
        ctx.custom_maps.solnode_to_region.get(&self.0)
    }
}
