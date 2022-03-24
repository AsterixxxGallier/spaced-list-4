use std::fmt::{Debug, Formatter};
use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

impl<S: Spacing + Debug, List: SpacedList<S>> Debug for Traversal<'_, S, List> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Traversal")
		 .field("degree", &self.degree)
		 .field("position", &self.position)
		 .field("node_index", &self.node_index)
		 .finish()
	}
}