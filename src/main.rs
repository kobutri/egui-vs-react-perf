use eframe::egui;
use egui::{Color32, Rounding, Vec2};

mod frame_history;

#[derive(Default)]
struct DemoApp {
    count: i32,
    color: [f32; 3], // Changed from Color32 to [f32; 4] for color picker compatibility
    scroll_offset: Vec2,
    frame_history: crate::frame_history::FrameHistory,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Repainting the UI each frame. FPS: {:.1}",
                self.frame_history.fps()
            ));
            // Title
            ui.vertical_centered(|ui| {
                ui.heading("React Compiler Demo");
                ui.add_space(20.0);
            });

            // Main content container
            ui.horizontal(|ui| {
                // Color Picker Panel
                egui::Frame::none()
                    .stroke((1.0, Color32::WHITE.linear_multiply(0.2)))
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(384.0, 256.0)); // w-96 h-64
                        ui.vertical(|ui| {
                            ui.heading("Color Picker");
                            ui.add_space(8.0);

                            // Color picker with correct type
                            ui.color_edit_button_rgb(&mut self.color);
                            ui.add_space(8.0);
                            ui.label("Current value:");
                            ui.monospace(format!(
                                "#{:02X}{:02X}{:02X}",
                                (self.color[0] * 255.0) as u8,
                                (self.color[1] * 255.0) as u8,
                                (self.color[2] * 255.0) as u8
                            ));
                        });
                    });

                ui.add_space(32.0); // gap-8

                // Counter Panel
                egui::Frame::none()
                    .stroke((1.0, Color32::WHITE.linear_multiply(0.2)))
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(384.0, 256.0));
                        ui.vertical(|ui| {
                            ui.heading("Counter");
                            ui.add_space(8.0);

                            if ui.button("Increase count").clicked() {
                                self.count += 1;
                            }

                            ui.add_space(8.0);
                            ui.label("Current value:");
                            ui.monospace(format!("{}", self.count));
                        });
                    });

                ui.add_space(32.0);

                // Slow Component Panel
                egui::Frame::none()
                    .stroke((1.0, Color32::WHITE.linear_multiply(0.2)))
                    .show(ui, |ui| {
                        ui.set_min_size(Vec2::new(384.0, 256.0));
                        ui.vertical(|ui| {
                            ui.heading("A Slow Component");
                            ui.label("(This component renders 100,000 boxes)");

                            // Scrollable area for boxes
                            egui::ScrollArea::both()
                                .auto_shrink([false; 2])
                                .show(ui, |ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        for i in 0..100000 {
                                            let color = Color32::from_rgb(
                                                (i % 255) as u8,
                                                ((i * 2) % 255) as u8,
                                                ((i * 3) % 255) as u8,
                                            );
                                            ui.spacing_mut().item_spacing.x = 4.0;
                                            ui.spacing_mut().item_spacing.y = 4.0;

                                            // Fixed rect allocation
                                            let size = Vec2::new(8.0, 8.0);
                                            let (_, rect) = ui.allocate_space(size);
                                            ui.painter().rect_filled(
                                                rect,
                                                Rounding::from(0.0),
                                                color,
                                            );
                                        }
                                    });
                                });
                        });
                    });
            });
        });
    }
}

// Entry point
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        vsync: false,
        ..Default::default()
    };

    eframe::run_native(
        "React Demo in Egui",
        native_options,
        Box::new(|_cc| Ok(Box::<DemoApp>::default())),
    )
}


// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::<DemoApp>::default())),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}