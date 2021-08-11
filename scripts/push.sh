#!/bin/sh

./scripts/build.sh

printf "%s" "commit: "
read -r commit_msg 

git add -A &&
git commit -a -s -m "$commit_msg" &&
git push origin develop &&
gh pr create -f &&
sleep 1 &&
gh pr merge --auto -b main -m &&
git checkout main &&
git pull origin main && 
git checkout develop &&
git fetch origin main &&
git merge origin main &&
git push &&
gh browse