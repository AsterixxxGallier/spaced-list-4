use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

// ╭───────────────────────────────╮
// ├───────────────╮               │
// ├───────╮       ├───────╮       │
// ├───╮   ├───╮   ├───╮   ├───╮   │
// │ 0 │ 1 │ 2 │ 3 │ 4 │ 5 │ 6 │ 7 │
// 0   1   2   3   4   5   6   7   8

/// The node index on `degree` after `node_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn node_index_right(node_index: usize, degree: usize) -> usize {
	node_index + (1 << degree)
}

/// The node index on `degree` before `node_index`.
///
/// Does not check bounds of the result, nor does it check the validity of the parameters.
///
/// # Panics
///
/// Panics if the result is negative.
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn node_index_left(node_index: usize, degree: usize) -> usize {
	node_index - (1 << degree)
}

/// The node index above `node_index`, which is just `node_index`
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn node_index_up(node_index: usize) -> usize {
	node_index
}

/// The node index below `node_index`, which is just `node_index`
pub(in crate::spaced_lists::skeleton::traversal::navigation)
const fn node_index_down(node_index: usize) -> usize {
	node_index
}