#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, State};
use std::collections::HashMap;
pub mod api;
pub mod viewer;
pub mod model;
pub mod error;
pub use api::*;
pub use error::*;
pub use viewer::*;
pub use model::*;   

pub use viewer::*;
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


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

// #[tauri::command]
// fn get_sources(profile: String, sites: Vec<Site>) -> Vec<Site> {
//     let sources = SourcesWindow::new(profile, sites, 0);
//     info!("sources: {:?}", sources);
//     sources.selected
// }


#[tauri::command]
fn fetch_profile(profile: String) {
    println!("profile: {}", profile);
    todo!()
}
//TODO: manage api state  
#[tauri::command]
async fn get_images_cmd(api_state: TestbooruAccess<'_>, url: PageUrl) -> Result<Vec<Image>, CrabbooruError> {
//Result<Vec<Image>>  { 
    let api = api_state.inner().clone();
    let images = api.get_image(url).await;
    Ok(images.unwrap())
    //Ok(images)
}
#[tauri::command]
async fn get_page_cmd(api_state: TestbooruAccess<'_>, url: String, headers: HashMap<String, String>) -> Result<PageUrl, CrabbooruError> {
    let api = api_state.inner().clone();
    let page = api.get_page(url, headers).await;
    Ok(page.unwrap())
}
#[tauri::command]
async fn get_md5_db_cmd(api_state: TestbooruAccess<'_>) -> Result<(), CrabbooruError>{
todo!()
}
#[tauri::command]
async fn connect_api_cmd(api_state: TestbooruAccess<'_>) -> Result<(), CrabbooruError>{
    let api = api_state.inner().clone();
    Ok(())
}
#[tauri::command]
fn main_window() {
    todo!()
}

fn main() {
//    tracing_subscriber::fmt().init();
    let _sites: Vec<Site> = Vec::new();
    let _profile = "default".to_string();
    // tracing_subscriber::fmt::init();
    // let mut sources =
    //     viewer::SourcesWindow::new(profile, sites, egui_winit::winit::window::WindowId::from(0));
    // let sources_window =
    //     viewer::SourcesWindow::sources_window(&mut sources, &egui::Context::default());
    let sources_config = CustomMenuItem::new("Config Sources".to_string(), "Sources Config");
    let sources_submenu = Submenu::new("Sources", Menu::new().add_item(sources_config));
    let new_tab = CustomMenuItem::new("new_tab".to_string(), "New Tab");
    let close_tab = CustomMenuItem::new("close_tab".to_string(), "Close Tab");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let file_submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let blacklist_fixer = CustomMenuItem::new("blacklist_fixer".to_string(), "Blacklist Fixer");
    let md5_db_converter = CustomMenuItem::new("md5_db_converter".to_string(), "MD5 Database Converter");
    let md5_list_fixer = CustomMenuItem::new("md5_list_fixer".to_string(), "MD5 List Fixer");
    let tools_submenu = Submenu::new("Tools", Menu::new().add_item(blacklist_fixer).add_item(md5_db_converter).add_item(md5_list_fixer));
    let tags_view = CustomMenuItem::new("view_tags".to_string(), "Tags");
    let view_submenu = Submenu::new("View", Menu::new().add_item(tags_view));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(file_submenu)
        .add_submenu(sources_submenu)
        .add_submenu(tools_submenu)
        .add_submenu(view_submenu);
    
    tauri::Builder::default()
        .manage(TestbooruClient{inner: Default::default()})
        .manage(DanbooruClient{inner: Default::default()})
    .invoke_handler(tauri::generate_handler![
                    connect_api_cmd,
                    //TODO: apiState
        //     main_window,
        //     get_sources,
        //     fetch_profile
        // get_images_cmd
        ])
        .setup(|app| {
                let window = tauri::WindowBuilder::new(app, "main-window".to_string(), tauri::WindowUrl::App("../index.html".into()))
                    .menu(menu)
                    .build()?;
                let app_handle = app.handle();
                let boxed_app_handle = Box::new(app_handle);
                let _window = window.clone();
                // window.on_menu_event(move |event| {
                //     "quit" => {
                //         std::process::exit(0);
                //     }
                //     _ => {}
                // });

            Ok(())
        })
       
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
