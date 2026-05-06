use crate::plugin::params::{ReverbParamsLocal, ReverbParamsShared};
use crate::plugin::shared::ReverbShared;
use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use clack_extensions::gui::*;
use egui_baseview::{
    EguiWindow, GraphicsConfig, Queue,
    egui::{self, Color32, Context, Label, Pos2, Rect, RichText, Ui},
};
use egui_knob::{Knob, KnobStyle};
use std::sync::Arc;

static KNOB_X: f32 = 50.0;
static KNOB_Y: f32 = 60.0;
static KNOB_SIZE: f32 = 85.0;
static KNOB_DX: f32 = 30.0;
static KNOB_LABEL_DY: f32 = 10.0;
static KNOB_LABEL_SIZE: f32 = 20.0;
static LABEL_FONT_SIZE: f32 = 17.0;

struct AppState {
    shared_params: Arc<ReverbParamsShared>,
    local_params: ReverbParamsLocal,
}

impl AppState {
    pub fn new(shared_params: &Arc<ReverbParamsShared>) -> Self {
        Self {
            local_params: ReverbParamsLocal::new(&shared_params),
            shared_params: Arc::clone(shared_params),
        }
    }
}

pub struct ReverbGui {
    handle: WindowHandle,
    egui_context: Context,
}

impl ReverbGui {
    /// Creates a new GUI window, and embeds it into the given `parent`.
    pub fn new(parent: Window<'_>, state: &ReverbShared) -> Self {
        let settings = WindowOpenOptions {
            title: "Reverb Csd".to_string(),
            size: Size::new(400.0, 200.0),
            scale: WindowScalePolicy::SystemScaleFactor,
            gl_config: Some(Default::default()),
        };

        let (tx, rx) = std::sync::mpsc::channel();

        let handle = EguiWindow::open_parented(
            &parent,
            settings,
            GraphicsConfig::default(),
            AppState::new(&state.params),
            move |egui_ctx: &Context, _queue: &mut Queue, _state: &mut AppState| {
                tx.send(egui_ctx.clone()).unwrap()
            },
            |egui_ctx: &Context, _queue: &mut Queue, state: &mut AppState| {
                state.local_params.fetch_updates(&state.shared_params);

                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Reverb Csd").color(Color32::WHITE));
                    });
                    let mut feedback_value = state.local_params.get_feedback();
                    let mut cut_off_value = state.local_params.get_cut_off();
                    let mut mix_value = state.local_params.get_mix();

                    let rect_feedback = Rect {
                        min: Pos2::new(KNOB_X, KNOB_Y),
                        max: Pos2::new(KNOB_X + KNOB_SIZE, KNOB_Y + KNOB_SIZE),
                    };

                    let label_y = KNOB_Y + KNOB_SIZE + KNOB_LABEL_DY;
                    let rect_feedback_label = Rect {
                        min: Pos2::new(KNOB_X, label_y),
                        max: Pos2::new(KNOB_X + KNOB_SIZE, label_y + KNOB_LABEL_SIZE),
                    };

                    let x_cut_off = KNOB_X + KNOB_SIZE + KNOB_DX;
                    let rect_cut_off = Rect {
                        min: Pos2::new(x_cut_off, KNOB_Y),
                        max: Pos2::new(x_cut_off + KNOB_SIZE, KNOB_Y + KNOB_SIZE),
                    };
                    let rect_cut_off_label = Rect {
                        min: Pos2::new(x_cut_off, label_y),
                        max: Pos2::new(x_cut_off + KNOB_SIZE, label_y + KNOB_LABEL_SIZE),
                    };

                    let x_mix = x_cut_off + KNOB_SIZE + KNOB_DX;
                    let rect_mix = Rect {
                        min: Pos2::new(x_mix, KNOB_Y),
                        max: Pos2::new(x_mix + KNOB_SIZE, KNOB_Y + KNOB_SIZE),
                    };
                    let rect_mix_label = Rect {
                        min: Pos2::new(x_mix, label_y),
                        max: Pos2::new(x_mix + KNOB_SIZE, label_y + KNOB_LABEL_SIZE),
                    };

                    let knob_feedback = get_knob(&mut feedback_value);
                    let feedback_ui = ui.put(rect_feedback, knob_feedback);
                    if feedback_ui.changed() {
                        state.local_params.set_feedback(feedback_value);
                        state
                            .local_params
                            .push_feedback_updates(&state.shared_params);
                    }
                    set_label(ui, rect_feedback_label, "size");
                    let knob_cut_off = get_knob(&mut cut_off_value);
                    let cut_off_ui = ui.put(rect_cut_off, knob_cut_off);
                    if cut_off_ui.changed() {
                        state.local_params.set_cut_off(cut_off_value);
                        state
                            .local_params
                            .push_cut_off_updates(&state.shared_params);
                    }
                    set_label(ui, rect_cut_off_label, "filter");
                    let knob_mix = get_knob(&mut mix_value);
                    let mix_ui = ui.put(rect_mix, knob_mix);
                    if mix_ui.changed() {
                        println!("Value: {:?}", mix_value);
                        state.local_params.set_mix(mix_value);
                        state.local_params.push_mix_updates(&state.shared_params);
                    }
                    set_label(ui, rect_mix_label, "mix");
                });
            },
        );

        let egui_context = rx.recv().unwrap();

        Self {
            handle,
            egui_context,
        }
    }

    /// Requests the UI to repaint itself, e.g. in response to events or parameter changes
    pub fn request_repaint(&self) {
        self.egui_context.request_repaint();
    }
}

impl Drop for ReverbGui {
    fn drop(&mut self) {
        self.handle.close();
    }
}

fn set_label(ui: &mut Ui, rect: Rect, name: &str) {
    ui.put(
        rect,
        Label::new(
            RichText::new(name)
                .size(LABEL_FONT_SIZE)
                .color(Color32::WHITE),
        ),
    );
}

fn get_knob<'a>(value: &'a mut f32) -> Knob<'a> {
    Knob::new(value, 0.0, 1.0, KnobStyle::Wiper)
        .with_size(KNOB_SIZE * 0.9)
        .with_font_size(14.0)
        .with_colors(Color32::BLUE, Color32::LIGHT_BLUE, Color32::WHITE)
        .with_sweep_range(0.1, 0.8)
        .with_stroke_width(4.0)
}
