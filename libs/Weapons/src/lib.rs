#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate Ammo;
extern crate Mods;

use std::collections::HashMap;

pub trait Weapon {
    fn get_name(&self) -> &str {
        "Empty"
    }

    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Shotgun_12x70)
    }

    fn get_silencer(&self) -> Mods::Mod {
        Mods::Mod {
            name: String::from("Empty"),
            prize: 0,
            mount_name: String::from("Empty"),
        }
    }
    fn get_sight(&self) -> Mods::Mod {
        Mods::Mod {
            name: String::from("Empty"),
            prize: 0,
            mount_name: String::from("Empty"),
        }
    }
}

struct M870;
impl Weapon for M870 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Shotgun_12x70)
    }
}

struct MR133;
impl Weapon for MR133 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Shotgun_12x70)
    }
}

struct MP153;
impl Weapon for MP153 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Shotgun_12x70)
    }
}

struct Saiga12;
impl Weapon for Saiga12 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Shotgun_12x70)
    }
}

struct TOZ106;
impl Weapon for TOZ106 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::NotFound)
    }
}

struct MP7A1;
impl Weapon for MP7A1 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Machinegun_4_6x30)
    }
}

struct MP7A2;
impl Weapon for MP7A2 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Machinegun_4_6x30)
    }
}

struct VeprAKMVPO209;
impl Weapon for VeprAKMVPO209 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Sniper_Scav_366)
    }
}

struct VPO215;
impl Weapon for VPO215 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Sniper_Scav_366)
    }
}

struct AK74M;
impl Weapon for AK74M {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_45x39)
    }
}

struct AKS74N;
impl Weapon for AKS74N {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_45x39)
    }
}

struct AK74N;
impl Weapon for AK74N {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_45x39)
    }
}

struct AKS74UNB;
impl Weapon for AKS74UNB {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_45x39)
    }
}

struct AK105;
impl Weapon for AK105 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_45x39)
    }
}

struct RPK16;
impl Weapon for RPK16 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_45x39)
    }
}

struct AK101;
impl Weapon for AK101 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct AK102;
impl Weapon for AK102 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct HK416A5;
impl Weapon for HK416A5 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct MDR;
impl Weapon for MDR {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct M4A1;
impl Weapon for M4A1 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct ADAR215;
impl Weapon for ADAR215 {
    fn get_name(&self) -> &str {
        "ADAR215"
    }

    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct LoneStarTX15;
impl Weapon for LoneStarTX15 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::AK_5_56x45)
    }
}

struct TTpistol;
impl Weapon for TTpistol {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::Pistol_7_62x25mm)
    }
}

struct AK103;
impl Weapon for AK103 {
    fn get_ammo(&self) -> std::boxed::Box<dyn Ammo::Ammunition> {
        std::boxed::Box::new(Ammo::SKS_7_62x39)
    }
}

pub struct WeaponFactory {
    weapons: HashMap<String, std::boxed::Box<dyn Weapon>>,
}

