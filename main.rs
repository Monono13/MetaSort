use std::io::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

//unixdate-xxxxxxxxxxxx-(filetype)

fn main() -> std::io::Result<()> {
    loop {
        let dirs = fs::read_dir("./testdir").unwrap();
        for dir in dirs {
            let file_path = dir.unwrap().path();
            let file = file_path.display().to_string();
            let metadata = fs::metadata(&file)?;
            if let Ok(created_time) = metadata.created() {
                if let Ok(duration) = created_time.duration_since(UNIX_EPOCH) {
                    let file_name = file;
                    let file_id: Vec<&str> = file_name.split('-').collect();
                    let unix_duration = duration.as_nanos();
                    let unix_id = format!("./testdir/unixdate-{unix_duration}");
                    if file_id[0] != "unixdate" {
                        fs::rename(&file_name, unix_id)?;
                    } else {
                        continue;
                    }
                    println!("El archivo {file_name} fue creado {unix_duration}");
                    continue;
                } else {
                    println!("Error: El tiempo de creación es anterior a la época Unix.");
                    break;
                }
            } else {
                println!("No se pudo obtener el tiempo de creación del archivo.");
                break;
            }
        }
    }
}
