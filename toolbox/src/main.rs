use git2::Repository;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Define the repository URL and the local path to clone to
    let repo_url = "https://github.com/aayushx402/sway";
    let local_path = "/tmp/sway-config";

    // Clone the repository
    match Repository::clone(repo_url, local_path) {
        Ok(_) => println!("Repository cloned successfully."),
        Err(e) => panic!("Failed to clone repository: {}", e),
    }

    // Define the paths for the Sway configuration
    let sway_config_path = Path::new("/home/yourusername/.config/sway");
    let swayr_config_path = Path::new("/home/yourusername/.config/swayr");

    // Remove the existing Sway configuration
    if sway_config_path.exists() {
        fs::remove_dir_all(sway_config_path).expect("Failed to remove existing Sway config");
    }

    // Copy the new configuration files from the cloned repository
    let new_config_path = Path::new(local_path).join("sway");
    fs::create_dir_all(sway_config_path).expect("Failed to create Sway config directory");
    fs::copy(new_config_path, sway_config_path).expect("Failed to copy new Sway config");

    // Ensure swayr is installed
    Command::new("sh")
        .arg("-c")
        .arg("which swayr || (command -v pacman > /dev/null && sudo pacman -S --noconfirm swayr) || sudo apt-get install swayr")
        .status()
        .expect("Failed to check/install swayr");

    println!("Sway configuration updated successfully.");
}