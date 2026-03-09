use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
	let filter = tracing_subscriber::filter::EnvFilter::builder()
		.with_default_directive(tracing_subscriber::filter::LevelFilter::WARN.into())
		.with_env_var("LOG_LEVEL")
		.from_env()
		.unwrap();
	tracing_subscriber::fmt().with_writer(std::io::stderr).with_env_filter(filter).try_init().unwrap();
	let options = moonlighter::Options::parse();
	let best_recipe = moonlighter::find_recipe(&options);
	match best_recipe {
		Some(mut recipe) => {
			tracing::debug!("Recipe: {:?}", recipe);
			println!("Best recipe found with {} vegetables!", recipe.vegs.len());
			println!("[ ] water");
			println!("[ ] {} sugars", recipe.sugars);
			println!("[ ] {} barleys", recipe.barleys);
			for cereal in &recipe.cereals {
				println!("[ ] {:?}", cereal)
			}
			for veg in recipe.vegs {
				while recipe.processings[0].1 == 0 {
					recipe.processings.remove(0);
				}
				let processing = match recipe.processings[0] {
					(processing, 1) => {
						recipe.processings.remove(0);
						processing
					}
					(processing, _) => {
						recipe.processings[0].1 -= 1;
						processing
					}
				};
				println!("[ ] {:?} {:?}", veg, processing)
			}
		}
		None => {
			println!("No recipe found")
		}
	}
	Ok(())
}
