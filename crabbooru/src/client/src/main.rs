pub mod api;
pub use api::*;
use session_types::*;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
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
    println!("Hello, world!");
}
