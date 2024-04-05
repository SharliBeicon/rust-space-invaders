use std::fs;
use std::path::Path;
use rusty_audio::Audio;

/* Look for files into the sounds folder and loads them */
pub fn load_audio() -> std::io::Result<Audio> {
    let folder_path = "./sounds";
    let mut audio = Audio::new();

    fs::read_dir(folder_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .for_each(|path| {
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if let Some(file_stem) = Path::new(file_name).file_stem().and_then(|f| f.to_str()) {
                    audio.add(file_stem, path.to_str().unwrap_or_default());
                }
            }
        });

    Ok(audio)
}
