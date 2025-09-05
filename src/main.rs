use std::{
    fs,
    io::{self},
    path::Path,
};

fn main() {
    println!("Folder name: ");
    let mut folder = String::new();

    loop {
        folder.clear();

        io::stdin()
            .read_line(&mut folder)
            .expect("Failed to read line");

        folder = folder.trim().to_string();

        if std::path::Path::new(&folder).exists() {
            break;
        }

        println!("Folder not found");
    }

    let mut extension = String::new();

    println!("Extension (pdf, jpg, png, txt): ");

    loop {
        extension.clear();

        io::stdin()
            .read_line(&mut extension)
            .expect("Failed to read line");

        extension = extension.trim().to_string();

        match FileExtension::from_str(&extension) {
            Some(_) => break,
            None => {
                println!("Invalid extension");
            }
        }
    }

    println!("folder {}, ext {}", folder, extension);

    let entries = fs::read_dir(&folder).expect("Failed to read dir");

    let files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some(extension.as_str()))
        .collect();

    if files.is_empty() {
        println!("No file with {} extension found.", extension);
        return;
    }

    println!("Target folder: ");
    let mut target_folder = String::new();

    io::stdin()
        .read_line(&mut target_folder)
        .expect("Failed to read line");

    target_folder = target_folder.trim().to_string();

    if target_folder.is_empty() {
        target_folder = match FileExtension::from_str(&extension) {
            Some(FileExtension::Pdf) => "Doc".to_string(),
            Some(FileExtension::Jpg) => "Pictures".to_string(),
            Some(FileExtension::Png) => "Pictures".to_string(),
            Some(FileExtension::Txt) => "Doc".to_string(),
            None => "Others".to_string(),
        }
    }

    let target = Path::new(&folder).join(&target_folder);

    if !std::path::Path::new(&target_folder).exists() {
        let _ = fs::create_dir(&target);
    }

    for file in files {
        let source_path = file.path();
        let destination = target.join(source_path.file_name().unwrap());

        let _ = fs::rename(&source_path, &destination);

        println!(
            "Moved {}",
            source_path.file_name().unwrap().to_string_lossy()
        );
    }

    println!(
        "Successfully moved {} files to {} directory!",
        extension,
        target.file_name().unwrap().to_string_lossy()
    );

    return;
}

#[derive(Debug)]
enum FileExtension {
    Pdf,
    Jpg,
    Png,
    Txt,
}

impl FileExtension {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "pdf" => Some(FileExtension::Pdf),
            "jpg" => Some(FileExtension::Jpg),
            "jpeg" => Some(FileExtension::Jpg),
            "png" => Some(FileExtension::Png),
            "txt" => Some(FileExtension::Txt),
            _ => None,
        }
    }
}
