// use comfy::*;
// use verlet::prelude::*;
// simple_game!("cloth cutting demo", VerletState, update);
// pub struct CameraSettings {
//     pub use_camera_override: bool,
//     pub offset: Vec2,
//     pub eye_z: f32,
//     pub fov: f32,
// }

// impl Default for CameraSettings {
//     fn default() -> Self {
//         Self {
//             use_camera_override: true,
//             offset: vec2(0.0, -10.0),
//             eye_z: 10.0,
//             fov: 0.8,
//         }
//     }
// }
// pub static CAMERA_SETTINGS: Lazy<AtomicRefCell<CameraSettings>> =
//     Lazy::new(|| AtomicRefCell::new(CameraSettings::default()));

// fn setup(c: &mut EngineContext, state: &mut VerletState) {
//     main_camera_mut().matrix_fn = Some(Box::new(|_, center: Vec2| {
//         let settings = CAMERA_SETTINGS.borrow();

//         let eye = vec3(center.x, center.y, settings.eye_z);
//         let up = vec3(0.0, 1.0, 0.0);

//         let perspective = Mat4::perspective_rh(settings.fov, aspect_ratio(), 0.01, 1000.0);

//         let view = Mat4::look_at_rh(eye + settings.offset.extend(0.0), center.extend(0.0), up);

//         perspective * view
//     }));
//     let (points_w, points_h) = (139, 80);
//     // let rbd_handle =

//     }

// fn update(_c: &mut EngineContext) {}
