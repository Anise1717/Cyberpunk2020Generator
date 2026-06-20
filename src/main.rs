use std::fs;

use charecter_gen_2020::character::Character;
use charecter_gen_2020::graphics::messages::{self, Message, StatEnum};
use iced::widget::{Column, Container, button, column, row, text, text_input};
use iced::{self, Element};
use rfd;
use serde_json::value::Serializer;
enum AppState {
    Prompt,
    Loaded,
}
struct App {
    state: AppState,
    character: Character,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::Prompt,
            character: Character::default(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewCharacter => {
                let mut file = fs::File::open("new_character.json").expect("failed to open file");
                self.load_char(file);
                self.state = AppState::Loaded;
            }
            Message::LoadCharacter => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("json", &["json"])
                    .pick_file()
                {
                    if let Ok(json_str) = fs::read_to_string(&path) {
                        if let Ok(character) = serde_json::from_str(&json_str) {
                            self.character = character;
                            self.state = AppState::Loaded;
                        }
                    }
                }
            }
            Message::SaveJson => saveFile(&self.character),
            _ => self.character.update(message),
        }
    }
    fn prompt_view(&self) -> Element<Message> {
        column![
            button("New Character").on_press(Message::NewCharacter),
            button("Load Character").on_press(Message::LoadCharacter),
        ]
        .spacing(20)
        .into()
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Prompt => self.prompt_view(),
            AppState::Loaded => self.normal_view(),
        }
    }
    fn load_char(&mut self, json: fs::File) {
        self.character = Character::new("Jane_doe".to_string(), 70, json);
    }
    fn normal_view(&self) -> Element<Message> {
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
            button("serialize").on_press(Message::SaveJson),
        ]
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Character Gen")
        .run()
}
fn saveFile(output: &Character) {
    let json = serde_json::to_string_pretty(&output).unwrap();
    fs::write("new_character.json", json).unwrap();
}
