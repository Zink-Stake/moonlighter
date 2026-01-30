use clap::Parser;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
struct Recipe {
	unique_vegs: BTreeMap<Veg, Processing>,
	cereal: Veg,
	filler_sugars: usize,
}

impl Recipe {
	fn affinity(&self) -> usize {
		let sum: usize = self.unique_vegs.iter().map(|(v, p)| v.value() + p.value()).sum();
		sum + self.cereal.value() + self.filler_sugars * 47
	}
}

impl Default for Recipe {
	fn default() -> Self {
		Recipe {
			unique_vegs: Default::default(),
			cereal: Veg::Wheat,
			filler_sugars: 1,
		}
	}
}

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
			Veg::Wheat => 25,
			Veg::Barley => 23,
			Veg::Pumpkin => 45,
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
	Wheat,
	Barley,
	Pumpkin,
}

impl Processing {
	fn value(&self) -> usize {
		match self {
			Processing::Whole => 0,
			Processing::Chopped => 16,
			Processing::Fried => 1,
			Processing::Mashed => 32,
			Processing::Roasted => 4,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default)]
enum Processing {
	#[default]
	Whole,
	Chopped,
	Fried,
	Mashed,
	Roasted,
}

#[derive(Parser)]
struct Options {
	#[arg(long)]
	rare: bool,
	target_value: usize,
	#[arg(long, default_value = "12")]
	max_length: usize,
	#[arg(long, default_value = "50")]
	max_sugars: usize,
}

type NodeType = Recipe;
type EdgeType = usize;
type RecipeGraph = petgraph::graph::Graph<NodeType, EdgeType>;

fn main() {
	let options = Options::parse();

	let pregraph = std::time::Instant::now();
	eprintln!("Populating graph...");
	let mut graph = RecipeGraph::default();

	let mut node_idxs: HashMap<NodeType, petgraph::graph::NodeIndex<petgraph::graph::DefaultIx>> = Default::default();

	let mut veg_pool: Vec<(Veg, Processing)> = Default::default();
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
		Veg::Pumpkin,
	] {
		for processing in [
			Processing::Chopped,
			//Processing::Fried,
			Processing::Mashed,
			// Processing::Roasted,
		] {
			veg_pool.push((veg, processing));
		}
	}

	let mut node_queue = vec![];
	let mut add_node = |idx: petgraph::graph::NodeIndex<petgraph::graph::DefaultIx>, graph: &mut RecipeGraph, node_queue: &mut Vec<petgraph::graph::NodeIndex<petgraph::graph::DefaultIx>>| {
		let current_node = &graph[idx].clone();
		if current_node.unique_vegs.len() < options.max_length {
			for (veg, processing) in veg_pool.clone() {
				if current_node.unique_vegs.contains_key(&veg) {
					continue;
				};
				let mut next_node = current_node.clone();
				next_node.unique_vegs.insert(veg, processing);
				let next_value = next_node.affinity();
				let next_idx = match node_idxs.get(&next_node) {
					Some(next_idx) => *next_idx,
					None => {
						// eprintln!("{:?}", next_node);
						let nnn = next_node.clone();
						let idx = graph.add_node(next_node);
						node_idxs.insert(nnn, idx);
						idx
					}
				};
				if graph.find_edge(idx, next_idx).is_none() {
					graph.add_edge(idx, next_idx, next_value);
					node_queue.push(next_idx);
				}
			}
		} else if current_node.filler_sugars == 1 {
			for add in 2..options.max_sugars {
				let mut next_node = current_node.clone();
				next_node.filler_sugars = add;
				let next_value = next_node.affinity();
				let next_idx = match node_idxs.get(&next_node) {
					Some(next_idx) => *next_idx,
					None => {
						// eprintln!("{:?}", next_node);
						let nnn = next_node.clone();
						let idx = graph.add_node(next_node);
						node_idxs.insert(nnn, idx);
						idx
					}
				};
				if graph.find_edge(idx, next_idx).is_none() {
					graph.add_edge(idx, next_idx, next_value);
					node_queue.push(next_idx);
				}
			}
		}
	};

	let w_idx = graph.add_node(Default::default());

	node_queue.push(w_idx);
	// node_queue.push(b_idx);

	// populate the graph
	while let Some(next_idx) = node_queue.pop() {
		add_node(next_idx, &mut graph, &mut node_queue);
		// eprintln!("Graph: {:?}", graph.edge_count());
		// eprintln!("Queue: {:?}", node_queue.len());
		// eprintln!(".");
	}

	let prefind = std::time::Instant::now();
	let tm = prefind.duration_since(pregraph).as_secs();
	eprintln!("Done in {tm}. Finding a recipe...");
	// find matching recipes

	let target_value = options.target_value;
	let mut offset = 0;
	offset += 40; // oven
	offset += 75; // cauldron
	offset += 6; // water
	if options.rare {
		offset += 1;
	}

	let mut dfs = petgraph::visit::Dfs::new(&graph, w_idx);

	let mut max_vegs = 0;
	let mut min_sugars = 99;
	while let Some(next) = dfs.next(&graph) {
		let next_node = &graph[next];
		let value = (next_node.affinity() + offset) % 138;

		if target_value == value && next_node.unique_vegs.len() > max_vegs || next_node.unique_vegs.len() == max_vegs && next_node.filler_sugars < min_sugars {
			max_vegs = next_node.unique_vegs.len();
			min_sugars = next_node.filler_sugars;
			eprintln!("Len: {} {:?}", graph[next].unique_vegs.len(), graph[next]);
		}
	}

	let postfind = std::time::Instant::now().duration_since(prefind).as_secs();
	let tm = prefind.duration_since(pregraph).as_secs();
	eprintln!("Done in {tm}.");
}
