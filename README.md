# Linux Setup Script 🚀

[![Version](https://img.shields.io/github/v/release/aayushx402/linux-project?color=%230567ff&label=Latest%20Release&style=for-the-badge)](https://github.com/aayushx402/linux-project/releases/latest)

**Linux Project** This script offers an easy way to choose and set up various window managers, install useful packages, and seamlessly update GRUB with a beautiful theme. It simplifies your Linux environment setup and customization.

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


