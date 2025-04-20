use super::Selected;
use eframe::egui;
use egui::widget_text::RichText;
use log::*;
use std::{cell::RefCell, rc::Rc, sync::Arc};

pub struct App {
    selected: Rc<RefCell<Selected>>,
}

impl App {
    pub fn new(ctx: &egui::Context, selected: Rc<RefCell<Selected>>) -> Self {
        setup_custom_fonts(ctx);
        Self { selected }
    }

    pub fn start(file_name: &str, selected: Rc<RefCell<Selected>>) {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            &format!("{} - Memory Palace", file_name),
            options,
            Box::new(move |cc| Ok(Box::new(Self::new(&cc.egui_ctx, selected)))),
        )
        .unwrap();
        debug!("GUI quits.");
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut items = self.selected.borrow().items();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    for (item, corr) in items.iter_mut() {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.checkbox(
                                corr,
                                RichText::new(item.question.clone()).monospace().size(16.0),
                            );
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.label(RichText::new(item.answer.clone()).monospace().size(16.0));
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
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
        ))),
    );
    if let Some(fs) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        fs.push("cjk".to_owned());
    }
    ctx.set_fonts(fonts);
}
