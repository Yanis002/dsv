use anyhow::Result;
use dsv_core::{gdb::client::GdbClient, state::State};
use eframe::egui::{self};

use crate::{
    client::{Client, Command},
    config::Config,
    views::{read_object, read_pointer_object},
};

const DATA_027E09A4_ADDRESS: u32 = 0x027e09a4;
const DATA_027E09A8_ADDRESS: u32 = 0x027e09a8;
const DATA_027E09B8_ADDRESS: u32 = 0x027e09b8;
const DATA_027E09BC_ADDRESS: u32 = 0x027e09bc;
const DATA_027E0CD8_ADDRESS: u32 = 0x027e0cd8;
const DATA_027E0CE0_ADDRESS: u32 = 0x027e0ce0;
const ACTOR_MANAGER_ADDRESS: u32 = 0x027e0ce4;
const DATA_027E0CE8_ADDRESS: u32 = 0x027e0ce8;
const DATA_027E0CEC_ADDRESS: u32 = 0x027e0cec;
const OVERLAY_MANAGER_ADDRESS: u32 = 0x02043e50;
const DATA_OV000_020B34C4_ADDRESS: u32 = 0x020b34c4;
const DATA_OV000_020B504C_ADDRESS: u32 = 0x020b504c;
const DATA_OV000_020B51B8_ADDRESS: u32 = 0x020b51b8;
const DATA_OV000_020B51C0_ADDRESS: u32 = 0x020b51c0;
const DATA_OV000_020B539C_ADDRESS: u32 = 0x020b539c;
const TREASURE_MANAGER_ADDRESS: u32 = 0x020b6510;
const DATA_OV024_020D8698_ADDRESS: u32 = 0x020d8698;

pub struct View {
    client: Client,
    windows: Windows,
}

struct Windows {
    actor_manager: ActorManagerWindow,
    basic_windows: Vec<BasicWindow>,
}

impl View {
    pub fn new(gdb_client: GdbClient) -> Self {
        View { client: Client::new(gdb_client), windows: Windows::default() }
    }
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            actor_manager: ActorManagerWindow::default(),
            basic_windows: vec![
                BasicWindow {
                    open: false,
                    title: "data_027e09a4",
                    type_name: "UnkStruct_027e09a4",
                    address: DATA_027E09A4_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e09a8",
                    type_name: "UnkStruct_027e09a8",
                    address: DATA_027E09A8_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e09b8",
                    type_name: "UnkStruct_027e09b8",
                    address: DATA_027E09B8_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e09bc",
                    type_name: "UnkStruct_027e09bc",
                    address: DATA_027E09BC_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e0cd8",
                    type_name: "UnkStruct_027e0cd8",
                    address: DATA_027E0CD8_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e0ce0",
                    type_name: "UnkStruct_027e0ce0",
                    address: DATA_027E0CE0_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e0ce8",
                    type_name: "UnkStruct_027e0ce8",
                    address: DATA_027E0CE8_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_027e0cec",
                    type_name: "UnkStruct_027e0cec",
                    address: DATA_027E0CEC_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "Overlay Manager",
                    type_name: "OverlayManager",
                    address: OVERLAY_MANAGER_ADDRESS,
                    pointer: false,
                },
                BasicWindow {
                    open: false,
                    title: "data_ov000_020b34c4",
                    type_name: "UnkStruct_ov000_020b34c4",
                    address: DATA_OV000_020B34C4_ADDRESS,
                    pointer: false,
                },
                BasicWindow {
                    open: false,
                    title: "data_ov000_020b504c",
                    type_name: "UnkStruct_ov000_02067bc4",
                    address: DATA_OV000_020B504C_ADDRESS,
                    pointer: false,
                },
                BasicWindow {
                    open: false,
                    title: "data_ov000_020b51b8",
                    type_name: "UnkStruct_ov000_020b51b8",
                    address: DATA_OV000_020B51B8_ADDRESS,
                    pointer: false,
                },
                BasicWindow {
                    open: false,
                    title: "data_ov000_020b51c0",
                    type_name: "UnkStruct_ov000_020b51c0",
                    address: DATA_OV000_020B51C0_ADDRESS,
                    pointer: false,
                },
                BasicWindow {
                    open: false,
                    title: "data_ov000_020b539c",
                    type_name: "UnkStruct_ov000_020b539c",
                    address: DATA_OV000_020B539C_ADDRESS,
                    pointer: false,
                },
                BasicWindow {
                    open: false,
                    title: "Treasure Manager",
                    type_name: "TreasureManager",
                    address: TREASURE_MANAGER_ADDRESS,
                    pointer: true,
                },
                BasicWindow {
                    open: false,
                    title: "data_ov024_020d8698",
                    type_name: "UnkStruct_020d8698",
                    address: DATA_OV024_020D8698_ADDRESS,
                    pointer: true,
                }
            ],
        }
    }
}

impl super::View for View {
    fn render_side_panel(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        _types: &type_crawler::Types,
        _config: &mut Config,
    ) -> Result<()> {
        egui::ScrollArea::vertical().max_width(100.0).show(ui, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.toggle_value(&mut self.windows.actor_manager.open, "Actor manager");
                    for window in &mut self.windows.basic_windows {
                        ui.toggle_value(&mut window.open, window.title);
                    }
                },
            );
        });
        Ok(())
    }

    fn render_central_panel(
        &mut self,
        ctx: &egui::Context,
        _ui: &mut egui::Ui,
        types: &type_crawler::Types,
        config: &mut Config,
    ) -> Result<()> {
        let mut state = self.client.state.lock().unwrap();

        let st_config = config.games.entry("st").or_insert_with(|| toml::Table::new().into());
        let st_config = st_config
            .as_table_mut()
            .ok_or_else(|| anyhow::anyhow!("Failed to get 'st' config as a table"))?;

        self.windows.actor_manager.render(ctx, types, &mut state);

        for window in &mut self.windows.basic_windows {
            window.render(ctx, types, &mut state);
        }

        Ok(())
    }

    fn exit(&mut self) -> Result<()> {
        self.client.send_command(Command::Disconnect)?;
        self.client.join_update_thread();
        Ok(())
    }
}

#[derive(Default)]
struct ActorManagerWindow {
    open: bool,
}

impl ActorManagerWindow {
    fn render(&mut self, ctx: &egui::Context, types: &type_crawler::Types, state: &mut State) {
        let mut open = self.open;
        egui::Window::new("Actor manager").open(&mut open).resizable(true).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let instance = match read_pointer_object(
                    types,
                    state,
                    "ActorManager",
                    ACTOR_MANAGER_ADDRESS,
                ) {
                    Ok(data) => data,
                    Err(err) => {
                        ui.label(err);
                        return;
                    }
                };

                instance.as_data_widget(ui, types).render_compound(ui, types, state);
            });
        });
        self.open = open;
    }
}

#[derive(Default)]
struct BasicWindow {
    open: bool,
    title: &'static str,
    type_name: &'static str,
    address: u32,
    pointer: bool,
}

impl BasicWindow {
    fn render(&mut self, ctx: &egui::Context, types: &type_crawler::Types, state: &mut State) {
        let mut open = self.open;
        egui::Window::new(self.title).open(&mut open).resizable(true).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let object = if self.pointer {
                    read_pointer_object(types, state, self.type_name, self.address)
                } else {
                    read_object(types, state, self.type_name, self.address)
                };

                let instance = match object {
                    Ok(instance) => instance,
                    Err(err) => {
                        ui.label(err);
                        return;
                    }
                };
                instance.as_data_widget(ui, types).render_compound(ui, types, state);
            });
        });
        self.open = open;
    }
}
