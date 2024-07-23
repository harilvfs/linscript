use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{self, Write, BufRead};
use std::fs::File;
use std::env;

fn main() {
    // Check if Sway is present
    if !is_sway_installed() {
        println!("Sway is not present in this system. Skipping Sway configuration part.");
    } else {
        configure_sway();
    }

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

    // Prompt the user for their choice of plugin manager
    println!("Choose your Neovim plugin manager (1 for vim-plug, 2 for packer.nvim):");

    let stdin = io::stdin();
    let mut choice = String::new();
    stdin.lock().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => install_vim_plug(),
        "2" => install_packer_nvim(),
        _ => println!("Invalid choice. Please run the program again and choose 1 or 2."),
    }
}

fn is_sway_installed() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("pgrep sway > /dev/null 2>&1")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn configure_sway() {
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
}

fn install_vim_plug() {
    // Ensure vim-plug is installed
    Command::new("sh")
        .arg("-c")
        .arg("curl -fLo ~/.local/share/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim")
        .status()
        .expect("Failed to install vim-plug");

    // Add the Catppuccin theme configuration to init.vim
    let init_vim_path = PathBuf::from(format!("{}/.config/nvim/init.vim", env::var("HOME").unwrap()));

    let config = r#"
call plug#begin('~/.config/nvim/plugged')

Plug 'catppuccin/nvim', {'as': 'catppuccin'}

call plug#end()

syntax enable
set background=dark
colorscheme catppuccin
"#;

    fs::create_dir_all(init_vim_path.parent().unwrap()).unwrap();
    let mut file = File::create(init_vim_path).unwrap();
    file.write_all(config.as_bytes()).unwrap();

    // Install plugins
    Command::new("nvim")
        .arg("+PlugInstall")
        .arg("+qall")
        .status()
        .expect("Failed to run :PlugInstall in Neovim");

    println!("vim-plug and Catppuccin theme installed successfully.");
}

fn install_packer_nvim() {
    // Ensure packer.nvim is installed
    Command::new("sh")
        .arg("-c")
        .arg("git clone --depth 1 https://github.com/wbthomason/packer.nvim ~/.local/share/nvim/site/pack/packer/start/packer.nvim")
        .status()
        .expect("Failed to install packer.nvim");

    // Add the Catppuccin theme configuration to init.lua
    let init_lua_path = PathBuf::from(format!("{}/.config/nvim/init.lua", env::var("HOME").unwrap()));

    let config = r#"
-- Ensure packer is installed
local ensure_packer = function()
  local fn = vim.fn
  local install_path = fn.stdpath('data')..'/site/pack/packer/start/packer.nvim'
  if fn.empty(fn.glob(install_path)) > 0 then
    fn.system({'git', 'clone', '--depth', '1', 'https://github.com/wbthomason/packer.nvim', install_path})
    vim.cmd [[packadd packer.nvim]]
    return true
  end
  return false
end

local packer_bootstrap = ensure_packer()

return require('packer').startup(function(use)
  use 'wbthomason/packer.nvim'
  use 'catppuccin/nvim'

  if packer_bootstrap then
    require('packer').sync()
  end
end)

vim.cmd('syntax enable')
vim.o.background = 'dark'
vim.cmd('colorscheme catppuccin')
"#;

    fs::create_dir_all(init_lua_path.parent().unwrap()).unwrap();
    let mut file = File::create(init_lua_path).unwrap();
    file.write_all(config.as_bytes()).unwrap();

    // Install plugins
    Command::new("nvim")
        .arg("+PackerSync")
        .arg("+qall")
        .status()
        .expect("Failed to run :PackerSync in Neovim");

    println!("packer.nvim and Catppuccin theme installed successfully.");
}

