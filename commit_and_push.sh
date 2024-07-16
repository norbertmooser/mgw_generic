#!/bin/bash

# Define an arbitrary commit message
COMMIT_MSG="Automated commit and push including submodules"

# Function to push changes and handle upstream branch setting
push_changes() {
  if git push; then
    echo "Changes pushed successfully."
  else
    # If the push fails due to no upstream branch, set the upstream and push
    current_branch=$(git branch --show-current)
    git push --set-upstream origin "$current_branch"
  fi
}

# Stage all changes in the main repository
git add .

# Commit the changes with the specified message
git commit -m "$COMMIT_MSG"

# Push the changes to the remote repository
push_changes

# Update, commit, and push changes for each submodule
git submodule foreach '
    git add . &&
    git commit -m "$COMMIT_MSG" ||
    echo "No changes to commit in submodule $name" &&
    push_changes
'

echo "All changes committed and pushed, including submodules."
