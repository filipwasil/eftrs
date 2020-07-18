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
    weapons_factory: Weapons::WeaponFactory
}

impl Application
{
    pub fn new() -> Application
    {
        Application {weapons_factory: Weapons::WeaponFactory::new()}
    }

    pub fn help()
    {
        println!("Wrong number of arguments");
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let argc = arguments.len();
    if argc < 2 {
        Application::help();
        return;
    }

    let app = Application::new();

    match app.weapons_factory.get_weapon(arguments[1].to_string()) {
        Ok(weapon) => println!("{}", weapon),
        Err(err) => println!("{}", err),
    }
}
