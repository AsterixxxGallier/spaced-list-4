use std::default::default;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::marker::PhantomData;

use num_traits::{Zero, zero};

use crate::spaced_lists::{CrateSpacedList, SpacedList, Spacing};
use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::skeleton::traversal::Traversal;

pub struct HollowSpacedList<S: Spacing> {
	skeleton: SpacedListSkeleton<S, Self>,
	size: usize,
	super_list: Option<Box<Self>>
}

impl<S: Spacing> Default for HollowSpacedList<S> {
	fn default() -> Self {
		Self {
			skeleton: default(),
			size: 0,
			super_list: None,
		}
	}
}

impl<S: Spacing> HollowSpacedList<S> {
	pub fn new() -> Self {
		default()
	}
}

impl<S: Spacing> CrateSpacedList<S> for HollowSpacedList<S> {
	fn skeleton(&self) -> &SpacedListSkeleton<S, Self> {
		&self.skeleton
	}

	fn skeleton_mut(&mut self) -> &mut SpacedListSkeleton<S, Self> {
		&mut self.skeleton
	}

	fn size_mut(&mut self) -> &mut usize {
		&mut self.size
	}
}

impl<S: Spacing> SpacedList<S> for HollowSpacedList<S> {
	fn length(&self) -> S {
		self.skeleton.length()
	}

	fn size(&self) -> usize {
		self.size
	}

	fn capacity(&self) -> usize {
		self.skeleton.capacity()
	}
}
