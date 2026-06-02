mod weapons {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    enum Dice {
        D6,
        D10,
        Drugs,
        Stun,
    }
    #[derive(Deserialize, Serialize)]
    enum Reliability {
        ST,
        UR,
        VR,
    }
    #[derive(Deserialize, Serialize)]
    struct Damage {
        quantity: isize,
        die: Dice,
        additional_damage: isize,
    }
    #[derive(Serialize, Deserialize)]
    struct Weapon {
        weapons_type: String,
        accuracy: isize,
        concealment: char,
        availability: char,
        damage_dice: Damage,
        ammo: String,
        capacity: isize,
        rate_of_fire: isize,
        reliability: Reliability,
        range: isize,
        cost: isize,
        description: String,
    }
}
