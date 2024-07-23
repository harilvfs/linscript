use git2::Repository;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{self, Write, BufRead};
use std::fs::File;
use std::env;
use colored::*;

fn main() {
    println!("{}", "Starting the configuration process...".bold().blue());

    if (!is_sway_installed()) {
        println!("{}", "Sway is not present in this system. Skipping Sway configuration part.".yellow());
    } else {
        configure_sway();
    }

    let vimrc_url = "https://raw.githubusercontent.com/aayushx402/MyVim/main/vimrc";
    let vimrc_path = "/etc/vimrc";

    println!("{}", "Downloading Vim configuration...".bold().blue());
    let response = reqwest::blocking::get(vimrc_url).unwrap();
    if response.status().is_success() {
        let mut file = File::create("temp_vimrc").unwrap();
        file.write_all(&response.bytes().unwrap()).unwrap();

        let status = Command::new("sudo")
            .arg("cp")
            .arg("temp_vimrc")
            .arg(vimrc_path)
            .status()
            .expect("Failed to execute sudo command to copy new Vim config");
        if status.success() {
            println!("{}", "Vim configuration file replaced successfully.".green());
        } else {
            println!("{}", "Failed to replace Vim configuration file with sudo.".red());
            panic!("Failed to replace Vim configuration file with sudo.");
        }

        fs::remove_file("temp_vimrc").unwrap();
    } else {
        println!("{}", "Failed to download the Vim configuration file.".red());
    }

    choose_neovim_plugin_manager();
    choose_browser();
    install_useful_packages();
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
    let repo_url = "https://github.com/aayushx402/sway";
    let local_path = Path::new("/tmp/sway-config");

    if local_path.exists() && local_path.read_dir().unwrap().next().is_some() {
        println!("{}", "Directory already exists and is not empty. Skipping clone.".yellow());
    } else {
        match Repository::clone(repo_url, local_path) {
            Ok(_) => println!("{}", "Repository cloned successfully.".green()),
            Err(e) => panic!("Failed to clone repository: {}", e),
        }
    }

    let username = env::var("USER").expect("Failed to get the username");
    let mut sway_config_path = PathBuf::new();
    sway_config_path.push("/home");
    sway_config_path.push(&username);
    sway_config_path.push(".config/sway");

    let sway_config_path = Path::new(&sway_config_path);

    if !sway_config_path.exists() || fs::metadata(sway_config_path).is_err() {
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

    Command::new("sh")
        .arg("-c")
        .arg("which swayr || (command -v pacman > /dev/null && sudo pacman -S --noconfirm swayr) || sudo apt-get install swayr")
        .status()
        .expect("Failed to check/install swayr");

    println!("{}", "Sway configuration updated successfully.".green());
}

fn install_vim_plug() {
    println!("{}", "Installing vim-plug...".bold().blue());
    Command::new("sh")
        .arg("-c")
        .arg("curl -fLo ~/.local/share/nvim/site/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim")
        .status()
        .expect("Failed to install vim-plug");

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

    Command::new("nvim")
        .arg("+PlugInstall")
        .arg("+qall")
        .status()
        .expect("Failed to run :PlugInstall in Neovim");

    println!("{}", "vim-plug and Catppuccin theme installed successfully.".green());
}

fn install_packer_nvim() {
    println!("{}", "Installing packer.nvim...".bold().blue());
    Command::new("sh")
        .arg("-c")
        .arg("git clone --depth 1 https://github.com/wbthomason/packer.nvim ~/.local/share/nvim/site/pack/packer/start/packer.nvim")
        .status()
        .expect("Failed to install packer.nvim");

    let init_lua_path = PathBuf::from(format!("{}/.config/nvim/init.lua", env::var("HOME").unwrap()));
    let config = r#"
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

    Command::new("nvim")
        .arg("+PackerSync")
        .arg("+qall")
        .status()
        .expect("Failed to run :PackerSync in Neovim");

    println!("{}", "packer.nvim and Catppuccin theme installed successfully.".green());
}

fn choose_neovim_plugin_manager() {
    println!("\n{}", "Choose your Neovim plugin manager:".bold().blue());
    println!("{}", "1. vim-plug".cyan());
    println!("{}", "2. packer.nvim".cyan());

    let stdin = io::stdin();
    let mut choice = String::new();
    stdin.lock().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => install_vim_plug(),
        "2" => install_packer_nvim(),
        _ => println!("{}", "Invalid choice. Please run the program again and choose 1 or 2.".red()),
    }
}

