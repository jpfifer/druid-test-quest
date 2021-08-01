mod timer_widget;
mod config;

use druid::*;
use druid::widget::*;
use std::sync::Arc;
use crate::timer_widget::TimerWidget;
use crate::config::{Classes, Races, Enemy};
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Debug, Clone, Data, Lens)]
struct Spell {
    name: String,
    level: String,
}

impl Spell {
    fn new(name: &str, level: &str) -> Self {
        Self {
            name: name.into(),
            level: level.into(),
        }
    }
}

#[derive(Debug, Clone, Data, Lens)]
struct InventoryItem {
    item_name: String,
    count: i32,
}

impl InventoryItem {
    fn new(name: &str, count: i32) -> Self {
        Self {
            item_name: name.into(),
            count,
        }
    }
}

#[derive(Debug, Clone, Copy, Data, Lens)]
struct Stats {
    str: i32,
    con: i32,
    dex: i32,
    int: i32,
    wis: i32,
    cha: i32,
    hp_max: i32,
    hp: i32,
}

impl Stats {
    fn random() -> Self {
        let mut rng = ThreadRng::default();
        Self {
            str: rng.gen_range(8..20),
            con: rng.gen_range(8..20),
            dex: rng.gen_range(8..20),
            int: rng.gen_range(8..20),
            wis: rng.gen_range(8..20),
            cha: rng.gen_range(8..20),
            hp_max: rng.gen_range(20..40),
            hp: 0,
        }
    }
}

#[derive(Debug, Clone, Data, Lens)]
struct Character {
    name: String,
    race: String,
    class: String,
    level: i32,
    experience_percent: f64,
    spells: Arc<Vec<Spell>>,
    stats: Stats,
    inventory: Arc<Vec<InventoryItem>>,
}

impl Character {
    fn new(name: &str, race: &str, class: &str, level: i32, stats: Stats) -> Self {
        Self {
            name: name.into(),
            race: race.into(),
            class: class.into(),
            level,
            experience_percent: 0.0,
            spells: Arc::new(vec![]),
            stats,
            inventory: Arc::new(vec![]),
        }
    }

    fn add_item(&mut self, item_name: &str, count: i32) {
        let mut new_vec: Vec<InventoryItem> = self.inventory.iter().map(|item| item.clone()).collect();

        if new_vec.iter().find(|i| i.item_name == item_name).is_none() {
            new_vec.push(InventoryItem {
                item_name: item_name.into(),
                count: count,
            })
        } else {
            new_vec.iter_mut().filter(|item| item.item_name == item_name).for_each(|item| item.count += count);
        }
        self.inventory = Arc::new(new_vec);
    }
}

#[derive(Debug, Copy, Data, Clone, Eq, PartialEq)]
enum GameState {
    Creation,
    Running,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    character: Character,
    game_state: GameState,
    class_selection: String,
    race_selection: String,
    current_message: String,
    current_progress: f64,
    current_enemy: Option<Enemy>,
    find_next_battle: bool,

}

impl AppData {
    fn new(character: &Character) -> Self {
        Self {
            character: character.clone(),
            game_state: GameState::Creation,
            class_selection: "".into(),
            race_selection: "".into(),
            current_message: "".into(),
            current_progress: 0.0,
            current_enemy: None,
            find_next_battle: true,
        }
    }

    // pub fn add_gold(&mut self, count: usize) {
    //     let mut new_vec: Vec<InventoryItem> = self.character.inventory.iter().map(|item| item.clone()).collect();
    //
    //     if new_vec.iter().find(|i| i.item_name == "Gold").is_none() {
    //         new_vec.push(InventoryItem {
    //             item_name: "Gold".into(),
    //             count: count,
    //         })
    //     } else {
    //         new_vec.iter_mut().filter(|item| item.item_name == "Gold").for_each(|item| item.count += count);
    //     }
    //     self.character.inventory = Arc::new(new_vec);
    // }

    pub fn is_running(&self) -> bool {
        self.game_state == GameState::Running
    }

