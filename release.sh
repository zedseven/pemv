#!/usr/bin/env bash

# Based on the release script for git-cliff, with additional adjustments

if [ -z "$1" ]; then
  echo 'Please provide the tag as an argument, in the format `vX.Y.Z`.'
  exit
fi

if [ -n "$(git status --porcelain)" ]; then
  echo 'You have uncommitted changes. Please clean up first.'
  exit
fi

TAG="$1"

# Update the version
msg="# Managed by release.sh"
sed "s/^version = .* $msg$/version = \"${TAG#v}\" $msg/" -i Cargo.toml || exit

# Run checks to ensure everything is good
cargo fmt --all --check || exit
cargo clippy || exit # This also updates Cargo.lock

# Generate the changelog
git cliff --tag "$TAG" > CHANGELOG.md || exit

# Commit the version update and new changelog
git add -A && git commit -m "misc(release): Prepare for $TAG."
git show

# Generate another changelog of just the unreleased changes, to be put in the tag notes
TAG_CHANGELOG=$(git cliff --tag "$TAG" --unreleased --strip all)

# Create a signed tag for the new version
git tag -s -a "$TAG" -m "$TAG_CHANGELOG" --cleanup=whitespace || exit

# Verify and show the new tag
git tag -v "$TAG" || exit

# Done
echo "New version created successfully."
