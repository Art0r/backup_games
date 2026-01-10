#!/usr/bin/bash

function compact_proton_game() {
  # parameters
  #   - $1 bkp file name
  #   - $2 game files folder
  #   - $3 save files folder

  bkp_name="${1,,}"

  # replace empty space for '_'
  bkp_name="${bkp_name// /_}"

  # adding file const name and extension
  bkp_name="${bkp_name}_bkp.tar.gz"

  cd "$2" || exit

  tar -czf "${bkp_name}" "$1"

  mv "$2/$bkp_name" "/tmp/"
  cp "/tmp/$bkp_name" "$HOME/Downloads"
}

# compact ds3
compact_proton_game "DarkSoulsIII" "$HOME/.local/share/Steam/steamapps/compatdata/374320/pfx/drive_c/users/steamuser/AppData/Roaming"

# compact dwarf fortress
compact_proton_game "Dwarf Fortress" "$HOME/.local/share/Bay 12 Games"
