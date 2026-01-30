use clap::Parser;

fn main() {
	let options = moonlighter::Options::parse();
	let best_recipe = moonlighter::find_recipe(&options);
	match best_recipe {
		Some(recipe) => {
			println!("Best recipe found with {} vegetables!", recipe.unique_vegs.len());
			println!("[ ] {:?}", recipe.cereal);
			println!("[ ] water");
			println!("[ ] {} sugars", recipe.filler_sugars);
			for (veg, processing) in recipe.unique_vegs {
				println!("[ ] {:?} {:?}", veg, processing)
			}
		}
		None => {
			println!("No recipe found")
		}
	}
}
