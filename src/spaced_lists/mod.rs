use std::default::default;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::{Zero, zero};
use crate::spaced_lists::crate_spaced_list::CrateSpacedList;

use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;

pub trait Spacing = Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Zero + Ord + Copy;

pub(crate) mod crate_spaced_list {
	use crate::spaced_lists::skeleton::SpacedListSkeleton;
	use crate::spaced_lists::skeleton::traversal::Traversal;
	use crate::spaced_lists::{SpacedList, Spacing};

	pub trait CrateSpacedList<S: Spacing>: Default {
		fn skeleton(&self) -> &SpacedListSkeleton<S, Self>;

		fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self>;

		fn traversal(&self) -> Traversal<S, Self> where Self: SpacedList<S> {
			Traversal::new(self)
		}

		fn grow(&mut self) {
			self.skeleton_mut().grow()
		}

		fn size_mut(&mut self) -> &mut usize;
	}
}

pub trait SpacedList<S: Spacing>: CrateSpacedList<S> {
	fn length(&self) -> S;

	fn size(&self) -> usize;

	fn capacity(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.size() == 0
	}

	fn is_full(&self) -> bool {
		self.size() == self.capacity()
	}

	fn append_node(&mut self, distance: S) {
		if self.is_full() {
			self.grow()
		}

		let size = self.size();
		self.skeleton_mut().inflate_at(size, distance);
		*self.size_mut() += 1;
	}

	fn node_before(&self, position: S) -> Option<Traversal<S, Self>> {
		let traversal = self.traversal().advance_while(|traversal| traversal.position < position);
		if traversal.position < position {
			Some(traversal)
		} else {
			None
		}
	}

	fn node_at_or_before(&self, position: S) -> Option<Traversal<S, Self>> {
		let traversal = self.traversal().advance_while(|traversal| traversal.position <= position);
		if traversal.position <= position {
			Some(traversal)
		} else {
			None
		}
	}

	fn node_at(&self, position: S) -> Option<Traversal<S, Self>> {
		let traversal = self.traversal().advance_while(|traversal| traversal.position <= position);
		if traversal.position == position {
			Some(traversal)
		} else {
			None
		}
	}

	fn node_at_or_after(&self, position: S) -> Option<Traversal<S, Self>> {
		if position > self.length() {
			return None;
		}
		let traversal = self.traversal().advance_while(|traversal| traversal.position <= position);
		if traversal.position == position {
			Some(traversal)
		} else {
			Some(traversal.advance())
		}
	}

	fn node_after(&self, position: S) -> Option<Traversal<S, Self>> {
		if position >= self.length() {
			return None;
		}
		let traversal = self.traversal().advance_while(|traversal| traversal.position <= position);
		Some(traversal.advance())
	}

	fn insert_node(&mut self, position: S) {
		if position >= self.length() || self.capacity() == 0 {
			self.append_node(position - self.length())
		} else {
			let mut traversal = self.traversal();
			traversal.advance_while_shallow(|traversal| traversal.position <= position);
			let Traversal {
				degree: _,
				position: sublist_position,
				node_index,
				link_index: _,
				list,
				super_traversal: _
			} = traversal;
			let sublist = self.skeleton_mut().get_sublist_at_mut(node_index).get_or_insert_default();
			sublist.insert_node(position - sublist_position)
		}
	}
}

pub(crate) mod hollow;

mod skeleton;
