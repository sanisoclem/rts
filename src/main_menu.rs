use bevy::prelude::*;
use crate::state::GameState;

#[derive(Default)]
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<ButtonMaterials>()
      .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup))
      .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(teardown))
      .add_system(button_system);
  }
}

struct ButtonMaterials {
  normal: Handle<ColorMaterial>,
  hovered: Handle<ColorMaterial>,
  pressed: Handle<ColorMaterial>,
}
impl FromWorld for ButtonMaterials {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
    ButtonMaterials {
      normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
      hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
      pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    }
  }
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  button_materials: Res<ButtonMaterials>,
) {
  // ui camera
  commands.spawn_bundle(UiCameraBundle::default());
  commands
    .spawn_bundle(ButtonBundle {
      style: Style {
        size: Size::new(Val::Px(500.0), Val::Px(65.0)),
        margin: Rect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      material: button_materials.normal.clone(),
      ..Default::default()
    })
    .with_children(|parent| {
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(
          "Start Game",
          TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
          },
          Default::default(),
        ),
        ..Default::default()
      });
    });
}

fn teardown(mut commands: Commands, entities: Query<Entity>) {
  for entity in entities.iter() {
      commands.entity(entity).despawn_recursive();
  }
}

fn button_system(
  button_materials: Res<ButtonMaterials>,
  mut state: ResMut<State<GameState>>,
  mut interaction_query: Query<
    (&Interaction, &mut Handle<ColorMaterial>, &Children),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut material, _children) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *material = button_materials.pressed.clone();
        state.set(GameState::Playing).unwrap();
      }
      Interaction::Hovered => {
        *material = button_materials.hovered.clone();
      }
      Interaction::None => {
        *material = button_materials.normal.clone();
      }
    }
  }
}