    pub fn tick(&mut self) {
        let mut rng = ThreadRng::default();
        let change = self.current_progress + rng.gen_range(0.01..0.1);
        if let Some(enemy) = self.current_enemy {

            // In a battle
            if change >= 1.0 {
                // Battle complete

                // did it drop gold
                if rng.gen_ratio(3, 10) {
                    let gold = rng.gen_range(0..10);
                    self.character.add_item("gold", gold);
                }
                if rng.gen_ratio(8, 10) {
                    let item_name = format!("{} {}", enemy.name(), enemy.item());
                    self.character.add_item(item_name.as_str(), 1);
                }

                self.current_progress = 1.0;
                self.find_next_battle = true;
                self.current_message = format!("Executed a {}", enemy.name());
                self.current_enemy = None;
            } else if change > 0.2 {
                self.current_message = format!("Executing a {}", enemy.name());
                self.current_progress = change;
            } else {
                self.current_progress = change;
            }
        } else {
            if self.find_next_battle {
                self.current_progress = 0.0;
                self.find_next_battle = false;
                self.current_message = "You start searching for another battle".into();
            }
            if change >= 1.0 {
                let enemy = Enemy::random_enemy();
                self.current_enemy = Some(enemy);
                self.current_message = format!("You challenge a {} to combat", enemy.name());
                self.current_progress = 0.0;
            } else {
                self.current_progress = change;
            }
        }
    }
}


fn build_ui() -> impl Widget<AppData> {
    TimerWidget::new(ViewSwitcher::new(
        |data: &AppData, _env| data.game_state,
        |selector, _data, _env| match selector {
            GameState::Creation => Box::new(create_character_ui()),
            GameState::Running => Box::new(game_ui())
        }))
}

#[derive(Debug, Data, Clone, Eq, PartialEq)]
struct RadioItem(String);

fn create_character_ui() -> impl Widget<AppData> {
    let race_items: Vec<(String, String)> = Races::all_races().iter().map(|c| (c.clone(), c.to_string())).collect();
    let races = RadioGroup::new(race_items).lens(AppData::race_selection);

    let class_items: Vec<(&str, String)> = Classes::all_classes().iter().map(|c| (*c, c.to_string())).collect();
    let classes = RadioGroup::new(class_items).lens(AppData::class_selection);
    // let classes = create_classes(RadioGroup::new(class_items));


    Flex::column().with_flex_child(
        Flex::row()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(Label::new("Race").expand_width())
                    .with_flex_child(races, 1.0)
                    .padding(Insets::uniform_xy(10.0, 5.0))
                    .expand_height()
                , 1.0)
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(Label::new("Class").expand_width())
                    .with_flex_child(classes, 1.0)
                    .padding(Insets::uniform_xy(10.0, 5.0))
                    .expand_height()
                , 1.0,
            )
            .with_flex_child(
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(Label::new("Stats").expand_width())
                    .with_flex_child(build_stats_creation(), 1.0)
                    .with_child(Label::new("Name"))
                    .with_flex_child(
                        TextBox::new()
                            .with_placeholder("Name")
                            .lens(AppData::character.then(Character::name))
                        , 1.0)
                , 1.0,
            )
            .expand_height()
        , 1.0,
    ).with_child(
        Button::new("Create Character").align_right().padding(5.0).on_click(|_, data: &mut AppData, _| {
            data.character.stats.hp = data.character.stats.hp_max;
            data.character.race = data.race_selection.clone();
            data.character.class = data.class_selection.clone();
            data.game_state = GameState::Running;
        })
    )
}

