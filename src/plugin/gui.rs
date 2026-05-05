use crate::plugin::params::ReverbParams;
use crate::plugin::shared::ReverbShared;
use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use clack_extensions::gui::*;
use clack_plugin::prelude::*;
use egui_baseview::{
    EguiWindow, GraphicsConfig, Queue,
    egui::{self, Color32, Context, Label, Pos2, Rect, RichText, Ui, WidgetText},
};
use egui_knob::{Knob, KnobStyle};
pub struct ReverbGui {
    handle: WindowHandle,
    egui_context: Context,
}

static KNOB_X: f32 = 50.0;
static KNOB_Y: f32 = 60.0;
static KNOB_SIZE: f32 = 85.0;
static KNOB_DX: f32 = 30.0;
static KNOB_LABEL_DY: f32 = 10.0;
static KNOB_LABEL_SIZE: f32 = 20.0;
static LABEL_FONT_SIZE: f32 = 17.0;

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

        let params = ReverbParams::default();
        params.set_feedback(state.params.get_feedback());
        params.set_cut_off(state.params.get_cut_off());
        params.set_mix(state.params.get_mix());

        let handle = EguiWindow::open_parented(
            &parent,
            settings,
            GraphicsConfig::default(),
            params,
            move |egui_ctx: &Context, _queue: &mut Queue, state: &mut ReverbParams| {
                tx.send(egui_ctx.clone()).unwrap()
            },
            |egui_ctx: &Context, _queue: &mut Queue, state: &mut ReverbParams| {
                // state.local_params.fetch_updates(&state.shared_params);

                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Reverb Csd").color(Color32::WHITE));
                    });
                    // state.local_params.get_volume();

                    // if slider.changed() {
                    // state.local_params.set_volume(value);
                    // state.local_params.push_updates(&state.shared_params);
                    // };

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

                    let mut mock_feedback = 0.0;
                    let knob_feedback = get_knob(&mut mock_feedback);
                    ui.put(rect_feedback, knob_feedback);
                    set_label(ui, rect_feedback_label, "size");
                    let mut mock_cut_off = 0.0;
                    let knob_cut_off = get_knob(&mut mock_cut_off);
                    ui.put(rect_cut_off, knob_cut_off);
                    set_label(ui, rect_cut_off_label, "filter");
                    let mut mock_mix = 0.0;
                    let knob_mix = get_knob(&mut mock_mix);
                    ui.put(rect_mix, knob_mix);
                    set_label(ui, rect_mix_label, "mix");
                    // state.local_params.has_gesture = slider.is_pointer_button_down_on();
                    // state.local_params.push_gesture(&state.shared_params);
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
        .with_colors(Color32::BLUE, Color32::WHITE, Color32::WHITE)
        .with_stroke_width(4.0)
}
