use crate::graphics::messages::StatEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    below_two: bool,
    unused_points: isize,
    intelligence: isize,
    reflex: isize,
    tech: isize,
    cool: isize,
    attractiveness: isize,
    luck: isize,
    movement_allowance: isize,
    body: isize,
    empathy: isize,
}

impl Stats {
    pub fn new(points: isize) -> Stats {
        Stats {
            below_two: false,
            unused_points: points,
            intelligence: 0,
            reflex: 0,
            tech: 0,
            cool: 0,
            attractiveness: 0,
            luck: 0,
            movement_allowance: 0,
            body: 0,
            empathy: 0,
        }
    }

    pub fn get_mut(&mut self, stat: StatEnum) -> &mut isize {
        match stat {
            StatEnum::Intelligence => &mut self.intelligence,
            StatEnum::Reflex => &mut self.reflex,
            StatEnum::Tech => &mut self.tech,
            StatEnum::Cool => &mut self.cool,
            StatEnum::Attractiveness => &mut self.attractiveness,
            StatEnum::Luck => &mut self.luck,
            StatEnum::Movement => &mut self.movement_allowance,
            StatEnum::Body => &mut self.body,
            StatEnum::Empathy => &mut self.empathy,
        }
    }
}
