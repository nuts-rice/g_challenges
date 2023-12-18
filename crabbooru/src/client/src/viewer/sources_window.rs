pub use crate::api::Api;
pub use crate::api::{Page, Site};
use std::collections::HashMap;
use url::Url;
use winit::raw_window_handle::HasRawWindowHandle;
use winit::window::{Window, WindowBuilder, WindowButtons, WindowId};
use winit::{
    event::ElementState, event::Event, event::MouseButton, event::WindowEvent,
    event_loop::EventLoop,
};

//TODO: Profile or import
pub type Profile = String;

type SourceUrl = String;
pub struct SourcesWindow {
    profile: Option<Profile>,
    selected: Vec<Site>,
    parent: Option<WindowId>,
    windows: HashMap<WindowId, Window>,
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub struct SourcesRow {
    site: Site,
    //todo: tie these to ui elements
    checkbox: bool,
    labels: Vec<String>,
    button: WindowButtons,
}

impl SourcesWindow {
    pub fn new(profile: Profile, selected: Vec<Site>, parent: WindowId) -> Self {
        Self {
            profile: Some(profile),
            selected,
            parent: Some(parent),
            windows: HashMap::new(),
        }
    }
    pub fn sources_window(&mut self, selected: Vec<Site>, parent: &mut WindowId) -> Result<()> {
        let event_loop = EventLoop::new().unwrap();
        let mut builder = WindowBuilder::new()
            .with_title("Sources")
            .with_inner_size(winit::dpi::LogicalSize::new(400, 200));
        let window = builder.build(&event_loop).unwrap();

        event_loop.run(move |event: Event<()>, elwt| {
            if let Event::WindowEvent { window_id, event } = event {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");

                        // let settings =
                        // let parent = self.parent
                        self.windows.clear();
                        elwt.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        if let Some(window) = self.windows.get(&window_id) {
                            println!("Redrawing window {:?}", window.id());
                        }
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if let Some(window) = self.windows.get(&window_id) {
                            println!("Mouse input on window {:?}", window.id());
                        }
                    }
                    // WindowEvent::MouseInput => {
                    //     todo!() }
                    _ => (),
                }
            }
        });

        todo!()
    }
    pub fn add_source(&mut self, url: SourceUrl) {
        todo!()
    }
    pub fn is_source_valid(&self, url: SourceUrl) -> bool {
        todo!()
    }
    pub fn check_all_sources(&self) {
        todo!()
    }
    pub fn prepare_site_settings(&self) {
        todo!()
    }
    pub fn delete_site(&self) {
        todo!()
    }

    pub fn check_updates(&self) {
        let mut is_checked: bool = false;
        let mut is_unchecked: bool = false;
        for row in self.selected.iter() {
            // if row.is_checked() {
            //     row.;
            // }
            // println!("Page: {:?}", page);
        }
    }
}
