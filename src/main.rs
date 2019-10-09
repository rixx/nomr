use chrono::prelude::Local;
use chrono::DateTime;
use structopt::StructOpt;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::iter::Map;
use std::process::Command;

#[derive(StructOpt, Debug)]
struct NomCommand {
    #[structopt(subcommand)]
    cmd: Option<NomArgs>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "nomr", about = "nom nom nom")]
enum NomArgs {
    #[structopt(
        name = "weight",
        visible_alias = "w",
        about = "Report a weight measurement",
        display_order = 3
    )]
    Weight { weight: f32 }, // TODO

    #[structopt(
        name = "search",
        visible_alias = "s",
        about = "Search for a food item on the web",
        display_order = 6
    )]
    Search { food: String }, // TODO

    #[structopt(
        name = "grep",
        visible_alias = "g",
        about = "Search in previous entries of your food log",
        display_order = 5
    )]
    Grep { food: String }, // TODO

    #[structopt(
        name = "log",
        visible_alias = "l",
        about = "Display the full food log",
        display_order = 1
    )]
    Log { food: String, number: f32 }, // TODO

    #[structopt(
        name = "yesterday",
        visible_alias = "y",
        about = "Log food for yesterday",
        display_order = 2
    )]
    Yesterday { food: String, number: f32 }, // TODO

    #[structopt(
        name = "plot",
        visible_alias = "p",
        about = "Show a weight/intake graph",
        display_order = 4
    )]
    Plot {}, // TODO

    #[structopt(name = "edit", visible_alias = "e", about = "Edit the food log")]
    Edit {},

    #[structopt(
        name = "editweight",
        visible_alias = "ew",
        about = "Edit the weight log"
    )]
    EditWeight {},

    #[structopt(name = "config", visible_alias = "c", about = "Edit the config")]
    Config {},
}

struct FoodEntry {
    name: String,
    calories: f32,
}

fn load_weights() -> Option<BTreeMap<String, f32>> {
    let weight_lines =
        fs::read_to_string(env::var("HOME").unwrap() + "/.nom/weight").expect("OOPS");
    let weights: BTreeMap<String, f32> = weight_lines
        .split('\n')
        .map(|kv| kv.split(' ').collect::<Vec<&str>>())
        .filter(|vec| vec.len() == 2)
        .map(|vec| (vec[0].to_string(), vec[1].to_string().parse().unwrap()))
        .collect();
    return Some(weights);
}

fn main() {
    let args = NomCommand::from_args();
    match args.cmd {
        None => println!("None match"),
        Some(ref cmd) => match cmd {
            NomArgs::Weight { weight } => {
                let local: DateTime<Local> = Local::now();
                println!("Plotting hard: {:.2} {}", weight, local.format("%Y-%m-%d"));
            }
            NomArgs::Search { food } => {
                println!("Plotting hard: {}", food);
            }
            NomArgs::Grep { food } => {
                println!("Plotting hard: {}", food);
            }
            NomArgs::Log { food, number } => {
                println!("Plotting hard: {} {}", food, number);
            }
            NomArgs::Yesterday { food, number } => {
                println!("Plotting hard: {} {}", food, number);
            }
            NomArgs::Plot {} => {
                println!("Plotting hard");
                match load_weights() {
                    None => println!("Bad!"),
                    Some(weights) => println!("{:#?}", weights),
                }
            }
            NomArgs::Edit {} => {
                Command::new(env::var("EDITOR").unwrap())
                    .arg(env::var("HOME").unwrap() + "/.nom/input")
                    .status()
                    .expect("File not accessible.");
            }
            NomArgs::EditWeight {} => {
                Command::new(env::var("EDITOR").unwrap())
                    .arg(env::var("HOME").unwrap() + "/.nom/weight")
                    .status()
                    .expect("File not accessible.");
            }
            NomArgs::Config {} => {
                Command::new(env::var("EDITOR").unwrap())
                    .arg(env::var("HOME").unwrap() + "/.nom/config")
                    .status()
                    .expect("File not accessible.");
            }
        },
    }
}
