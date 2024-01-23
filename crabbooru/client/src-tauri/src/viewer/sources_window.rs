pub use crate::api::Api;
pub use crate::api::{Page, Site};

//TODO: Profile or import
pub type Profile = String;

type SourceUrl = String;
#[derive(Debug, Clone)]
pub struct SourcesWindow {
    profile: Option<Profile>,
    pub selected: Vec<Site>,
    parent_id: Option<u32>,
    // windows: HashMap<u32, >,
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//#[derive(Debug, Clone)]
//pub struct SourcesRow {
//    site: Site,
//    //todo: tie these to ui elements
//    checkbox: bool,
//    labels: Vec<String>,
//    // button: WindowButtons,
//}

impl SourcesWindow {
    pub fn new(profile: Profile, selected: Vec<Site>, parent_id: u32) -> Self {
        Self {
            profile: Some(profile),
            selected,
            parent_id: Some(parent_id),
            // windows: HashMap::new(),
        }
    }
    // fn add_check_boxes(&self) {
    //     for row in self.selected.iter() {
    //         let label = format!("{}", row.url);
    //         let source_row = SourcesRow {
    //             site: row.clone(),
    //             checkbox: true,
    //             labels: vec![label],
    //             button: WindowButtons::CLOSE,
    //         };
    //     }
    // }
    pub fn sources_window(&mut self) -> Result<()> {
        //    let event_loop = EventLoop::new();
        //    let mut builder = WindowBuilder::new()
        //        .with_title("Sources")
        //        .with_inner_size(winit::dpi::LogicalSize::new(400, 200));
        //    let mut platform = Platform::new(PlatformDescriptor {
        //        physical_width: 400,
        //        physical_height: 200,
        //        scale_factor: 1.0,
        //        font_definitions: Default::default(),
        //        style: Default::default(),
        //    });

        //    let mut selected_state = false;
        //    let window = builder.build(&event_loop).unwrap();

        //    event_loop.run(move |event: Event<()>, _, control_flow| {
        //        *control_flow = ControlFlow::Wait;
        //        platform.handle_event(&event);
        //        match event {
        //            winit::event::Event::RedrawRequested(_) => {
        //                platform.begin_frame();

        //                egui::CentralPanel::default().show(
        //                    &platform.context(),
        //                    //&platform.context(),
        //                    |ui| {
        //                        ui.label("source");
        //                        ui.checkbox(&mut selected_state, "Source check");
        //                    },
        //                );
        //                // let (output, paint_commands) = platform.end_frame(Some(&window));
        //                // let paint_jobs = platform.context().tessellate(paint_commands);

        //                window.request_redraw();
        //            }
        //            winit::event::Event::MainEventsCleared => {
        //                window.request_redraw();
        //            }
        //            _ => (),
        //        }
        //    });

        // if let Event::WindowEvent { window_id, event } = event {
        //     match event {
        //         WindowEvent::CloseRequested => {
        //             println!("The close button was pressed; stopping");

        //             // let settings =
        //             // let parent = self.parent
        //             self.windows.clear();
        //             elwt.exit();
        //         }
        //         WindowEvent::RedrawRequested => {
        //             if let Some(window) = self.windows.get(&window_id) {
        //                 println!("Redrawing window {:?}", window.id());
        //             }
        //         }
        //         WindowEvent::MouseInput {
        //             state: ElementState::Pressed,
        //             button: MouseButton::Left,
        //             ..
        //         } => {
        //             if let Some(window) = self.windows.get(&window_id) {
        //                 println!("Mouse input on window {:?}", window.id());
        //             }
        //         }
        //         // WindowEvent::MouseInput => {
        //         //     todo!() }
        //         _ => (),

        todo!();
    }
    pub fn add_source(&mut self, _url: SourceUrl) {
        todo!()
    }
    pub fn is_source_valid(&self, _url: SourceUrl) -> bool {
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
        let _is_checked: bool = false;
        let _is_unchecked: bool = false;
        for _row in self.selected.iter() {
            // if row.is_checked() {
            //     row.;
            // }
            // println!("Page: {:?}", page);
        }
    }
}
