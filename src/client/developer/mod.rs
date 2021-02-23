use bevy::prelude::*;
use bevy_egui::{egui::Window, EguiContext, EguiPlugin};
use std::time::Instant;

#[derive(Debug, Default)]
pub struct DeveloperPlugin {}
impl Plugin for DeveloperPlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        app_builder.add_plugin(EguiPlugin);
        app_builder.insert_resource(DebugFpsCounter {
            last_measurment_instant: Instant::now(),
        });
        app_builder.add_system(fps_counter.system());

        if cfg!(feature = "steam") {
            app_builder.add_system(steam.system());
        }
    }
}

#[derive(Debug)]
struct DebugFpsCounter {
    last_measurment_instant: Instant,
}

fn fps_counter(
    mut fps_counter_context: ResMut<DebugFpsCounter>,
    mut resource_egui_context: ResMut<EguiContext>,
) {
    let egui_context = &mut resource_egui_context.ctx;

    let frame_duration = fps_counter_context.last_measurment_instant.elapsed().as_secs_f64() * 1000.0;

    fps_counter_context.last_measurment_instant = Instant::now();

    Window::new("Dev menu").show(egui_context, |ui| {
        ui.label(format!("frame time: {:.2}ms", frame_duration));
    });
}

#[cfg(not(feature = "steam"))]
fn steam() {}

#[cfg(feature = "steam")]
fn steam(
    mut resource_egui_context: ResMut<EguiContext>,
) {
    let egui_context = &mut resource_egui_context.ctx;

    Window::new("Dev menu Steam").show(egui_context, |ui| {
        ui.label(format!("friends"));
    });
}

