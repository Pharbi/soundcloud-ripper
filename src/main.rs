use std::fs;
use std::process::Command;

fn download_audio(links: &[&str], download_folder: &str) {
    if let Err(e) = fs::create_dir_all(download_folder) {
        eprintln!("Failed to create download folder: {}", e);
        return;
    }

    for &link in links {
        println!("Downloading: {}", link);
        let output = Command::new("youtube-dl")
            .arg("-f")
            .arg("bestaudio")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--audio-quality")
            .arg("0")
            .arg("-o")
            .arg(format!("{}/%(title)s.%(ext)s", download_folder))
            .arg(link)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!(
                        "Failed to download {}. Error: {}",
                        link,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to execute youtube-dl for {}. Error: {}", link, e);
            }
        }
    }
}

fn main() {
    let links = [];
    let download_folder = "downloads";
    download_audio(&links, download_folder);
}
