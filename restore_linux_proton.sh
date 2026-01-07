#!/usr/bin/bash

function restore_proton_game() {
  # parameters
  #   - $1 bkp file name
  #   - $2 game folder to extract to

  bkp_name="${1,,}"

  # replace empty space for '_'
  bkp_name="${bkp_name// /_}"

  # adding file const name and extension
  bkp_name="/tmp/${bkp_name}_bkp.tar.gz"

  rm -rf "${2:?}/${1}"

  tar -xf "$bkp_name" -C "$2"
}

restore_proton_game "DarkSoulsIII" "$HOME/.local/share/Steam/steamapps/compatdata/374320/pfx/drive_c/users/steamuser/AppData/Roaming"

restore_proton_game "Dwarf Fortress" "$HOME/.local/share/Bay 12 Games"
