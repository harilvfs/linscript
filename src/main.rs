use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;
use std::env;
use std::fs::create_dir_all;
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        // Clear the terminal
        Command::new("clear").status().unwrap();

        println!(
            "{}",
            " ___       ___  ________   ___  ___     ___    ___   "
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "|\\  \\     |\\  \\|\\   ___  \\|\\  \\|\\  \\   |\\  \\  /  /|"
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "\\ \\  \\    \\ \\  \\ \\  \\\\ \\  \\ \\  \\\\   \\  \\ \\  \\/  / /"
                .bold()
                .yellow()
        );
        println!(
            "{}",
            " \\ \\  \\    \\ \\  \\ \\  \\\\ \\  \\ \\  \\\\   \\  \\ \\    / / "
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "  \\ \\  \\____\\ \\  \\ \\  \\\\ \\  \\ \\  \\\\   \\  /     \\/  "
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "   \\ \\_______\\ \\__\\ \\__\\\\ \\__\\ \\_______\\/  /\\   \\  "
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "    \\|_______|\\|__|\\|__| \\|__|\\|_______/__/ /\\ __\\ "
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "                                       |__|/ \\|__| "
                .bold()
                .yellow()
        );
        println!(
            "{}",
            "       Use arrow keys to navigate the menu 󰄼"
                .bold()
                .bright_blue()
        );
        println!(
            "{}",
            format!("            󰔚 Last Updated 2024-09-20     ",)
                .bold()
                .bright_green()
        );
        println!(
            "{}",
            format!("                      ").bold().bright_green()
        );

        let options = vec![
            " Setup Window Manager",
            " Install Browsers",
            " Install Packages",
            " Setup GRUB",
            " Setup SDDM",
            " Setup Fonts",
            " Setup Rofi",
            " Setup Alacritty",
            " Setup Kitty",
            " Setup Neovim",
            " Setup Fastfetch",
            " Install LTS Kernal",
            " Aur Helper",
            "󰸉 Nord Backgrounds",
            " Instructions",
            "󰿅 Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0)
            .interact();

        match selection {
            Ok(index) => match index {
                0 => setup_window_manager(),
                1 => choose_browser(),
                2 => install_useful_packages(),
                3 => choose_and_apply_grub_theme(),
                4 => setup_sddm_theme(),
                5 => setup_fonts(),
                6 => setup_rofi(),
                7 => setup_alacritty(),
                8 => setup_kitty(),
                9 => setup_neovim(),
                10 => setup_fastfetch(),
                11 => setup_ltskernal(),
                12 => setup_aurhelper(),
                13 => nord_backgrounds(),
                14 => show_instructions(),
                15 => {
                    println!("{}", "Exiting the program.".yellow());
                    break;
                }
                _ => {
                    println!("{}", "Invalid choice.".red());
                }
            },
            Err(_) => {
                println!("{}", "Error with selection.".red());
            }
        }
    }
}

