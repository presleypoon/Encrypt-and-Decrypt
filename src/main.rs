use std::io::{Write, stdin, stdout};

type Table = [Vec<bool>; 256];

#[derive(Clone, Debug)]
struct Tree {
	tree: TreeChild,
	freq: usize,
	num_child: usize,
}
impl Tree {
	fn new(input: String) -> Self {
		let mut nodes: Vec<Tree> = vec![
			Self {
				tree: TreeChild::Char { item: '\0' },
				freq: 0,
				num_child: 0
			};
			256
		];
		for (i, item) in nodes.iter_mut().enumerate() {
			item.tree = TreeChild::Char {
				item: i as u8 as char,
			};
		}
		for r#char in input.chars() {
			nodes[r#char as usize].freq += 1;
		}

		nodes.sort_by(|a: &Tree, b: &Tree| -> std::cmp::Ordering {
			a.freq
				.cmp(&b.freq)
				.then_with(|| -> std::cmp::Ordering { a.num_child.cmp(&b.num_child) })
		});
		while nodes[0].freq == 0 {
			nodes[0] = Tree {
				tree: TreeChild::Null,
				freq: usize::MAX,
				num_child: 0,
			};

			nodes.sort_by(|a: &Tree, b: &Tree| -> std::cmp::Ordering {
				a.freq
					.cmp(&b.freq)
					.then_with(|| -> std::cmp::Ordering { a.num_child.cmp(&b.num_child) })
			});
		}
		while nodes[1].tree != TreeChild::Null {
			nodes[0] = Tree {
				tree: TreeChild::Tree {
					item: Box::new([nodes[0].tree.clone(), nodes[1].tree.clone()]),
				},
				freq: nodes[0].freq + nodes[1].freq,
				num_child: match nodes[0].tree {
					TreeChild::Char { item: _ } => 1,
					TreeChild::Tree { item: _ } => nodes[0].num_child,
					TreeChild::Null => unreachable!(),
				} + match nodes[1].tree {
					TreeChild::Char { item: _ } => 1,
					TreeChild::Tree { item: _ } => nodes[1].num_child,
					TreeChild::Null => unreachable!(),
				},
			};
			nodes[1] = Tree {
				tree: TreeChild::Null,
				freq: usize::MAX,
				num_child: 0,
			};

			nodes.sort_by(|a: &Tree, b: &Tree| -> std::cmp::Ordering {
				a.freq
					.cmp(&b.freq)
					.then_with(|| -> std::cmp::Ordering { a.num_child.cmp(&b.num_child) })
			});
		}

		<Tree as Clone>::clone(&nodes[0])
	}
}

#[derive(Clone, Debug, PartialEq)]
enum TreeChild {
	Tree { item: Box<[TreeChild; 2]> },
	Char { item: char },
	Null,
}

fn main() {
	let mut input: String = String::new();
	print!("Input: ");
	stdout().flush().expect("Cannot flush");
	stdin().read_line(&mut input).expect("Failed to read line");
	input = input.trim().to_string();

	let output: String = encrypt(input);
	todo!();
}

fn encrypt(input: String) -> String {
	let tree: Tree = Tree::new(input);
	let table: Table = table(tree);
	replace_str_with_table(input, table);
}

fn table(tree: Tree) -> Table {
	let mut ret: Table = [const { Vec::new() }; 256];
	let mut path: Vec<bool> = vec![];
	let mut last: Option<bool> = None;
	let mut ran: bool = false;

	while !path.is_empty() || last == Some(false) || !ran {
		let this_child: TreeChild = branch_finder(&tree, path.clone()).unwrap_or_else(|| -> TreeChild { unreachable!() });
		
		match this_child {
			TreeChild::Tree { .. } => {
				if let Some(bool_last) = last {
					if bool_last {
						last = path.pop();
					} else {
						path.push(true);
						last = None;
					}
				} else {
					path.push(false);
					last = None;
				}
			}
			TreeChild::Char { item } => {
				ret[item as usize] = path.clone();
				last = path.pop();
			}
			TreeChild::Null => unreachable!(),
		}
		ran = true;
	}

	ret
}

fn branch_finder(tree: &Tree, path: Vec<bool>) -> Option<TreeChild> {
	let mut this_tree: &TreeChild = &tree.tree;

	for this_bool in path {
		match this_tree {
			TreeChild::Tree { item } => {
				this_tree = &item[this_bool as usize];
			}
			_ => return None,
		}
	}

	Some(this_tree.clone())
}

fn replace_str_with_table(input: String, mut table: Table) -> String {
	let mut ret: Vec<bool> = Vec::new();

	for r#char in input.chars() {
		ret.append(&mut table[r#char as usize]);
	}

	

	todo!()
}
