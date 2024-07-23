## Linux Project
- This script customizes your Linux setup by grabbing my configs and replacing the default settings.

## Installation

The installation script is designed for Arch-based systems.

> CAUTION!
> This script configures default settings and adds my custom configs. It’s designed for Arch-based systems and currently supports SwayWM. If SwayWM isn't installed, the script will skip that section and apply my vimrc config and plugins instead.

After installing Vim and SwayWM, run:

```shell
sudo pacman -Syy git
git clone https://github.com/aayushx402/linux-project
cd linux-project/
chmod +x setup.sh
./setup.sh
```
If anything goes wrong run this into your terminal

```shell
sudo pacman -Syy git
git clone --depth 1 https://github.com/aayushx402/linux-project
cd linux-project/
chmod +x setup.sh
./setup.sh
```
## Update
- Added my vimrc and plugins
- Added Script For Downloading Fav Browser's
- Added Script For Some Useful Packages

### Tips for Further Enhancement:
1. **Add Colors:** You could add color codes to make it more eye-catching, but this may depend on the terminal you use.
2. **Use Emojis:** Adding emojis to indicate success or error can make the instructions more engaging. For example:
   - ✅ for success
   - ⚠️ for warnings

Here's how it might look with emojis:

```markdown
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