impl WeaponFactory {
    pub fn new() -> WeaponFactory {
        let mut weapons: HashMap<String, std::boxed::Box<dyn Weapon>> = HashMap::new();
        weapons.insert(String::from("M870"), std::boxed::Box::new(M870));
        weapons.insert(String::from("MR133"), std::boxed::Box::new(MR133));
        weapons.insert(String::from("MP153"), std::boxed::Box::new(MP153));
        weapons.insert(String::from("Saiga12"), std::boxed::Box::new(Saiga12));
        weapons.insert(String::from("TOZ106"), std::boxed::Box::new(TOZ106));
        weapons.insert(String::from("MP7A1"), std::boxed::Box::new(MP7A1));
        weapons.insert(String::from("MP7A2"), std::boxed::Box::new(MP7A2));
        weapons.insert(
            String::from("VeprAKMVPO209"),
            std::boxed::Box::new(VeprAKMVPO209),
        );
        weapons.insert(String::from("VPO215"), std::boxed::Box::new(VPO215));
        weapons.insert(String::from("AK74N"), std::boxed::Box::new(AK74N));
        weapons.insert(String::from("AKS74N"), std::boxed::Box::new(AKS74N));
        weapons.insert(String::from("AK74M"), std::boxed::Box::new(AK74M));
        weapons.insert(String::from("AKS74UNB"), std::boxed::Box::new(AKS74UNB));
        weapons.insert(String::from("AK105"), std::boxed::Box::new(AK105));
        weapons.insert(String::from("RPK16"), std::boxed::Box::new(RPK16));
        weapons.insert(String::from("AK101"), std::boxed::Box::new(AK101));
        weapons.insert(String::from("AK102"), std::boxed::Box::new(AK102));
        weapons.insert(String::from("HK416A5"), std::boxed::Box::new(HK416A5));
        weapons.insert(String::from("MDR"), std::boxed::Box::new(MDR));
        weapons.insert(String::from("M4A1"), std::boxed::Box::new(M4A1));
        weapons.insert(String::from("ADAR215"), std::boxed::Box::new(ADAR215));
        weapons.insert(
            String::from("LoneStarTX15"),
            std::boxed::Box::new(LoneStarTX15),
        );

        /// not done
        weapons.insert(String::from("TTpistol"), std::boxed::Box::new(TTpistol));
        weapons.insert(String::from("AK103"), std::boxed::Box::new(AK103));
        // weapons.insert(
        //     String::from("AK104"),
        //     std::boxed::Box::new(AK104),
        // );
        // weapons.insert(
        //     String::from("AKMN"),
        //     std::boxed::Box::new(AKMN),
        // );
        // weapons.insert(
        //     String::from("AKMSN"),
        //     std::boxed::Box::new(AKMSN),
        // );
        // weapons.insert(
        //     String::from("VeprKMVPO136"),
        //     std::boxed::Box::new(VeprKMVPO136),
        // );
        // weapons.insert(String::from("SKS"), std::boxed::Box::new(SKS));
        // weapons.insert(
        //     String::from("OPSKS"),
        //     std::boxed::Box::new(OPSKS),
        // );
        // weapons.insert(
        //     String::from("SA58"),
        //     std::boxed::Box::new(SA58),
        // );
        // weapons.insert(
        //     String::from("M1A"),
        //     std::boxed::Box::new(M1A),
        // );
        // weapons.insert(
        //     String::from("RSASS"),
        //     std::boxed::Box::new(RSASS),
        // );
        // weapons.insert(
        //     String::from("DVL10"),
        //     std::boxed::Box::new(DVL10),
        // );
        // weapons.insert(
        //     String::from("M700"),
        //     std::boxed::Box::new(M700),
        // );
        // weapons.insert(
        //     String::from("MDR762x51"),
        //     std::boxed::Box::new(MDR762x51),
        // );
        // weapons.insert(
        //     String::from("SR25"),
        //     std::boxed::Box::new(SR25),
        // );
        // weapons.insert(
        //     String::from("T5000"),
        //     std::boxed::Box::new(T5000),
        // );
        // weapons.insert(
        //     String::from("SVDS"),
        //     std::boxed::Box::new(SVDS),
        // );
        // weapons.insert(
        //     String::from("Mosin"),
        //     std::boxed::Box::new(Mosin),
        // );
        // weapons.insert(
        //     String::from("SV98"),
        //     std::boxed::Box::new(SV98),
        // );
        // weapons.insert(
        //     String::from("PP9Klin"),
        //     std::boxed::Box::new(PP9Klin),
        // );
        // weapons.insert(
        //     String::from("PP91Kedr"),
        //     std::boxed::Box::new(PP91Kedr),
        // );
        // weapons.insert(
        //     String::from("PP9101KedrB"),
        //     std::boxed::Box::new(PP9101KedrB),
        // );
        // weapons.insert(String::from("PB"), std::boxed::Box::new(PB));
        // weapons.insert(String::from("APS"), std::boxed::Box::new(APS));
        // weapons.insert(
        //     String::from("PMpistol"),
        //     std::boxed::Box::new(PMpistol),
        // );
        // weapons.insert(
        //     String::from("PMtpistol"),
        //     std::boxed::Box::new(PMtpistol),
        // );
        // weapons.insert(String::from("MP5"), std::boxed::Box::new(MP5));
        // weapons.insert(
        //     String::from("MP5Kurz"),
        //     std::boxed::Box::new(MP5Kurz),
        // );
        // weapons.insert(String::from("MPX"), std::boxed::Box::new(MPX));
        // weapons.insert(
        //     String::from("VityazSN"),
        //     std::boxed::Box::new(VityazSN),
        // );
        // weapons.insert(
        //     String::from("Saiga9"),
        //     std::boxed::Box::new(Saiga9),
        // );
        // weapons.insert(
        //     String::from("GLOCK17"),
        //     std::boxed::Box::new(GLOCK17),
        // );
        // weapons.insert(
        //     String::from("GLOCK18C"),
        //     std::boxed::Box::new(GLOCK18C),
        // );
        // weapons.insert(
        //     String::from("MP443Grach"),
        //     std::boxed::Box::new(MP443Grach),
        // );
        // weapons.insert(
        //     String::from("P226R"),
        //     std::boxed::Box::new(P226R),
        // );
        // weapons.insert(
        //     String::from("M9A3"),
        //     std::boxed::Box::new(M9A3),
        // );
        // weapons.insert(
        //     String::from("MP9"),
        //     std::boxed::Box::new(MP9),
        // );
        // weapons.insert(
        //     String::from("SR1MPGyurza"),
        //     std::boxed::Box::new(SR1MPGyurza),
        // );
        // weapons.insert(
        //     String::from("ASVA"),
        //     std::boxed::Box::new(ASVA),
        // );
        // weapons.insert(
        //     String::from("VSSVintorez"),
        //     std::boxed::Box::new(VSSVintorez),
        // );
        // weapons.insert(
        //     String::from("Ash12"),
        //     std::boxed::Box::new(Ash12),
        // );
        // weapons.insert(
        //     String::from("P90"),
        //     std::boxed::Box::new(P90),
        // );
        // weapons.insert(
        //     String::from("FN57"),
        //     std::boxed::Box::new(FN57),
        // );

        WeaponFactory { weapons }
    }

