#!/bin/bash

# Define an arbitrary commit message
COMMIT_MSG="Automated commit and push including submodules"

# Stage all changes in the main repository
git add .

# Commit the changes with the specified message
git commit -m "$COMMIT_MSG"

# Push the changes to the remote repository
git push

# Update, commit, and push changes for each submodule
git submodule foreach '
    git add . &&
    git commit -m "$COMMIT_MSG" ||
    echo "No changes to commit in submodule $name" &&
    git push
'

echo "All changes committed and pushed, including submodules."
