use git2::Repository;
use std::fs::{self, File}; 
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{self, Write};
use std::env;
use colored::*;
use std::fs::create_dir_all;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    loop {
        // Clear the terminal screen
        Command::new("clear").status().unwrap();

        // Display header
        println!("                                         ");
        println!("{}", " _     ___ _   _ _   ___  __".bold().green());
        println!("{}", "| |   |_ _| \\ | | | | \\ \\/ /".bold().green());
        println!("{}", "| |    | ||  \\| | | | |\\  / ".bold().green());
        println!("{}", "| |___ | || |\\  | |_| |/  \\ ".bold().green());
        println!("{}", "|_____|___|_| \\_|\\___//_/\\\\_".bold().green());

        // Setup menu options
        let options = vec![
            " Setup Window Manager",
            " Setup Vim",
            " Install Neovim Plugin Manager",
            "󰖟 Install Browsers",
            " Install Packages",
            " Setup GRUB",
            "󰔎 Setup SDDM",
            "󰀺 Setup Fonts",
            " Setup Rofi",
            " Setup Alacritty",
            " Setup Neovim",
            " Aur Helper",
            "󰚯 Instructions",
            "󰿅 Exit"
        ];

        // Use dialoguer to create an interactive menu
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a setup option ")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Match the user's selection and call the appropriate function
        match selection {
            0 => setup_window_manager(),
            1 => setup_vim(),
            2 => choose_neovim_plugin_manager(),
            3 => choose_browser(),
            4 => install_useful_packages(),
            5 => choose_and_apply_grub_theme(),
            6 => setup_sddm_theme(),
            7 => setup_fonts(),
            8 => setup_rofi(),
            9 => setup_alacritty(),
            10 => setup_neovim(),
            11 => setup_aurhelper(),  // This now returns to the menu after skipping
            12 => show_instructions(),
            13 => {
                println!("{}", "Exiting the program.".yellow());
                break;  // Exit the loop, ending the program
            },
            _ => {
                println!("{}", "Invalid choice.".red());
            }
        }
    }
}


fn setup_window_manager() {
    let options = vec![
        "DWM",
        "Exit"
    ];

    loop {
        // Display window manager options using dialoguer
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your window manager:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Match the user's selection and handle accordingly
        match selection {
            0 => {
                install_dwm();
                break;
            },
            1 => {
                println!("{}", "Exiting window manager setup.".yellow());
                break;
            },
            _ => {
                println!("{}", "Invalid choice.".red());
                continue;
            }
        }
    }
}