fn setup_window_manager() {
    let options = vec!["󰞯 DWM", "󰒘 Skip and return to Main Menu"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your window manager:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                install_dwm();
                break;
            }
            1 => {
                println!("{}", "Exiting window manager setup.".yellow());
                break;
            }
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
        println!(
            "{}",
            "Directory already exists and is not empty. Skipping clone.".yellow()
        );
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

fn choose_browser() {
    let options = vec![
        "󰯯 Brave",
        " Firefox",
        " Thorium",
        "󰒘 Skip and return to Main Menu",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your favorite browser:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => install_browser("brave"),
            1 => install_browser("firefox"),
            2 => install_browser("thorium"),
            3 => {
                println!("{}", "Exiting browser selection.".yellow());
                return; // Return to main menu
            }
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
        println!(
            "{}",
            "Unsupported package manager. Please install the browser manually.".red()
        );
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
    println!(
        "{}",
        format!("Installing {} with pacman...", package)
            .bold()
            .blue()
    );
    Command::new("sh")
        .arg("-c")
        .arg(format!("sudo pacman -S --noconfirm {}", package))
        .status()
        .expect("Failed to install package with pacman");
    println!("{}", format!("{} installed successfully.", package).green());
}

fn install_with_apt(package: &str) {
    println!(
        "{}",
        format!("Installing {} with apt...", package).bold().blue()
    );
    Command::new("sh")
        .arg("-c")
        .arg(format!("sudo apt install -y {}", package))
        .status()
        .expect("Failed to install package with apt");
    println!("{}", format!("{} installed successfully.", package).green());
}

fn install_thorium_arch() {
    println!(
        "{}",
        "Installing Thorium browser on Arch-based system..."
            .bold()
            .blue()
    );
    Command::new("sh")
        .arg("-c")
        .arg("curl -L -o /tmp/thorium-browser-bin.tar.gz https://aur.archlinux.org/cgit/aur.git/snapshot/thorium-browser-bin.tar.gz && \
              tar -xvf /tmp/thorium-browser-bin.tar.gz -C /tmp && \
              cd /tmp/thorium-browser-bin && \
              makepkg -si --noconfirm")
        .status()
        .expect("Failed to install Thorium browser on Arch-based system");
    println!(
        "{}",
        "Thorium browser installed successfully on Arch-based system.".green()
    );
}

fn install_thorium_debian() {
    println!(
        "{}",
        "Installing Thorium browser on Debian-based system..."
            .bold()
            .blue()
    );
    Command::new("sh")
        .arg("-c")
        .arg("sudo rm -fv /etc/apt/sources.list.d/thorium.list && \
              sudo wget --no-hsts -P /etc/apt/sources.list.d/ http://dl.thorium.rocks/debian/dists/stable/thorium.list && \
              sudo apt update && \
              sudo apt install -y thorium-browser")
        .status()
        .expect("Failed to install Thorium browser on Debian-based system");
    println!(
        "{}",
        "Thorium browser installed successfully on Debian-based system.".green()
    );
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
        println!(
            "{}",
            "Unsupported package manager. Please install the packages manually.".red()
        );
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

    let choices = &["Yes", "No"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to proceed with the installation?")
        .items(choices)
        .default(0) // "Yes" is the default option
        .interact()
        .expect("Selection failed");

    if selection == 0 {  // User selected "Yes"
        for package in &packages {
            match package_manager {
                "pacman" => install_with_pacman(package),
                "apt" => install_with_apt(package),
                _ => println!("{}", "Unsupported package manager.".red()),
            }
        }
        println!("{}", "Installation completed successfully.".green());
    } else {
        println!("{}", "Installation cancelled.".yellow());
    }
}

fn choose_and_apply_grub_theme() {
    let options = vec![
        "󰔎 Catppuccin Macchiato",
        " CyberRe",
        " CyberEXS",
        "󰒘 Skip",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a GRUB theme to install and apply:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => install_catppuccin_macchiato_theme(),
            1 => install_grub_theme("https://github.com/aayushx402/i3-CatDotfiles", "CyberRe"),
            2 => install_cyberexs_theme(),
            3 => {
                println!("{}", "Skipping GRUB theme selection.".yellow());
                return;
            }
            _ => {
                println!("{}", "Invalid choice.".red());
                continue;
            }
        }
    }
}

fn install_catppuccin_macchiato_theme() {
    println!(
        "{}",
        "Cloning Catppuccin GRUB theme repository...".bold().blue()
    );

    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let local_path = home_dir.join("grub");
    if local_path.exists() {
        fs::remove_dir_all(&local_path).unwrap();
    }

    match Command::new("git")
        .arg("clone")
        .arg("https://github.com/catppuccin/grub.git")
        .arg(local_path.display().to_string())
        .status()
    {
        Ok(status) if status.success() => println!("{}", "Repository cloned successfully.".green()),
        _ => panic!("Failed to clone repository."),
    }

    let src_path = local_path.join("src");

    if !src_path.exists() {
        panic!("src directory not found: {:?}", src_path.display());
    }

    let theme_path = src_path.join("catppuccin-macchiato-grub-theme");

    if !theme_path.exists() {
        panic!("Theme directory not found: {:?}", theme_path.display());
    }

    let status = Command::new("sudo")
        .arg("cp")
        .arg("-r")
        .arg(theme_path.display().to_string())
        .arg("/usr/share/grub/themes/")
        .status()
        .expect("Failed to execute sudo command to copy GRUB theme");
    if status.success() {
        println!("{}", "GRUB theme copied successfully.".green());
    } else {
        println!("{}", "Failed to copy GRUB theme with sudo.".red());
        panic!("Failed to copy GRUB theme with sudo.");
    }

    let theme_path_in_grub =
        format!("/usr/share/grub/themes/catppuccin-macchiato-grub-theme/theme.txt");

    let grub_config_path = "/etc/default/grub";
    let new_grub_theme_line = format!(r#"GRUB_THEME="{}""#, theme_path_in_grub);

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(format!(
            r#"
            sed -i 's|^GRUB_THEME=.*|{}|' {}
            "#,
            new_grub_theme_line, grub_config_path
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
        println!(
            "{}",
            "Failed to regenerate GRUB configuration with sudo.".red()
        );
        panic!("Failed to regenerate GRUB configuration with sudo.");
    }
}

fn install_cyberexs_theme() {
    println!(
        "{}",
        "Cloning CyberEXS GRUB theme repository...".bold().blue()
    );

    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let local_path = home_dir.join("themeGrub.CyberEXS");
    if local_path.exists() {
        fs::remove_dir_all(&local_path).unwrap();
    }

    match Command::new("git")
        .arg("clone")
        .arg("https://github.com/HenriqueLopes42/themeGrub.CyberEXS.git")
        .arg(local_path.display().to_string())
        .status()
    {
        Ok(status) if status.success() => println!("{}", "Repository cloned successfully.".green()),
        _ => panic!("Failed to clone repository."),
    }

    let theme_dir = Path::new("/usr/share/grub/themes/CyberEXS");
    if !theme_dir.exists() {
        Command::new("sudo")
            .arg("mkdir")
            .arg("-p")
            .arg(theme_dir.display().to_string())
            .status()
            .expect("Failed to execute sudo command to create theme directory");
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && sudo cp -r * {}",
            local_path.display(),
            theme_dir.display()
        ))
        .status()
        .expect("Failed to execute shell command to copy GRUB theme");
    if status.success() {
        println!("{}", "GRUB theme copied successfully.".green());
    } else {
        println!("{}", "Failed to copy GRUB theme with sudo.".red());
        panic!("Failed to copy GRUB theme with sudo.");
    }

    let theme_path_in_grub = format!("/usr/share/grub/themes/CyberEXS/theme.txt");

    let grub_config_path = "/etc/default/grub";
    let new_grub_theme_line = format!(r#"GRUB_THEME="{}""#, theme_path_in_grub);

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(format!(
            r#"
            sed -i 's|^GRUB_THEME=.*|{}|' {}
            "#,
            new_grub_theme_line, grub_config_path
        ))
        .status()
        .expect("Failed to execute sudo command to update GRUB theme in config file");
    if status.success() {
        println!("{}", "GRUB configuration updated successfully.".green());
    } else {
        println!("{}", "Failed to update GRUB configuration with sudo.".red());
        panic!("Failed to update GRUB configuration with sudo.");
    }

    let status = Command::new("sudo")
        .arg("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .status()
        .expect("Failed to execute sudo command to update GRUB");
    if status.success() {
        println!("{}", "GRUB configuration regenerated successfully.".green());
    } else {
        println!(
            "{}",
            "Failed to regenerate GRUB configuration with sudo.".red()
        );
        panic!("Failed to regenerate GRUB configuration with sudo.");
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

    let theme_path = local_path.join("grub").join(theme_name);
    let grub_theme_dir = Path::new("/usr/share/grub/themes");

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

    let theme_path_in_grub = format!("{}/CyberRe/theme.txt", grub_theme_dir.display());

    let grub_config_path = "/etc/default/grub";
    let new_grub_theme_line = format!(r#"GRUB_THEME="{}""#, theme_path_in_grub);

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(format!(
            r#"
            sed -i 's|^GRUB_THEME=.*|{}|' {}
            "#,
            new_grub_theme_line, grub_config_path
        ))
        .status()
        .expect("Failed to execute sudo command to update GRUB theme in config file");
    if status.success() {
        println!("{}", "GRUB configuration updated successfully.".green());
    } else {
        println!("{}", "Failed to update GRUB configuration with sudo.".red());
        panic!("Failed to update GRUB configuration with sudo.");
    }

    let status = Command::new("sudo")
        .arg("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .status()
        .expect("Failed to execute sudo command to update GRUB");
    if status.success() {
        println!("{}", "GRUB configuration regenerated successfully.".green());
    } else {
        println!(
            "{}",
            "Failed to regenerate GRUB configuration with sudo.".red()
        );
        panic!("Failed to regenerate GRUB configuration with sudo.");
    }
}

fn setup_sddm_theme() {
    let theme_url =
        "https://github.com/catppuccin/sddm/releases/download/v1.0.0/catppuccin-mocha.zip";
    let theme_zip_path = "/tmp/catppuccin-mocha.zip";
    let _theme_dir = "/usr/share/sddm/themes/catppuccin-mocha";

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

    println!(
        "{}",
        "Copying SDDM theme to /usr/share/sddm/themes/..."
            .bold()
            .blue()
    );
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

    println!("{}", "SDDM theme applied: catppuccin-mocha".green());
}

fn setup_fonts() {
    let options = vec![
        " FiraCode",
        " FiraMono",
        " JetBrainsMono",
        " Meslo",
        " Hack",
        "󰒘 Skip",
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a font to install:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => install_font("FiraCode", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/FiraCode.zip"),
            1 => install_font("FiraMono", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/FiraMono.zip"),
            2 => install_font("JetBrainsMono", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/JetBrainsMono.zip"),
            3 => install_font("Meslo", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Meslo.zip"),
            4 => install_font("Hack", "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/Hack.zip"),
            5 => {
                println!("{}", "Skipping font installation.".yellow());
                return;
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

    println!(
        "{}",
        format!("Downloading {} font...", font_name).bold().blue()
    );
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

    println!(
        "{}",
        format!("{} font applied successfully.", font_name).green()
    );
}

fn setup_rofi() {
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

    println!("Setting up Rofi configuration...");
    if let Err(e) = create_dir_all(&rofi_config_dir) {
        eprintln!("Failed to create Rofi config directory: {}", e);
        return;
    }

    let config_url =
        "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/rofi/config.rasi";
    let theme_dir = format!("{}/themes", rofi_config_dir);
    let nord_theme_url =
        "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/rofi/themes/nord.rasi";
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

    println!("Setting up Alacritty configuration...");
    if let Err(e) = create_dir_all(&alacritty_config_dir) {
        eprintln!("Failed to create Alacritty config directory: {}", e);
        return;
    }

    let alacritty_config_url =
        "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/alacritty/alacritty.toml";
    let nordic_theme_url =
        "https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/alacritty/nordic.toml";

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

    println!("Alacritty setup complete!");
}

fn setup_kitty() {
    let kitty_installed = Command::new("which")
        .arg("kitty")
        .status()
        .unwrap()
        .success();

    if !kitty_installed {
        println!("Kitty not found. Installing with pacman...");
        Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("kitty")
            .arg("--noconfirm")
            .status()
            .expect("Failed to install Kitty");
        println!("Kitty installed.");
    } else {
        println!("Kitty is already installed. Skipping installation.");
    }

    let kitty_config_path = format!("{}/.config/kitty", dirs::home_dir().unwrap().display());
    let backup_path = format!("{}/.config/kitty-bak", dirs::home_dir().unwrap().display());

    if Path::new(&kitty_config_path).exists() {
        if Path::new(&backup_path).exists() {
            println!("Removing old backup at ~/.config/kitty-bak...");
            fs::remove_dir_all(&backup_path).expect("Failed to remove old kitty backup");
        }
        println!("Backing up existing kitty configuration...");
        fs::rename(&kitty_config_path, &backup_path).expect("Failed to backup kitty configuration");
        println!("Kitty configuration backed up to ~/.config/kitty-bak.");
    }

    fs::create_dir_all(&kitty_config_path).expect("Failed to create kitty config directory");

    println!("Downloading kitty configuration files...");
    Command::new("wget")
        .arg("-O")
        .arg(format!("{}/kitty.conf", &kitty_config_path))
        .arg("https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/kitty/kitty.conf")
        .status()
        .expect("Failed to download kitty.conf");

    Command::new("wget")
        .arg("-O")
        .arg(format!("{}/nord.conf", &kitty_config_path))
        .arg("https://raw.githubusercontent.com/aayushx402/dwm-ayx/main/config/kitty/nord.conf")
        .status()
        .expect("Failed to download nord.conf");

    println!("Kitty configuration applied successfully!");
}

fn setup_neovim() {
    println!("{}", "Important: If you have an existing Neovim configuration, make sure to back it up before proceeding".bold().red());
    print!(
        "{}",
        "Do you want to continue with the Neovim setup (y/n): "
            .bold()
            .white()
    );
    io::stdout().flush().expect("Failed to flush stdout");

    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read user input");
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

    let toolbox_neovim_dir = toolbox_dir.join("neovim");
    if !toolbox_neovim_dir.exists() {
        fs::create_dir_all(&toolbox_neovim_dir)
            .expect("Failed to create $TOOLBOX/neovim directory");
    }

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
            // Use a recursive function to copy the directory
            copy_dir_all(&path, &dest).expect("Failed to copy directory");
            fs::remove_dir_all(&path).expect("Failed to remove old directory");
        } else {
            // Copy files and then delete them
            fs::copy(&path, &dest).expect("Failed to copy file");
            fs::remove_file(&path).expect("Failed to remove old file");
        }
    }

    let os_release_path = Path::new("/etc/os-release");
    if os_release_path.exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(
                r#"
                source /etc/os-release
                if [[ $XDG_SESSION_TYPE == "wayland" ]]; then
                    echo "wl-clipboard"
                else
                    echo "xclip"
                fi
            "#,
            )
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

// Recursive function to copy directories
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn setup_fastfetch() {
    let home_dir = env::var("HOME").unwrap();
    let config_dir = format!("{}/.config/fastfetch", home_dir);
    let backup_dir = format!("{}/fastfetch_backup", home_dir);

    let fastfetch_installed = Command::new("which")
        .arg("fastfetch")
        .output()
        .expect("Failed to check Fastfetch installation")
        .status
        .success();

    if !fastfetch_installed {
        println!("Fastfetch not found, installing...");
        Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("fastfetch")
            .status()
            .expect("Failed to install Fastfetch");
    } else {
        println!("Fastfetch is already installed.");
    }

    if Path::new(&config_dir).exists() {
        println!("Fastfetch config directory already exists. Creating a backup...");

        if !Path::new(&backup_dir).exists() {
            create_dir_all(&backup_dir).expect("Failed to create backup directory");
        }

        let backup_path = format!("{}/fastfetch_backup", backup_dir);
        Command::new("mv")
            .arg(&config_dir)
            .arg(&backup_path)
            .status()
            .expect("Failed to move existing Fastfetch config to backup");

        println!("Moved existing Fastfetch config to: {}", backup_path);
    }

    println!("Generating new Fastfetch config...");
    Command::new("fastfetch")
        .arg("--gen-config")
        .status()
        .expect("Failed to generate Fastfetch config");

    env::set_current_dir(&config_dir).expect("Failed to change directory");

    let config_file = format!("{}/config.jsonc", config_dir);
    if Path::new(&config_file).exists() {
        println!("Removing autogenerated config.jsonc...");
        fs::remove_file(config_file).expect("Failed to remove config.jsonc");
    }

    println!("Cloning Fastfetch repository...");
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/aayushx402/fastfetch")
        .arg(".")
        .status()
        .expect("Failed to clone the Fastfetch repository");

    println!("Cleaning up unnecessary files...");
    let git_dir = format!("{}/.git", config_dir);
    let readme_file = format!("{}/README.md", config_dir);
    let license_file = format!("{}/LICENSE", config_dir);

    if Path::new(&git_dir).exists() {
        fs::remove_dir_all(&git_dir).expect("Failed to remove .git directory");
    }
    if Path::new(&readme_file).exists() {
        fs::remove_file(&readme_file).expect("Failed to remove README.md");
    }
    if Path::new(&license_file).exists() {
        fs::remove_file(&license_file).expect("Failed to remove LICENSE file");
    }

    let shell = env::var("SHELL").unwrap_or_else(|_| String::new());

    if shell.contains("bash") {
        let bashrc_path = format!("{}/.bashrc", home_dir);
        if let Ok(content) = fs::read_to_string(&bashrc_path) {
            if !content.contains("fastfetch") {
                println!("Adding Fastfetch to .bashrc...");
                fs::write(bashrc_path, format!("{}\nfastfetch\n", content))
                    .expect("Failed to write to .bashrc");
            }
        }
    } else if shell.contains("zsh") {
        let zshrc_path = format!("{}/.zshrc", home_dir);
        if let Ok(content) = fs::read_to_string(&zshrc_path) {
            if !content.contains("fastfetch") {
                println!("Adding Fastfetch to .zshrc...");
                fs::write(zshrc_path, format!("{}\nfastfetch\n", content))
                    .expect("Failed to write to .zshrc");
            }
        }
    } else if shell.contains("fish") {
        let fish_config_path = format!("{}/.config/fish/config.fish", home_dir);
        if let Ok(content) = fs::read_to_string(&fish_config_path) {
            if !content.contains("fastfetch") {
                println!("Adding Fastfetch to config.fish...");
                fs::write(fish_config_path, format!("{}\nfastfetch\n", content))
                    .expect("Failed to write to fish config");
            }
        }
    } else {
        println!(
            "Shell not recognized. Please add Fastfetch to your shell configuration manually."
        );
    }

    println!("Fastfetch setup completed. Please reopen the terminal.");
}

fn setup_ltskernal() {

    let kernel_output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to detect current kernel");
    let current_kernel = String::from_utf8_lossy(&kernel_output.stdout).trim().to_string();

    println!("\x1b[1;33mCurrent kernel version: {}\x1b[0m", current_kernel);
    
    println!("\x1b[1;36m\nMake sure to select the LTS kernel in GRUB when booting!\n\x1b[0m");

    let choices = &["Yes", "No"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to install the LTS kernel and remove the current kernel?")
        .items(choices)
        .default(1) 
        .interact()
        .expect("Selection failed");

    if selection == 0 {  
        let remove_command = format!("sudo pacman -Rnss linux-{}", current_kernel);
        Command::new("sh")
            .arg("-c")
            .arg(&remove_command)
            .status()
            .expect("Failed to remove existing kernel");

        println!("Current kernel removed: {}", current_kernel);

        Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("linux-lts")
            .status()
            .expect("Failed to install LTS kernel");

        println!("LTS kernel installed successfully!");
    } else {
        let install_only_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to install only the LTS kernel?")
            .items(choices)
            .default(0) 
            .interact()
            .expect("Selection failed");

        if install_only_selection == 0 {  
            Command::new("sudo")
                .arg("pacman")
                .arg("-S")
                .arg("linux-lts")
                .status()
                .expect("Failed to install LTS kernel");

            println!("LTS kernel installed successfully!");
        } else {
            println!("No changes made to the kernel.");
        }
    }
}

fn setup_aurhelper() {
    let aur_helpers = vec!["󰞯 Paru", "󰜷 Yay", "󰒘 Skip and return to Main Menu"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("󰘵 Choose an AUR Helper ")
        .items(&aur_helpers)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => install_aur_helper("paru"), // Install Paru
        1 => install_aur_helper("yay"),  // Install Yay
        2 => {
            println!(
                "{}",
                "Skipping AUR helper setup and returning to the main menu.".yellow()
            );
        }
        _ => println!("{}", "Invalid selection.".red()),
    }
}

fn install_aur_helper(helper: &str) {
    match helper {
        "paru" => {
            println!("{}", "Installing paru...".cyan());

            let status = Command::new("sudo")
                .arg("pacman")
                .arg("-S")
                .arg("--needed")
                .arg("base-devel")
                .status()
                .expect("Failed to install base-devel");
            if !status.success() {
                println!("{}", "Failed to install base-devel.".red());
                return;
            }

            let status = Command::new("git")
                .arg("clone")
                .arg("https://aur.archlinux.org/paru.git")
                .status()
                .expect("Failed to clone paru repository");
            if !status.success() {
                println!("{}", "Failed to clone paru repository.".red());
                return;
            }

            let status = Command::new("sh")
                .arg("-c")
                .arg("cd paru && makepkg -si")
                .status()
                .expect("Failed to build and install paru");
            if !status.success() {
                println!("{}", "Failed to build and install paru.".red());
                return;
            }

            println!("{}", "Successfully installed paru.".green());
        }
        "yay" => {
            println!("{}", "Installing yay...".cyan());

            let status = Command::new("sudo")
                .arg("pacman")
                .arg("-S")
                .arg("--needed")
                .arg("git")
                .arg("base-devel")
                .status()
                .expect("Failed to install base-devel and git");
            if !status.success() {
                println!("{}", "Failed to install base-devel and git.".red());
                return;
            }

            let status = Command::new("git")
                .arg("clone")
                .arg("https://aur.archlinux.org/yay.git")
                .status()
                .expect("Failed to clone yay repository");
            if !status.success() {
                println!("{}", "Failed to clone yay repository.".red());
                return;
            }

            let status = Command::new("sh")
                .arg("-c")
                .arg("cd yay && makepkg -si")
                .status()
                .expect("Failed to build and install yay");
            if !status.success() {
                println!("{}", "Failed to build and install yay.".red());
                return;
            }

            println!("{}", "Successfully installed yay.".green());
        }
        _ => println!("{}", "Invalid AUR helper selection.".red()),
    }
}

fn nord_backgrounds() {
    println!("   ");
    println!(
        "{}",
        "This Nord background is from ChrisTitusTech."
            .bold()
            .white()
    );
    println!(
        "{}",
        "You can check out his GitHub: https://github.com/christitustech/"
            .bold()
            .white()
    );

    let options = vec!["Download Nord Background", "Back to Menu"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => {
            let home_dir = env::var("HOME").expect("Failed to find home directory");
            let pictures_dir = format!("{}/Pictures", home_dir);
            let nord_background_dir = format!("{}/nord-background", pictures_dir);

            if !fs::metadata(&pictures_dir).is_ok() {
                fs::create_dir_all(&pictures_dir).expect("Failed to create Pictures directory");
            }

            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "cd {} && git clone https://github.com/christitustech/nord-background",
                    pictures_dir
                ))
                .status()
                .expect("Failed to download Nord background");

            let git_dir = format!("{}/.git", nord_background_dir);
            if fs::metadata(&git_dir).is_ok() {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("rm -rf {}", git_dir))
                    .status()
                    .expect("Failed to remove .git directory");
            }

            println!("Nord background downloaded in ~/Pictures, and .git directory removed.");
        }
        1 => {
            println!("Returning to main menu...");
        }
        _ => println!("Invalid selection"),
    }
}

fn show_instructions() {
    let options = vec!["󰰃 Show Instructions", "󰒘 Skip and Return to Menu"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Instructions Menu:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("\nInstructions:");
                println!(
                    "{}",
                    "Use the arrow keys to navigate the options.".bold().white()
                );
                println!(
                    "{}",
                    "For example, select 'Setup Window Manager' to proceed with that setup."
                        .bold()
                        .white()
                );
                println!(
                    "{}",
                    "You can return to this menu to review instructions or skip the setup."
                        .bold()
                        .white()
                );
            }
            1 => {
                println!("{}", "Returning to the main menu.".yellow());
                return;
            }
            _ => {
                println!("{}", "Invalid choice. Please choose a valid option.".red());
                continue;
            }
        }
    }
}
