use charecter_gen_2020::character::Character;
use charecter_gen_2020::graphics::messages::{self, Message};
use iced::widget::{Column, button, column, row, text};
use iced::{self, Color, Element};

struct App {
    character: Character,
}

impl App {
    fn update(&mut self, message: Message) {
        self.character.update(message);
    }

    fn view(&self) -> Element<Message> {
        let skills: Column<Message> =
            self.character
                .special_abilities
                .iter()
                .fold(column![], |col, (name, modifiers)| {
                    let total: isize = modifiers.iter().map(|m| m.value()).sum();
                    col.push(
                        row![
                            text(name).width(200),
                            text(total.to_string()).width(50),
                            button("+").on_press(Message::SkillIncreased(name.clone())),
                            button("-").on_press(Message::SkillDecreased(name.clone())),
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

fn main() {
    iced::run("Character Gen", App::update, App::view);
}