fn install_dwm() {
    let repo_url = "https://github.com/aayushx402/dwm-ayx";
    let local_path = Path::new("/tmp/dwm-ayx");

    if local_path.exists() && local_path.read_dir().unwrap().next().is_some() {
        println!("{}", "Directory already exists and is not empty. Skipping clone.".yellow());
    } else {
        match Repository::clone(repo_url, local_path) {
            Ok(_) => println!("{}", "Repository cloned successfully.".green()),
            Err(e) => panic!("Failed to clone repository: {}", e),
        }
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg("cd /tmp/dwm-ayx && chmod +x setup.sh && ./setup.sh && sudo make clean && sudo make clean install")
        .status()
        .expect("Failed to install dwm");
    if status.success() {
        println!("{}", "dwm installed successfully.".green());
    } else {
        panic!("Failed to install dwm.");
    }
}

fn setup_vim() {
let vimrc_url = "https://raw.githubusercontent.com/aayushx402/MyVim/main/vimrc";
    let vimrc_path = "/etc/vimrc";

    // Check if Vim is installed
    let vim_check = Command::new("vim")
        .arg("--version")
        .output();

    if vim_check.is_err() {
        println!("{}", "Vim is not installed on your system.".red());
        println!("Do you want to install Vim? (y/n):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            install_vim();
        } else {
            println!("{}", "Vim installation skipped.".yellow());
            return;
        }
    }

    println!("{}", "Downloading Vim configuration...".bold().blue());
    download_vimrc(vimrc_url, vimrc_path);
}

fn install_vim() {
    let package_manager_check = Command::new("sh")
        .arg("-c")
        .arg("which apt-get || which pacman")
        .output()
        .expect("Failed to determine the package manager");

    let package_manager = String::from_utf8_lossy(&package_manager_check.stdout);

    if package_manager.contains("apt-get") {
        let install_status = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("vim")
            .status()
            .expect("Failed to install Vim with apt-get");

        if !install_status.success() {
            panic!("{}", "Failed to install Vim using apt-get.".red());
        }
        println!("{}", "Vim installed successfully using apt-get.".green());
    } else if package_manager.contains("pacman") {
        let install_status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("vim")
            .status()
            .expect("Failed to install Vim with pacman");

        if !install_status.success() {
            panic!("{}", "Failed to install Vim using pacman.".red());
        }
        println!("{}", "Vim installed successfully using pacman.".green());
    } else {
        println!("{}", "Unsupported package manager.".red());
    }
}

fn download_vimrc(vimrc_url: &str, vimrc_path: &str) {
    let response = reqwest::blocking::get(vimrc_url).unwrap();
    if response.status().is_success() {
        let mut file = File::create("temp_vimrc").unwrap();
        file.write_all(&response.bytes().unwrap()).unwrap();

        let status = Command::new("sudo")
            .arg("cp")
            .arg("temp_vimrc")
            .arg(vimrc_path)
            .status()
            .expect("Failed to copy new Vim config");
        if status.success() {
            println!("{}", "Vim configuration file replaced successfully.".green());
        } else {
            println!("{}", "Failed to replace Vim configuration file.".red());
            panic!("Failed to replace Vim configuration file.");
        }

        fs::remove_file("temp_vimrc").unwrap();
    } else {
        println!("{}", "Failed to download the Vim configuration file.".red());
    }
}

fn choose_neovim_plugin_manager() {
    let options = vec![
        "vim-plug",
        "packer.nvim",
        "Skip"
    ];

    loop {
        // Display plugin manager options using dialoguer
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your Neovim plugin manager:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Match the user's selection and handle accordingly
        match selection {
            0 => {
                install_vim_plug();
                break;
            },
            1 => {
                install_packer_nvim();
                break;
            },
            2 => {
                println!("{}", "Skipping plugin manager selection.".yellow());
                break;
            },
            _ => {
                println!("{}", "Invalid choice.".red());
                continue;
            }
        }
    }
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

fn choose_browser() {
    let options = vec![
        "Brave",
        "Firefox",
        "Thorium",
        "Exit"
    ];

    loop {
        // Display browser options using dialoguer
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your favorite browser:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Match the user's selection and handle accordingly
        match selection {
            0 => install_browser("brave"),
            1 => install_browser("firefox"),
            2 => install_browser("thorium"),
            3 => {
                println!("{}", "Exiting browser selection.".yellow());
                return; // Return to main menu
            },
            _ => {
                println!("{}", "Invalid choice.".red());
                continue;
            }
        }
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
    println!("\n{}", "Installing Packages...".bold().blue());

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
        "nemo",
        "github-desktop",
        "telegram-desktop",
        "gedit",
        "neovim",
        "vim",
        "discord",
    ];

    println!("The following packages will be installed:");
    for package in &packages {
        println!("{}", package);
    }

    print!("{}","Do you want to proceed with the installation? (y/n): ".bold().blue());
    io::stdout().flush().unwrap();  // Ensure the prompt is printed immediately

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim().eq_ignore_ascii_case("y") {
        for package in &packages {
            match package_manager {
                "pacman" => install_with_pacman(package),
                "apt" => install_with_apt(package),
                _ => println!("{}", "Unsupported package manager.".red()),
            }
        }
    } else {
        println!("{}", "Installation cancelled.".yellow());
    }
}

fn choose_and_apply_grub_theme() {
    let options = vec![
        "Catppuccin Macchiato",
        "CyberEXS",
        "Skip"
    ];

    loop {
        // Display GRUB theme options using dialoguer
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a GRUB theme to install and apply:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Match the user's selection and handle accordingly
        match selection {
            0 => install_grub_theme("https://github.com/catppuccin/grub.git", "catppuccin-macchiato-grub-theme"),
            1 => install_grub_theme("https://github.com/HenriqueLopes42/themeGrub.CyberEXS.git", "CyberEXS"),
            2 => {
                println!("{}", "Skipping GRUB theme selection.".yellow());
                return; // Return to main menu
            },
            _ => {
                println!("{}", "Invalid choice.".red());
                continue;
            }
        }
    }
}

fn install_grub_theme(repo_url: &str, theme_name: &str) {
    println!("{}", "Cloning GRUB theme repository...".bold().blue());

    let local_path = Path::new("/tmp/grub-theme");
    if local_path.exists() {
        fs::remove_dir_all(local_path).unwrap();
    }

    match Repository::clone(repo_url, local_path) {
        Ok(_) => println!("{}", "Repository cloned successfully.".green()),
        Err(e) => panic!("Failed to clone repository: {}", e),
    }

    let theme_path = local_path.join("src").join(theme_name);
    let grub_theme_dir = Path::new("/usr/share/grub/themes").join(theme_name);

    if !grub_theme_dir.exists() {
        fs::create_dir_all(&grub_theme_dir).expect("Failed to create theme directory.");
    }

    // Copy the theme to the GRUB theme directory
    let status = Command::new("sudo")
        .arg("cp")
        .arg("-r")
        .arg(theme_path.display().to_string())
        .arg(grub_theme_dir.display().to_string())
        .status()
        .expect("Failed to execute sudo command to copy GRUB theme");
    if status.success() {
        println!("{}", "GRUB theme copied successfully.".green());
    } else {
        println!("{}", "Failed to copy GRUB theme with sudo.".red());
        panic!("Failed to copy GRUB theme with sudo.");
    }

    // Define the path to the new theme
    let theme_path_in_grub = format!("{}/theme.txt", grub_theme_dir.display());

    // Update the GRUB configuration file to use the new theme
    let grub_config_path = "/etc/default/grub";
    let new_grub_theme_line = format!(
        r#"GRUB_THEME="{}""#,
        theme_path_in_grub
    );

    // Update the GRUB configuration file with the new theme line
    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(format!(
            r#"
            sed -i 's|^GRUB_THEME=.*|{}|' {}
            "#,
            new_grub_theme_line,
            grub_config_path
        ))
        .status()
        .expect("Failed to execute sudo command to update GRUB theme in config file");
    if status.success() {
        println!("{}", "GRUB configuration updated successfully.".green());
    } else {
        println!("{}", "Failed to update GRUB configuration with sudo.".red());
        panic!("Failed to update GRUB configuration with sudo.");
    }

    // Regenerate the GRUB configuration
    let status = Command::new("sudo")
        .arg("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .status()
        .expect("Failed to execute sudo command to update GRUB");
    if status.success() {
        println!("{}", "GRUB configuration regenerated successfully.".green());
    } else {
        println!("{}", "Failed to regenerate GRUB configuration with sudo.".red());
        panic!("Failed to regenerate GRUB configuration with sudo.");
    }
}


fn setup_sddm_theme() {
    let theme_url = "https://github.com/catppuccin/sddm/releases/download/v1.0.0/catppuccin-mocha.zip";
    let theme_zip_path = "/tmp/catppuccin-mocha.zip";
    let _theme_dir = "/usr/share/sddm/themes/catppuccin-mocha";

    // Download the theme zip file
    println!("{}", "Downloading SDDM theme...".bold().blue());
    let status = Command::new("wget")
        .arg(theme_url)
        .arg("-O")
        .arg(theme_zip_path)
        .status()
        .expect("Failed to download SDDM theme");
    if !status.success() {
        panic!("Failed to download SDDM theme.");
    }

    // Unzip the downloaded file
    println!("{}", "Unzipping SDDM theme...".bold().blue());
    let status = Command::new("unzip")
        .arg(theme_zip_path)
        .arg("-d")
        .arg("/tmp")
        .status()
        .expect("Failed to unzip SDDM theme");
    if !status.success() {
        panic!("Failed to unzip SDDM theme.");
    }

    // Copy the unzipped theme to the SDDM themes directory
    println!("{}", "Copying SDDM theme to /usr/share/sddm/themes/...".bold().blue());
    let status = Command::new("sudo")
        .arg("cp")
        .arg("-r")
        .arg("/tmp/catppuccin-mocha")
        .arg("/usr/share/sddm/themes/")
        .status()
        .expect("Failed to copy SDDM theme");
    if !status.success() {
        panic!("Failed to copy SDDM theme.");
    }

    // Update the SDDM configuration file to apply the new theme
    println!("{}", "Updating SDDM configuration...".bold().blue());
    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg("sed -i 's/^Current=.*/Current=catppuccin-mocha/' /etc/sddm.conf")
        .status()
        .expect("Failed to update SDDM configuration");
    if !status.success() {
        panic!("Failed to update SDDM configuration.");
    }

    // Notify the user that the theme has been applied
    println!("{}", "SDDM theme applied: catppuccin-mocha".green());
}

fn setup_fonts() {
    let options = vec![
        "FiraCode",
        "FiraMono",
        "JetBrainsMono",
        "Meslo",
        "Hack",
        "Skip"
    ];

    loop {
        // Display font options using dialoguer
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a font to install:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Match the user's selection and handle accordingly
        match selection {
            0 => install_font("FiraCode", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/FiraCode.zip"),
            1 => install_font("FiraMono", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/FiraMono.zip"),
            2 => install_font("JetBrainsMono", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/JetBrainsMono.zip"),
            3 => install_font("Meslo", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Meslo.zip"),
            4 => install_font("Hack", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Hack.zip"),
            5 => {
                println!("{}", "Skipping font installation.".yellow());
                return;  // Return to main menu
            },
            _ => {
                println!("{}", "Invalid choice.".red());
                continue;
            }
        }
    }
}

fn install_font(font_name: &str, font_url: &str) {
    let font_dir = format!("{}/.local/share/fonts", std::env::var("HOME").unwrap());
    let font_zip_path = format!("/tmp/{}.zip", font_name);

    println!("{}", format!("Downloading {} font...", font_name).bold().blue());
    let status = Command::new("wget")
        .arg(font_url)
        .arg("-O")
        .arg(&font_zip_path)
        .status()
        .expect("Failed to download font");
    if !status.success() {
        panic!("Failed to download font.");
    }

    println!("{}", "Unzipping font...".bold().blue());
    let status = Command::new("unzip")
        .arg(&font_zip_path)
        .arg("-d")
        .arg("/tmp")
        .status()
        .expect("Failed to unzip font");
    if !status.success() {
        panic!("Failed to unzip font.");
    }

    println!("{}", "Moving font to fonts directory...".bold().blue());
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("mv /tmp/{}* {}", font_name, font_dir))
        .status()
        .expect("Failed to move font");
    if !status.success() {
        panic!("Failed to move font.");
    }

    println!("{}", "Reloading font cache...".bold().blue());
    let status = Command::new("fc-cache")
        .arg("-fv")
        .status()
        .expect("Failed to reload font cache");
    if !status.success() {
        panic!("Failed to reload font cache.");
    }

    println!("{}", format!("{} font applied successfully.", font_name).green());
}


fn setup_rofi() {
// Check if Rofi is installed
let check_rofi = Command::new("pacman")
.arg("-Qs")
.arg("rofi")
.output()
.expect("Failed to check if Rofi is installed");

if !check_rofi.status.success() {
println!("Rofi is not installed. Installing it now...");
let install_rofi = Command::new("sudo")
    .arg("pacman")
    .arg("-S")
    .arg("--noconfirm")
    .arg("rofi")
    .output()
    .expect("Failed to install Rofi");

if !install_rofi.status.success() {
    eprintln!("Error installing Rofi.");
    return;
}
} else {
println!("Rofi is already installed. Skipping installation...");
}

// Backup existing Rofi config if exists
let home_dir = env::var("HOME").unwrap();
let rofi_config_dir = format!("{}/.config/rofi", home_dir);
let backup_dir = format!("{}/.config/rofi.bak", home_dir);

if Path::new(&rofi_config_dir).exists() {
println!("Backing up existing Rofi configuration...");
let backup_status = Command::new("cp")
    .arg("-r")
    .arg(&rofi_config_dir)
    .arg(&backup_dir)
    .output()
    .expect("Failed to backup Rofi config");

if !backup_status.status.success() {
    eprintln!("Failed to create backup: {:?}", backup_status.stderr);
    return;
}
}

// Create the Rofi config directory
println!("Setting up Rofi configuration...");
if let Err(e) = create_dir_all(&rofi_config_dir) {
eprintln!("Failed to create Rofi config directory: {}", e);
return;
}

// Download the new Rofi config and themes
let config_url = "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/rofi/config.rasi";
let theme_dir = format!("{}/themes", rofi_config_dir);
let nord_theme_url = "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/rofi/themes/nord.rasi";
let sidetab_nord_theme_url = "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/rofi/themes/sidetab-nord.rasi";

if let Err(e) = create_dir_all(&theme_dir) {
eprintln!("Failed to create Rofi themes directory: {}", e);
return;
}

let download_commands = vec![
("config.rasi", config_url),
("themes/nord.rasi", nord_theme_url),
("themes/sidetab-nord.rasi", sidetab_nord_theme_url),
];

for (file, url) in download_commands {
let output_file = format!("{}/{}", rofi_config_dir, file);
let wget = Command::new("wget")
    .arg("-O")
    .arg(&output_file)
    .arg(url)
    .output();

match wget {
    Ok(_) => println!("Downloaded {}", file),
    Err(e) => eprintln!("Failed to download {}: {}", file, e),
}
}

// Notify the user setup is complete
println!("Rofi setup complete!");

}

fn setup_alacritty() {
    // Check if Alacritty is installed
    let check_alacritty = Command::new("pacman")
        .arg("-Qs")
        .arg("alacritty")
        .output()
        .expect("Failed to check if Alacritty is installed");

    if !check_alacritty.status.success() {
        println!("Alacritty is not installed. Installing it now...");
        let install_alacritty = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("alacritty")
            .output()
            .expect("Failed to install Alacritty");

        if !install_alacritty.status.success() {
            eprintln!("Error installing Alacritty.");
            return;
        }
    } else {
        println!("Alacritty is already installed. Skipping installation...");
    }

    // Backup existing Alacritty config if exists
    let home_dir = env::var("HOME").unwrap();
    let alacritty_config_dir = format!("{}/.config/alacritty", home_dir);
    let backup_dir = format!("{}/.config/alacritty-bak", home_dir);

    if Path::new(&alacritty_config_dir).exists() {
        println!("Backing up existing Alacritty configuration...");
        let backup_status = Command::new("cp")
            .arg("-r")
            .arg(&alacritty_config_dir)
            .arg(&backup_dir)
            .output()
            .expect("Failed to backup Alacritty config");

        if !backup_status.status.success() {
            eprintln!("Failed to create backup: {:?}", backup_status.stderr);
            return;
        }
    }

    // Create the Alacritty config directory
    println!("Setting up Alacritty configuration...");
    if let Err(e) = create_dir_all(&alacritty_config_dir) {
        eprintln!("Failed to create Alacritty config directory: {}", e);
        return;
    }

    // Download the new Alacritty config files
    let alacritty_config_url = "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/alacritty/alacritty.toml";
    let nordic_theme_url = "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/alacritty/nordic.toml";

    let download_commands = vec![
        ("alacritty.toml", alacritty_config_url),
        ("nordic.toml", nordic_theme_url),
    ];

    for (file, url) in download_commands {
        let output_file = format!("{}/{}", alacritty_config_dir, file);
        let wget = Command::new("wget")
            .arg("-O")
            .arg(&output_file)
            .arg(url)
            .output();

        match wget {
            Ok(_) => println!("Downloaded {}", file),
            Err(e) => eprintln!("Failed to download {}: {}", file, e),
        }
    }

    // Notify the user setup is complete
    println!("Alacritty setup complete!");
}

fn setup_neovim() {

        println!("{}", "Important: If you have an existing Neovim configuration, make sure to back it up before proceeding".bold().red());
        print!("{}", "Do you want to continue with the Neovim setup (y/n): ".bold().white());
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read user input");
        let response = response.trim().to_lowercase();
    
        if response != "y" {
            println!("Neovim setup canceled.");
            return;
        }

    let check_neovim = Command::new("pacman")
        .arg("-Qi")
        .arg("neovim")
        .status()
        .expect("Failed to check if Neovim is installed");

    if !check_neovim.success() {
        println!("Neovim not found, installing Neovim...");
        Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("neovim")
            .arg("--noconfirm")
            .status()
            .expect("Failed to install Neovim");
    } else {
        println!("Neovim is already installed. Skipping installation...");
    }


    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let toolbox_dir = home_dir.join("TOOLBOX");
    if !toolbox_dir.exists() {
        println!("Creating $HOME/TOOLBOX directory...");
        fs::create_dir_all(&toolbox_dir).expect("Failed to create $HOME/TOOLBOX directory");
    }

    // Create $TOOLBOX/neovim directory if it doesn't exist
    let toolbox_neovim_dir = toolbox_dir.join("neovim");
    if !toolbox_neovim_dir.exists() {
        fs::create_dir_all(&toolbox_neovim_dir).expect("Failed to create $TOOLBOX/neovim directory");
    }

    //  Backup existing .config/nvim if it exists
    let nvim_config_dir = home_dir.join(".config/nvim");
    if nvim_config_dir.exists() {
        println!("Backing up existing Neovim configuration...");
        let backup_dir = toolbox_neovim_dir.join("nvim-backup");
        if backup_dir.exists() {
            fs::remove_dir_all(&backup_dir).expect("Failed to remove old backup");
        }
        fs::rename(&nvim_config_dir, &backup_dir).expect("Failed to move nvim directory to backup");
        println!("Neovim configuration moved to $TOOLBOX/neovim.");
    }

    let repo_url = "https://github.com/aayushx402/neovim";
    let clone_dir = Path::new("/tmp/neovim-repo");
    if clone_dir.exists() {
        fs::remove_dir_all(clone_dir).unwrap();
    }

    println!("Cloning Neovim configuration repository...");
    match Repository::clone(repo_url, clone_dir) {
        Ok(_) => println!("Repository cloned successfully."),
        Err(e) => panic!("Failed to clone repository: {}", e),
    }

    let nvim_target_dir = home_dir.join(".config/nvim");
    if nvim_target_dir.exists() {
        fs::remove_dir_all(&nvim_target_dir).expect("Failed to remove old nvim config");
    }
    fs::create_dir_all(&nvim_target_dir).expect("Failed to create .config/nvim directory");

    for entry in fs::read_dir(clone_dir).expect("Failed to read cloned repository") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let dest = nvim_target_dir.join(entry.file_name());
        if path.is_dir() {
            fs::rename(&path, &dest).expect("Failed to move directory to .config/nvim");
        } else {
            fs::copy(&path, &dest).expect("Failed to copy file to .config/nvim");
        }
    }

    //  Share system clipboard with unnamedplus
    let os_release_path = Path::new("/etc/os-release");
    if os_release_path.exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(r#"
                source /etc/os-release
                if [[ $XDG_SESSION_TYPE == "wayland" ]]; then
                    echo "wl-clipboard"
                else
                    echo "xclip"
                fi
            "#)
            .output()
            .expect("Failed to determine clipboard manager");

        let clipboard_pkg = String::from_utf8_lossy(&output.stdout).trim().to_string();

        println!("Installing clipboard packages and other Neovim dependencies...");
        Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("ripgrep")
            .arg("fzf")
            .arg(&clipboard_pkg)
            .arg("neovim")
            .arg("python-virtualenv")
            .arg("luarocks")
            .arg("go")
            .arg("shellcheck")
            .arg("--noconfirm")
            .status()
            .expect("Failed to install Neovim dependencies");
    } else {
        println!("Unable to determine OS. Please install the required packages manually.");
    }

    println!("Neovim setup completed successfully.");
}

fn setup_aurhelper() {
    // Define the AUR helper options
    let aur_helpers = vec![
        "󰞯 Paru",
        "󰜷 Yay",
        "󰒘 Skip and return to Main Menu"
    ];

    // Use dialoguer to create an interactive menu for AUR helpers
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("󰘵 Choose an AUR Helper ")
        .items(&aur_helpers)
        .default(0) // Default selection is the first item
        .interact()
        .unwrap();

    // Match the user's selection
    match selection {
        0 => install_aur_helper("paru"), // Install Paru
        1 => install_aur_helper("yay"),  // Install Yay
        2 => {
            println!("{}", "Skipping AUR helper setup and returning to the main menu.".yellow());
            return; // Return to the main menu when "Skip" is selected
        },
        _ => println!("{}", "Invalid selection.".red()),
    }
}

fn install_aur_helper(helper: &str) {
    match helper {
        "paru" => {
            println!("{}", "Installing paru...".cyan());
            let output = Command::new("sh")
                .arg("-c")
                .arg("sudo pacman -S --needed base-devel && git clone https://aur.archlinux.org/paru.git && cd paru && makepkg -si")
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                println!("{}", "Failed to install paru. Check the output for errors.".red());
                return;
            }

            println!("{}", "Successfully installed paru.".green());
        },
        "yay" => {
            println!("{}", "Installing yay...".cyan());
            let output = Command::new("sh")
                .arg("-c")
                .arg("sudo pacman -S --needed git base-devel && git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si")
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                println!("{}", "Failed to install yay. Check the output for errors.".red());
                return;
            }

            println!("{}", "Successfully installed yay.".green());
        },
        _ => println!("{}", "Invalid AUR helper selection.".red()),
    }
}

fn show_instructions() {
    let options = vec![
        "Show Instructions",
        "Skip and Return to Menu"
    ];

    loop {
        // Display instructions options using dialoguer
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Instructions Menu:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Handle the user's selection
        match selection {
            0 => {
                println!("\nInstructions:");
                println!("{}", "Use the arrow keys to navigate the options.".bold().white());
                println!("{}", "For example, select 'Setup Window Manager' to proceed with that setup.".bold().white());
                println!("{}", "You can return to this menu to review instructions or skip the setup.".bold().white());
            },
            1 => {
                println!("{}", "Returning to the main menu.".yellow());
                return; // Skip and return to the main menu
            },
            _ => {
                // This should never happen as there are only two valid options
                println!("{}", "Invalid choice. Please choose a valid option.".red());
                continue;
            }
        }
    }
}