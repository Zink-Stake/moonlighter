use std::collections::{BTreeSet, HashSet};

impl Veg {
	fn value(&self) -> usize {
		match self {
			Veg::Cabbage => 42,
			Veg::Carrot => 41,
			Veg::Corn => 44,
			Veg::Cucumber => 17,
			Veg::Lettuce => 45,
			Veg::Onion => 91,
			Veg::Pea => 58,
			Veg::PeaPod => 46,
			Veg::Potato => 47,
			Veg::Tomato => 43,
			Veg::Garlic => 92,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default)]
enum Veg {
	#[default]
	Cabbage,
	Carrot,
	Corn,
	Cucumber,
	Lettuce,
	Onion,
	Pea,
	PeaPod,
	Potato,
	Tomato,
	Garlic,
}

impl Processing {
	fn value(&self) -> usize {
		match self {
			Processing::Whole => 0,
			Processing::Chopped => 16,
			Processing::Fried => 1,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default)]
enum Processing {
	#[default]
	Whole,
	Chopped,
	Fried,
}

fn chain_value(chain: &BTreeSet<(Veg, Processing)>) -> usize {
	chain.iter().map(|(v, p)| v.value() + p.value()).sum()
}

struct Recipe {
	items: BTreeSet<(Veg, Processing)>,
}

#[derive(Default)]
struct RecipeGraph {
	nodes: Vec<BTreeSet<(Veg, Processing)>>,
	edges: Vec<(BTreeSet<(Veg, Processing)>, (Veg, Processing))>,
	visit_map: HashSet<usize>,
}

impl petgraph::visit::GraphBase for RecipeGraph {
	type NodeId = usize;
	type EdgeId = usize;
}

impl petgraph::visit::Visitable for RecipeGraph {
	type Map = HashSet<usize>;

	fn visit_map(&self) -> Self::Map {
		self.visit_map.clone()
	}
	fn reset_map(&self, map: &mut Self::Map) {
		map.clear()
	}
}

impl<'a> petgraph::visit::IntoNeighbors for &'a RecipeGraph {
	type Neighbors = std::slice::Iter<'a, (Veg, Processing)>;

	fn neighbors(self, a: usize) -> Self::Neighbors {
		let walked_nodes = &self.nodes[a];
		let mut v = vec![];
		for veg in [
			Veg::Cabbage,
			Veg::Carrot,
			Veg::Corn,
			Veg::Cucumber,
			Veg::Lettuce,
			Veg::Onion,
			Veg::Pea,
			Veg::PeaPod,
			Veg::Potato,
			Veg::Tomato,
			Veg::Garlic,
		] {
			for processing in [Processing::Whole, Processing::Chopped, Processing::Fried] {
				if !walked_nodes.contains(&(veg, processing)) {
					v.push((veg, processing));
				}
			}
		}
		v.into_iter()
	}
}

fn main() {
	let mut recipe_graph = RecipeGraph::default();

	recipe_graph.nodes.push(Default::default());

	let mut dfs = petgraph::visit::Dfs::new(&recipe_graph, 0);

	dfs.next(&recipe_graph);

	// let mut dead_ends: BTreeSet<BTreeSet<(Veg, Processing)>> = BTreeSet::default();
	// let target = 100;
	// let max_ingredients = 10;
	// let mut current_chain = BTreeSet::default();
	// let mut options = BTreeSet::default();
	// loop {
	// 	if current_chain.len() == max_ingredients {
	// 		dead_ends.insert(current_chain.clone());
	// 		current_chain.clear();
	// 	}
	// 	let mut progress = false;
	// 	for veg in [Veg::Cabbage,
	// 		Veg::Carrot,
	// 		Veg::Corn,
	// 		Veg::Cucumber,
	// 		Veg::Lettuce,
	// 		Veg::Onion,
	// 		Veg::Pea,
	// 		Veg::PeaPod,
	// 		Veg::Potato,
	// 		Veg::Tomato,
	// 		Veg::Garlic
	// 	] {
	// 		let mut keep_breaking = false;
	// 		for processing in [Processing::Whole, Processing::Chopped, Processing::Fried] {
	// 			let current_pair = (veg, processing);
	// 			// eprintln!("Pair: {:?}", current_pair);
	// 			if !current_chain.insert(current_pair) {
	// 				// eprintln!("failed to insert");
	// 				continue
	// 			}
	// 			if dead_ends.contains(&current_chain) {
	// 				current_chain.remove(&current_pair);
	// 				// eprintln!("dead end");
	// 				continue
	// 			}
	// 			if chain_value(&current_chain) == target {
	// 				// eprintln!("new hit!");
	// 				if options.insert(current_chain.clone()) {
	// 					eprintln!("New option found: {:?}", current_chain);
	// 				}
	// 			}
	// 			progress = true;
	// 			break
	//
	// 		}
	// 		if progress {
	// 			// eprintln!("made progress, that's enough");
	// 			break
	// 		}
	// 	}
	// 	if progress == false {
	// 		if current_chain.len() == 0 {
	// 			break
	// 		}
	// 		dead_ends.insert(current_chain.clone());
	// 		current_chain.clear();
	// 	}
	// 	// println!("dead ends: {:?}, chain: {}, options: {}", dead_ends.len(), current_chain.len(), options.len());
	// 	// std::thread::sleep(std::time::Duration::from_secs(1));
	// }
	// for option in options {
	// 	println!("{:?}", option);
	// }
}
