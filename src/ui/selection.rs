// [[file:../../bevy.note::c828433d][c828433d]]
use bevy::ecs::system::Query;
use bevy_egui::egui;
use bevy_mod_picking::prelude::PickSelection;
use egui::Context;
use egui::Ui;
use gut::prelude::*;

use std::collections::HashMap;
// c828433d ends here

// [[file:../../bevy.note::34be17e4][34be17e4]]
#[derive(Debug, Deserialize, Serialize)]
pub struct State {
    atom_selection_window_open: bool,
    atom_selection_input: String,
    /// The selection input
    selection: String,
    /// The name of selection that can be saved.
    selection_name: String,
    /// The saved selections with associated names
    named_selections: HashMap<String, String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selection: String::new(),
            atom_selection_input: String::new(),
            atom_selection_window_open: false,
            selection_name: "selected".to_owned(),
            named_selections: HashMap::new(),
        }
    }
}
// 34be17e4 ends here

// [[file:../../bevy.note::0cf53cc2][0cf53cc2]]
/// Show menu when user right click selection input area
fn show_context_menu(ui: &mut Ui, state: &mut State, selection_query: &Query<(&crate::base::AtomIndex, &mut PickSelection)>) {
    if ui
        .button("Read")
        .on_hover_text("Read selection from active molecule view")
        .clicked()
    {
        let selected_atoms = crate::molecule::get_selected_atoms(&selection_query);
        if !selected_atoms.is_empty() {
            if let Ok(s) = gut::utils::abbreviate_numbers_human_readable(&selected_atoms) {
                state.selection = s.to_owned();
            }
        }
    }
    // copy selection to clipboard
    if ui.button("Copy").on_hover_text("copy selection").clicked() {
        ui.output_mut(|o| o.copied_text = state.selection.clone());
    }
    ui.horizontal(|ui| {
        if ui.button("Save").on_hover_text("save selection for later uses").clicked() {
            state
                .named_selections
                .insert(state.selection_name.clone(), state.selection.clone());
        }
        ui.add(egui::TextEdit::singleline(&mut state.selection_name));
    });

    if !state.named_selections.is_empty() {
        let mut to_remove = None;
        ui.menu_button("Saved", |ui| {
            for (k, v) in &state.named_selections {
                ui.horizontal(|ui| {
                    if ui.button("❌").on_hover_text("remove saved selection").clicked() {
                        to_remove = Some(k.to_string());
                    }
                    if ui.button(k).on_hover_text("Load saved selection").clicked() {
                        state.selection = v.to_owned();
                    }
                });
            }
            if let Some(k) = to_remove {
                state.named_selections.remove(&k);
            }
        });
    }
}
// 0cf53cc2 ends here

// [[file:../../bevy.note::3a54aa74][3a54aa74]]
impl State {
    /// Show ui for atom selection
    pub fn show(&mut self, ui: &mut Ui, selection_query: &mut Query<(&crate::base::AtomIndex, &mut PickSelection)>) {
        ui.label("Atom selection");
        ui.horizontal(|ui| {
            let button = ui
                .button("select ")
                .on_hover_text("select atoms in active molecule using user input");
            ui.add(egui::TextEdit::singleline(&mut self.selection).clip_text(false))
                .context_menu(|ui| show_context_menu(ui, self, &selection_query))
                .on_hover_text("Select atoms using a human readable string. For example: 1,5,8-10,12");
            if button.clicked() {
                if let Ok(selected_atoms) = gut::utils::parse_numbers_human_readable(&self.selection) {
                    for (ai, mut selection) in selection_query.iter_mut() {
                        let selected = selected_atoms.contains(&ai.0);
                        selection.is_selected = selected;
                    }
                    // update selection with normalized text
                    if let Ok(s) = gut::utils::abbreviate_numbers_human_readable(&selected_atoms) {
                        self.selection = s;
                    }
                }
            }
        });
    }
}
// 3a54aa74 ends here
