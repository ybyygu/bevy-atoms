// [[file:../../bevy.note::83513dad][83513dad]]
use bevy_egui::egui;
use egui::Ui;
use gut::prelude::*;
// 83513dad ends here

// [[file:../../bevy.note::ba17983a][ba17983a]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct State {
    templates: Vec<String>,
    current_template: String,
    rendered_input: String,
    input_template: String,
}
// ba17983a ends here
