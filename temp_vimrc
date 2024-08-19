" All system-wide defaults are set in $VIMRUNTIME/archlinux.vim (usually just
" /usr/share/vim/vimfiles/archlinux.vim) and sourced by the call to :runtime
" you can find below.  If you wish to change any of those settings, you should
" do it in this file (/etc/vimrc), since archlinux.vim will be overwritten
" everytime an upgrade of the vim packages is performed.  It is recommended to
" make changes after sourcing archlinux.vim since it alters the value of the
" 'compatible' option.

" This line should not be removed as it ensures that various options are
" properly set to work with the Vim-related packages.
runtime! archlinux.vim

" If you prefer the old-style vim functionalty, add 'runtime! vimrc_example.vim'
" Or better yet, read /usr/share/vim/vim80/vimrc_example.vim or the vim manual
" and configure vim to your own liking!

" do not load defaults if ~/.vimrc is missing
"let skip_defaults_vim=1

" Specify a directory for plugins
call plug#begin('~/.vim/plugged')

" Appearance Plugins
Plug 'morhetz/gruvbox'       " Colorscheme
Plug 'vim-airline/vim-airline' " Statusline
Plug 'vim-airline/vim-airline-themes' " Themes for Airline
Plug 'ryanoasis/vim-devicons' " File icons

" Utility Plugins
Plug 'preservim/nerdtree'    " File explorer
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } } " Fuzzy Finder
Plug 'junegunn/fzf.vim'      " FZF Vim integration
Plug 'scrooloose/nerdcommenter' " Commenting utility
Plug 'Yggdroot/indentLine'   " Indentation guides

" Initialize plugin system
call plug#end()

" General settings
set number                   " Show line numbers
set relativenumber           " Relative line numbers
set background=dark          " Use dark background
syntax on                    " Enable syntax highlighting
set termguicolors            " Enable true color support

" Gruvbox colorscheme
colorscheme gruvbox

" Airline configuration
let g:airline_theme='gruvbox'
let g:airline_powerline_fonts = 1

" NERDTree settings
map <C-n> :NERDTreeToggle<CR>

" FZF settings
set rtp+=~/.fzf
command! -bang -nargs=* FZF
  \ call fzf#vim#files('', fzf#vim#with_preview(), <bang>0)

" IndentLine settings
let g:indentLine_char = '┆'
let g:indentLine_color_gui = '#A4E57E'

" Set leader key
let mapleader=" "

