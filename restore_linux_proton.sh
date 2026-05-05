#!/usr/bin/bash

# parameters
#   - $1 save files folder
#   - $2 ziped file path

if [ $# -eq 0 ]; then
  echo "Nenhum argumento fornecido"
  exit 1
fi

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Argumento invalido"
  exit 1
fi

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

rm -rf "$1"

unzip "$1" -d "$2"
