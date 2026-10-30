use super::Selected;
use eframe::egui;
use egui::widget_text::RichText;
use log::*;
use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};

pub struct App {
    selected: Rc<RefCell<Selected>>,
}

impl App {
    pub fn new(ctx: &egui::Context, selected: Rc<RefCell<Selected>>) -> Self {
        setup_custom_fonts(ctx);
        setup_light_theme(ctx);
        Self { selected }
    }

    pub fn start(file_name: &Path, selected: Rc<RefCell<Selected>>) {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            &format!("{} - Memory Palace", file_name.display()),
            options,
            Box::new(move |cc| Ok(Box::new(Self::new(&cc.egui_ctx, selected)))),
        )
        .unwrap();
        debug!("GUI quits.");
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut items = self.selected.borrow().items();
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    for (item, corr) in items.iter_mut() {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.checkbox(
                                corr,
                                RichText::new(item.question.clone())
                                    .text_style(egui::TextStyle::Body)
                                    .size(18.0),
                            );
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.label(
                                RichText::new(item.answer.clone())
                                    .text_style(egui::TextStyle::Body)
                                    .size(18.0),
                            );
                        });
                        ui.end_row();
                    }
                });
            });
        });
        for (i, (_, corr)) in items.iter().enumerate() {
            if *corr {
                self.selected.borrow_mut().set(i);
            } else {
                self.selected.borrow_mut().unset(i);
            }
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "cjk".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "/usr/local/share/fonts/lxgw-wenkai-v1.530/LXGWWenKai-Regular.ttf"
        ))),
    );
    if let Some(fs) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        fs.push("cjk".to_owned());
    } else {
        unreachable!()
    }
    ctx.set_fonts(fonts);
}

fn setup_light_theme(ctx: &egui::Context) {
    // Always use light mode (ignore the system theme), with a pure white
    // background and black text.
    ctx.set_theme(egui::Theme::Light);
    ctx.global_style_mut(|style| {
        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = egui::Color32::WHITE;
        visuals.window_fill = egui::Color32::WHITE;
        visuals.override_text_color = Some(egui::Color32::BLACK);
        style.visuals = visuals;
    });
}
