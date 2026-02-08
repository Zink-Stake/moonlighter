use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Hash, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
	pub unique_vegs: BTreeMap<Veg, Processing>,
	pub cereals: Vec<Cereal>,
	pub filler_sugars: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Cereal {
	#[default]
	Barley,
	Oat,
	Rye,
	Wheat,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default, Serialize, Deserialize, clap::ValueEnum)]
pub enum Affinity {
	#[default]
	AggressiveFighting,
	Alchemy,
	AnimalHusbandry,
	AnimalTaming,
	Archaeology,
	Archery,
	ArmourSmithing,
	Axes,
	Baking,
	Cartography,
	Beverages,
	Blacksmithing,
	BladesSmithing,
	Body,
	BodyControl,
	BodyStamina,
	BodyStrength,
	Botanizing,
	Bowyery,
	Butchering,
	ButcheringKnife,
	Carpentry,
	CarvingKnife,
	Catapults,
	ChainArmourSmithing,
	Channeling,
	Climbing,
	ClothTailoring,
	Clubs,
	CoalMaking,
	Cooking,
	DairyFoodMaking,
	DefensiveFighting,
	Digging,
	Exorcism,
	Farming,
	Fighting,
	FineCarpentry,
	Firemaking,
	FirstAid,
	Fishing,
	Fletching,
	Foraging,
	Forestry,
	Gardening,
	Halberd,
	Hammer,
	Hammers,
	Hatchet,
	Healing,
	HotFoodCooking,
	HugeAxe,
	HugeClub,
	JewelrySmithing,
	Knives,
	LargeAxe,
	LargeMaul,
	LargeMetalShield,
	LargeWoodenShield,
	Leatherworking,
	LockPicking,
	Locksmithing,
	LongBow,
	LongSpear,
	Longsword,
	Masonry,
	Mauls,
	Meditating,
	ReflexBow,
	MediumMaul,
	MediumMetalShield,
	MediumWoodenShield,
	Metallurgy,
	Milking,
	Milling,
	Mind,
	MindLogic,
	MindSpeed,
	Mining,
	MiscItems,
	NaturalSubstances,
	Nature,
	NormalFighting,
	Papyrusmaking,
	Paving,
	Pickaxe,
	PlateArmourSmithing,
	Polearms,
	Pottery,
	Praying,
	Preaching,
	Prospecting,
	Puppeteering,
	Rake,
	Religion,
	Repairing,
	Restoration,
	Ropemaking,
	Saw,
	Scythe,
	ShieldBashing,
	ShieldSmithing,
	Shields,
	ShipBuilding,
	ShortBow,
	Shortsword,
	Shovel,
	Sickle,
	SmallAxe,
	SmallMaul,
	SmallMetalShield,
	SmallWoodenShield,
	Smithing,
	Soul,
	SoulDepth,
	SoulStrength,
	Staff,
	Stealing,
	StoneChisel,
	StoneCutting,
	Swords,
	Tailoring,
	Taunting,
	Thatching,
	Thievery,
	ToyMaking,
	Toys,
	Tracking,
	Traps,
	Trebuchets,
	TwoHandedSword,
	WarMachines,
	Warhammer,
	WeaponHeadsSmithing,
	WeaponSmithing,
	WeaponlessFighting,
	Woodcutting,
	Yoyo,
}

