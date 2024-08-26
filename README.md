# Linux Setup Script 🚀

[![Version](https://img.shields.io/github/v/release/aayushx402/linux-project?color=%230567ff&label=Latest%20Release&style=for-the-badge)](https://github.com/aayushx402/linux-project/releases/latest)

**Linux Project** This script offers an easy way to choose and set up various window managers, install useful packages, and seamlessly update GRUB with a beautiful theme. It simplifies your Linux environment setup and customization.


> [!CAUTION]
> This project is still in development, and you may encounter some bugs. Please feel free to submit an issue if you run into any problems: [Submit an Issue](https://github.com/aayushx402/linux-project/issues).

<h2>✨ Features</h2>
<ul>
    <li><strong>🔧 Personal Configurations:</strong> Automatically applies custom Vim configurations (<code>vimrc</code>) and plugins to enhance your editing experience.</li>
    <li><strong>🪟 Window Manager Installation:</strong> Easily install and set up popular window managers like <code>dwm</code>, <code>Sway</code>, and <code>i3</code> for a customized desktop environment.</li>
    <li><strong>🎨 GRUB Theme Customization:</strong> Switch your GRUB theme to the elegant <code>Catppuccin</code> theme with a single script.</li>
    <li><strong>📦 Package Installation:</strong> Installs a range of useful packages to boost your system's functionality and performance.</li>
    <li><strong>🔒 Secure Browsers:</strong> Simple installation of secure browsers like <code>Firefox</code>, <code>Brave</code>, and <code>Tor</code> for safer web browsing.</li>
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


