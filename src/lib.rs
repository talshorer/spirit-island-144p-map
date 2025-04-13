use bevy::{
    color::palettes::css::{GREEN, RED},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    winit::WinitSettings,
};
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

const MINIMAP_PX: f32 = 300.0;

const BORDER: UiRect = UiRect::all(Val::Px(1.0));

#[derive(Component)]
struct Sidebar;

#[derive(Component)]
struct IsletList;

#[derive(Component)]
struct Minimap;

#[derive(Component)]
struct TtsDisplay;

#[derive(strum::Display, strum::EnumIter, Clone, Copy, PartialEq, Eq)]
enum Islet {
    Cactus,
    Cheese,
    Moon,
    Butterfly,
    Ghost,
    Puzzle,
    Rat,
    Snake,
    Infinity,
    Mauritius,
    Football,
    Seedling,
    Sunglasses,
    Island,
    Love,
    Books,
    Banana,
    Wolf,
    Pizza,
    Hamburger,
    Phoenix,
    Balloon,
    Bagel,
    Cowboy,
    Cookie,
    Dragon,
    Mountain,
    Sheep,
    Lightning,
    Robot,
}

impl Islet {
    const fn emoji(&self) -> &str {
        match self {
            Islet::Cactus => "🌵",
            Islet::Cheese => "🧀",
            Islet::Moon => "🌙",
            Islet::Butterfly => "🦋",
            Islet::Ghost => "👻",
            Islet::Puzzle => "🧩",
            Islet::Rat => "🐀",
            Islet::Snake => "🐍",
            Islet::Infinity => "♾️",
            Islet::Mauritius => "🇲🇺",
            Islet::Football => "⚽",
            Islet::Seedling => "🌱",
            Islet::Sunglasses => "😎",
            Islet::Island => "🏝️",
            Islet::Love => "💖",
            Islet::Books => "📚",
            Islet::Banana => "🍌",
            Islet::Wolf => "🐺",
            Islet::Pizza => "🍕",
            Islet::Hamburger => "🍔",
            Islet::Phoenix => "🐦‍🔥",
            Islet::Balloon => "🎈",
            Islet::Bagel => "🥯",
            Islet::Cowboy => "🤠",
            Islet::Cookie => "🍪",
            Islet::Dragon => "🐉",
            Islet::Mountain => "⛰️",
            Islet::Sheep => "🐑",
            Islet::Lightning => "⚡",
            Islet::Robot => "🤖",
        }
    }
}

#[derive(Component)]
struct SelectIslet(Islet);

struct SelectedIsletInner {
    islet: Islet,
    entity: Entity,
}

#[derive(Resource)]
struct SelectedIslet(Option<SelectedIsletInner>);

fn setup_sidebar(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sidebar)>,
    asset_server: Res<AssetServer>,
) {
    const DONT_BLOCK_LOWER: PickingBehavior = PickingBehavior {
        should_block_lower: false,
        is_hoverable: true,
    };

    let (entity, _) = query.single_mut();
    let mut sidebar = commands.get_entity(entity).unwrap();
    sidebar.with_children(|parent| {
        parent
            .spawn((
                Node {
                    align_self: AlignSelf::Stretch,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::scroll_y(),
                    border: BORDER,
                    ..default()
                },
                BorderColor(RED.into()),
                IsletList,
            ))
            .with_children(|parent| {
                for islet in Islet::iter() {
                    parent
                        .spawn((Button, SelectIslet(islet), DONT_BLOCK_LOWER))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(islet.emoji()),
                                TextFont {
                                    font: asset_server.load("NotoEmoji-SemiBold.ttf"),
                                    ..default()
                                },
                                DONT_BLOCK_LOWER,
                            ));
                            parent.spawn((
                                Text::new(islet.to_string().to_lowercase()),
                                TextFont {
                                    font: asset_server.load("FiraSans-Bold.ttf"),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                DONT_BLOCK_LOWER,
                            ));
                        });
                }
            });
        parent.spawn((
            Node {
                min_height: Val::Px(MINIMAP_PX),
                max_height: Val::Px(MINIMAP_PX),
                border: BORDER,
                ..default()
            },
            Minimap,
            BorderColor(RED.into()),
        ));
        parent.spawn((
            Node {
                min_height: Val::Px(MINIMAP_PX),
                max_height: Val::Px(MINIMAP_PX),
                border: BORDER,
                ..default()
            },
            Minimap,
            BorderColor(RED.into()),
        ));
    });
}

fn setup_screen(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                border: BORDER,
                ..default()
            },
            BorderColor(RED.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    min_width: Val::Px(MINIMAP_PX),
                    max_width: Val::Px(MINIMAP_PX),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                Sidebar,
            ));
            parent.spawn((
                Node {
                    border: BORDER,
                    ..default()
                },
                BorderColor(RED.into()),
                TtsDisplay,
            ));
        });
}

fn islet_button_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &SelectIslet),
        (Changed<Interaction>, With<Button>),
    >,
    children_query: Query<&Children>,
    mut text_color_query: Query<&mut TextColor>,
    mut selected_islet: ResMut<SelectedIslet>,
) {
    const COLOR_SELECTED: Color = Color::Srgba(RED);
    const COLOR_HOVERED: Color = Color::Srgba(GREEN);
    const COLOR_NONE: Color = Color::WHITE;

    let mut change_color = |entity, color| {
        for child in children_query.get(entity).unwrap().iter() {
            if let Ok(mut text_color) = text_color_query.get_mut(*child) {
                text_color.0 = color;
            }
        }
    };

    for (entity, interaction, select_islet) in &mut interaction_query {
        let color = if selected_islet
            .0
            .as_ref()
            .is_some_and(|inner| inner.islet == select_islet.0)
        {
            COLOR_SELECTED
        } else {
            match *interaction {
                Interaction::Pressed => {
                    if let Some(inner) = selected_islet.0.as_mut() {
                        change_color(inner.entity, COLOR_NONE);
                    }
                    selected_islet.0 = Some(SelectedIsletInner {
                        islet: select_islet.0,
                        entity,
                    });
                    COLOR_SELECTED
                }
                Interaction::Hovered => COLOR_HOVERED,
                Interaction::None => COLOR_NONE,
            }
        };
        change_color(entity, color);
    }
}

fn update_islet_list_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<(&mut ScrollPosition, &IsletList)>,
) {
    const LINE_HEIGHT: f32 = 20.0;

    for mouse_wheel_event in mouse_wheel_events.read() {
        let dy = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => mouse_wheel_event.y * LINE_HEIGHT,
            MouseScrollUnit::Pixel => mouse_wheel_event.y,
        };
        for (_, pointer_map) in hover_map.iter() {
            for (entity, _) in pointer_map.iter() {
                if let Ok((mut scroll_position, _)) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#mygame-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(SelectedIslet(None))
        .add_systems(Startup, setup_screen)
        .add_systems(Startup, setup_sidebar.after(setup_screen))
        .add_systems(Update, islet_button_system)
        .add_systems(Update, update_islet_list_scroll)
        .run();
}
