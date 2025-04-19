mod asset_json5;

use std::ops::Add;

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{BLUE, GREEN, RED},
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

include!(concat!(env!("OUT_DIR"), "/islets.rs"));

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
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    flex_shrink: 1.0,
                    flex_grow: 1.0,
                    ..default()
                })
                .with_child((
                    Node {
                        border: config.0.border(),
                        max_width: Val::Percent(80.0),
                        max_height: Val::Percent(100.0),
                        ..default()
                    },
                    BorderColor(BLUE.into()),
                    TtsDisplay,
                    ImageNode::new(asset_server.load("bigmap.png")),
                ));
        });
}

fn blueness(image: &Image) -> bool {
    let normalized_sum = (0..image.width())
        .map(|x| {
            let x = (0..image.height())
                .map(|y| image.get_color_at(x, y).unwrap().to_srgba())
                .reduce(Add::add)
                .unwrap();
            x
        })
        .reduce(Add::add)
        .unwrap()
        / ((image.width() * image.height()) as f32);
    normalized_sum.red < 0.7 && normalized_sum.green < 0.7 && normalized_sum.blue > 0.4
}

fn on_img_response(
    trigger: Trigger<ReqwestResponseEvent>,
    mut tts_display_query: Query<(&mut ImageNode, &TtsDisplay)>,
    mut images: ResMut<Assets<Image>>,
) {
    let response = trigger.event();
    let data = response.body().iter().as_slice();
    let (mut image_node, _) = tts_display_query.single_mut();
    let image = Image::from_buffer(
        data,
        ImageType::Format(ImageFormat::Png),
        CompressedImageFormats::all(),
        true,
        ImageSampler::Default,
        RenderAssetUsages::default(),
    )
    .unwrap();
    if blueness(&image) {
        image_node.image = images.add(image);
    }
}

fn on_game_response(
    trigger: Trigger<ReqwestResponseEvent>,
    mut client: BevyReqwest,
    config: Res<ConfigRes>,
) {
    const SEARCH_FOR: &str = "<img src=\"";

    let response = trigger.event();
    let data = response.as_str().unwrap();
    for line in data.split("\n") {
        if let Some(imgsrc) = line
            .find(SEARCH_FOR)
            .map(|idx| &line[(idx + SEARCH_FOR.len())..])
            .map(|line| &line[..line.find("\" ").unwrap()])
        {
            if !imgsrc.starts_with("/screenshot") {
                continue;
            }
            let url = config.0.base_url.clone() + imgsrc;
            let req = client.get(&url).build().unwrap();
            client.send(req).on_response(on_img_response);
        }
    }
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
