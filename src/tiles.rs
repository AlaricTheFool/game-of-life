use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;

use crate::control::SimControls;

struct UpdateTimer(Timer);

const MAP_WIDTH  : u32 = 32;
const MAP_HEIGHT : u32 = 32;

const CHUNK_SIZE : u32 = 8;
const TILE_SIZE : f32 = 16.;

const TIME_PER_UPDATE: f32 = 0.25;

#[derive(Component)]
struct LifeTile {
    is_alive: bool,
    will_be_alive: bool,
}

impl LifeTile {
    fn new() -> Self {
        LifeTile {
            is_alive: false,
            will_be_alive: false,
        }
    }
}

pub fn add_conway_tiles_to_app(mut app: App) -> App {
    app.add_plugin(TilemapPlugin)
        .insert_resource(UpdateTimer(Timer::from_seconds(TIME_PER_UPDATE, true)))
        .add_startup_system_to_stage(StartupStage::PreStartup, build_map)
        .add_startup_system(randomize_tiles)
        .add_system(set_texture_filters_to_nearest)
        .add_system(simulate_tiles)
        .add_system_to_stage(CoreStage::PostUpdate, change_tiles_to_new_states)
        .add_system_to_stage(CoreStage::PreUpdate, read_tile_controls);
    
    app
}

fn read_tile_controls(mut controls: ResMut<SimControls>, tiles: Query<(&mut Tile, &mut LifeTile)>) {
    if controls.should_restart {
        randomize_tiles(tiles);
        controls.should_restart = false;
    } else if controls.should_clear {
        clear_tiles(tiles);
        controls.should_clear = false;
    }
}

fn build_map(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    let texture_handle = asset_server.load("tiles.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(MAP_WIDTH / CHUNK_SIZE, MAP_HEIGHT / CHUNK_SIZE),
            ChunkSize(CHUNK_SIZE, CHUNK_SIZE),
            TileSize(TILE_SIZE, TILE_SIZE),
            TextureSize(TILE_SIZE * 6., TILE_SIZE),
        ),
        0u16,
        0u16,
    );

    layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
        // True here refers to tile visibility.
        *tile_data = Some(TileBundle::default());
        // Tile entity might not exist at this point so you'll need to create it.
        if tile_entity.is_none() {
            *tile_entity = Some(commands.spawn().id());
        }
        commands
            .entity(tile_entity.unwrap())
            .insert(LifeTile::new());
    });

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle.clone());

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-TILE_SIZE * MAP_WIDTH as f32 * 0.5, -TILE_SIZE * MAP_HEIGHT as f32 * 0.5, 0.0))
        .insert(GlobalTransform::default());
}

fn randomize_tiles(mut tiles: Query<(&mut Tile, &mut LifeTile)>) {
    let mut rng = rand::thread_rng();
    for (mut tile, mut life) in tiles.iter_mut() {
        let is_alive = rng.gen::<f64>() > 0.5;
        tile.texture_index = if is_alive { 1 } else { 0 };
        life.is_alive = is_alive;
        life.will_be_alive = is_alive;
    }
}

fn clear_tiles(mut tiles: Query<(&mut Tile, &mut LifeTile)>) {
    for (mut tile, mut life) in tiles.iter_mut() {
        let is_alive = false;
        tile.texture_index = if is_alive { 1 } else { 0 };
        life.is_alive = is_alive;
        life.will_be_alive = is_alive;
    }
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}


fn simulate_tiles(controls: Res<SimControls>, time: Res<Time>, mut timer: ResMut<UpdateTimer>, tile_query: Query<(Entity, &TilePos)>, mut life_query: Query<&mut LifeTile>, mut map_query: MapQuery) {
    if controls.is_paused {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        for (entity, pos) in tile_query.iter() {
            let alive_neighbors = map_query
                .get_tile_neighbors(*pos, 0u16, 0u16)
                .iter()
                .filter(|&&neighboring_result| {
                    if neighboring_result.is_ok() {
                        let life_component: &LifeTile = life_query
                            .get_component::<LifeTile>(neighboring_result.unwrap())
                            .unwrap();
                        life_component.is_alive
                    }
                    else
                    {
                        false
                    }
                })
                .count();
            
            let will_be_alive: bool;
            let mut my_life = life_query
            .get_component_mut::<LifeTile>(entity)
            .unwrap();

            if my_life.is_alive {
                will_be_alive = alive_neighbors >= 2 && alive_neighbors <= 3
            } else {
                will_be_alive = alive_neighbors == 3;
            }

            my_life.will_be_alive = will_be_alive
        }
    }
}

fn change_tiles_to_new_states(mut map_query: MapQuery, mut tile_query : Query<(&mut Tile, &mut TilePos, &mut LifeTile)>) {
    for (mut tile, tilepos, mut life) in tile_query.iter_mut() {
        life.is_alive = life.will_be_alive;

        if life.is_alive {
            tile.texture_index = 1;
        }
        else {
            tile.texture_index = 0;
        }

        let tpos = TilePos(tilepos.0, tilepos.1);

        map_query.notify_chunk_for_tile(tpos, 0u16, 0u16);
    }
}

