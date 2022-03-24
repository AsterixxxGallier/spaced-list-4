use std::fmt::{Debug, Display, Error, Formatter, Write};
use std::io::Write as IOWrite;
use std::iter;
use std::marker::PhantomData;
use std::ops::Neg;

use num_traits::zero;

use crate::spaced_lists::{CrateSpacedList, SpacedList};
use crate::spaced_lists::Spacing;

#[derive(Eq, PartialEq)]
pub struct SpacedListSkeleton<S: Spacing, Sub: CrateSpacedList<S>> {
	link_lengths: Vec<S>,
	sublists: Vec<Option<Sub>>,
}

mod display;

impl<S: Spacing, Sub: CrateSpacedList<S>> Default for SpacedListSkeleton<S, Sub> {
	fn default() -> Self {
		Self {
			link_lengths: vec![],
			sublists: vec![],
		}
	}
}

impl<S: Spacing, Sub: CrateSpacedList<S>> SpacedListSkeleton<S, Sub> {
	/// # Panics
	///
	/// Panics when `index` is out of bounds.
	pub(crate) fn get_link_length_at(&self, index: usize) -> S {
		self.link_lengths[index]
	}

	/// # Panics
	///
	/// Panics when `index` is out of bounds.
	pub(crate) fn get_link_length_at_mut(&mut self, index: usize) -> &mut S {
		&mut self.link_lengths[index]
	}

	/// # Panics
	///
	/// Panics when `index` is out of bounds.
	pub(crate) fn get_sublist_at(&self, index: usize) -> &Option<Sub> {
		&self.sublists[index]
	}

	/// # Panics
	///
	/// Panics when `index` is out of bounds.
	pub(crate) fn get_sublist_at_mut(&mut self, index: usize) -> &mut Option<Sub> {
		&mut self.sublists[index]
	}

	/// # Panics
	///
	/// Panics when `index` is out of bounds.
	pub(crate) fn get_or_add_sublist_at(&mut self, index: usize) -> &Sub {
		self.get_sublist_at_mut(index).get_or_insert_default()
	}

	/// # Panics
	///
	/// Panics when `index` is out of bounds.
	pub(crate) fn get_or_add_sublist_at_mut(&mut self, index: usize) -> &mut Sub {
		self.get_sublist_at_mut(index).get_or_insert_default()
	}

	fn depth(&self) -> usize {
		if self.link_lengths.is_empty() {
			0
		} else {
			(self.link_lengths.len().trailing_zeros() + 1) as usize
		}
	}

	fn is_in_bounds(&self, index: usize) -> bool {
		index < self.capacity()
	}

	/// The number of nodes that fit in this list, excluding node zero (= number of link lengths)
	pub(crate) fn capacity(&self) -> usize {
		self.link_lengths.len()
	}

	pub(crate) fn length(&self) -> S {
		*self.link_lengths.last().unwrap_or(&zero())
	}

	/// Doubles this lists capacity, or increase it to one if it is zero.
	pub(crate) fn grow(&mut self) {
		if self.link_lengths.is_empty() {
			self.link_lengths.push(zero());
			self.sublists.push(None);
		} else {
			let length = self.length();
			self.sublists.extend(iter::repeat_with(|| None).take(self.capacity()));
			self.link_lengths.extend(iter::repeat_with(|| S::zero()).take(self.capacity() - 1));
			self.link_lengths.push(length);
		}
	}

	/// Inflates the link at the specified index.
	pub(crate) fn inflate_at(&mut self, link_index: usize, amount: S) {
		let mut link_index = link_index;
		for degree in 0..self.depth() {
			if (link_index >> degree) & 1 == 0 {
				*self.get_link_length_at_mut(link_index) += amount;
				link_index += 1 << degree;
			}
		}
	}

	/// Negation of [`Self::inflate_at`]
	pub(crate) fn deflate_at(&mut self, link_index: usize, amount: S) where S: Neg<Output = S> {
		self.inflate_at(link_index, -amount)
	}
}

pub(crate) mod traversal;

mod tests;
