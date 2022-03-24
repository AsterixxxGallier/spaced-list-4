use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

impl<'a, S: Spacing, List: SpacedList<S>> Traversal<'a, S, List> {
	/// IMPORTANT: After calling this method, `self.link_index` no longer refers to the link length
	/// between the current node and the next one at `self.degree`, because after calling this,
	/// there cannot be such an index.
	pub(in crate::spaced_lists::skeleton::traversal::navigation)
	fn advance_unchecked(&mut self) {
		self.position += self.list.skeleton().get_link_length_at(self.link_index);
		self.node_index += 1 << self.degree;
	}

	/// ╭───────────────────────────────────────────────────────────────╮
	/// ├───────────────────────────────╮                               │
	///C├───────────────╮D              ├───────────────╮               │
	/// ├───────╮B     E├───────╮       ├───────╮       ├───────╮       │
	/// ├───╮   ├───╮A F├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
	/// ╵ 0 ╵ 1 ╵ 2 ╵ 3 ╵ 4 ╵ 5 ╵ 6 ╵ 7 ╵ 8 ╵ 9 ╵ A ╵ B ╵ C ╵ D ╵ E ╵ F ╵
	/// Assuming that node_index = 3, and degree = 0,
	/// in order to advance, one would have to
	/// unwind to B, see that advance_unchecked is still not an option,
	/// unwind to C, see that advance_unchecked now is an option,
	/// advance_unchecked to D,
	/// descend to E,
	/// descend to F
	pub fn advance(mut self) -> Self {
		if self.node_index == self.list.skeleton().capacity() {
			match self.super_traversal {
				Some(super_traversal) => {
					self = *super_traversal;
					return self.advance();
				}
				None => {
					panic!("Cannot advance past skeleton capacity")
				}
			}
		}

		let degree_before = self.degree;
		let link_index_before = self.link_index;
		while self.node_index & (1 << self.degree) > 0 {
			self.position -= self.list.skeleton().get_link_length_at(self.link_index);
			self.node_index -= 1 << self.degree;
			self.link_index -= 1 << self.degree;
			self.degree += 1;
		}
		self.advance_unchecked();
		self.degree = degree_before;
		self.link_index = link_index_before + (1 << self.degree);
		self
	}
}