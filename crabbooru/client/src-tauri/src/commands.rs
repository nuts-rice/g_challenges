use crate::api::*;
use crate::error::*;

#[tauri::command]
pub fn get_source(_api_state: TestbooruAccess<'_>) -> Result<String, CrabbooruError> {
    // let api = api_state.inner();
    // let source = api::DanbooruClient::URL;
    // Ok(PageUrl { error: String::from("Url error"), url: String::from(source), headers: HashMap::new() })
    todo!()
}

#[tauri::command]
pub fn fetch_profile(profile: String) {
    println!("profile: {}", profile);
    todo!()
}

#[tauri::command]
pub fn main_window() {
    todo!()
}
