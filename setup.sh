#!/bin/bash


echo -ne "

                  ████████╗ ██████╗  ██████╗ ██╗     ██████╗  ██████╗ ██╗  ██╗
                  ╚══██╔══╝██╔═══██╗██╔═══██╗██║     ██╔══██╗██╔═══██╗╚██╗██╔╝
                     ██║   ██║   ██║██║   ██║██║     ██████╔╝██║   ██║ ╚███╔╝ 
                     ██║   ██║   ██║██║   ██║██║     ██╔══██╗██║   ██║ ██╔██╗ 
                     ██║   ╚██████╔╝╚██████╔╝███████╗██████╔╝╚██████╔╝██╔╝ ██╗
                     ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝
-----------------------------------------------------------------------------------------------------
        This script lets you install a window manager and customize your setup.
-----------------------------------------------------------------------------------------------------                                                            
"
# Color theming
GREEN="\033[0;32m"
RED="\033[0;31m"
YELLOW="\033[1;33m"
NC="\033[0m" # No color

# Ensure the script is run as root (with sudo)
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}This script requires root privileges. Please run it with sudo.${NC}"
    exec sudo "$0" "$@"
    exit 1
fi

# Clone the repository
REPO_URL="https://github.com/aayushx402/linux-project"
REPO_NAME="linux-project"

echo -e "${GREEN}Cloning the repository from $REPO_URL...${NC}"
git clone "$REPO_URL" || { echo -e "${RED}Failed to clone the repository. Please check the URL or your internet connection.${NC}"; exit 1; }

# Navigate to the cloned repository directory
cd "$REPO_NAME" || { echo -e "${RED}Failed to navigate to the repository directory.${NC}"; exit 1; }

# Make the toolbox script executable
echo -e "${YELLOW}Making the toolbox script executable...${NC}"
chmod +x toolbox

# Run the toolbox script
echo -e "${GREEN}Running the toolbox script...${NC}"
./toolbox

echo -e "${GREEN}Setup and execution complete.${NC}"