impl Affinity {
	pub fn offset(&self) -> usize {
		match self {
			Affinity::AggressiveFighting => 109,
			Affinity::Alchemy => 25,
			Affinity::AnimalHusbandry => 130,
			Affinity::AnimalTaming => 129,
			Affinity::Archaeology => 120,
			Affinity::Archery => 16,
			Affinity::ArmourSmithing => 22,
			Affinity::Axes => 11,
			Affinity::Baking => 62,
			Affinity::Cartography => 135,
			Affinity::Beverages => 63,
			Affinity::Blacksmithing => 59,
			Affinity::BladesSmithing => 54,
			Affinity::Body => 1,
			Affinity::BodyControl => 3,
			Affinity::BodyStamina => 4,
			Affinity::BodyStrength => 5,
			Affinity::Botanizing => 122,
			Affinity::Bowyery => 103,
			Affinity::Butchering => 77,
			Affinity::ButcheringKnife => 73,
			Affinity::Carpentry => 78,
			Affinity::CarvingKnife => 72,
			Affinity::Catapults => 128,
			Affinity::ChainArmourSmithing => 56,
			Affinity::Channeling => 118,
			Affinity::Climbing => 123,
			Affinity::ClothTailoring => 52,
			Affinity::Clubs => 14,
			Affinity::CoalMaking => 97,
			Affinity::Cooking => 19,
			Affinity::DairyFoodMaking => 60,
			Affinity::DefensiveFighting => 110,
			Affinity::Digging => 44,
			Affinity::Exorcism => 119,
			Affinity::Farming => 33,
			Affinity::Fighting => 28,
			Affinity::FineCarpentry => 102,
			Affinity::Firemaking => 79,
			Affinity::FirstAid => 112,
			Affinity::Fishing => 94,
			Affinity::Fletching => 104,
			Affinity::Foraging => 121,
			Affinity::Forestry => 38,
			Affinity::Gardening => 36,
			Affinity::Halberd => 70,
			Affinity::Hammer => 91,
			Affinity::Hammers => 15,
			Affinity::Hatchet => 50,
			Affinity::Healing => 29,
			Affinity::HotFoodCooking => 61,
			Affinity::HugeAxe => 88,
			Affinity::HugeClub => 75,
			Affinity::JewelrySmithing => 101,
			Affinity::Knives => 12,
			Affinity::LargeAxe => 87,
			Affinity::LargeMaul => 65,
			Affinity::LargeMetalShield => 85,
			Affinity::LargeWoodenShield => 83,
			Affinity::Leatherworking => 51,
			Affinity::LockPicking => 125,
			Affinity::Locksmithing => 95,
			Affinity::LongBow => 132,
			Affinity::LongSpear => 69,
			Affinity::Longsword => 64,
			Affinity::Masonry => 53,
			Affinity::Mauls => 13,
			Affinity::Meditating => 37,
			Affinity::ReflexBow => 133,
			Affinity::MediumMaul => 66,
			Affinity::MediumMetalShield => 86,
			Affinity::MediumWoodenShield => 82,
			Affinity::Metallurgy => 99,
			Affinity::Milking => 115,
			Affinity::Milling => 98,
			Affinity::Mind => 0,
			Affinity::MindLogic => 6,
			Affinity::MindSpeed => 7,
			Affinity::Mining => 43,
			Affinity::MiscItems => 23,
			Affinity::NaturalSubstances => 100,
			Affinity::Nature => 26,
			Affinity::NormalFighting => 111,
			Affinity::Papyrusmaking => 34,
			Affinity::Paving => 92,
			Affinity::Pickaxe => 45,
			Affinity::PlateArmourSmithing => 57,
			Affinity::Polearms => 17,
			Affinity::Pottery => 47,
			Affinity::Praying => 117,
			Affinity::Preaching => 116,
			Affinity::Prospecting => 93,
			Affinity::Puppeteering => 106,
			Affinity::Rake => 39,
			Affinity::Religion => 30,
			Affinity::Repairing => 96,
			Affinity::Restoration => 137,
			Affinity::Ropemaking => 48,
			Affinity::Saw => 76,
			Affinity::Scythe => 40,
			Affinity::ShieldBashing => 114,
			Affinity::ShieldSmithing => 58,
			Affinity::Shields => 24,
			Affinity::ShipBuilding => 134,
			Affinity::ShortBow => 131,
			Affinity::Shortsword => 89,
			Affinity::Shovel => 46,
			Affinity::Sickle => 41,
			Affinity::SmallAxe => 42,
			Affinity::SmallMaul => 67,
			Affinity::SmallMetalShield => 84,
			Affinity::SmallWoodenShield => 81,
			Affinity::Smithing => 20,
			Affinity::Soul => 2,
			Affinity::SoulDepth => 8,
			Affinity::SoulStrength => 9,
			Affinity::Staff => 71,
			Affinity::Stealing => 126,
			Affinity::StoneChisel => 74,
			Affinity::StoneCutting => 124,
			Affinity::Swords => 10,
			Affinity::Tailoring => 18,
			Affinity::Taunting => 113,
			Affinity::Thatching => 35,
			Affinity::Thievery => 31,
			Affinity::ToyMaking => 107,
			Affinity::Toys => 27,
			Affinity::Tracking => 80,
			Affinity::Traps => 127,
			Affinity::Trebuchets => 136,
			Affinity::TwoHandedSword => 90,
			Affinity::WarMachines => 32,
			Affinity::Warhammer => 68,
			Affinity::WeaponHeadsSmithing => 55,
			Affinity::WeaponSmithing => 21,
			Affinity::WeaponlessFighting => 108,
			Affinity::Woodcutting => 49,
			Affinity::Yoyo => 105,
		}
	}
}

