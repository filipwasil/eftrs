#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate Ammo;

use std::collections::HashMap;

pub struct Weapon<'a> {
    name: String,
    barrel: String,
    charge: String,
    gas: String,
    handguard: String,
    magazine: String,
    mount: String,
    muzzle: String,
    optic: String,
    grip_pistol: String,
    grip_tactical: String,
    receiver: String,
    sight: String,
    stick: String,
    mod_tactical: String,
    pub ammo: &'a std::boxed::Box<dyn Ammo::Ammunition>,
}

pub struct AmmunitionManager {
    ammos: HashMap<String, std::boxed::Box<dyn Ammo::Ammunition>>,
}

impl AmmunitionManager {
    pub fn new() -> AmmunitionManager {
        let mut ammos: HashMap<String, std::boxed::Box<dyn Ammo::Ammunition>> = HashMap::new();
        ammos.insert(
            String::from("M870"),
            std::boxed::Box::new(Ammo::Shotgun_12x70),
        );
        ammos.insert(
            String::from("MR133"),
            std::boxed::Box::new(Ammo::Shotgun_12x70),
        );
        ammos.insert(
            String::from("MP153"),
            std::boxed::Box::new(Ammo::Shotgun_12x70),
        );
        ammos.insert(
            String::from("Saiga12"),
            std::boxed::Box::new(Ammo::Shotgun_12x70),
        );
        ammos.insert(String::from("TOZ106"), std::boxed::Box::new(Ammo::NotFound));
        ammos.insert(
            String::from("MP7A1"),
            std::boxed::Box::new(Ammo::Machinegun_4_6x30),
        );
        ammos.insert(
            String::from("MP7A2"),
            std::boxed::Box::new(Ammo::Machinegun_4_6x30),
        );
        ammos.insert(
            String::from("VeprAKMVPO209"),
            std::boxed::Box::new(Ammo::Sniper_Scav_366),
        );
        ammos.insert(
            String::from("VPO215"),
            std::boxed::Box::new(Ammo::Sniper_Scav_366),
        );
        ammos.insert(
            String::from("AK74N"),
            std::boxed::Box::new(Ammo::AK_5_45x39),
        );
        ammos.insert(
            String::from("AKS74N"),
            std::boxed::Box::new(Ammo::AK_5_45x39),
        );
        ammos.insert(
            String::from("AK74M"),
            std::boxed::Box::new(Ammo::AK_5_45x39),
        );
        ammos.insert(
            String::from("AKS74UNB"),
            std::boxed::Box::new(Ammo::AK_5_45x39),
        );
        ammos.insert(
            String::from("AK105"),
            std::boxed::Box::new(Ammo::AK_5_45x39),
        );
        ammos.insert(
            String::from("RPK16"),
            std::boxed::Box::new(Ammo::AK_5_45x39),
        );
        ammos.insert(
            String::from("AK101"),
            std::boxed::Box::new(Ammo::AK_5_56x45),
        );
        ammos.insert(
            String::from("AK102"),
            std::boxed::Box::new(Ammo::AK_5_56x45),
        );
        ammos.insert(
            String::from("HK416A5"),
            std::boxed::Box::new(Ammo::AK_5_56x45),
        );
        ammos.insert(String::from("MDR"), std::boxed::Box::new(Ammo::AK_5_56x45));
        ammos.insert(String::from("M4A1"), std::boxed::Box::new(Ammo::AK_5_56x45));
        ammos.insert(
            String::from("ADAR215"),
            std::boxed::Box::new(Ammo::AK_5_56x45),
        );
        ammos.insert(
            String::from("LoneStarTX15"),
            std::boxed::Box::new(Ammo::AK_5_56x45),
        );
        ammos.insert(
            String::from("TTpistol"),
            std::boxed::Box::new(Ammo::Pistol_7_62x25mm),
        );
        ammos.insert(
            String::from("AK103"),
            std::boxed::Box::new(Ammo::SKS_7_62x39),
        );
        ammos.insert(
            String::from("AK104"),
            std::boxed::Box::new(Ammo::SKS_7_62x39),
        );
        ammos.insert(
            String::from("AKMN"),
            std::boxed::Box::new(Ammo::SKS_7_62x39),
        );
        ammos.insert(
            String::from("AKMSN"),
            std::boxed::Box::new(Ammo::SKS_7_62x39),
        );
        ammos.insert(
            String::from("VeprKMVPO136"),
            std::boxed::Box::new(Ammo::SKS_7_62x39),
        );
        ammos.insert(String::from("SKS"), std::boxed::Box::new(Ammo::SKS_7_62x39));
        ammos.insert(
            String::from("OPSKS"),
            std::boxed::Box::new(Ammo::SKS_7_62x39),
        );
        ammos.insert(
            String::from("SA58"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("M1A"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("RSASS"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("DVL10"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("M700"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("MDR762x51"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("SR25"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("T5000"),
            std::boxed::Box::new(Ammo::Sniper_7_62x51),
        );
        ammos.insert(
            String::from("SVDS"),
            std::boxed::Box::new(Ammo::Sniper_7_62x54R),
        );
        ammos.insert(
            String::from("Mosin"),
            std::boxed::Box::new(Ammo::Sniper_7_62x54R),
        );
        ammos.insert(
            String::from("SV98"),
            std::boxed::Box::new(Ammo::Sniper_7_62x54R),
        );
        ammos.insert(
            String::from("PP9Klin"),
            std::boxed::Box::new(Ammo::Pistol_9x18),
        );
        ammos.insert(
            String::from("PP91Kedr"),
            std::boxed::Box::new(Ammo::Pistol_9x18),
        );
        ammos.insert(
            String::from("PP9101KedrB"),
            std::boxed::Box::new(Ammo::Pistol_9x18),
        );
        ammos.insert(String::from("PB"), std::boxed::Box::new(Ammo::Pistol_9x18));
        ammos.insert(String::from("APS"), std::boxed::Box::new(Ammo::Pistol_9x18));
        ammos.insert(
            String::from("PMpistol"),
            std::boxed::Box::new(Ammo::Pistol_9x18),
        );
        ammos.insert(
            String::from("PMtpistol"),
            std::boxed::Box::new(Ammo::Pistol_9x18),
        );
        ammos.insert(String::from("MP5"), std::boxed::Box::new(Ammo::Pistol_9x19));
        ammos.insert(
            String::from("MP5Kurz"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(String::from("MPX"), std::boxed::Box::new(Ammo::Pistol_9x19));
        ammos.insert(
            String::from("VityazSN"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("Saiga9"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("GLOCK17"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("GLOCK18C"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("MP443Grach"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("P226R"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("M9A3"),
            std::boxed::Box::new(Ammo::Pistol_9x19),
        );
        ammos.insert(
            String::from("MP9"),
            std::boxed::Box::new(Ammo::Shotgun_12x70),
        );
        ammos.insert(
            String::from("SR1MPGyurza"),
            std::boxed::Box::new(Ammo::Pistol_9x21),
        );
        ammos.insert(
            String::from("ASVA"),
            std::boxed::Box::new(Ammo::Sniper_9x39),
        );
        ammos.insert(
            String::from("VSSVintorez"),
            std::boxed::Box::new(Ammo::Sniper_9x39),
        );
        ammos.insert(
            String::from("Ash12"),
            std::boxed::Box::new(Ammo::Machinegun_12_7x55),
        );
        ammos.insert(
            String::from("P90"),
            std::boxed::Box::new(Ammo::Machinegun_5_7x28mm),
        );
        ammos.insert(
            String::from("FN57"),
            std::boxed::Box::new(Ammo::Machinegun_5_7x28mm),
        );
        AmmunitionManager { ammos }
    }

    // pub fn get_ammunition_impl(&self, weapon_name: &str) -> Option<&std::boxed::Box<dyn Ammo::Ammunition>> {
    //     match self.ammos.get(weapon_name) {
    //         Some(ammunition) => ammunition,
    //         None => {}
    //     }
    // }

    pub fn get_ammunition(&self, weapon_name: &str) -> std::result::Result<&std::boxed::Box<dyn Ammo::Ammunition>, &str> {
        let result = self.ammos.get(weapon_name);
        if let Some(ammunition) = self.ammos.get(weapon_name) {
            return Ok(ammunition);
        }

        println!("\nWeapon not found. Trying to find one that may fit...\n");
        if let Ok(ammunition) = self.suggest_closest_match(String::from(weapon_name)) {
            if let Some(result) = self.ammos.get(ammunition.as_str()) {
                return Ok(result);
            }
        }

        return  Err("Weapon not found");
    }

    fn suggest_closest_match(&self, weapon_name: String) -> Result <String, &str> {
        for w in &self.ammos
        {
            if let Some(matching_pattern) = w.0.find(&weapon_name) {
                return Ok(w.0.clone());
            }
        }
        Err("weapon not found")
    }
}

impl<'a> Weapon<'a> {
    pub fn new(weapon_name: &str, ammo: &'a std::boxed::Box<dyn Ammo::Ammunition>) -> Weapon<'a> {
        println!("WEAPON: {}", weapon_name);
        Weapon {
            name: String::from(weapon_name),
            barrel: "".to_string(),
            charge: "".to_string(),
            gas: "".to_string(),
            handguard: "".to_string(),
            magazine: "".to_string(),
            mount: "".to_string(),
            muzzle: "".to_string(),
            optic: "".to_string(),
            grip_pistol: "".to_string(),
            grip_tactical: "".to_string(),
            receiver: "".to_string(),
            sight: "".to_string(),
            stick: "".to_string(),
            mod_tactical: "".to_string(),
            ammo: ammo
        }
    }
}

impl<'a> std::fmt::Display for Weapon<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "BUDGET: {:?}\nTHE BEST: {:?}\nSPECIAL: {:?}\n",
            self.ammo.get_budget_ammo(),
            self.ammo.get_the_best_ammo(),
            self.ammo.get_rare_ammo()
        )
    }
}
