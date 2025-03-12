// El programa se encuentra escrito en Rust puro con las librerias que contiene el lenguaje al ser instalado, por el momento aun no se a agregado
// ninguna libreria que tenga que ser instalada aparte del lenguaje de Rust

use chrono::prelude::*;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread, time::Duration};
//Estados de escaneo y renombrado de los archivos
enum Estate {
    Escaneando,
    Renombrando,
}

fn main() -> io::Result<()> {
    let mut estate = Estate::Escaneando;
    let mut input = String::new();
    let mut counter = 0;
    println!("Enter the directory name:");

    io::stdin()
        .read_line(&mut input)
        .expect("Error: Failed to read user input");
    let dir_name = input.trim();

    loop {
        match estate {
            Estate::Escaneando => {
                println!("Scanning directory...");
                thread::sleep(Duration::from_millis(12000));
                let dirs = fs::read_dir(dir_name).expect("Error: Directory not found");
                let mut renamed_files = false;

                for dir in dirs {
                    let file_path = dir.unwrap().path();
                    let file = file_path.display().to_string();
                    let metadata = fs::metadata(&file)?;
                    if !file.contains("filefrom:") {
                        if let Ok(created_time) = metadata.created() {
                            if let Ok(duration) = created_time.duration_since(UNIX_EPOCH) {
                                let unix_duration = duration.as_secs() as i64;
                                let naive = NaiveDateTime::from_timestamp(unix_duration, 0);
                                let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                                let date = datetime.format("%Y-%m-%d %H:%M:%S");
                                let unix_id = format!("./{}/filefrom:{}", dir_name, date);
                                counter += 1;
                                fs::rename(&file, unix_id)?;
                                println!("File {} was renamed to filefrom:{}", file, date);
                                renamed_files = true;
                            }
                        }
                    }
                }
                if renamed_files {
                    estate = Estate::Renombrando;
                } else {
                    continue;
                }
            }
            //Cambiando entre los diferentes estados llega a ser mas eficiente para el sistema en tener el codigo corriendo en el background
            Estate::Renombrando => {
                println!("{} Files were renamed...", counter);
                counter = 0;
                thread::sleep(Duration::from_millis(4000));
                estate = Estate::Escaneando;
            }
        }
    }
}

//cargo build --release para recompilar