fn choose_browser() {
    println!("\n{}", "Choose your favorite browser:".bold().blue());
    println!("{}", "1. Brave".cyan());
    println!("{}", "2. Firefox".cyan());
    println!("{}", "3. Thorium".cyan());

    let stdin = io::stdin();
    let mut choice = String::new();
    stdin.lock().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => install_browser("brave"),
        "2" => install_browser("firefox"),
        "3" => install_browser("thorium"),
        _ => println!("{}", "Invalid choice. Please run the program again and choose 1, 2, or 3.".red()),
    }
}

fn install_browser(browser: &str) {
    let package_manager = if Command::new("sh")
        .arg("-c")
        .arg("command -v pacman")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        "pacman"
    } else if Command::new("sh")
        .arg("-c")
        .arg("command -v apt")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        "apt"
    } else {
        println!("{}", "Unsupported package manager. Please install the browser manually.".red());
        return;
    };

    match (browser, package_manager) {
        ("brave", "pacman") => install_with_pacman("brave-browser"),
        ("brave", "apt") => install_with_apt("brave-browser"),
        ("firefox", "pacman") => install_with_pacman("firefox"),
        ("firefox", "apt") => install_with_apt("firefox"),
        ("thorium", "pacman") => install_thorium_arch(),
        ("thorium", "apt") => install_thorium_debian(),
        _ => println!("{}", "Invalid browser or package manager.".red()),
    }
}

fn install_with_pacman(package: &str) {
    println!("{}", format!("Installing {} with pacman...", package).bold().blue());
    Command::new("sh")
        .arg("-c")
        .arg(format!("sudo pacman -S --noconfirm {}", package))
        .status()
        .expect("Failed to install package with pacman");
    println!("{}", format!("{} installed successfully.", package).green());
}

fn install_with_apt(package: &str) {
    println!("{}", format!("Installing {} with apt...", package).bold().blue());
    Command::new("sh")
        .arg("-c")
        .arg(format!("sudo apt install -y {}", package))
        .status()
        .expect("Failed to install package with apt");
    println!("{}", format!("{} installed successfully.", package).green());
}

fn install_thorium_arch() {
    println!("{}", "Installing Thorium browser on Arch-based system...".bold().blue());
    Command::new("sh")
        .arg("-c")
        .arg("curl -L -o /tmp/thorium-browser-bin.tar.gz https://aur.archlinux.org/cgit/aur.git/snapshot/thorium-browser-bin.tar.gz && \
              tar -xvf /tmp/thorium-browser-bin.tar.gz -C /tmp && \
              cd /tmp/thorium-browser-bin && \
              makepkg -si --noconfirm")
        .status()
        .expect("Failed to install Thorium browser on Arch-based system");
    println!("{}", "Thorium browser installed successfully on Arch-based system.".green());
}

fn install_thorium_debian() {
    println!("{}", "Installing Thorium browser on Debian-based system...".bold().blue());
    Command::new("sh")
        .arg("-c")
        .arg("sudo rm -fv /etc/apt/sources.list.d/thorium.list && \
              sudo wget --no-hsts -P /etc/apt/sources.list.d/ http://dl.thorium.rocks/debian/dists/stable/thorium.list && \
              sudo apt update && \
              sudo apt install -y thorium-browser")
        .status()
        .expect("Failed to install Thorium browser on Debian-based system");
    println!("{}", "Thorium browser installed successfully on Debian-based system.".green());
}

fn install_useful_packages() {
    println!("\n{}", "Installing useful packages...".bold().blue());

    let package_manager = if Command::new("sh")
        .arg("-c")
        .arg("command -v pacman")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        "pacman"
    } else if Command::new("sh")
        .arg("-c")
        .arg("command -v apt")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        "apt"
    } else {
        println!("{}", "Unsupported package manager. Please install the packages manually.".red());
        return;
    };

    let packages = [
        "obs-studio",
        "thunar",
        "github-desktop",
        "telegram-desktop",
        "gedit",
        "neovim",
        "vim",
    ];

    for package in &packages {
        match package_manager {
            "pacman" => install_with_pacman(package),
            "apt" => install_with_apt(package),
            _ => println!("{}", "Unsupported package manager.".red()),
        }
    }
}