impl PartialOrd for Recipe {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(
			self.unique_vegs
				.len()
				.cmp(&other.unique_vegs.len())
				.then_with(|| self.filler_sugars.cmp(&other.filler_sugars).reverse()),
		)
	}
}

impl Recipe {
	pub fn affinity(&self) -> usize {
		let sum: usize = self.unique_vegs.iter().map(|(v, p)| v.value() + p.value()).sum();
		let cereal_sum: usize = self.cereals.iter().map(Cereal::value).sum();
		sum + cereal_sum + self.filler_sugars * 47
	}
}

impl Default for Recipe {
	fn default() -> Self {
		Recipe {
			unique_vegs: Default::default(),
			filler_sugars: 1,
			cereals: Default::default(),
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
			Veg::Garlic => 92,
			Veg::Lettuce => 45,
			Veg::Onion => 91,
			Veg::Pea => 58,
			Veg::PeaPod => 46,
			Veg::Potato => 47,
			Veg::Pumpkin => 45,
			Veg::Tomato => 43,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Veg {
	#[default]
	Cabbage,
	Carrot,
	Corn,
	Cucumber,
	Lettuce,
	Garlic,
	Onion,
	Pea,
	PeaPod,
	Potato,
	Pumpkin,
	Tomato,
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

impl Cereal {
	fn value(&self) -> usize {
		match self {
			Cereal::Barley => 23,
			Cereal::Oat => 25,
			Cereal::Rye => 23,
			Cereal::Wheat => 25,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Processing {
	#[default]
	Whole,
	Chopped,
	Fried,
	Mashed,
	Roasted,
}

#[derive(Parser)]
pub struct Options {
	#[arg(long)]
	pub rare: bool,
	pub player_number: usize,
	pub affinity: Affinity,
	#[arg(long, default_value = "12")]
	pub max_vegetables: usize,
	#[arg(long, default_value = "50")]
	pub max_sugars: usize,
	#[arg(long)]
	pub full_cereals: bool,
}

type NodeType = Recipe;
type EdgeType = usize;
type RecipeGraph = petgraph::graph::Graph<NodeType, EdgeType>;

pub fn find_recipe(options: &Options) -> Option<Recipe> {
	// let pregraph = std::time::Instant::now();
	tracing::debug!("Populating graph...");
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
		if current_node.unique_vegs.len() < options.max_vegetables {
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
			// add sugar chain
			let mut current_node_idx = idx;
			for add in (2..options.max_sugars + 1).rev() {
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
				if graph.find_edge(current_node_idx, next_idx).is_none() {
					graph.add_edge(current_node_idx, next_idx, next_value);
				}
				current_node_idx = next_idx;
			}
		}
	};

	let mut first_node: Recipe = Default::default();

	if options.full_cereals {
		first_node.cereals = vec![Cereal::Barley, Cereal::Oat, Cereal::Rye, Cereal::Wheat];
	} else {
		first_node.cereals = vec![Cereal::Wheat];
	}

	let w_idx = graph.add_node(first_node);

	node_queue.push(w_idx);
	// node_queue.push(b_idx);

	// populate the graph
	while let Some(next_idx) = node_queue.pop() {
		add_node(next_idx, &mut graph, &mut node_queue);
		// eprintln!("Graph: {:?}", graph.edge_count());
		// eprintln!("Queue: {:?}", node_queue.len());
		// eprintln!(".");
	}

	// let prefind = std::time::Instant::now();
	// let tm = prefind.duration_since(pregraph).as_millis();
	// tracing::debug!("Done in {tm}ms. Finding a recipe...");
	// find matching recipes

	let target_value = (138 - options.player_number - 3 + options.affinity.offset()) % 138;
	let mut offset = 0;
	offset += 40; // oven
	offset += 75; // cauldron
	offset += 6; // water
	if options.rare {
		offset += 1;
	}

	let mut dfs = petgraph::visit::Dfs::new(&graph, w_idx);

	let mut best_recipe = None;
	while let Some(next) = dfs.next(&graph) {
		let next_node = &graph[next];
		let value = (next_node.affinity() + offset) % 138;

		if target_value == value && best_recipe.as_ref().is_none_or(|r| r < next_node) {
			best_recipe = Some(next_node.clone());
			// eprintln!("Len: {} {:?}", graph[next].unique_vegs.len(), graph[next]);
		}
	}

	// let postfind = std::time::Instant::now();
	// let tm = postfind.duration_since(prefind).as_millis();
	// tracing::debug!("Done in {tm}ms.");
	best_recipe
}
