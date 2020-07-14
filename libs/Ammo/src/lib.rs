#[cfg(test)]
mod tests {
    #[test]
    fn check_universal_ammo_case() {
        let sut1 = Sniper_Scav_366();
        assert_eq!(sut1.get_budget_ammo(), sut1.get_rare_ammo());
        assert_eq!(sut1.get_budget_ammo(), sut1.get_the_best_ammo()());
    }
}

pub trait Ammunition {
    fn get_budget_ammo(&self) -> &str {
        ""
    }

    fn get_the_best_ammo(&self) -> &str {
        self.get_budget_ammo()
    }

    fn get_rare_ammo(&self) -> &str {
        self.get_the_best_ammo()
    }
}

pub struct Shotgun_12x70;

pub struct Machinegun_4_6x30;

pub struct Sniper_Scav_366;

pub struct AK_5_45x39;
pub struct AK_5_56x45;

pub struct Pistol_9x18;
pub struct Pistol_9x19;
pub struct Pistol_9x21;
pub struct Pistol_7_62x25mm;

pub struct SKS_7_62x39;

pub struct Sniper_9x39;
pub struct Sniper_7_62x51;
pub struct Sniper_7_62x54R;

pub struct Machinegun_12_7x55;

pub struct Machinegun_5_7x28mm;

pub struct NotFound;

impl Ammunition for NotFound {}

impl Ammunition for Shotgun_12x70 {
    fn get_budget_ammo(&self) -> &str {
        "12/70 8.5 mm Magnum Buckshot"
    }

    fn get_the_best_ammo(&self) -> &str {
        "12/70 AP-20 Slug"
    }

    fn get_rare_ammo(&self) -> &str {
        "12x70 RIP"
    }
}

impl Ammunition for Pistol_7_62x25mm {
    fn get_budget_ammo(&self) -> &str {
        "7.62x25mm TT Pst gzh"
    }
}

impl Ammunition for SKS_7_62x39 {
    fn get_budget_ammo(&self) -> &str {
        "7.62x39 mm PS"
    }

    fn get_the_best_ammo(&self) -> &str {
        "7.62x39 mm BP"
    }
}

impl Ammunition for Machinegun_4_6x30 {
    fn get_budget_ammo(&self) -> &str {
        "4.6x30mm Subsonic SX"
    }

    fn get_the_best_ammo(&self) -> &str {
        "4.6x30mm AP SX"
    }
}

impl Ammunition for Sniper_Scav_366 {
    fn get_budget_ammo(&self) -> &str {
        ".366 TKM EKO"
    }
}

impl Ammunition for AK_5_45x39 {
    fn get_budget_ammo(&self) -> &str {
        "5.45x39 mm BT"
    }

    fn get_the_best_ammo(&self) -> &str {
        "5.45x39 mm BS"
    }

    fn get_rare_ammo(&self) -> &str {
        "5.45x39 mm Igolnik"
    }
}

impl Ammunition for AK_5_56x45 {
    fn get_budget_ammo(&self) -> &str {
        "5.56x45 mm M856A1"
    }

    fn get_the_best_ammo(&self) -> &str {
        "5.56x45 mm M995"
    }
}

impl Ammunition for Sniper_7_62x51 {
    fn get_budget_ammo(&self) -> &str {
        "7.62x51 mm M80"
    }
}

impl Ammunition for Sniper_7_62x54R {
    fn get_budget_ammo(&self) -> &str {
        "7.62x54R LPS Gzh"
    }

    fn get_the_best_ammo(&self) -> &str {
        "7.62x54R 7N1 Sniper cartridge"
    }
}

impl Ammunition for Pistol_9x18 {
    fn get_budget_ammo(&self) -> &str {
        "9x18 mm PM SP7 gzh"
    }

    fn get_the_best_ammo(&self) -> &str {
        "9x18 mm PM PBM || 9x18 mm PM PMM"
    }
}

impl Ammunition for Pistol_9x19 {
    fn get_budget_ammo(&self) -> &str {
        "9x19 mm Pst gzh"
    }

    fn get_the_best_ammo(&self) -> &str {
        "9x19 mm AP 6.3"
    }

    fn get_rare_ammo(&self) -> &str {
        "9mm RIP"
    }
}

impl Ammunition for Pistol_9x21 {
    fn get_budget_ammo(&self) -> &str {
        "9x21 mm SP13"
    }
}

impl Ammunition for Sniper_9x39 {
    fn get_budget_ammo(&self) -> &str {
        "9x39 mm SP-6"
    }

    fn get_the_best_ammo(&self) -> &str {
        "9x39 mm 7N12 BP"
    }
}

impl Ammunition for Machinegun_12_7x55 {
    fn get_budget_ammo(&self) -> &str {
        "12.7x55 mm PS12"
    }

    fn get_the_best_ammo(&self) -> &str {
        "12.7x55 mm PS12B"
    }
}
impl Ammunition for Machinegun_5_7x28mm {
    fn get_budget_ammo(&self) -> &str {
        "5.7x28 mm L191"
    }

    fn get_the_best_ammo(&self) -> &str {
        "5.7x28 mm SB193"
    }
}
