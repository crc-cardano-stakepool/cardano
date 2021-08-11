#!/bin/sh

scripts/build.sh

printf "%s" "commit: "
read -r commit_msg 

git stash save &&
git checkout main &&
git pull origin main && 
git checkout develop &&
git fetch origin main &&
git merge origin main &&
git push &&
git checkout develop &&
git stash apply &&
git add -A &&
git commit -a -s -m "$commit_msg" &&
git push origin develop &&
gh pr create -f -p Cardano -r clemenshorn -l enhancement &&
gh browse
