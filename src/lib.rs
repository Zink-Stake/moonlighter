use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, btree_map};

#[derive(Hash, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
	pub vegs: Vec<Veg>,
	pub processings: Vec<(Processing, u64)>,
	pub cereals: Vec<Cereal>,
	pub sugars: u64,
	pub barleys: u64,
}

pub trait HasAffinity {
	fn affinity(&self) -> u64;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Cereal {
	#[default]
	Barley,
	Oat,
	Rye,
	Wheat,
}

pub const TOTAL_AFFINITIES: u64 = 138;

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
	pub fn offset(&self) -> u64 {
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
		Some(self.vegs.len().cmp(&other.vegs.len()).then_with(|| self.sugars.cmp(&other.sugars).reverse()))
	}
}

impl HasAffinity for Recipe {
	fn affinity(&self) -> u64 {
		let sum: u64 = self.vegs.iter().map(HasAffinity::affinity).sum();
		let cereal_sum: u64 = self.cereals.iter().map(HasAffinity::affinity).sum();
		let processing_sum: u64 = self.processings.iter().map(|(v, c)| v.affinity() * c).sum();
		let sugars_sum = self.sugars * 47;
		let barleys_sum = self.barleys * 23;
		tracing::debug!("Recipe affinity: vegs {sum} + cereals {cereal_sum} + processing {processing_sum} + sugars {sugars_sum} + barleys {barleys_sum}");
		sum + cereal_sum + processing_sum + sugars_sum + barleys_sum
	}
}

impl Default for Recipe {
	fn default() -> Self {
		Recipe {
			vegs: Default::default(),
			sugars: 1,
			barleys: 1,
			cereals: Default::default(),
			processings: Default::default(),
		}
	}
}

impl HasAffinity for Veg {
	fn affinity(&self) -> u64 {
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

impl HasAffinity for Processing {
	fn affinity(&self) -> u64 {
		match self {
			Processing::Whole => 0,
			Processing::Chopped => 16,
			Processing::Fried => 1,
			Processing::Mashed => 32,
			Processing::Roasted => 4,
		}
	}
}

impl HasAffinity for Cereal {
	fn affinity(&self) -> u64 {
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
	#[arg(long, default_value = "0")]
	pub custom_offset: u64,
	pub player_number: u64,
	pub affinity: Affinity,
	#[arg(long, default_value = "12")]
	pub vegs: u64,
	#[arg(long, default_value = "70")]
	pub max_fillers: u64,
	#[arg(long)]
	pub full_cereals: bool,
	#[arg(long)]
	pub complex_processing: bool,
}

#[derive(PartialEq)]
struct Adjustment {
	processings: Vec<(Processing, u64)>,
	sugars: u64,
	barleys: u64,
}

impl HasAffinity for Adjustment {
	fn affinity(&self) -> u64 {
		let processing_sum: u64 = self.processings.iter().map(|(p, c)| p.affinity() * c).sum();
		(processing_sum + self.sugars * 47 + self.barleys * 23) % TOTAL_AFFINITIES
	}
}

// Compare labor cost. first check sugars, then if the first processing is dominating. NB! fails for zero processing.
impl PartialOrd for Adjustment {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(
			(self.sugars + self.barleys)
				.cmp(&(other.sugars + other.barleys))
				.then_with(|| self.processings[0].1.cmp(&other.processings[0].1).reverse()),
		)
	}
}

pub fn find_recipe(options: &Options) -> Option<Recipe> {
	let veg_pool: Vec<Veg> = vec![
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
	];

	let (processing1, processing2) = if options.complex_processing {
		(Processing::Fried, Processing::Roasted)
	} else {
		(Processing::Chopped, Processing::Mashed)
	};

	// Assemble a set of shortest adjustments
	let mut adjustments: BTreeMap<u64, Adjustment> = Default::default();

	for processings1 in 0..options.vegs.strict_add(1) {
		let processings2 = options.vegs.strict_sub(processings1);
		for sugars in 0..options.max_fillers.strict_add(1) {
			for barleys in 0..options.max_fillers.strict_add(1).strict_sub(sugars) {
				let adjustment = Adjustment {
					processings: vec![(processing1, processings1), (processing2, processings2)],
					sugars,
					barleys,
				};
				let current_adjustment = adjustments.entry(adjustment.affinity());
				match current_adjustment {
					btree_map::Entry::Vacant(v) => {
						v.insert(adjustment);
					}
					btree_map::Entry::Occupied(mut o) => {
						if o.get() > &adjustment {
							o.insert(adjustment);
						}
					}
				}
			}
		}
	}

	if adjustments.len() != TOTAL_AFFINITIES as usize {
		tracing::warn!("Not all adjustments could be found, {}/{}", adjustments.len(), TOTAL_AFFINITIES);
	}

	let mut recipe = Recipe {
		vegs: veg_pool.into_iter().take(options.vegs as usize).collect(),
		..Default::default()
	};

	if options.full_cereals {
		recipe.cereals = vec![Cereal::Oat, Cereal::Rye, Cereal::Wheat];
	} else {
		recipe.cereals = vec![];
	}

	let target_value = ((TOTAL_AFFINITIES * 3).strict_sub(options.player_number).strict_sub(3).strict_add(options.affinity.offset())) % TOTAL_AFFINITIES;
	let mut offset = 0;
	offset += 40; // oven
	offset += 75; // cauldron
	offset += 6; // water
	offset += options.custom_offset;
	let value = (recipe.affinity() + offset) % TOTAL_AFFINITIES;
	tracing::debug!("Value of the base recipe: {}", value);
	tracing::debug!("Needed value: {}", target_value);
	let adjustment_needed = (target_value + TOTAL_AFFINITIES.strict_sub(value)) % TOTAL_AFFINITIES;

	tracing::debug!("Needed {} adjustment", adjustment_needed);
	match adjustments.get(&adjustment_needed) {
		Some(adjustment) => {
			recipe.sugars += adjustment.sugars;
			recipe.barleys += adjustment.barleys;
			recipe.processings = adjustment.processings.clone();
		}
		None => {
			return None;
		}
	}

	Some(recipe)
}
