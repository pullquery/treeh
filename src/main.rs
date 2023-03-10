use std::path::Path;
use std::{env, fs};

fn read(path: &Path, head: String) {
    let entries = fs::read_dir(path).unwrap();
    let count = fs::read_dir(path).unwrap().count(); // repetition

    for (index, entry) in entries.into_iter().enumerate() {
        let is_last = index + 1 < count;
        let new_path = entry.unwrap().path();

        let print_head = head.clone() + if is_last { "├──" } else { "└──" };
        let read_head = head.clone() + if is_last { "│  " } else { "   " };

        print(&new_path, &print_head);
        if new_path.is_dir() {
            read(&new_path, read_head);
        }
    }
}

fn print(path: &Path, head: &String) {
    let file_name = path.file_name().unwrap_or_default().to_str().unwrap();
    let extension = path.extension().unwrap_or_default().to_str().unwrap();
    let color_code = format!(
        "\x1b[38;5;{}m",
        match extension.to_lowercase().as_str() {
            "" => 11,
            "txt" => 253,
            "pdf" => 198,
            "png" | "jpg" | "jpeg" => 69,
            "mp3" | "ogg" | "flac" => 222,
            "mp4" => 191,
            _ => 248,
        },
    );
    let color_reset_code = "\x1b[0m";

    println!("{}{}{}{}", head, color_code, file_name, color_reset_code);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let head = String::new();

    print(&path, &head);
    read(&path, head);
}
