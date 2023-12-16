pub mod api;
pub mod viewer;
pub use api::*;
use session_types::*;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
pub use viewer::*;
use winit::{
    dpi::{LogicalPosition, LogicalSize, Position},
    window::{WindowBuilder, WindowId},
};
type Gallery<I: Api + Any> = Vec<I::Image>;
type Client<I: Api + Any> = Send<u32, Recv<Gallery<I>, Eps>>;
// fn query_id<I: Api + Any>(c: Chan<(), Client<I>>){
//     let id = c.send(42);
//     let mut c = {
//         let (c, id) = c.recv();
//     }
// }
fn main() {
    let mut sites: Vec<Site> = Vec::new();
    let mut sources = viewer::SourcesWindow::new();
    let sources_window =
        viewer::SourcesWindow::sources_window(&mut sources, sites, &mut WindowId::from(0));
    println!("Hello, world!");
}
