use std::collections::HashMap;
use std::default::default;
use std::fmt::{Debug, Display, Error, Formatter, Write};
use std::io::Write as IOWrite;
use std::iter;
use std::ops::Neg;

use num_traits::zero;

use crate::custom_fmt::{CustomFormat, CustomFormatWrapper};
use crate::spaced_lists::skeleton::SpacedListSkeleton;
use crate::spaced_lists::SpacedList;
use crate::spaced_lists::Spacing;

#[derive(Clone)]
pub struct SkeletonFormatOptions {
	pub show_link_lengths: bool,
	pub highlighted_links: Vec<usize>,
	pub highlighted_nodes: Vec<usize>,
	pub show_sublists: bool,
	pub sublist_options: HashMap<usize, SkeletonFormatOptions>,
}

impl Default for SkeletonFormatOptions {
	fn default() -> Self {
		Self {
			show_link_lengths: true,
			show_sublists: true,
			highlighted_links: Vec::new(),
			highlighted_nodes: Vec::new(),
			sublist_options: HashMap::new(),
		}
	}
}

impl<S: Spacing + Display, Sub: SpacedList<S>> SpacedListSkeleton<S, Sub> {
	pub(crate) fn highlighted_format(&self, links: Vec<usize>, nodes: Vec<usize>) -> CustomFormatWrapper<Self> {
		self.custom_format(SkeletonFormatOptions {
			highlighted_links: links,
			highlighted_nodes: nodes,
			..default()
		})
	}
}

impl<S: Spacing + Display, Sub: SpacedList<S>> CustomFormat for SpacedListSkeleton<S, Sub> {
	type Options = SkeletonFormatOptions;

	fn fmt(&self, f: &mut Formatter<'_>, options: &SkeletonFormatOptions) -> std::fmt::Result {
		let depth = self.depth();
		let capacity = self.capacity();

		if capacity == 0 {
			write!(f, "[empty skeleton]")?;
			return Ok(());
		}

		for degree in (0..depth).rev() {
			if degree == depth - 1 {
				if options.highlighted_links.contains(&(self.capacity() - 1)) {
					if options.show_link_lengths {
						write!(f, "┍━{:━^width$}┑", self.length(), width = (1 << degree) * 4 - 2)?;
					} else {
						write!(f, "┍━{:━^width$}┑", "", width = (1 << degree) * 4 - 2)?;
					}
				} else {
					if options.show_link_lengths {
						write!(f, "╭─{:─^width$}╮", self.length(), width = (1 << degree) * 4 - 2)?;
					} else {
						write!(f, "╭─{:─^width$}╮", "", width = (1 << degree) * 4 - 2)?;
					}
				}
			} else {
				for index in 0..(1 << (depth - degree - 2)) {
					let link_index = (2 << degree) * index + (1 << degree) - 1;
					let link_length = self.get_link_length_at(link_index);
					if options.highlighted_links.contains(&link_index) {
						if options.show_link_lengths {
							if format!("{}", link_length).len() >= 3 {
								write!(f, "┝{:━^width$}┑", link_length, width = (1 << degree) * 4 - 1)?;
							} else {
								write!(f, "┝━{:━^width$}┑", link_length, width = (1 << degree) * 4 - 2)?;
							}
						} else {
							write!(f, "┝━{:━^width$}┑", "", width = (1 << degree) * 4 - 2)?;
						}
					} else {
						if options.show_link_lengths {
							if format!("{}", link_length).len() >= 3 {
								write!(f, "├{:─^width$}╮", link_length, width = (1 << degree) * 4 - 1)?;
							} else {
								write!(f, "├─{:─^width$}╮", link_length, width = (1 << degree) * 4 - 2)?;
							}
						} else {
							write!(f, "├─{:─^width$}╮", "", width = (1 << degree) * 4 - 2)?;
						}
					}

					write!(f, "{:<width$}", " ", width = (1 << degree) * 4 - 1)?;
				}
				f.write_char('│')?;
			}
			f.write_char('\n')?;
		}

		if options.show_sublists && self.sublists.iter().any(|it| it.is_some()) {
			let mut sublists = vec![];
			for (index, sublist) in self.sublists.iter().enumerate() {
				match sublist {
					Some(sublist) => {
						write!(f, "╰{:03}", sublists.len())?;
						sublists.push((index, sublist));
					}
					None => {
						if options.highlighted_nodes.contains(&index) {
							write!(f, "╹   ")?;
						} else {
							write!(f, "╵   ")?;
						}
					}
				}
			}
			if options.highlighted_nodes.contains(&self.capacity()) {
				write!(f, "╹")?;
			} else {
				write!(f, "╵")?;
			}

			for (id, (index, sublist)) in sublists.into_iter().enumerate() {
				if id == 0 {
					writeln!(f);
					writeln!(f, " ┍╸{:03}: ", id)?;
				} else {
					writeln!(f, " ┝╸{:03}: ", id)?;
				}
				let mut vec = vec![];
				let sublist_options = options.sublist_options.get(&index).cloned()
				                             .unwrap_or_else(|| SkeletonFormatOptions {
					                             highlighted_links: vec![],
					                             highlighted_nodes: vec![],
					                             sublist_options: HashMap::new(),
					                             ..options.clone()
				                             });
				write!(vec, "{:?}", sublist.skeleton().custom_format(sublist_options));
				let string = String::from_utf8(vec).unwrap();
				for line in string.lines() {
					writeln!(f, " │ {}", line)?;
				}
			}
		} else {
			for index in 0..self.capacity() {
				if options.highlighted_nodes.contains(&index) {
					write!(f, "╹   ")?;
				} else {
					write!(f, "╵   ")?;
				}
			}
			if options.highlighted_nodes.contains(&self.capacity()) {
				write!(f, "╹")?;
			} else {
				write!(f, "╵")?;
			}
		}

		Ok(())
	}
}