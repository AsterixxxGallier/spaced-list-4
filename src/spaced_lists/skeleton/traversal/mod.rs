use std::mem;
use std::ops::Deref;
use std::rc::Rc;

use num_traits::zero;

use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

pub struct Traversal<'a, S: 'a + Spacing, List: SpacedList<S>> {
	pub degree: usize,
	pub position: S,
	pub node_index: usize,
	pub link_index: usize,
	pub list: &'a List,
	pub super_traversal: Option<Box<Traversal<'a, S, List>>>,
}

impl<'a, S: Spacing, List: SpacedList<S>> Clone for Traversal<'a, S, List> {
	fn clone(&self) -> Self {
		Self {
			degree: self.degree,
			position: self.position,
			node_index: self.node_index,
			link_index: self.link_index,
			list: self.list,
			super_traversal: None,
		}
	}
}

// impl<'a, S: Spacing, List: SpacedList<S>> Copy for Traversal<'a, S, List> {}

impl<'a, S: Spacing, List: SpacedList<S>> Traversal<'a, S, List> {
	pub fn new(list: &'a List) -> Self {
		Self {
			degree: if list.skeleton().depth() > 0 { list.skeleton().depth() - 1 } else { 0 },
			position: zero(),
			node_index: 0,
			link_index: if list.skeleton().capacity() > 0 { list.skeleton().capacity() - 1 } else { 0 },
			list,
			super_traversal: None,
		}
	}
}

mod display;

mod navigation;

mod tests;
