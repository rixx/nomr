use structopt::StructOpt;

use std::env;
use std::process::Command;

#[derive(StructOpt, Debug)]
#[structopt(name = "nomp", about = "nom nom nom")]
enum NomArgs {
    #[structopt(name = "weight", alias = "w")]
    Weight {
        weight: f32,
    },

    #[structopt(name = "search", alias = "s")]
    Search {
        food: String,
    },

    #[structopt(name = "grep", alias = "g")]
    Grep {
        food: String,
    },

    #[structopt(name = "log", alias = "l")]
    Log {
        food: String,
        number: f32,
    },

    #[structopt(name = "yesterday", alias = "y")]
    Yesterday {
        food: String,
        number: f32,
    },

    #[structopt(name = "plot", alias = "p")]
    Plot {},

    #[structopt(name = "edit", alias = "e")]
    Edit {},

    #[structopt(name = "editweight", alias = "ew")]
    EditWeight {},

    #[structopt(name = "config", alias = "c")]
    Config {},
}

fn main() {
    let args = NomArgs::from_args();
    match args {
        NomArgs::Weight{weight} => {
            println!("Plotting hard: {}", weight);
        },
        NomArgs::Search{food} => {
            println!("Plotting hard: {}", food);
        },
        NomArgs::Grep{food} => {
            println!("Plotting hard: {}", food);
        },
        NomArgs::Log{food, number} => {
            println!("Plotting hard: {} {}", food, number);
        },
        NomArgs::Yesterday{food, number} => {
            println!("Plotting hard: {} {}", food, number);
        },
        NomArgs::Plot{} => {
            println!("Plotting hard");
        },
        NomArgs::Edit{} => {
            Command::new(env::var("EDITOR").unwrap())
                .arg(env::var("HOME").unwrap() + "/.nom/input")
                .status()
                .expect("File not accessible.");
        },
        NomArgs::EditWeight{} => {
            Command::new(env::var("EDITOR").unwrap())
                .arg(env::var("HOME").unwrap() + "/.nom/weight")
                .status()
                .expect("File not accessible.");
        },
        NomArgs::Config{} => {
            Command::new(env::var("EDITOR").unwrap())
                .arg(env::var("HOME").unwrap() + "/.nom/config")
                .status()
                .expect("File not accessible.");
        },
    }
}
