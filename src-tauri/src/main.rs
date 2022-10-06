
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf};
use walkdir::{ WalkDir};
use regex::Regex;
use std::path::Path;

fn helper(path: &str) -> std::io::Result<()> {
    let file_array = get_filenames(path);

    // fs::create_dir_all(format!("{}/converted", path))?;
    fs::create_dir_all("./converted")?;

    for f in file_array.iter() {
        write_converted_hands(&convert_hands(read_original_hands(&f)?), &f)?;
    }

    Ok(())
}

#[tauri::command]
fn convert(name: &str) -> String {
    let target_path = Path::new(name);
    match target_path.is_dir() {
        true => {
            let result = helper(name);
            match result {
                Ok(_) => "success".to_string(),
                Err(e) => e.to_string(),
            }
        },
        false => return "Directory not found".to_string(),
    }
}

fn get_filenames(path: &str) -> Vec<PathBuf> {
    let mut v = vec![];
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.file_name().to_str().map(|s|s.ends_with(".txt")).unwrap_or(true) {
            v.push(file.path().to_owned());
        }
    }
    v
}

fn read_original_hands(file: &PathBuf) -> std::io::Result<String> {
    let hands = fs::read_to_string(file).unwrap();

    Ok(hands)
}

fn convert_hands(hands: String) -> String {
    let mut converted = str::replace(&hands, "Poker Hand #RC", "PokerStars Hand #20");

    converted = str::replace(&converted, "Dealt to Hero", "XXXXXXXXXX");

    let re = Regex::new(r"Dealt to [0-9A-Za-z_]{0,}\s+").unwrap();
    converted = re.replace_all(&converted, "").to_string();

    converted = str::replace(&converted, "XXXXXXXXXX", "Dealt to Hero");

    converted
}

fn write_converted_hands(hands: &String, filename: &PathBuf) -> std::io::Result<()> {
    let name: PathBuf = PathBuf::from(filename.file_name().unwrap());
    fs::write(std::path::Path::new("./converted/").join(name), hands)?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
    .setup(|_app| {
        Ok(())
      })
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
