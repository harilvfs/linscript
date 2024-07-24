## 🌟 Linux Project
- This script customizes your Linux setup by applying my configurations and replacing the default settings. It offers extensive customization options, allowing you to modify and minimize default settings, install various packages, browsers, and GRUB themes. The script will continue to evolve with new features and add-ons in future updates.

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

**If anything goes wrong, run the following commands in your terminal:**

```shell
# 🆙 Update your system and install git
sudo pacman -Syy git

# 📥 Clone the project repository
git clone --depth 1 https://github.com/aayushx402/linux-project

# 📂 Navigate to the project directory
cd linux-project/

# 🔒 Make the setup script executable
chmod +x setup.sh

# 🚀 Run the setup script
./setup.sh
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
- Implemented a script for selecting a GRUB theme.