    pub fn get_weapon(
        &self,
        weapon_name: String,
    ) -> std::result::Result<&std::boxed::Box<dyn Weapon>, &str> {
        if let Some(weapon) = self.weapons.get(&weapon_name) {
            return Ok(weapon);
        }

        if let Ok(name) = self.suggest_closest_match(weapon_name) {
            if let Some(weapon) = self.weapons.get(&name) {
                return Ok(weapon);
            }
        }

        return Err("Weapon not found");
    }

    fn suggest_closest_match(&self, weapon_name: String) -> Result<String, &str> {
        for w in &self.weapons {
            if let Some(matching_pattern) = w.0.find(&weapon_name) {
                return Ok(w.0.clone());
            }
        }
        Err("weapon not found")
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ammo = self.get_ammo();
        let silencer = self.get_silencer();
        let sight = self.get_sight();
        write!(
            f,
            "WEAPON: {:?}\nBUDGET: {:?}\nTHE BEST: {:?}\nSPECIAL: {:?}\nSILENCER: {:?} WITH MOUNT: {:?}\nSIGHT: {:?} WITH MOUNT: {:?}\n ",
            self.get_name(),
            ammo.get_budget_ammo(),
            ammo.get_the_best_ammo(),
            ammo.get_rare_ammo(),
            silencer.name,
            silencer.mount_name,
            sight.name,
            sight.mount_name
        )
    }
}
