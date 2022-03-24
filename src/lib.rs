//!
//! Link length storage:
//! ```text
//! capacity = 1:
//! 0
//! capacity = 2:
//! 01
//! capacity = 4:
//! 0102
//! capacity = 8:
//! 01020103
//! capacity = 16
//! 0102010301020104
//! capacity = 32
//! 01020103010201040102010301020105
//! capacity = 64
//! 0102010301020104010201030102010501020103010201040102010301020106
//! ```
//!
//! A node is at every ^, the indices of these nodes are above, as are the indices of the links.
//! ```text
//!  0   1   2   3   4   5   6   7   8  (node indices)
//!  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |  (link indices)
//!  ^ 0 ^ 1 ^ 0 ^ 2 ^ 0 ^ 1 ^ 0 ^ 3 ^  (link degrees)
//!  ´‾‾‾´ | ´‾‾‾´ | ´‾‾‾´ | ´‾‾‾´ |    (link structure)
//!  ´‾‾‾‾‾‾‾´     | ´‾‾‾‾‾‾‾´     |
//!  ´‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾´             |
//!  ´‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾´
//! ```
//! The link index at index `i` of degree `d` stores the distance between the node at index
//! `i - 2^d + 1` and the node at index `i + 1`.
//!
//! The distance from node 1 to node 2, for example, could be calculated as the distance from node
//! 0 to node 2 minus the distance from node 0 to node 1.
//!
//!
//! Traversal example:
//!
//! ```text
//! 0   2   3   6   9 (node positions)
//! 0   1   2   3   4 (node indices)
//! | 0 | 1 | 2 | 3 | (link indices)
//! ^ 0 ^ 1 ^ 0 ^ 2 ^ (link degrees)
//! | 2 | 3 | 3 | 9 | (link lengths)
//! |   |   |   |   |
//! ´‾0‾´ | ´‾2‾´ |
//! ´‾‾‾1‾‾‾´     |
//! ´‾‾‾‾‾‾‾3‾‾‾‾‾‾‾´
//! ```
//!
//! target_position: 7
//!
//! degree: 2
//! position: 0
//! index: 0
//! link_index: 3
//! peek_distance: 9
//! peek_position: 9
//!
//! peek_position >= target_position => don't advance, only descend
//!
//! after descend:
//! degree: 1
//! position: 0
//! index: 0
//! link_index: 1
//! peek_distance: 3
//! peek_position: 3
//!
//! peek_position < target_position => advance and descend
//!
//! after advance:
//! degree: 1
//! position: 3
//! index: 2
//! link_index: $!/§&!/(%$&/="(§$& there is no link index at this degree, at this index
//! peek_distance: !§/%)$/)"(§$=!"§
//! peek_position: &)%/&$"=)%"=§$=$("
//!
//! after descend:
//! degree: 0
//! position: 3
//! index: 2
//! link_index: 2
//! peek_distance: 3
//! peek_position: 6
//!
//!
//! ```text
//!
//! ╭─────────────────────────────────────────────────────────────┬─╮
//! ├─────────────────────────────┬─╮                             ╎ │
//! ├─────────────┬─╮             ╎ ├─────────────┬─╮             ╎ │
//! ├─────┬─╮     ╎ ├─────┬─╮     ╎ ├─────┬─╮     ╎ ├─────┬─╮     ╎ │
//! ├─┬─╮ ╎ ├─┬─╮ ╎ ├─┬─╮ ╎ ├─┬─╮ ╎ ├─┬─╮ ╎ ├─┬─╮ ╎ ├─┬─╮ ╎ ├─┬─╮ ╎ │
//! ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
//!
//!
//!
//!
//!
//!
//! Structure:
//! ╭───────────────────────────────────────────────────────────────╮
//! ├───────────────────────────────╮                               │
//! ├───────────────╮               ├───────────────╮               │
//! ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
//! ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
//! ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
//!          0010        0101        1000    1010
//!          0011        0111        1001    1011
//!          0111        1111        1011    1111
//!          1111                    1111
//!
//! Structure:
//! ╭───────────────────────────────────────────────────────────────╮
//! ├───────────────────────────────╮                               │
//! ├───────────────╮               ├───────────────╮               │
//! ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
//! ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
//! ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
//! 00000   00010   00100   00110   01000   01010   01100   01110
//!     00001   00011   00101   00111   01001   01011   01101   01111
//!
//!
//!
//!
//! Traversal:
//!
//! First iteration: go down or go right-down?
//! ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//! ┞───────────────────────────────╮                               ┃
//! ├───────────────╮               ├───────────────╮               ┃
//! ├───────╮       ├───────╮       ├───────╮       ├───────╮       ┃
//! ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ┃
//! ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╹
//!
//! Go down.
//!
//! Second iteration: go down or go right-down?
//! ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
//! ┞───────────────╮               ┞───────────────╮
//! ├───────╮       ├───────╮       ├───────╮       ├───────╮
//! ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮
//! ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵
//!
//! Go right-down.
//!
//! Third iteration: go down or go right-down?
//!                                 ┏━━━━━━━━━━━━━━━┓
//!                                 ┞───────╮       ┞───────╮
//!                                 ├───╮   ├───╮   ├───╮   ├───╮
//!                                 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵
//!
//! Go down.
//!
//! Fourth iteration: go 
//!                                 ┏━━━━━━━┓
//!                                 ┞───╮   ┞───╮
//!                                 ╵ 0 ╵ 1 ╵ 0 ╵
//!
//!                                 ┏━━━┓
//!                                 ╹ 0 ╹
//!
//!                                     ╹
//!
//!
//!
//! ```

//! Traversal
//! - contains SpacedList
//!   - contains SpacedListSkeleton
//!     - contains S
//!
//! S must outlive SpacedListSkeleton
//! SpacedListSkeleton must outlive SpacedList
//! SpacedList must outlive Traversal
//!

#![feature(trait_alias)]
#![feature(int_log)]
#![feature(default_free_fn)]
#![feature(option_get_or_insert_default)]
#![allow(unused, clippy::collapsible_else_if)]

mod spaced_lists;

pub use spaced_lists::Spacing;
pub use spaced_lists::SpacedList;
pub use spaced_lists::hollow::HollowSpacedList;

mod custom_fmt;

