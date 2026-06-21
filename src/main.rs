use charecter_gen_2020::character::Character;
use charecter_gen_2020::graphics::messages::{self, Message, StatEnum};
use charecter_gen_2020::skills::Modifier;
use iced::theme::Palette;
use iced::widget::{Column, Row, button, column, container, row, text, text_input, tooltip};
use iced::{self, Border, Color, Element, Fill, Font, Theme};
use rfd;
use serde_json::value::Serializer;
use std::fs;

const PHOSPHOR_GREEN: Color = Color {
    r: 0.2,
    g: 1.0,
    b: 0.4,
    a: 1.0,
};
const CRT_BLACK: Color = Color {
    r: 0.02,
    g: 0.02,
    b: 0.02,
    a: 1.0,
};

fn crt_theme() -> Theme {
    Theme::custom(
        "CRT".to_string(),
        Palette {
            background: CRT_BLACK,
            text: PHOSPHOR_GREEN,
            primary: PHOSPHOR_GREEN,
            success: PHOSPHOR_GREEN,
            warning: Color::from_rgb(1.0, 0.8, 0.2),
            danger: Color::from_rgb(1.0, 0.3, 0.3),
        },
    )
}

enum AppState {
    Prompt,
    Loaded,
    Edit,
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
                self.state = AppState::Edit;
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
        container(
            column![
                button("New Character").on_press(Message::NewCharacter),
                button("Load Character").on_press(Message::LoadCharacter),
            ]
            .spacing(20),
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
    fn normal_view(&self) -> Element<Message> {
        const ITEMS_PER_COLUMN: usize = 12;
        const CONTAINER_HEIGHT: f32 = 500.0;

        let mono = |s: String| text(s).font(Font::MONOSPACE);

        let mut stats: Vec<_> = self.character.skills.iter().collect();
        stats.sort_by_key(|(stat, _)| format!("{:?}", stat));

        let mut flat_items: Vec<Element<Message>> = Vec::new();

        for (stat, skill_map) in stats {
            flat_items.push(mono(format!("[{:?}]", stat)).size(20).into());

            let mut skills: Vec<_> = skill_map.iter().collect();
            skills.sort_by_key(|(name, _)| (*name).clone());

            for (name, modifiers) in skills {
                let total: isize = modifiers.iter().map(|m| m.value()).sum();

                let breakdown = modifiers.iter().fold(column![], |bcol, m| {
                    let line = if m.has_condition {
                        format!("> {}: {} (inactive)", m.origin, m.modifier)
                    } else {
                        format!("> {}: {}", m.origin, m.modifier)
                    };
                    bcol.push(mono(line))
                });

                let entry = row![
                    mono(name.clone()).width(150),
                    tooltip(
                        mono(total.to_string()),
                        container(breakdown)
                            .padding(8)
                            .style(|_theme: &Theme| container::Style {
                                background: Some(CRT_BLACK.into()),
                                text_color: Some(PHOSPHOR_GREEN),
                                border: Border {
                                    radius: 0.0.into(),
                                    width: 1.0,
                                    color: PHOSPHOR_GREEN,
                                },
                                ..Default::default()
                            }),
                        tooltip::Position::Bottom,
                    ),
                ]
                .spacing(10);

                flat_items.push(entry.into());
            }
        }

        // Drain into fixed-size groups, since Element isn't Clone and
        // .chunks() can't move out of a borrowed slice.
        let mut columns_row = row![].spacing(40);
        let mut iter = flat_items.into_iter();
        loop {
            let mut col = column![].spacing(8);
            let mut count = 0;
            let mut pushed_any = false;
            while count < ITEMS_PER_COLUMN {
                match iter.next() {
                    Some(item) => {
                        col = col.push(item);
                        count += 1;
                        pushed_any = true;
                    }
                    None => break,
                }
            }
            if !pushed_any {
                break;
            }
            columns_row = columns_row.push(col);
        }

        column![
            mono(format!(
                "POINTS REMAINING: {}",
                self.character.character_points
            )),
            container(columns_row).height(CONTAINER_HEIGHT).padding(10),
            button(mono("[ SERIALIZE ]".to_string())).on_press(Message::SaveJson),
        ]
        .into()
    }
    fn edit_view(&self) -> Element<Message> {
        let stats_row: Row<Message> = self.character.skills.iter().fold(
            row![].spacing(30),
            |outer_row, (stat, skill_map)| {
                let stat = *stat;

                let stat_column = skill_map
                    .iter()
                    .fold(
                        column![text(format!("{:?}", stat)).size(20)],
                        |col, (name, modifiers)| {
                            let total: isize = modifiers.iter().map(|m| m.value()).sum();
                            let total_str = total.to_string();
                            let name_owned = name.clone();

                            col.push(
                                row![
                                    text(name).width(150),
                                    text_input(&total_str, &total_str)
                                        .on_input(move |value| {
                                            Message::SetSkill(name_owned.clone(), stat, value)
                                        })
                                        .width(50),
                                    button("+")
                                        .on_press(Message::SkillIncreased(name.clone(), stat)),
                                    button("-")
                                        .on_press(Message::SkillDecreased(name.clone(), stat)),
                                ]
                                .spacing(10),
                            )
                        },
                    )
                    .spacing(8);

                outer_row.push(stat_column)
            },
        );

        column![
            text(format!(
                "Points remaining: {}",
                self.character.character_points
            )),
            stats_row,
            button("serialize").on_press(Message::SaveJson),
        ]
        .into()
    }
    fn view(&self) -> Element<Message> {
        match self.state {
            AppState::Prompt => self.prompt_view(),
            AppState::Loaded => self.normal_view(),
            AppState::Edit => self.edit_view(),
        }
    }
    fn load_char(&mut self, json: fs::File) {
        self.character = Character::new("Jane_doe".to_string(), 70, json);
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Character Gen")
        .theme(|_app: &App| crt_theme())
        .run()
}
fn saveFile(output: &Character) {
    let json = serde_json::to_string_pretty(&output).unwrap();
    fs::write("new_character.json", json).unwrap();
}
