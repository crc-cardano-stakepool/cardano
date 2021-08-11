#!/bin/sh

printf "%s" "submodule url: "
read -r submodule_url

submodule="$(echo "$submodule_url" | awk -F '/' '{print $5}' )"

git submodule add "$submodule_url" contrib/"$submodule"