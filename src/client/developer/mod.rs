use bevy::prelude::*;
// use bevy::{ecs::TypeInfo, prelude::*};
use bevy_egui::{egui::Window, EguiContext, EguiPlugin};
use std::time::Instant;

#[derive(Debug, Default)]
pub struct DeveloperPlugin {}
impl Plugin for DeveloperPlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        // app_builder.add_plugin(EguiPlugin);
        // app_builder.insert_resource(DebugFpsCounter {
        //     last_measurment_instant: Instant::now(),
        // });
        // app_builder.add_system(fps_counter.system());

        // app_builder
        //     .insert_resource(EditorState::default())
        //     .add_system(editor.exclusive_system());

        // if cfg!(feature = "steam") {
        //     app_builder.add_system(steam.system());
        // }
    }
}

#[derive(Debug)]
struct DebugFpsCounter {
    last_measurment_instant: Instant,
}

fn fps_counter(
    mut fps_counter_context: ResMut<DebugFpsCounter>,
    mut egui_context: ResMut<EguiContext>,
) {
    let frame_duration = fps_counter_context
        .last_measurment_instant
        .elapsed()
        .as_secs_f64()
        * 1000.0;

    fps_counter_context.last_measurment_instant = Instant::now();

    Window::new("Dev menu").show(egui_context.ctx(), |ui| {
        ui.label(format!("frame time: {:.2}ms", frame_duration));
    });
}

// #[derive(Debug, Default)]
// struct EditorState {
//     selected_entity: Option<(Entity, Vec<TypeInfo>)>,
// }

// fn editor(world: &mut World, resources: &mut Resources) {
//     let mut editor_state = resources.get_mut::<EditorState>().unwrap();
//     let mut resource_egui_context = resources.get_mut::<EguiContext>().unwrap();

//     let egui_context = &mut resource_egui_context.ctx;

//     Window::new("Archetypes").show(egui_context, |ui| {
//         for archetype in world.archetypes.iter() {
//             let archetype_info = archetype.types();

//             for entity in archetype.iter_entities() {
//                 if ui.button(format!("{}", entity.id())).clicked() {
//                     editor_state.selected_entity = Some((*entity, archetype_info.to_vec()));
//                 }
//             }
//         }
//     });

//     Window::new("Inspector").show(egui_context, |ui| {
//         if let Some((entity, types_info)) = &editor_state.selected_entity {
//             ui.label(format!("selected: {}", entity.id()));

//             for type_info in types_info.iter() {
//                 ui.label(format!("component: {}", type_info.type_name()));

//                 ui.separator();
//             }
//         }
//     });
// }

#[cfg(not(feature = "steam"))]
fn steam() {}

#[cfg(feature = "steam")]
fn steam(mut resource_egui_context: ResMut<EguiContext>) {
    let egui_context = &mut resource_egui_context.ctx;

    Window::new("Dev menu Steam").show(egui_context, |ui| {
        ui.label(format!("friends"));
    });
}
