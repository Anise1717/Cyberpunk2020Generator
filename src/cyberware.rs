use crate::weapons::Damage;

struct Cyberware {
    name: String,
    surg: SurgeryCode,
    slot: CyberSlot,
    description: String,
    cost: isize,
    humanity_loss: isize,
    size: isize,
}
struct BaseCyberware {
    name: String,
    surg: SurgeryCode,
    slot: CyberSlot,
    cost: isize,
    humanity_loss: Damage,
}
enum CyberSlot {
    Fashionware,
    Neuralware,
    Implant,
    Bioware,
    Hands,
    Mouth,
    Eyes,
    Audio,
    Arms,
    Leg,
    Foot,
    BuiltIn,
    LinearFrame,
    BodyPlating,
}
enum SurgeryCode {
    N,
    M,
    MA,
    CR,
}
