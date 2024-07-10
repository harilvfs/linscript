use git2::Repository;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::ErrorKind;

fn main() {
    // Define the repository URL and the local path to clone to
    let repo_url = "https://github.com/aayushx402/sway";
    let local_path = Path::new("/tmp/sway-config");

    // Check if the local path exists and is not empty
    if local_path.exists() && local_path.read_dir().unwrap().next().is_some() {
        println!("Directory already exists and is not empty. Skipping clone.");
    } else {
        // Clone the repository
        match Repository::clone(repo_url, local_path) {
            Ok(_) => println!("Repository cloned successfully."),
            Err(e) => panic!("Failed to clone repository: {}", e),
        }
    }

    // Define the paths for the Sway configuration
    let sway_config_path = Path::new("/home/yourusername/.config/sway");
    let swayr_config_path = Path::new("/home/yourusername/.config/swayr");

    // Check if we have write permissions to the sway config path
    if !sway_config_path.exists() || fs::metadata(sway_config_path).is_err() {
        // Prompt for sudo permissions
        let status = Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(sway_config_path)
            .status()
            .expect("Failed to execute sudo command");

        if !status.success() {
            panic!("Failed to gain sudo permissions to create the Sway config directory.");
        }
    }

    // Remove the existing Sway configuration
    if sway_config_path.exists() {
        fs::remove_dir_all(sway_config_path).expect("Failed to remove existing Sway config");
    }

    // Copy the new configuration files from the cloned repository
    let new_config_path = local_path.join("sway");
    fs::create_dir_all(sway_config_path).unwrap_or_else(|e| {
        if e.kind() == ErrorKind::PermissionDenied {
            panic!("Failed to create Sway config directory: Permission denied. Try running the program with elevated permissions (e.g., using sudo).");
        } else {
            panic!("Failed to create Sway config directory: {}", e);
        }
    });
    fs::copy(new_config_path, sway_config_path).expect("Failed to copy new Sway config");

    // Ensure swayr is installed
    Command::new("sh")
        .arg("-c")
        .arg("which swayr || (command -v pacman > /dev/null && sudo pacman -S --noconfirm swayr) || sudo apt-get install swayr")
        .status()
        .expect("Failed to check/install swayr");

    println!("Sway configuration updated successfully.");
}