fn build_stats_creation() -> impl Widget<AppData> {
    Flex::column()
        .with_flex_child(add_trait_row("STR", |data| format!("{}", data.character.stats.str)).padding(2.0), 1.0)
        .with_flex_child(add_trait_row("CON", |data| format!("{}", data.character.stats.con)), 1.0)
        .with_flex_child(add_trait_row("DEX", |data| format!("{}", data.character.stats.dex)), 1.0)
        .with_flex_child(add_trait_row("INT", |data| format!("{}", data.character.stats.int)), 1.0)
        .with_flex_child(add_trait_row("WIS", |data| format!("{}", data.character.stats.wis)), 1.0)
        .with_flex_child(add_trait_row("CHA", |data| format!("{}", data.character.stats.cha)), 1.0)
        .with_flex_child(
            Button::new("Re-roll 'em").on_click(|_, data: &mut AppData, _| {
                data.character.stats = Stats::random();
            })
            , 1.0,
        ).border(Color::grey(0.0), 1.0)
}

fn game_ui() -> impl Widget<AppData> {
    Flex::column()
        .with_flex_child(
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .with_flex_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Fill)
                .with_flex_child(
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(Label::new("Character Sheet"))
                        .with_flex_child(
                            build_character_ui(),
                            1.0,
                        ),
                    1.0,
                ),
            1.0,
        )
        .with_flex_child(build_inventory(), 1.0)
        .background(BackgroundBrush::Color(Color::grey(0.25))).expand_height(), 1.0
        ).with_child(
    Flex::column()
        .with_child(Label::dynamic(|message: &String, _| message.clone()).lens(AppData::current_message).expand_width())
        .with_child(ProgressBar::new().expand_width().padding(2.0).lens(AppData::current_progress))
    )
}

fn build_stats() -> impl Widget<AppData> {
    Container::new(
        Flex::column().with_flex_child(
            Flex::row()
                .cross_axis_alignment(CrossAxisAlignment::Fill)
                .with_flex_child(
                    Label::new("Trait")
                        .with_text_color(Color::grey(0.0))
                        .border(Color::grey(0.1), 0.5)
                        .background(BackgroundBrush::Color(Color::grey(0.75)))
                        .expand_width()
                    , 1.5)
                .with_flex_child(
                    Label::new("Value")
                        .with_text_color(Color::grey(0.0))
                        .border(Color::grey(0.1), 0.5)
                        .background(BackgroundBrush::Color(Color::grey(0.75)))
                        .expand_width()
                    , 1.0),
            1.0)
            .with_flex_child(add_trait_row("Name", |data| data.character.name.clone()), 1.0)
            .with_flex_child(add_trait_row("Race", |data| data.character.race.clone()), 1.0)
            .with_flex_child(add_trait_row("Class", |data| data.character.class.clone()), 1.0)
            .with_flex_child(add_trait_row("Level", |data| format!("{}", data.character.level)), 1.0)
            .with_flex_child(
                Flex::row().cross_axis_alignment(CrossAxisAlignment::Fill).with_flex_child(
                    Label::new("Stat")
                        .with_text_color(Color::grey(0.0))
                        .border(Color::grey(0.1), 0.5)
                        .background(BackgroundBrush::Color(Color::grey(0.75)))
                        .expand_width()
                    , 1.5)
                    .with_flex_child(
                        Label::new("Value")
                            .with_text_color(Color::grey(0.0))
                            .border(Color::grey(0.1), 0.5)
                            .background(BackgroundBrush::Color(Color::grey(0.75)))
                            .expand_width()
                        , 1.0)
                , 1.0)
            .with_flex_child(add_trait_row("STR", |data| format!("{}", data.character.stats.str)), 1.0)
            .with_flex_child(add_trait_row("CON", |data| format!("{}", data.character.stats.con)), 1.0)
            .with_flex_child(add_trait_row("DEX", |data| format!("{}", data.character.stats.dex)), 1.0)
            .with_flex_child(add_trait_row("INT", |data| format!("{}", data.character.stats.int)), 1.0)
            .with_flex_child(add_trait_row("WIS", |data| format!("{}", data.character.stats.wis)), 1.0)
            .with_flex_child(add_trait_row("CHA", |data| format!("{}", data.character.stats.cha)), 1.0)
            .with_flex_child(add_trait_row("HP Max", |data| format!("{}", data.character.stats.hp_max)), 1.0)
            .with_flex_child(add_trait_row("HP", |data| format!("{}", data.character.stats.hp)), 1.0)
    ).border(Color::grey(0.0), 1.0)
}

