// [[file:../../bevy.note::9c9c603b][9c9c603b]]
use bevy_egui::egui;
use egui::Ui;
use gut::prelude::*;
// 9c9c603b ends here

// [[file:../../bevy.note::7ae276e4][7ae276e4]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(serialize = "UPPERCASE"))]
struct Settings {
    // general
    encut: usize,
    // non-spin polarised DFT
    ispin: usize,

    // electronic relaxation
    ismear: usize,
    // smearing value in eV
    sigma: f64,
    // max electronic SCF steps
    nelm: usize,
    // min electronic SCF steps
    nelmin: usize,
    ediff: f64,

    // useful ionic relaxation
    nsw: usize,
    ibrion: usize,
    isif: usize,
    ediffg: f64,

    // symmetry
    isym: isize,

    // output verbosity
    nwrite: usize,
    lwave: bool,
    lcharg: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            // general
            encut: 400,
            // non-spin polarised DFT
            ispin: 1,

            // electronic relaxation
            ismear: 0,
            // smearing value in eV
            sigma: 0.05,
            // max electronic SCF steps
            nelm: 60,
            // min electronic SCF steps
            nelmin: 5,
            ediff: 1E-08,

            // useful ionic relaxation
            nsw: 100,
            ibrion: 2,
            isif: 2,
            ediffg: -0.01,

            // symmetry
            isym: 0,

            // output verbosity
            nwrite: 2,
            lwave: false,
            lcharg: false,
        }
    }
}
// 7ae276e4 ends here

// [[file:../../bevy.note::d7d12e5e][d7d12e5e]]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct State {
    settings: Settings,
    current_template: String,
    rendered_input: String,
    input_template: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            current_template: "sp/INCAR.jinja".to_owned(),
            input_template: String::new(),
            rendered_input: String::new(),
        }
    }
}
// d7d12e5e ends here

// [[file:../../bevy.note::6785c6e2][6785c6e2]]
use std::collections::HashMap;
use std::sync::OnceLock;

// the templates loaded from files
static TEMPLATES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn render_template<S: Serialize>(template: &str, settings: S) -> Result<String> {
    use minijinja::{context, Environment};

    let mut env = Environment::new();
    env.add_template("hello", template)?;
    let tmpl = env.get_template("hello")?;

    let s = tmpl.render(settings)?;
    Ok(s)
}

impl State {
    fn templates() -> &'static HashMap<String, String> {
        let tpl_root_dir: &std::path::Path = "tests/files/vasp-templates".as_ref();
        TEMPLATES.get_or_init(|| {
            dbg!();
            let mut s = include_str!("../../tests/files/vasp-templates/sp/INCAR.jinja");
            let files = gchemol::io::find_files(".jinja", tpl_root_dir, true);
            let templates: HashMap<String, String> = files
                .map(|f| {
                    let tpl_key = f.strip_prefix(tpl_root_dir).unwrap().to_str().unwrap().to_owned();
                    let tpl_txt = gut::fs::read_file(f).unwrap();
                    (tpl_key, tpl_txt)
                })
                .collect();
            info!("Loaded {} templates from {:?}", templates.len(), tpl_root_dir);
            templates
        })
    }

    fn show_template_selection(&mut self, ui: &mut Ui) {
        let templates = Self::templates();

        ui.horizontal(|ui| {
            // clipboard button
            let tooltip = "Click to copy generated input";
            if ui.button("📋 Render & Copy").on_hover_text(tooltip).clicked() {
                self.rendered_input = render_template(&self.input_template, &self.settings).unwrap_or_default();
                match render_template(&self.input_template, &self.settings) {
                    Ok(s) => {
                        self.rendered_input = s;
                    }
                    Err(e) => {
                        self.rendered_input = format!("minijinja template render issue:\n{e:?}");
                    }
                }
                ui.output_mut(|o| o.copied_text = self.rendered_input.clone());
            }

            ui.label("Render template:");
            egui::ComboBox::from_id_source("orca-template")
                .width(200.0)
                .selected_text(&self.current_template)
                .show_ui(ui, |ui| {
                    for t in templates.keys() {
                        ui.selectable_value(&mut self.current_template, t.to_string(), t);
                    }
                });
            ui.hyperlink_to(
                "Syntax Reference",
                "https://docs.rs/minijinja/latest/minijinja/syntax/index.html",
            )
        });

        ui.separator();
        match self.current_template.as_str() {
            "sp/INCAR.jinja" => {
                let mut s = templates["sp/INCAR.jinja"].clone();
                selectable_text(ui, &mut s, "template");
                self.input_template = s.to_string();
            }
            "custom" => {
                editable_text(ui, &mut self.input_template, "template");
            }
            t => {
                let mut s = templates[t].clone();
                selectable_text(ui, &mut s, "template");
                self.input_template = s.to_string();
            }
        }

        selectable_text(ui, &mut self.rendered_input.as_str(), "rendered");
    }
}

