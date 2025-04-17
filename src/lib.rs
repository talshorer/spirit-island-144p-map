mod asset_json5;

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{GREEN, RED},
    image::{CompressedImageFormats, ImageSampler, ImageType},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    winit::WinitSettings,
};
use bevy_mod_reqwest::{BevyReqwest, ReqwestPlugin, ReqwestResponseEvent};
use serde::Deserialize;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;

use asset_json5::Json5AssetLoader;

#[derive(Asset, TypePath, Deserialize)]
struct Config {
    pub(crate) base_url: String,
    border_px: f32,
}

impl Config {
    fn border(&self) -> UiRect {
        UiRect::all(Val::Px(self.border_px))
    }
}

const MINIMAP_PX: f32 = 300.0;

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

    fn screenshot_method(&self) -> ScreenshotMethod {
        use ScreenshotMethod as S;

        match self {
            Islet::Cactus => S::Bitcrafter("c93a3401-57c2-4f82-91e7-84453cea5c44"),
            Islet::Cheese => S::Bitcrafter("39d059ee-d8b6-4eca-8f83-e39e418921eb"),
            Islet::Moon => S::Bitcrafter("9ce0657c-ab9d-42c1-b48a-ee86ddd49fbf"),
            Islet::Butterfly => S::Bitcrafter("9291ed98-b15c-464c-ae58-a206751e85f1"),
            Islet::Ghost => S::Bitcrafter("1e5881ce-834d-488e-b3a9-884dcbbac257"),
            Islet::Puzzle => S::Bitcrafter("c79ad3f5-8836-4c11-8888-e34a0968f31f"),
            Islet::Rat => S::Bitcrafter("8ff037d8-2f40-4b56-8526-daa82310fd20"),
            Islet::Snake => S::Bitcrafter("698c01fd-36af-4e5b-b689-0ece85be74f3"),
            Islet::Infinity => S::Bitcrafter("c212aa05-8394-44eb-bb59-52bc54c64920"),
            Islet::Mauritius => S::Bitcrafter("172d4b18-c734-495a-b456-3d1a2b4cf6f1"),
            Islet::Football => S::Bitcrafter("36b43f3b-4ac9-4931-af01-205a0dd51615"),
            Islet::Seedling => S::Bitcrafter("eb617149-9229-4212-91ce-6ee760b23325"),
            Islet::Sunglasses => S::AbandonedIslet,
            Islet::Island => S::AbandonedIslet,
            Islet::Love => S::AbandonedIslet,
            Islet::Books => S::Bitcrafter("c27f4831-d688-47c4-ac8b-f092ea527e8c"),
            Islet::Banana => S::Bitcrafter("9c07308b-a526-4276-b477-9ec6f0f11a24"),
            Islet::Wolf => S::Bitcrafter("fe482166-68c4-4a84-a6f3-03a9ef47bcd4"),
            Islet::Pizza => S::Bitcrafter("e7264e57-7f5d-47b4-8975-b7c1d801ead8"),
            Islet::Hamburger => S::Bitcrafter("7e8bbce0-9fcb-49b8-8e2a-ebe75d3e35c6"),
            Islet::Phoenix => S::Bitcrafter("58c681c0-a806-41d9-968a-d46b45b56e8c"),
            Islet::Balloon => S::Bitcrafter("22787c21-a6e7-4df8-9032-0a0f9dc60d25"),
            Islet::Bagel => S::Bitcrafter("3b7e540b-cfea-48d1-8ce2-3e2a52ba9344"),
            Islet::Cowboy => S::Bitcrafter("b16152f5-cc8c-45ec-8eb2-7941f4b59792"),
            Islet::Cookie => S::Bitcrafter("1f8732fd-13c1-471b-b5e8-691e574e3445"),
            Islet::Dragon => S::Bitcrafter("2a5e6973-217c-45ef-9218-e23d6d488839"),
            Islet::Mountain => S::Bitcrafter("cf7a95fd-d323-4a73-b8fa-eda66b10dd64"),
            Islet::Sheep => S::AbandonedIslet,
            Islet::Lightning => S::AbandonedIslet,
            Islet::Robot => S::AbandonedIslet,
        }
    }
}

enum ScreenshotMethod<'a> {
    Bitcrafter(&'a str),
    AbandonedIslet,
}

#[derive(Component)]
struct SelectIslet(Islet);

struct SelectedIsletInner {
    islet: Islet,
    entity: Entity,
}

#[derive(Resource)]
struct SelectedIslet(Option<SelectedIsletInner>);

