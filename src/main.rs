use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use std::collections::VecDeque;

fn main() {
  App::build()
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(WindowDescriptor {
        title: "Biology!".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .init_resource::<DebugConsole>()
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_system(debug_gui.system())
    .add_system(exit_on_esc_system.system())
    .run();
}

#[derive(Default)]
struct DebugConsole {
  console_open: bool,
  cmd_buffer: String,
  cmd_history: VecDeque<String>,
  output: VecDeque<String>,
}

impl DebugConsole {
  pub fn invoke(&mut self)  {
    let cur_cmd = std::mem::replace(&mut self.cmd_buffer, String::default());
    self.output.push_front(format!("Command {} not found", cur_cmd));
    self.cmd_history.push_front(cur_cmd);
  }
}

fn dark_light_mode_switch(ui: &mut egui::Ui) {
  let style: egui::Style = (*ui.ctx().style()).clone();
  let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
  if let Some(visuals) = new_visuals {
      ui.ctx().set_visuals(visuals);
  }
}
fn debug_gui(egui_context: ResMut<EguiContext>,
  mut dbg_state: ResMut<DebugConsole>) {
  let wants_kbinput = egui_context.ctx().wants_keyboard_input();
  let toggle_console = egui_context.ctx().input().key_pressed(egui::Key::Tab);

  if toggle_console {
    dbg_state.console_open = !dbg_state.console_open
  }

  egui::TopBottomPanel::top("Menu").show(egui_context.ctx(), |ui| {
    ui.horizontal_wrapped(|ui| {
      dark_light_mode_switch(ui);

      ui.checkbox(&mut dbg_state.console_open, "💻 Console");
      ui.separator();
      egui::warn_if_debug_build(ui);
    });
  });

  if dbg_state.console_open {
    egui::SidePanel::left("Console").show(egui_context.ctx(), |ui| {
      let cmd_te = ui.add(egui::TextEdit::singleline(&mut dbg_state.cmd_buffer).hint_text("Enter command").desired_width(ui.available_width()));
      if wants_kbinput && cmd_te.has_focus() && ui.input().key_pressed(egui::Key::Enter) {
        dbg_state.invoke();
      }
      else if toggle_console && dbg_state.console_open {
        cmd_te.request_focus();
      }
      ui.separator();
      egui::ScrollArea::auto_sized().show(ui, |ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);

        for entry in &dbg_state.output {
          ui.label(entry);
        }
      });
    });
  }
}