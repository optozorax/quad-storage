use macroquad::prelude::*;
use quad_storage::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "quad-storage".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let storage = &mut STORAGE.lock().unwrap();
    let mut key_field = String::new();
    let mut key_field2 = String::new();
    let mut key_field3 = String::new();
    let mut getted = None;
    let mut value_field = String::new();
    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Test of storage").collapsible(false).show(egui_ctx, |ui| {
                ui.label("Try close this tab or browser after adding some data. It will persist between runs.");
                ui.separator();
                ui.label("Data:");
                for i in 0..storage.len() {
                    let key = storage.key(i).ok_or(i).unwrap();
                    let value = storage.get(&key).ok_or(i).unwrap();
                    ui.monospace(format!("• {:?} = {:?}", key, value));
                }
                ui.separator();
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut key_field);
                    ui.label("Key");    
                });
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut value_field);
                    ui.label("Value");    
                });
                if ui.button("Set").clicked() {
                    storage.set(&key_field, &value_field);
                }
                ui.separator();
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut key_field2);
                    ui.label("Key");    
                });
                if ui.button("Get").clicked() {
                    getted = storage.get(&key_field2);
                }
                ui.label(format!("Getted: {:?}", getted));
                ui.separator();
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut key_field3);
                    ui.label("Key");    
                });
                if ui.button("Remove key").clicked() {
                    storage.remove(&key_field3);
                }
                ui.separator();
                if ui.button("Clear").clicked() {
                    storage.clear();
                }
            });
        });

        egui_macroquad::draw();

        next_frame().await
    }
}
