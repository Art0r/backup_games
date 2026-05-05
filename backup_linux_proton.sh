#!/usr/bin/bash

# parameters
#   - $2 game files folder
#   - $3 save files folder

if [ $# -eq 0 ]; then
  echo "Nenhum argumento fornecido"
  exit 1
fi

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Argumento invalido"
  exit 1
fi

cd "$1" || exit

# primeiro dividimos o path fornecido pelo usuário para que possamos iterar
readarray -d "/" -t paths_array <<<"$1"

# para que possamos criar o os diretórios com cada path vamos começar a string com o $HOME
path_to_create="$HOME"

# itere os paths para criar o diretório caso não exista
# sempre adicionando o caminho ao $path_to_create
for path in "${paths_array[@]}"; do
  path_to_create="$path_to_create/$path"
  mkdir -p "$path_to_create"
done

bkp_name=$(date +"%Y%m%d%H%M")
bkp_name="/tmp/${bkp_name}.zip"

zip -r "${bkp_name}" "$1"

mv "${bkp_name}" "$2"
