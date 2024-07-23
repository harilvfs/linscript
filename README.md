## 🌟 Linux Project
- This script customizes your Linux setup by grabbing my configs and replacing the default settings.

## 🚀 Installation

The installation script is designed for Arch-based systems.

> [!CAUTION]
> This script configures default settings and adds my custom configs. It’s designed for Arch-based systems and currently supports SwayWM. If SwayWM isn't installed, the script will skip that section and apply my vimrc config and plugins instead.

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
- Added my vimrc and plugins
- Added Script For Downloading Fav Browser's
- Added Script For Some Useful Packages


