use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dir = "/home/hermes/Desktop/projects/admin-exercita/well-known/assets/exercises_images/pacotes_gifs_8/";
    let prefix = "Exercicios_livres_8";
    let mut nindex: i32 = 1;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "jpg"
                    || extension == "jpeg"
                    || extension == "png"
                    || extension == "gif"
                    || extension == "GIF"
                {
                    let new_name = format!("{}_{}", prefix, nindex);
                    let new_path = Path::new(path.parent().unwrap()).join(new_name);
                    fs::rename(&path, &new_path)?;
                    nindex += 1;
                }
            }
        }
    }
    Ok(())
}

