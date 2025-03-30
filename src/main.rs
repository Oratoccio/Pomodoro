use eframe::egui;
use egui::{CentralPanel, Label, RichText, Visuals};
use std::time::{Duration, Instant};
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, model::{PlayContextId, PlaylistId}};
use open;

struct PomodoroApp {
    duration: Duration,
    end_time: Option<Instant>,
    is_running: bool,
    spotify: AuthCodeSpotify,
    playlist_uri: String,
    playback_started: bool,
}

impl Default for PomodoroApp {
    fn default() -> Self {
        let creds = Credentials::new("SPOTIFY_CLIENT_ID", "SPOTIFY_CLIENT_SECRET");
        let oauth = OAuth {
            redirect_uri: "http://localhost:8888/callback".to_string(),
            scopes: scopes!("user-modify-playback-state"),
            ..Default::default()
        };
        let spotify = AuthCodeSpotify::new(creds, oauth);
        Self {
            duration: Duration::from_secs(25 * 60), // 25 minutes
            end_time: None,
            is_running: false,
            spotify,
            playlist_uri: "spotify:playlist:YOUR_PLAYLIST_ID".to_string(),
            playback_started: false,
        }
    }
}

impl eframe::App for PomodoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::light()); // thème clair type Sakura

        CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("🌸 Sakura Pomodoro 🌸").size(30.0));
            ui.separator();

            if self.is_running {
                let remaining = self.end_time.unwrap().saturating_duration_since(Instant::now());

                if !self.playback_started {
                    let spotify = self.spotify.clone();
                    let playlist_uri = self.playlist_uri.clone();
                    tokio::spawn(async move {
                        let playlist_id = PlaylistId::from_uri(&playlist_uri).unwrap();
                        spotify
                            .start_context_playback(
                                PlayContextId::Playlist(playlist_id),
                                None,
                                None,
                                None,
                            )
                            .await
                            .unwrap();
                    });
                    self.playback_started = true;
                }

                if remaining.is_zero() {
                    ui.label(RichText::new("⏰ Pause ! 🎶").size(25.0));
                } else {
                    ui.add(Label::new(
                        RichText::new(format!(
                            "Temps restant: {:02}:{:02}",
                            remaining.as_secs() / 60,
                            remaining.as_secs() % 60
                        ))
                            .size(25.0),
                    ));
                }

                if ui.button("Réinitialiser").clicked() {
                    self.is_running = false;
                    self.end_time = None;
                    self.playback_started = false;
                }
            } else {
                if ui.button("Démarrer le Pomodoro 🍅").clicked() {
                    self.end_time = Some(Instant::now() + self.duration);
                    self.is_running = true;
                    self.playback_started = false;
                }
            }
        });

        ctx.request_repaint();
    }
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sakura Pomodoro",
        native_options,
        Box::new(|_cc| Box::<PomodoroApp>::default()),
    )
}
