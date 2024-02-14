use clap::{Arg, ArgAction, Command};
use std::fs;
use std::path::Path;
use std::ops::Deref;

pub enum Mode {
    Folder,
    File,
    None,
}

fn main() {
    let matches = build_command().get_matches();
    run(matches);
}
fn build_command() -> clap::Command {
    clap::Command::new("image_cleanup")
        .arg(clap::Arg::new("folder")
            .short('f')
            .long("folder")
            .help("Specifies the folders to be deleted"))
        .arg(clap::Arg::new("file")
            .short('i')
            .long("file")
            .help("Specifies the files to be deleted"))
        .arg(clap::Arg::new("self-delete")
            .short('s')
            .long("self-delete")
            .action(ArgAction::SetTrue)
            .global(true)
            .help("Deletes the executable after cleanup"))
        .mut_args(|a| {
            if let Some(l) = a.get_long().map(|l| Box::leak(format!("prefix-{}", l).into_boxed_str()) as &'static str) {
                a.long(l)
            } else {
                a
            }
        })
}

fn run(matches: clap::ArgMatches) {
    // println!("Matches: {:?}", matches);
    let folders: Vec<String> = matches.get_many::<String>("folder")
        .unwrap_or_default()
        .map(|value| value.to_string())
        .collect();

    let files: Vec<String> = matches.get_many::<String>("file")
    .unwrap_or_default()
    .map(|value| value.to_string())
    .collect();

     if folders.len() > 0 {
        for folder in &folders {
            let path = format!("/{}", folder);
            if Path::new(&path).is_dir() {
                let metadata = fs::metadata(&path).unwrap();
                let size_before = metadata.len();
                println!("Removing folder {} (Size: {} bytes)...", folder, size_before);
                fs::remove_dir_all(path).unwrap();
                println!("Removed folder {}, saved: {} bytes", folder, size_before);
            } else {
                println!("Folder {} does not exist.", folder);
            }
        }
    }

    if files.len() > 0 {
        for file in &files {
            let path = format!("/{}", file);
            if Path::new(&path).is_file() {
                let metadata = fs::metadata(&path).unwrap();
                let size_before = metadata.len();
                println!("Removing file {} (Size: {} bytes)...", file, size_before);
                fs::remove_file(path).unwrap();
                println!("Removed file {}, saved: {} bytes", file, size_before);
            } else {
                println!("File {} does not exist.", file);
            }
        }
    }

    if folders.len() == 0 && files.len() == 0 {
        println!("No folders or files to delete.");
    }
    println!("Cleanup completed.");

    let self_delete= matches.get_flag("self-delete");

    if self_delete {
        let path = std::env::current_exe().unwrap();
        println!("Removing executable {}...", path.display());
        fs::remove_file(path).unwrap();
        println!("Removed executable.");

    }   

}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, ArgAction, Command};
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_args_parsing_and_file_deletion() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file");
        let folder_path = dir.path().join("test_folder");
        fs::create_dir(&folder_path).unwrap();
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();
        file.sync_all().unwrap();
        assert!(file_path.exists());
        assert!(folder_path.exists());

        let matches = build_command().get_matches_from(vec![
            "image_cleaner",
            "--prefix-file",
            file_path.to_str().unwrap(),
            "--prefix-folder",
            folder_path.to_str().unwrap(),
        ]);
        
        // Run the main function with the test arguments
        run(matches);

        // Check that the file no longer exists
        assert!(!file_path.exists());
        assert!(!folder_path.exists());

        // Clean up the temp directory
        dir.close().unwrap();
    }
}