use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::{navigation, Traversal};
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

impl<'a, S: Spacing, List: SpacedList<S>> Traversal<'a, S, List> {
	/// # Panics
	///
	/// Panics if `self.degree` is `0`.
	pub(in crate::spaced_lists::skeleton::traversal::navigation)
	fn descend_shallow(&mut self) {
		self.degree -= 1;
		self.link_index -= 1 << self.degree
	}

	/// # Panics
	///
	/// Panics if `self.degree` is `0` and there is no sublist to descend into.
	pub(in crate::spaced_lists::skeleton::traversal::navigation)
	fn descend(mut self) -> Self {
		if self.degree > 0 {
			self.descend_shallow();
			self
		} else {
			let sublist = self.list.skeleton().get_sublist_at(self.node_index).as_ref().unwrap();
			let new = Traversal {
				degree: sublist.skeleton().depth() - 1,
				position: self.position,
				node_index: 0,
				link_index: sublist.skeleton().capacity() - 1,
				list: sublist,
				super_traversal: Some(Box::new(self)),
			};
			new
		}
	}

	pub(in crate::spaced_lists::skeleton::traversal::navigation)
	fn can_descend(&self) -> bool {
		self.degree > 0 || self.list.skeleton().get_sublist_at(self.node_index).is_some()
	}
}