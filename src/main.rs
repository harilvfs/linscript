use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{self, Write};
use std::fs::File;
use std::env;

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

    // Get the current username
    let username = env::var("USER").expect("Failed to get the username");

    // Define the paths for the Sway configuration using PathBuf
    let mut sway_config_path = PathBuf::new();
    sway_config_path.push("/home");
    sway_config_path.push(&username);
    sway_config_path.push(".config/sway");

    // Now convert PathBuf to Path
    let sway_config_path = Path::new(&sway_config_path);

    // Check if we have write permissions to the sway config path
    if !sway_config_path.exists() || fs::metadata(sway_config_path).is_err() {
        // Prompt for sudo permissions to create the directory
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

    // Remove the existing Sway configuration with sudo if necessary
    if sway_config_path.exists() {
        let status = Command::new("sudo")
            .arg("rm")
            .arg("-rf")
            .arg(sway_config_path)
            .status()
            .expect("Failed to execute sudo command to remove existing Sway config");

        if !status.success() {
            panic!("Failed to remove existing Sway config with sudo.");
        }
    }

    // Copy the new configuration files from the cloned repository
    let new_config_path = local_path.join("sway");
    let status = Command::new("sudo")
        .arg("cp")
        .arg("-r")
        .arg(&new_config_path)
        .arg(sway_config_path)
        .status()
        .expect("Failed to execute sudo command to copy new Sway config");

    if !status.success() {
        panic!("Failed to copy new Sway config with sudo.");
    }

    // Ensure swayr is installed
    Command::new("sh")
        .arg("-c")
        .arg("which swayr || (command -v pacman > /dev/null && sudo pacman -S --noconfirm swayr) || sudo apt-get install swayr")
        .status()
        .expect("Failed to check/install swayr");

    println!("Sway configuration updated successfully.");

    // URL of the Vim configuration file
    let vimrc_url = "https://raw.githubusercontent.com/aayushx402/MyVim/main/vimrc";
    // Path to the original vimrc file
    let vimrc_path = "/etc/vimrc";

    // Download the Vim configuration file
    let response = reqwest::blocking::get(vimrc_url).unwrap();
    if response.status().is_success() {
        // Save the downloaded Vim configuration to a temporary file
        let mut file = File::create("temp_vimrc").unwrap();
        file.write_all(&response.bytes().unwrap()).unwrap();

        // Replace the original vimrc file with sudo permissions
        let status = Command::new("sudo")
            .arg("cp")
            .arg("temp_vimrc")
            .arg(vimrc_path)
            .status()
            .expect("Failed to execute sudo command to copy new Vim config");

        if status.success() {
            println!("Vim configuration file replaced successfully.");
        } else {
            panic!("Failed to replace Vim configuration file with sudo.");
        }

        // Remove the temporary file
        fs::remove_file("temp_vimrc").unwrap();
    } else {
        println!("Failed to download the Vim configuration file.");
    }
}
