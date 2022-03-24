use std::mem;
use std::ops::Deref;
use std::rc::Rc;

use num_traits::zero;

use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

mod horizontal;

mod vertical;

mod links;

mod nodes;

impl<'a, S: Spacing, List: SpacedList<S>> Traversal<'a, S, List> {
	pub fn advance_while_shallow<F: Fn(&Self) -> bool>(&mut self, condition: F) {
		while self.degree > 0 && self.node_index < self.list.size() - 1 {
			let super_traversal = self.super_traversal.take();
			let mut next = self.clone();
			next.descend_and_advance_shallow();
			if condition(&next) {
				*self = next;
			} else {
				self.descend_shallow();
			}
			self.super_traversal = super_traversal;
		}
		if self.node_index < self.list.size() - 1 {
			let mut next = self.clone();
			next.advance_unchecked();
			if condition(&next) {
				*self = next;
			}
		}
	}

	pub fn advance_while<F: Fn(&Self) -> bool>(mut self, condition: F) -> Self {
		// fixme link index overflow error, reproduce by calling node_at with position = length
		while self.can_descend() && self.node_index < self.list.size() - 1 {
			let super_traversal = self.super_traversal.take();
			let mut next = self.clone();
			next = next.descend_and_advance();
			if condition(&next) {
				self = next;
			} else {
				self = self.descend();
			}
			self.super_traversal = super_traversal;
		}
		if self.node_index < self.list.size() - 1 {
			let mut next = self.clone();
			next.advance_unchecked();
			if condition(&next) {
				self = next;
			}
		}
		self
	}

	/// # Panics
	///
	/// Panics if `self.degree` is `0`.
	fn descend_and_advance_shallow(&mut self) {
		self.degree -= 1;
		self.position += self.list.skeleton().get_link_length_at(self.link_index);
		self.node_index += 1 << self.degree;
		self.link_index += 1 << self.degree;
	}

	/// # Panics
	///
	/// Panics if `self.degree` is `0` and there is no sublist to descend into.
	fn descend_and_advance(mut self) -> Self {
		if self.degree > 0 {
			self.descend_and_advance_shallow();
			self
		} else {
			let sublist = self.list.skeleton().get_sublist_at(self.node_index).as_ref().unwrap();
			let new = Traversal {
				degree: sublist.skeleton().depth() - 1,
				node_index: 0,
				link_index: sublist.skeleton().capacity() - 1,
				position: self.position + sublist.skeleton().get_link_length_at(0),
				list: sublist,
				super_traversal: Some(Box::new(self)),
			};
			new
		}
	}
}