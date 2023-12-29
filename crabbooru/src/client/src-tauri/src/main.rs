#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod api;
pub mod viewer;
pub use api::*;
pub use viewer::*; 
// use session_types::*;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
pub use viewer::*;
// type Gallery<I: Api + Any> = Vec<I::Image>;
// type Client<I: Api + Any> = Send<u32, Recv<Gallery<I>, Eps>>;
// fn query_id<I: Api + Any>(c: Chan<(), Client<I>>){
//     let id = c.send(42);
//     let mut c = {
//         let (c, id) = c.recv();
//     }
// }

#[tauri::command]
fn get_sources(profile: String, sites: Vec<Site>) -> Vec<Site> {
    let mut sources = SourcesWindow::new(profile, sites, 
                                         0);
    sources.selected
}

fn main() {

    let sites: Vec<Site> = Vec::new();
    let mut profile = "default".to_string();
    // let mut sources =
    //     viewer::SourcesWindow::new(profile, sites, egui_winit::winit::window::WindowId::from(0));
    // let sources_window =
    //     viewer::SourcesWindow::sources_window(&mut sources, &egui::Context::default());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_sources
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
