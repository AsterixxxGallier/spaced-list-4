use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

impl<S: Spacing, Sub: SpacedList<S>> SpacedListSkeleton<S, Sub> {
	pub(in crate::spaced_lists::skeleton::traversal::navigation)
	fn last_link_index(&self) -> usize {
		self.capacity() - 1
	}

	pub(in crate::spaced_lists::skeleton::traversal::navigation)
	fn has_link_index_at(&self, node_index: usize, degree: usize) -> bool {
		can_have_link_index_at(node_index, degree) && self.is_in_bounds(node_index)
	}
}

// ╭───────────────────────────────╮
// ├───────────────╮               │
// ├───────╮       ├───────╮       │
// ├───╮   ├───╮   ├───╮   ├───╮   │
// │ 0 │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │
// 0   1   2   3   4   5   6   7   8

/// If there can be a link at `node_index` on `degree`.
///
/// Does not check bounds, only if there can logically be such a link index.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn can_have_link_index_at(node_index: usize, degree: usize) -> bool {
	node_index & degree == 0
}

/// The link index on `degree` after `link_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn link_index_right(link_index: usize, degree: usize) -> usize {
	link_index + (1 << (degree + 1))
}

/// The link index on `degree` before `link_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
///
/// # Panics
///
/// Panics if the result is negative.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn link_index_left(link_index: usize, degree: usize) -> usize {
	link_index - (1 << (degree + 1))
}

/// The link index on `degree` that is after `link_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn link_index_up_right(link_index: usize, degree: usize) -> usize {
	link_index + (1 << degree)
}

/// The link index above `degree` that is before `link_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
///
/// # Panics
///
/// Panics if the result is negative.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn link_index_up_left(link_index: usize, degree: usize) -> usize {
	link_index - (1 << degree)
}

/// The link index on `degree - 1` that is after `link_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
///
/// # Panics
///
/// Panics if `degree` is zero.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn link_index_down_right(link_index: usize, degree: usize) -> usize {
	link_index + (1 << (degree - 1))
}

/// The link index on `degree - 1` that is before `link_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
///
/// # Panics
///
/// Panics if the result is negative or `degree` is zero.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn link_index_down_left(link_index: usize, degree: usize) -> usize {
	link_index - (1 << (degree - 1))
}
