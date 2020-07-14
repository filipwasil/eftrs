use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate log;

extern crate Ammo;
extern crate Weapons;

struct Application
{
    ammunitionManager: Weapons::AmmunitionManager,
}

impl Application {
    fn new() -> Application {
        Application { ammunitionManager: Weapons::AmmunitionManager::new() }
    }

    fn get_weapon(&self, name: &str) -> Result<Weapons::Weapon, &str> {
        Ok(Weapons::Weapon::new(name, self.ammunitionManager.get_ammunition(name)?))
    }

    fn help() {
        println!("Please pass weapon name:")
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        Application::help();
    }

    let app = Application::new();

    match app.get_weapon(&arguments[1])
    {
        Ok(weapon) => println!("{}", weapon),
        Err(err) => println!("{}", err) 
    }
}
