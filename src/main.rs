use charecter_gen_2020::character::Character;
use charecter_gen_2020::graphics::messages::{self, Message};
use iced::widget::{Column, button, column, row, text, text_input};
use iced::{self, Color, Element};

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
        let skills: Column<Message> =
            self.character
                .intelligence_skills
                .iter()
                .fold(column![], |col, (name, modifiers)| {
                    let total: isize = modifiers.iter().map(|m| m.value()).sum();
                    col.push(
                        row![
                            text(name).width(200),
                            text_input(&total.to_string(), &total.to_string())
                                .on_input(|value| Message::SetSkill(
                                    name.to_string(),
                                    messages::StatEnum::Intelligence,
                                    value.to_ascii_lowercase()
                                ))
                                .width(50),
                            button("+").on_press(Message::SkillIncreased(
                                name.clone(),
                                messages::StatEnum::Intelligence
                            )),
                            button("-").on_press(Message::SkillDecreased(
                                name.clone(),
                                messages::StatEnum::Intelligence
                            )),
                        ]
                        .spacing(10),
                    )
                });

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
