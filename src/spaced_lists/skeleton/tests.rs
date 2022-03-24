#![cfg(test)]

use std::fs::File;
use std::io::Write;
use rand::Rng;
use crate::spaced_lists::hollow::HollowSpacedList;
use crate::spaced_lists::skeleton::SpacedListSkeleton;

#[test]
#[ignore]
fn many_inflations() {
	let mut skeleton = SpacedListSkeleton::<i32, HollowSpacedList<i32>>::default();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();

	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	skeleton.grow();
	println!("capacity: {}, depth: {}", skeleton.capacity(), skeleton.depth());
	let mut rng = rand::thread_rng();
	for link_index in 0..skeleton.capacity() {
		// skeleton.inflate_at(link_index, rng.gen_range(0..1000))
		skeleton.inflate_at(link_index, 2)
	}
	// println!("{:?}", skeleton);
}
