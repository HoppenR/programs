#!/bin/bash
set -u

## DESCRIPTION
# A simple nice looking script that checks if a git repository has
# unstaged/uncommitted changes. It also compares your local files to those in
# the corresponding git repository and lets you easily choose what to do when
# the two files differ. This script WILL stage ALL changed files in ALL git
# repositories automatically.

## ADDING A REPOSITORY TO SYNC
# To add a git repository to sync, you need to add a path to the repository
# location in $RepoLocations relative to the ~ (home) folder, this can be
# multiple folders at once since the script is using pushd and popd. Then you
# should add a path to the $RepoFilesLocations pointing to the location of the
# files to sync inside of the repository, the path needs to be relative to the
# path you put in $RepoLocations. This can be "." if you want to stay in the
# repo "root" folder or if you want to use full paths inside of your
# $MyReposFilesArray instead. Which is explained below.
#
# You also need to create an array containing all the repository files to be
# synced inside of the RepoFileLocation, and an array for the corresponding
# local files, using the same index location as the corresponding file in the
# other array. You also need to add those arrays to $ReposArrays and
# $LocalArrays using the template shown below.
#
# ReposArrays=( [n]=${MyReposFiles[@]} ) and same for local
# LocalArrays=( [n]=${MyLocalFiles[@]} )

### ------------------ CONFIGURATION VARIABLES ------------------
## Repository Locations
# This should have 1 entry per repo.
declare -a RepoLocations=(config-files/ programs/)
## Repository File Locations
# This should have 1 entry per repo.
declare -a RepoFilesLocations=("$HOSTNAME/" ./)

## SYNC #1
# config-files repository files
declare -a ConfReposFiles=(.zshrc picom.conf i3/config DIR_COLORS dunstrc
.gtkrc-2.0 highlight.theme i3status/config lightdm-gtk-greeter.conf personal.vim
.profile rc.conf rifle.conf scope.sh settings.ini init.lua plugins.lua
alacritty.yml .Xresources)
# config-files corresponding local files
declare -a ConfLocalFiles=(~/.config/zsh/.zshrc ~/.config/picom.conf
~/.config/i3/config ~/.config/DIR_COLORS ~/.config/dunst/dunstrc ~/.gtkrc-2.0
~/.config/highlight/highlight.theme ~/.config/i3status/config
/etc/lightdm/lightdm-gtk-greeter.conf ~/.config/nvim/colors/personal.vim
~/.profile ~/.config/ranger/rc.conf ~/.config/ranger/rifle.conf
~/.config/ranger/scope.sh ~/.config/gtk-3.0/settings.ini ~/.config/nvim/init.lua
~/.config/nvim/lua/plugins.lua ~/.config/alacritty/alacritty.yml ~/.Xresources)

## SYNC #n
#declare -a MyReposFiles(File1 File2)
#declare -a MyLocalFiles(/path/File1 /path/File2)

## $ReposArrays and $LocalArrays
# Arrays containing strings of each repository- and each local-array.
# These strings are instantly converted back into arrays later because bash
# does not support two-dimensional arrays.
declare -a ReposArrays=("${ConfReposFiles[*]}" "" "" "")
declare -a LocalArrays=("${ConfLocalFiles[*]}" "" "" "")
### ---------------- END CONFIGURATION VARIABLES ----------------

## Choose what to do when two files differ
# $1 = repo file, $2 = local file
function actionFileDiffers {
    select answer in "copy from local to repo" \
        "copy from local to repo and commit" \
        "vimdiff files" \
        "copy from repo to local" \
        "skip"
    do
        case "$answer" in
            "copy from local to repo")
                cp -i "$2" "$1"
                break
                ;;
            "copy from local to repo and commit")
                cp -i "$2" "$1"
                git commit --all
                break
                ;;
            "vimdiff files")
                vimdiff "$1" "$2"
                ;;
            "copy from repo to local")
                cp -i "$1" "$2"
                break
                ;;
            "skip")
                echo "Skipping..."
                break
                ;;
            *) echo "try again"
                ;;
        esac
    done
    return 0
}

## Choose what to do when a repository's HEAD differs from its staged files
function actionRepoDiffers {
    select answer in "commit all files" \
        "hard reset to HEAD" \
        "display full git diff" \
        "skip"
    do
        case "$answer" in
            "commit all files")
                git commit --all
                break
                ;;
            "hard reset to HEAD")
                read -erp "Are you sure you want to hard reset to HEAD? (y/N)" \
                    confirm
                case "$confirm" in
                    "y"|"yes")
                        git reset --hard HEAD
                        break
                        ;;
                    *)
                        echo "Cancelling..."
                        break
                        ;;
                esac
                break
                ;;
            "display full git diff")
                git diff --staged --minimal
                echo -e "1) commit all files\\t  3) display full git diff"
                echo -e "2) hard reset to HEAD\\t  4) skip"
                ;;
            "skip")
                echo "Skipping..."
                break
                ;;
            *)
                echo "try again"
                ;;
        esac
    done
    return 0
}

# underlineWord() prints and underlines a word by storing the amount of spaces
# in $line equal to the length of the first parameter passed to the function
# and then replaces each space with a ─
# $1 = word, $2 = insert tab (boolean)
function underlineWord {
    printf -v line "%${#1}s"
    if [[ "$2" == true ]]
    then
        echo -e "\\t$1\\n\\t${line// /─}"
    else
        echo -e "$1\\n${line// /─}"
    fi
    return 0
}

#### ----------------------- SCRIPT START -----------------------
cd ~ || exit 8
# Loop from 0 to length of $RepoLocations - 1
for i in $(seq 0 $(( ${#RepoLocations[@]} - 1 )))
do
    # Redirect the output to /dev/null to silently use pushd
    pushd "${RepoLocations[i]}" > /dev/null || exit 8
        # Only send the bottom level directory in pwd by using basename
        underlineWord "$(basename "$(pwd)")/" false
        # Stage all changed files for commit inside the git repo
        git add --all
        # Diffs the files staged for commit in comparison to HEAD
        if ! git diff --staged --stat --exit-code
        then
            actionRepoDiffers
        fi
        pushd "${RepoFilesLocations[i]}" > /dev/null || exit 8
            underlineWord "$(basename "$(pwd)")/" true
            # Convert the strings back into arrays
            read -ra CurReposArray <<< "${ReposArrays[i]}"
            read -ra CurLocalArray <<< "${LocalArrays[i]}"
            for j in $(seq 0 $(( ${#CurReposArray[@]} - 1 )))
            do
                rf="${CurReposArray[j]}"
                lf="${CurLocalArray[j]}"
                echo -e "\\t$rf"
                if ! git diff "$rf" "$lf"
                then
                    actionFileDiffers "$rf" "$lf"
                fi
            done
        popd > /dev/null || exit 8
    popd > /dev/null || exit 8
done
#### ------------------------ SCRIPT END ------------------------
