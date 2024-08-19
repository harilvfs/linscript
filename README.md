🚀 Linux Script 
<p align="center">
  <a href="https://github.com/aayushx402/linux-project" target="_blank" rel="noreferrer">
    <img src="https://img.shields.io/badge/Check%20Out%20My%20Linux%20Script-Repo-61DAFB?style=for-the-badge&logo=github&logoColor=white" alt="Linux Project">
  </a>
</p>
<p align="center">
  <br>
<a href="https://github.com/aayushx402/linux-project/releases/tag/v0.4.0" target="_blank" rel="noreferrer">
<img src="https://img.shields.io/badge/Version-0.4-brightgreen?style=for-the-badge&logo=github&logoColor=white" alt="Version">
</a>
</p>

<h1 align="center">Customizable Linux Setup Script</h1>

<p align="center">Welcome to a versatile setup script designed for Arch-based systems. Effortlessly configure your Linux environment with my personal settings and additional features.</p>

<h2>✨ Features</h2>
<ul>
    <li><strong>🔧 Personal Configurations:</strong> Automatically applies my custom Vim configuration (<code>vimrc</code>) and plugins to enhance your editing experience.</li>
    <li><strong>🪟 SwayWM Support:</strong> Integrates seamlessly with SwayWM for a smooth and modern window management experience.</li>
    <li><strong>🎨 GRUB Theme Customization:</strong> Switch your GRUB theme to the elegant Catppuccin theme with a single script.</li>
    <li><strong>📦 Package Installation:</strong> Installs a range of useful packages to boost your system's functionality and performance.</li>
    <li><strong>🔒 Secure Browsers:</strong> Easy installation of secure browsers like Firefox, Brave, and Thorium for safer web browsing.</li>
</ul>

<h2>🚀 Getting Started</h2>
<p>Follow the instructions in the repository to quickly set up your system and apply your preferred configurations. Enjoy a streamlined and personalized Linux experience!</p>

## 🚀 Installation

The installation script is designed for Arch-based systems.

> [!IMPORTANT]
> This script sets up default configurations and incorporates my custom settings. It is tailored for Arch-based systems and is currently compatible with SwayWM. If SwayWM is not installed, the script will bypass that section and proceed with the remaining configurations.

To setup, execute the following commands:

```shell
sudo pacman -Syy git
git clone https://github.com/aayushx402/linux-project
cd linux-project/
chmod +x setup.sh
./setup.sh
```

Alternatively, you can run the binary file directly:

```shell
git clone https://github.com/aayushx402/linux-project
cd linux-project/
chmod +x toolbox
./toolbox
```

### Updating
To update script, you will need to pull the latest changes from GitHub and restore the configs by running the following commands:

```shell
cd ~/linux-project
git pull
./setup.sh
```

## 📈 Update Log
- Added custom Vim configuration (vimrc) and plugins.
- Included a script for downloading favorite browsers.
- Added a script for installing useful packages.
- Added a script for selecting the Catppuccin theme for your GRUB bootloader.


