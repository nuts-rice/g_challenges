#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
pub mod api;
pub mod viewer;
pub use api::*;
use tracing::info;

pub use viewer::*;
// use session_types::*;
// extern crate model;

// type Gallery<I: Api + Any> = Vec<I::Image>;
// type Client<I: Api + Any> = Send<u32, Recv<Gallery<I>, Eps>>;
// fn query_id<I: Api + Any>(c: Chan<(), Client<I>>){
//     let id = c.send(42);
//     let mut c = {
//         let (c, id) = c.recv();
//     }

// }
struct Args {
    profile: String,
    sites: Vec<Site>,
}



#[tauri::command]
fn get_sources(profile: String, sites: Vec<Site>) -> Vec<Site> {
    let sources = SourcesWindow::new(profile, sites, 0);
    info!("sources: {:?}", sources);
    sources.selected
}

#[tauri::command]
fn fetch_profile(profile: String) {
    println!("profile: {}", profile);
    todo!()
}
#[tauri::command]
async fn search_image(tags: Vec<String>,) {

let builder = ApiBuilder::<TestbooruClient>::new()
            .tag(tags.join(" "));
let _api = builder.build().get().await;

            
}
#[tauri::command]
fn main_window() {
    todo!()
}

fn main() {
    tracing_subscriber::fmt().init();
    let _sites: Vec<Site> = Vec::new();
    let _profile = "default".to_string();

    // tracing_subscriber::fmt::init();
    // let mut sources =
    //     viewer::SourcesWindow::new(profile, sites, egui_winit::winit::window::WindowId::from(0));
    // let sources_window =
    //     viewer::SourcesWindow::sources_window(&mut sources, &egui::Context::default());
    let sources_config = CustomMenuItem::new("Config Sources".to_string(), "Sources Config");
    let sources = Submenu::new("Sources", Menu::new().add_item(sources_config));
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
        .add_submenu(sources);
        
    tauri::Builder::default()
        .setup(|app| {
                tauri::WindowBuilder::new(app, "main-window".to_string(), tauri::WindowUrl::App("index.html".into()))
                                        
                    .menu(menu)
                    
                    .build()?;
                app.path_resolver().resolve_resource("../../src/components/tab.svelte").unwrap().to_string_lossy().to_string();
            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![
        //     main_window,
        //     get_sources,
        //     fetch_profile
        // ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
