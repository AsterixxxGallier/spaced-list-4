#![cfg(test)]

use std::default::default;
use num_traits::real::Real;
use crate::custom_fmt::CustomFormat;
use crate::spaced_lists::hollow::HollowSpacedList;
use crate::spaced_lists::skeleton::display::SkeletonFormatOptions;
use crate::spaced_lists::skeleton::traversal::Traversal;
use crate::spaced_lists::{CrateSpacedList, SpacedList};

#[test]
fn advance_while() {
	let mut list = HollowSpacedList::<i64>::new();
	// list.insert_node(2);
	// list.insert_node(-5);
	// list.insert_node(5);
	// list.insert_node(-2);
	// list.insert_node(-1);
	for n in 0..=1000000 {
		list.insert_node(n)
	}
	// println!("{:?}", list.skeleton().default_format());

	// println!("{:?}", list.node_at(-1));

	for n in 0..1000000 {
		list.node_at(n);
	}

	// TODO test traversal with sublists
	// TODO test advance
	// TODO test advance with sublists
	// TODO implement higher-level methods on skeletons and lists
}