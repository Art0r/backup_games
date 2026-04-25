#!/usr/bin/bash

# parameters
#   - $1 function [backup, restore]
#   - $2 game files folder
#   - $3 save files folder

if [ $# -eq 0 ]; then
  echo "Nenhum argumento fornecido"
  exit 1
fi

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
  echo "Argumento invalido"
  exit 1
fi

function backup_from_game() {
  bkp_name=$(date +"%Y%m%d%H%M")
  bkp_name="/tmp/${bkp_name}.zip"

  # Change to the parent directory of your game folder
  cd "$1" || return

  # Zip just the folder name (not full path)
  zip -9 -r "${bkp_name}" .

  mv "${bkp_name}" "$2"
}

function restore_from_file() {
  # parameters
  #   - $1 save files folder
  #   - $2 ziped file path

  rm -rf "$1"

  unzip -d "$1" "$2"
}

if [ "$1" = "backup" ]; then
  backup_from_game "$2" "$3"
fi

if [ "$1" = "restore" ]; then
  restore_from_file "$2" "$3"
fi