fn build_spells() -> impl Widget<AppData> {
    Flex::column().with_flex_child(
        Flex::row()
            .cross_axis_alignment(CrossAxisAlignment::Fill)
            .with_flex_child(
                Label::new("Spell")
                    .with_text_color(Color::grey(0.0))
                    .border(Color::grey(0.1), 0.5)
                    .background(BackgroundBrush::Color(Color::grey(0.75)))
                    .expand_width()
                , 2.0)
            .with_flex_child(
                Label::new("Level")
                    .with_text_color(Color::grey(0.0))
                    .border(Color::grey(0.1), 0.5)
                    .background(BackgroundBrush::Color(Color::grey(0.75)))
                    .expand_width()
                , 1.0)
        , 1.0,
    ).with_flex_child(
        List::new(|| {
            Flex::row()
                .with_flex_child(Label::dynamic(|name: &String, _: &Env| name.clone()).expand_width().lens(Spell::name), 2.0)
                .with_flex_child(Label::dynamic(|level: &String, _: &Env| level.clone()).lens(Spell::level).expand_width(), 1.0)
        }).lens(AppData::character.then(Character::spells))
        , 1.0,
    )
}

fn build_character_ui() -> impl Widget<AppData> {
    Flex::column()
        .with_flex_child(build_stats(), 3.0)
        .with_spacer(1.0)
        .with_flex_child(Label::new("Experience").expand_width(), 1.0)
        .with_flex_child(
            ProgressBar::new()
                .expand_width()
                .lens(AppData::character.then(Character::experience_percent))
            , 1.0)
        .with_spacer(1.0)
        .with_flex_child(build_spells(), 1.0)
        .expand_height()
        .border(Color::grey(0.0), 1.0)
}

fn add_trait_row(name: &str, text: impl Fn(&AppData) -> String + 'static) -> impl Widget<AppData> {
    Flex::row()
        .with_flex_child(Label::new(name).expand_width(), 1.5)
        .with_flex_child(Label::dynamic(move |data: &AppData, _: &Env| text(&data)).expand_width(), 1.0)
}

fn build_equip_inventory() -> impl Widget<AppData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Fill)
        .with_child(Label::new("Equipment").expand_width())
}

fn build_inventory() -> impl Widget<AppData> {
    Flex::column().with_child(
        Flex::row()
            .with_flex_child(
                Label::new("Item")
                    .with_text_color(Color::grey(0.0))
                    .border(Color::grey(0.1), 0.5)
                    .background(BackgroundBrush::Color(Color::grey(0.75)))
                    .expand_width(), 3.0)
            .with_flex_child(
                Label::new("Count")
                    .with_text_color(Color::grey(0.0))
                    .border(Color::grey(0.1), 0.5)
                    .background(BackgroundBrush::Color(Color::grey(0.75)))
                    .expand_width(), 1.0)
    ).with_child(
        List::new(|| {
            Flex::row()
                .with_flex_child(
                    Label::dynamic(|item: &String, _: &Env| item.clone())
                        .lens(InventoryItem::item_name)
                        .expand_width()
                    , 3.0)
                .with_flex_child(
                    Label::dynamic(|count: &i32, _: &Env| format!("{}", count))
                        .expand_width()
                        .lens(InventoryItem::count)
                    , 1.0)
        }).lens(AppData::character.then(Character::inventory))
    )
}


fn main() {
    let app_data = AppData::new(&Character::new("", "", "", 0, Stats::random()));
    let window_desc = WindowDesc::new(build_ui()).title("Druid Test Quest").window_size((1024.0, 768.0));
    AppLauncher::with_window(window_desc)
        // .log_to_console()
        .launch(app_data)
        .expect("Failed to start");
}
