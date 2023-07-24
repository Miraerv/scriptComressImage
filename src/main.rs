use std::{fs, io};
use std::process::Command;
use std::path::{Path, PathBuf};
use rayon::prelude::*;

fn compress_file(input_path: &Path) -> io::Result<()> {
    let extension = input_path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase());
    let input_path_str = input_path.to_str().unwrap();
    let file_stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap();

    let (command, output_extension, args) = match extension.as_deref() {
        Some("jpg") | Some("jpeg") => {
            ("jpegoptim", "jpg", vec!["-m72"])
        }
        Some("png") => {
            ("pngquant", "png", vec!["-f","--quality=65-80", input_path_str, "-o"])
        }
        Some("tiff") => {
            ("convert", "tiff", vec![input_path_str, "-quality", "80"])
        }
        _ => {
            println!("Unsupported type: {:?}", extension);
            return Ok(());
        }
    };
    let output_path_str = format!("{}/{}.{}", input_path.parent().unwrap().to_str().unwrap(), file_stem, output_extension);
    let args = [args, vec![&output_path_str]].concat();

    let output = Command::new(command)
        .args(args)
        .output()?;

/* 
    if !output.stdout.is_empty() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
*/
    if !output.status.success() {
        println!("{} process exited with code {}", command, output.status);
    } else {
        println!("Compressed {}", input_path_str);
    }

    Ok(())
}

fn compress_directory(directory_path: &Path) -> io::Result<()> { 
    if directory_path.is_dir() { 
        let entries: Vec<_> = fs::read_dir(directory_path)?.collect();

        entries.into_par_iter().try_for_each(|entry| {
            let entry = entry?; 
            let path = entry.path(); 
            if path.is_dir() { 
                compress_directory(&path)
            } else { 
                compress_file(&path)
            } 
        })?;
    } 
    Ok(()) 
}

fn main() -> io::Result<()> { 
    compress_directory(&PathBuf::from("/data"))?; 
    Ok(()) 
}