#[derive(Resource)]
struct ConfigHandle(Handle<Config>);

#[derive(Resource)]
struct ConfigRes(Config);

fn load_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ConfigHandle(asset_server.load("main.cfg.json5")));
}

fn wait_for_config(
    mut commands: Commands,
    config_handle: Res<ConfigHandle>,
    mut configs: ResMut<Assets<Config>>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let Some(config) = configs.remove(config_handle.0.id()) {
        commands.insert_resource(ConfigRes(config));
        state.set(AppState::App);
    }
}

fn setup_sidebar(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sidebar)>,
    asset_server: Res<AssetServer>,
    config: Res<ConfigRes>,
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
                    border: config.0.border(),
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

        let mut spawn_minimap = |asset| {
            parent
                .spawn((
                    Node {
                        min_height: Val::Px(MINIMAP_PX),
                        max_height: Val::Px(MINIMAP_PX),
                        border: config.0.border(),
                        ..default()
                    },
                    Minimap,
                    BorderColor(RED.into()),
                ))
                .with_child(ImageNode::new(asset_server.load(asset)));
        };
        spawn_minimap("ContinentBlue.png");
        spawn_minimap("ContinentOrange.png");
    });
}

fn setup_screen(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<ConfigRes>) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                border: config.0.border(),
                ..default()
            },
            BorderColor(GREEN.into()),
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
                    border: config.0.border(),
                    ..default()
                },
                BorderColor(RED.into()),
                TtsDisplay,
                ImageNode::new(asset_server.load("bigmap.png")),
            ));
        });
}

fn on_img_response(
    trigger: Trigger<ReqwestResponseEvent>,
    mut tts_display_query: Query<(&mut ImageNode, &TtsDisplay)>,
    mut images: ResMut<Assets<Image>>,
) {
    let response = trigger.event();
    let data = response.body().iter().as_slice();
    let (mut image_node, _) = tts_display_query.single_mut();
    image_node.image = images.add(
        Image::from_buffer(
            data,
            ImageType::Format(ImageFormat::Png),
            CompressedImageFormats::all(),
            false,
            ImageSampler::Default,
            RenderAssetUsages::default(),
        )
        .unwrap(),
    );
}

fn on_game_response(
    trigger: Trigger<ReqwestResponseEvent>,
    mut client: BevyReqwest,
    config: Res<ConfigRes>,
) {
    const SEARCH_FOR: &str = "<img src=\"";

    let response = trigger.event();
    let data = response.as_str().unwrap();
    let imgsrc = data
        .split("\n")
        .find_map(|line| {
            line.find(SEARCH_FOR)
                .map(|idx| &line[(idx + SEARCH_FOR.len())..])
                .map(|line| &line[..line.find("\" ").unwrap()])
        })
        .unwrap();
    let url = config.0.base_url.clone() + imgsrc;
    let req = client.get(&url).build().unwrap();
    client.send(req).on_response(on_img_response);
}

fn get_islet_image(config: &Config, client: &mut BevyReqwest, islet: Islet) {
    let uuid = match islet.screenshot_method() {
        ScreenshotMethod::Bitcrafter(uuid) => uuid,
        ScreenshotMethod::AbandonedIslet => return,
    };
    let url = config.base_url.clone() + "/game/" + uuid;
    let req = client.get(&url).build().unwrap();
    client.send(req).on_response(on_game_response);
}

fn islet_button_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &SelectIslet),
        (Changed<Interaction>, With<Button>),
    >,
    children_query: Query<&Children>,
    mut text_color_query: Query<&mut TextColor>,
    mut selected_islet: ResMut<SelectedIslet>,
    mut client: BevyReqwest,
    config: Res<ConfigRes>,
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
                    get_islet_image(&config.0, &mut client, select_islet.0);
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

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum AppState {
    #[default]
    Loading,
    App,
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
        .init_state::<AppState>()
        .add_plugins(ReqwestPlugin::default())
        .init_asset::<Config>()
        .register_asset_loader(Json5AssetLoader::<Config>::new(&[".cfg.json5"]))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(SelectedIslet(None))
        .add_systems(Startup, load_config)
        .add_systems(Update, wait_for_config.run_if(in_state(AppState::Loading)))
        .add_systems(OnEnter(AppState::App), setup_screen.after(load_config))
        .add_systems(OnEnter(AppState::App), setup_sidebar.after(setup_screen))
        .add_systems(Update, islet_button_system.run_if(in_state(AppState::App)))
        .add_systems(
            Update,
            update_islet_list_scroll.run_if(in_state(AppState::App)),
        )
        .run();
}
