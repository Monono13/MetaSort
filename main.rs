use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread, time::Duration};

enum Estate {
    Escaneando,
    Renombrando,
}
fn main() -> io::Result<()> {
    let mut estate = Estate::Escaneando;
    let mut input = String::new();
    println!("Escriba el nombre del directorio:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error: No se pudo leer el input del usuario");
    let dir_name = input.trim();
    loop {
        match estate {
            Estate::Escaneando => {
                println!("Estado: Escaneando");
                thread::sleep(Duration::from_millis(4000));
                let dirs = fs::read_dir(dir_name).expect("Error: No se encontró el directorio");
                let mut renamed_files = false;
                for dir in dirs {
                    let file_path = dir.unwrap().path();
                    let file = file_path.display().to_string();
                    let metadata = fs::metadata(&file)?;
                    if !file.contains("unixdate") {
                        if let Ok(created_time) = metadata.created() {
                            if let Ok(duration) = created_time.duration_since(UNIX_EPOCH) {
                                let unix_duration = duration.as_nanos();
                                let unix_id = format!("./{}/unixdate-{}", dir_name, unix_duration);
                                fs::rename(&file, unix_id)?;
                                println!("El archivo {} fue renombrado a {}", file, unix_duration);
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
            Estate::Renombrando => {
                println!("Estado: Renombrando");
                thread::sleep(Duration::from_millis(4000));
                estate = Estate::Escaneando;
            }
        }
    }
}
