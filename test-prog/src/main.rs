use eframe::egui;
use echovr::{Client, Session};
use tts::Tts;

struct App {
    client: Client,
    session: Option<Session>,
    fetch_error: Option<String>,
    tts_enabled: bool,
    tts: Option<Tts>,
    last_throw_speed: f32,
}

impl App {
    fn new() -> Self {
        let tts = Tts::default().ok();

        App {
            client: Client::new(),
            session: None,
            fetch_error: None,
            tts_enabled: false,
            tts,
            last_throw_speed: 0.0,
        }
    }

    fn poll(&mut self) {
        match self.client.fetch_session() {
            Ok(session) => {
                let new_speed = session.last_throw.total_speed;
                if self.tts_enabled && (new_speed - self.last_throw_speed).abs() > 0.01 {
                    if let Some(tts) = &mut self.tts {
                        let msg = format!("{:.1} meters per second", new_speed);
                        let _ = tts.speak(msg, true);
                    }
                    self.last_throw_speed = new_speed;
                }

                self.session = Some(session);
                self.fetch_error = None;
            }
            Err(e) => {
                self.fetch_error = Some(e.to_string());
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Echo VR");
            ui.separator();

            if let Some(err) = &self.fetch_error {
                ui.colored_label(egui::Color32::RED, format!("API error: {}", err));
                ui.label("Make sure Echo VR is running and the API is enabled.");
                return;
            }

            let Some(session) = &self.session else {
                ui.label("Waiting for session...");
                return;
            };

            // Score display.
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.vertical(|ui| {
                        ui.colored_label(egui::Color32::from_rgb(100, 160, 255), "BLUE");
                        ui.heading(session.blue_points.to_string());
                    });

                    ui.add_space(40.0);

                    ui.label("vs");

                    ui.add_space(40.0);

                    ui.vertical(|ui| {
                        ui.colored_label(egui::Color32::from_rgb(255, 160, 60), "ORANGE");
                        ui.heading(session.orange_points.to_string());
                    });
                });
            });

            ui.separator();

            // Game state info.
            ui.label(format!("Status: {:?}", session.game_status));
            ui.label(format!("Clock: {}", session.game_clock_display));

            ui.separator();

            ui.label(format!(
                "Last throw speed {:.2} m/s",
                session.last_throw.total_speed
            ));

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.tts_enabled, "Announce throw speed via TTS");
                if self.tts.is_none() {
                    ui.colored_label(egui::Color32::YELLOW, "(TTS unavailable on this system)");
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Echo VR API")
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Echo VR API",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}