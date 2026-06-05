pub enum Message {
    SetSkill(isize, StatEnum),
    SetStat(isize, StatEnum),
}
pub enum StatEnum {
    Body,
    Luck,
    Intelligence,
    Movement,
    Reflex,
    Tech,
    Cool,
    Attractiveness,
}
