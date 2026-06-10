use charecter_gen_2020::character::Character;
use charecter_gen_2020::graphics::messages::{self, Message, StatEnum};
use iced::widget::{Column, button, column, row, text, text_input};
use iced::{self, Element};

struct App {
    character: Character,
}

impl App {
    fn new() -> Self {
        Self {
            character: Character::new("Jane Doe".to_string(), 70),
        }
    }

    fn update(&mut self, message: Message) {
        self.character.update(message);
    }

    fn view(&self) -> Element<Message> {
        let skills: Column<Message> = self
            .character
            .skills
            .get(&StatEnum::Intelligence)
            .map(|skill_map| {
                skill_map.iter().fold(column![], |col, (name, modifiers)| {
                    let total: isize = modifiers.iter().map(|m| m.value()).sum();
                    let total_str = total.to_string();
                    col.push(
                        row![
                            text(name).width(200),
                            text_input(&total_str, &total_str)
                                .on_input(|value| {
                                    if value.chars().all(|c| c.is_numeric()) {
                                        Message::SetSkill(
                                            name.to_string(),
                                            StatEnum::Intelligence,
                                            value,
                                        )
                                    } else {
                                        Message::SetSkill(
                                            name.to_string(),
                                            StatEnum::Intelligence,
                                            value,
                                        )
                                    }
                                })
                                .width(50),
                            button("+").on_press(Message::SkillIncreased(
                                name.clone(),
                                StatEnum::Intelligence,
                            )),
                            button("-").on_press(Message::SkillDecreased(
                                name.clone(),
                                StatEnum::Intelligence,
                            )),
                        ]
                        .spacing(10),
                    )
                })
            })
            .unwrap_or_else(|| column![]);

        column![
            text(format!(
                "Points remaining: {}",
                self.character.character_points
            )),
            skills,
        ]
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Character Gen")
        .run()
}
