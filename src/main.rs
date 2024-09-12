use git2::Repository;
use std::fs::{self, File}; 
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::{self, Write, BufRead};
use std::env;
use colored::*;

fn main() {
    
    println!("{}", "                                         ");
    println!("{}", "███████╗███████╗████████╗██╗   ██╗██████╗".bold().green()); 
    println!("{}", "██╔════╝██╔════╝╚══██╔══╝██║   ██║██╔══██╗".bold().blue());
    println!("{}", "███████╗█████╗     ██║   ██║   ██║██████╔╝".bold().blue());
    println!("{}", "╚════██║██╔══╝     ██║   ██║   ██║██╔═══╝".bold().blue()); 
    println!("{}", "███████║███████╗   ██║   ╚██████╔╝██║".bold().green());     
    println!("{}", "╚══════╝╚══════╝   ╚═╝    ╚═════╝ ╚═╝".bold().blue());     
                                                     
    // Main Menu Options
    println!("{}", "\nChoose a setup option:".bold().blue());
    println!("{}", "1. Setup Window Manager".bold().cyan());
    println!("{}", "2. Setup Vim".bold().cyan());
    println!("{}", "3. Install Neovim Plugin Manager".bold().cyan());
    println!("{}", "4. Install Browser".bold().cyan());
    println!("{}", "5. Install Useful Packages".bold().cyan());
    println!("{}", "6. Setup GRUB".bold().cyan());
    println!("{}", "7. Setup SDDM".bold().cyan());
    println!("{}", "8. Setup Fonts".bold().cyan());
    println!("{}", "9. Exit (Press Ctrl+C or type 'exit')".bold().red()); // Updated exit message

    loop {
        // Capture input from user
        let mut choice = String::new();
        print!("{}", "Please enter your setup choice: ".bold().blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        if choice.trim().to_lowercase() == "exit" || choice.trim() == "\u{3}" { // Check for 'exit' or Ctrl+C
            println!("{}", "Exiting the program.".yellow());
            break;
        }
        match choice.trim() {
            "1" => setup_window_manager(),
            "2" => setup_vim(),
            "3" => choose_neovim_plugin_manager(),
            "4" => choose_browser(),
            "5" => install_useful_packages(),
            "6" => choose_and_apply_grub_theme(),
            "7" => setup_sddm_theme(),
            "8" => setup_fonts(),
            _ => {
                println!("{}", "Invalid choice. Please choose a valid option or type 'exit' to exit.".red());
            },
        }
    }
}

fn setup_window_manager() {
    println!("{}", "Setting up window manager:".bold().blue());
    println!("{}", "1. DWM".bold().cyan()); 
    println!("{}", "2. Skip".bold().cyan());

    println!("\nOptions:");
    println!("  d for DWM");
    println!("  e for Exit");

    println!("\nNote:");
    println!("{}", "If you want to use i3 or Sway, check my GitHub for dotfiles:".bold().red());
    println!("{}", "https://github.com/aayushx402".bold().blue().underline());

    let stdin = io::stdin();
    loop {
        let mut choice = String::new();
        print!("{}", "Which window manager do you want to install (d/e): ".bold().blue());
        io::stdout().flush().unwrap();
        stdin.lock().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().to_lowercase().as_str() {
            "d" => {
                install_dwm();
                break;
            },
            "e" => {
                println!("{}", "Skipping window manager setup.".yellow());
                break;
            },
            _ => println!("{}", "Invalid choice. Please choose a valid option (d/e).".red()),
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
    println!("\n{}", "Choose your Neovim plugin manager:".bold().blue());
    println!("{}", "1. vim-plug".bold().cyan());
    println!("{}", "2. packer.nvim".bold().cyan());
    println!("{}", "3. Skip".bold().cyan());

    println!("\nOptions:");
    println!("V for vim-plug");
    println!("P for packer.nvim");
    println!("S for Skip");

    let stdin = io::stdin();
    loop {
        let mut choice = String::new();
        print!("{}", "Enter your plugin manager choice (V/P/S): ".bold().blue());
        io::stdout().flush().unwrap();
        stdin.lock().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().to_uppercase().as_str() {
            "V" => {
                install_vim_plug();
                break;
            },
            "P" => {
                install_packer_nvim();
                break;
            },
            "S" => {
                println!("{}", "Skipping plugin manager selection.".yellow());
                break;
            },
            _ => println!("{}", "Invalid choice. Please choose a valid option (V/P/S).".red()),
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
    println!("\n{}", "Choose your favorite browser (B/F/T/E):".bold().blue());

    let stdin = io::stdin();
    loop {
        println!("{}", "1.Brave".bold().cyan());
        println!("{}", "2.Firefox".bold().cyan());
        println!("{}", "3.Thorium".bold().cyan());
        println!("{}", "4.Exit".bold().cyan());

        println!("\nOptions:");
        println!("B for Brave");
        println!("F for Firefox");
        println!("T for Thorium");
        println!("E for Exit");

        let mut choice = String::new();
        print!("{}", "Enter your browser choice: ".bold().blue());
        io::stdout().flush().unwrap();
        stdin.lock().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().to_uppercase().as_str() {
            "B" => {
                install_browser("brave");
                break;
            },
            "F" => {
                install_browser("firefox");
                break;
            },
            "T" => {
                install_browser("thorium");
                break;
            },
            "E" => {
                println!("{}", "Exiting browser selection.".yellow());
                break;
            },
            _ => println!("{}", "Invalid choice. Please choose a valid option (B/F/T/E).".red()),
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

    print!("\nDo you want to proceed with the installation? (y/n): ");
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
    println!("\n{}", "Choose a GRUB theme to install and apply:".bold().blue());
    println!("{}", "1. Catppuccin Macchiato".cyan());

    let stdin = io::stdin();
    let mut choice = String::new();
    stdin.lock().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => install_grub_theme("https://github.com/catppuccin/grub.git", "catppuccin-macchiato-grub-theme"),
        _ => println!("{}", "Invalid choice. Please run the program again and choose 1.".red()),
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
    let grub_theme_dir = Path::new("/usr/share/grub/themes");

    if theme_path.exists() && theme_path.read_dir().unwrap().next().is_some() {
        // Copy the theme to the GRUB theme directory
        let status = Command::new("sudo")
            .arg("cp")
            .arg("-r")
            .arg(&theme_path)
            .arg(grub_theme_dir)
            .status()
            .expect("Failed to execute sudo command to copy GRUB theme");
        if status.success() {
            println!("{}", "GRUB theme copied successfully.".green());
        } else {
            println!("{}", "Failed to copy GRUB theme with sudo.".red());
            panic!("Failed to copy GRUB theme with sudo.");
        }

        // Define the path to the new theme
        let theme_path_in_grub = format!("/usr/share/grub/themes/{}/theme.txt", theme_name);

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
    } else {
        println!("{}", "Theme directory does not exist or is empty.".red());
        panic!("Theme directory does not exist or is empty.");
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
    println!("{}", "Choose a font to install:".bold().blue());
    println!("{}", "1. FiraCode".cyan());
    println!("{}", "2. FiraMono".cyan());
    println!("{}", "3. JetBrainsMono".cyan());
    println!("{}", "4. Meslo".cyan());
    println!("{}", "5. Hack".cyan());

    let stdin = io::stdin();
    let mut choice = String::new();
    stdin.lock().read_line(&mut choice).expect("Failed to read line");

    let (font_name, font_url) = match choice.trim() {
        "1" => ("FiraCode", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/FiraCode.zip"),
        "2" => ("FiraMono", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/FiraMono.zip"),
        "3" => ("JetBrainsMono", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/JetBrainsMono.zip"),
        "4" => ("Meslo", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Meslo.zip"),
        "5" => ("Hack", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Hack.zip"),
        _ => {
            println!("{}", "Invalid choice. Please run the program again and choose a valid option.".red());
            return;
        }
    };

    let font_dir = format!("{}/.local/share/fonts", env::var("HOME").unwrap());
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
