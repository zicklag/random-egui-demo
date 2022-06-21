use egui::{collapsing_header::CollapsingState, Frame, Id, Layout, TextEdit, Ui};

pub struct State {
    tab: Tab,
    scene_filter: String,
    root_node: SceneNode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tab: Default::default(),
            scene_filter: Default::default(),
            root_node: SceneNode {
                label: "Root".into(),
                children: vec![
                    SceneNode {
                        label: "Player".into(),
                        children: vec![],
                        visible: true,
                    },
                    SceneNode {
                        label: "Obstacle".into(),
                        children: vec![],
                        visible: true,
                    },
                    SceneNode {
                        label: "Aliens".into(),
                        children: vec![
                            SceneNode {
                                label: "Alien 1".into(),
                                children: vec![],
                                visible: true,
                            },
                            SceneNode {
                                label: "Alien 2".into(),
                                children: vec![],
                                visible: false,
                            },
                        ],
                        visible: true,
                    },
                ],
                visible: true,
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum Tab {
    Scene,
    Import,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Scene
    }
}

pub(crate) fn gui(state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::SidePanel::left("sidebar").show(ctx, |ui| {
        ui.set_width(ui.available_width());

        ui.add_space(ui.spacing().item_spacing.y);

        ui.horizontal(|ui| {
            if ui
                .selectable_label(state.tab == Tab::Scene, "Scene")
                .clicked()
            {
                state.tab = Tab::Scene
            }

            if ui
                .selectable_label(state.tab == Tab::Import, "Import")
                .clicked()
            {
                state.tab = Tab::Import
            }
        });

        ui.separator();

        match state.tab {
            Tab::Scene => scene_tab_gui(state, ui),
            Tab::Import => {
                ui.label("Import tab TODO");
            }
        }
    });
}

fn scene_tab_gui(state: &mut State, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.button("➕");

        ui.button("⛓");

        // Now we start laying things out from the right side.
        // We have to do this because we want the filter textbox to take up the rest of the available width,
        // but because egui doesn't know how much width is available until it places all the buttons first,
        // we have to place the button on the far right first and then place the text box last,
        // so the text box knows how much room it has.
        ui.with_layout(Layout::right_to_left(), |ui| {
            ui.button("📰");

            ui.add(
                TextEdit::singleline(&mut state.scene_filter)
                    .desired_width(f32::INFINITY)
                    .hint_text("Filter Nodes"),
            );
        });
    });

    ui.separator();

    Frame::dark_canvas(ui.style()).show(ui, |ui| {
        scene_node_gui(ui, &mut state.root_node, None);
    });
}

struct SceneNode {
    label: String,
    children: Vec<SceneNode>,
    visible: bool,
}

fn scene_node_gui(ui: &mut Ui, node: &mut SceneNode, parent_id: Option<Id>) {
    let id = if let Some(parent_id) = parent_id {
        parent_id.with(&node.label)
    } else {
        Id::new(&node.label)
    };

    if node.children.is_empty() {
        ui.horizontal(|ui| {
            scene_item_bar_gui(ui, &node.label, &mut node.visible);
        });
    } else {
        CollapsingState::load_with_default_open(ui.ctx(), id, true)
            .show_header(ui, |ui| {
                scene_item_bar_gui(ui, &node.label, &mut node.visible);
            })
            .body(|ui| {
                for child in &mut node.children {
                    scene_node_gui(ui, child, Some(id));
                }
            });
    }
}

fn scene_item_bar_gui(ui: &mut Ui, label: &str, visible: &mut bool) {
    ui.with_layout(Layout::right_to_left(), |ui| {
        ui.checkbox(visible, "👁").on_hover_text("Visible");

        ui.with_layout(Layout::left_to_right(), |ui| {
            ui.label(label);
        });
    });
}
