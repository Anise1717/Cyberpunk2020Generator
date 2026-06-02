mod armor {
    enum ArmorAreas {
        Head,
        Torso,
        Legs,
        Arms,
    }
    struct BodyArmor {
        name: String,
        area: Vec<ArmorAreas>,
        stopping_power: i32,
        enumcumberance_value: i32,
        cost: i32,
    }
}
