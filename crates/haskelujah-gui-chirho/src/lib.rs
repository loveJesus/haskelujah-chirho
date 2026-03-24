// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

pub fn run_gui_chirho(_file_path_chirho: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Delegate to the binary — for now just return an error if called as library
    // The real GUI runs via `haskelujah-gui` binary or `haskelujah edit --gui`
    Err("GUI editor: run `haskelujah edit --gui` or `cargo run -p haskelujah-gui`".into())
}
