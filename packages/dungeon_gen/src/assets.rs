use crate::*;

pub static ASSET_DIR : include_dir::Dir<_>= include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets/");


fn base_path(path: &str) -> String {
    format!(concat!(env!("CARGO_MANIFEST_DIR"), "assets/tilesets/{}"), path)
}
pub fn load_assets() {
    init_asset_source(&ASSET_DIR, base_path);
    let textures = vec![
        ("bend", "bend.png"),
        ("corner", "corner.png"),
        ("corrider", "corrider.png"),
        ("door", "door.png"),
        ("empty", "empty.png"),
        ("side", "side.png"),
        ("t", "t.png"),
        ("turn", "turn.png"),
        ("wall", "wall.png"),        
    ];
    load_multiple_textures(textures
                        .iter().map(|(a, b)| (a.to_string(), b.to_string()))
        .collect_vec(),
        );

        
}