fn editable_text(ui: &mut Ui, text: &mut String, label: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    });
}

// NOTE: read-only
fn selectable_text(ui: &mut Ui, mut text: &str, label: &str) {
    ui.collapsing(label, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .hint_text(label)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace.resolve(ui.style())),
            );
        });
    });
}
// 6785c6e2 ends here

// [[file:../../bevy.note::905e0f65][905e0f65]]
impl State {
    /// Show egui UI
    pub fn show(&mut self, ui: &mut Ui) {
        egui::Grid::new("vasp_grid_core").num_columns(2).show(ui, |ui| {
            ui.hyperlink_to("ENCUT", "https://www.vasp.at/wiki/index.php/ENCUT")
                .on_hover_text("specifies the cutoff energy for the plane-wave-basis set in eV");
            ui.add(egui::DragValue::new(&mut self.settings.encut).speed(10));
            ui.hyperlink_to("ISPIN", "https://www.vasp.at/wiki/index.php/ISPIN")
                .on_hover_text("ISPIN specifies spin polarization");
            ui.add(egui::DragValue::new(&mut self.settings.ispin).clamp_range(1..=2).speed(1));
            ui.end_row();
        });

        ui.collapsing("Electronic relaxation", |ui| {
            egui::Grid::new("vasp_grid_elec")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("EDIFF", "https://www.vasp.at/wiki/index.php/EDIFF")
                        .on_hover_text("EDIFF specifies the global break condition for the electronic SC-loop.");
                    ui.add(
                        egui::DragValue::new(&mut self.settings.ediff)
                            .custom_formatter(|n, _| format!("{n:-8.0E}"))
                            .speed(0),
                    );
                    ui.hyperlink_to("SIGMA", "https://www.vasp.at/wiki/index.php/SIGMA")
                        .on_hover_text("specifies the width of the smearing in eV");
                    ui.add(
                        egui::DragValue::new(&mut self.settings.sigma)
                            .clamp_range(0.0..=2.0)
                            .speed(0.1),
                    );
                    ui.end_row();
                    ui.hyperlink_to("NELM", "https://www.vasp.at/wiki/index.php/NELM")
                        .on_hover_text("sets the maximum number of electronic SC (self-consistency) steps.");
                    ui.add(egui::DragValue::new(&mut self.settings.nelm).speed(10));
                    ui.hyperlink_to("NELMIN", "https://www.vasp.at/wiki/index.php/NELMIN")
                        .on_hover_text("specifies the minimum number of electronic self-consistency steps. ");
                    ui.add(egui::DragValue::new(&mut self.settings.nelmin).speed(10));
                    ui.end_row();
                });
        });

        ui.collapsing("Ionic relaxation", |ui| {
            egui::Grid::new("vasp_grid_ionic")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("NSW", "https://www.vasp.at/wiki/index.php/NSW")
                        .on_hover_text("defines the maximum number of ionic steps");
                    ui.add(egui::DragValue::new(&mut self.settings.nsw).speed(1));
                    ui.hyperlink_to("EDIFFG", "https://www.vasp.at/wiki/index.php/EDIFFG")
                        .on_hover_text("defines the break condition for the ionic relaxation loop");
                    ui.add(egui::DragValue::new(&mut self.settings.ediffg).speed(0.01));
                    ui.hyperlink_to("ISYM", "https://www.vasp.at/wiki/index.php/ISYM")
                        .on_hover_text("determines the way VASP treats symmetry");
                    ui.add(egui::DragValue::new(&mut self.settings.isym).clamp_range(-1..=3).speed(1));
                });
        });

        ui.collapsing("Output control", |ui| {
            egui::Grid::new("vasp_grid_output")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.hyperlink_to("NWRITE", "https://www.vasp.at/wiki/index.php/NWRITE")
                        .on_hover_text("determines how much will be written to the file OUTCAR ('verbosity tag')");
                    ui.add(egui::DragValue::new(&mut self.settings.nwrite).clamp_range(0..=4).speed(1));
                    ui.hyperlink_to("LWAVE", "https://www.vasp.at/wiki/index.php/LWAVE")
                        .on_hover_text("determines whether the wavefunctions are written to the WAVECAR file");
                    ui.toggle_value(&mut self.settings.lwave, "write WAVECAR");
                    ui.hyperlink_to("LCHARG", "https://www.vasp.at/wiki/index.php/LCHARG")
                        .on_hover_text("determines whether the charge densities");
                    ui.toggle_value(&mut self.settings.lcharg, "write CHGCAR/CHG");
                });
        });
        ui.separator();
        self.show_template_selection(ui);
    }
}
// 905e0f65 ends here
