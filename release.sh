#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting release process...${NC}\n"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "Current version: ${YELLOW}$CURRENT_VERSION${NC}"

# Parse version components
IFS='.' read -r -a VERSION_PARTS <<< "$CURRENT_VERSION"
MAJOR="${VERSION_PARTS[0]}"
MINOR="${VERSION_PARTS[1]}"
PATCH="${VERSION_PARTS[2]}"

# Increment patch version
NEW_PATCH=$((PATCH + 1))
NEW_VERSION="$MAJOR.$MINOR.$NEW_PATCH"

echo -e "New version: ${GREEN}$NEW_VERSION${NC}\n"

# Update version in Cargo.toml
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

echo -e "${GREEN}✓ Updated Cargo.toml to version $NEW_VERSION${NC}\n"

# Update Cargo.lock to reflect the new version
echo -e "${YELLOW}Updating Cargo.lock...${NC}"
cargo update --workspace --offline 2>/dev/null || cargo check --quiet
echo -e "${GREEN}✓ Updated Cargo.lock${NC}\n"

# Format Leptos code
echo -e "${YELLOW}Formatting Leptos code...${NC}"
leptosfmt src
echo -e "${GREEN}✓ Formatted Leptos code${NC}\n"

# Check if there are any changes to commit
if [[ -z $(git status -s) ]]; then
    echo -e "${RED}No changes to commit!${NC}"
    exit 1
fi

# Run cargo check to ensure everything compiles
# echo -e "${YELLOW}Running cargo check...${NC}"
# cargo build --release
# echo -e "${GREEN}✓ Cargo check passed${NC}\n"

# Show what will be committed
echo -e "${YELLOW}Changes to be committed:${NC}"
git status -s
echo ""

# Stage all changes including Cargo.lock and deleted files
git add -A
echo -e "${GREEN}✓ Staged all changes (including Cargo.lock and deletions)${NC}\n"

# Generate commit message from git diff
echo -e "${YELLOW}Generating commit message from changes...${NC}"

# Get a summary of changed files
CHANGED_FILES=$(git diff --cached --name-only | head -5)
NUM_FILES=$(git diff --cached --name-only | wc -l)

# Try to generate a meaningful commit message
COMMIT_MSG="$NEW_VERSION"

# Check for specific patterns in the diff
if git diff --cached | grep -q "fn .*{"; then
    COMMIT_MSG="$COMMIT_MSG - code changes"
elif git diff --cached Cargo.toml | grep -q "dependencies"; then
    COMMIT_MSG="$COMMIT_MSG - dependency updates"
fi

# Add file context if not too many files
if [ "$NUM_FILES" -le 3 ]; then
    FILE_LIST=$(echo "$CHANGED_FILES" | tr '\n' ', ' | sed 's/,$//')
    COMMIT_MSG="$COMMIT_MSG ($FILE_LIST)"
fi

echo -e "Default commit message: ${GREEN}$COMMIT_MSG${NC}\n"

# Generate commit message with Claude CLI
echo -e "\n${YELLOW}Generating commit message with Claude...${NC}"

# Get git diff for context
DIFF_CONTEXT=$(git diff --cached)

# Use Claude to generate commit message
CLAUDE_MSG=$(claude --print "Based on this git diff, write a concise commit message starting with just the new version, a dash, and max 20 words describing the changes made.:

$DIFF_CONTEXT" 2>/dev/null || echo "")

if [ -z "$CLAUDE_MSG" ]; then
    echo -e "${RED}Failed to generate message with Claude. Using default.${NC}"
    CLAUDE_MSG="$COMMIT_MSG"
fi

echo -e "\nCommit message: ${GREEN}$CLAUDE_MSG${NC}"
# Allow editing: ENTER to accept, or edit the message
read -e -p "Press ENTER to proceed or edit: " -i "$CLAUDE_MSG" FINAL_MSG
FINAL_MSG=${FINAL_MSG:-$CLAUDE_MSG}  # Use Claude message if empty

echo -e "\nUsing commit message: ${GREEN}$FINAL_MSG${NC}\n"

# Create commit
git commit -m "$FINAL_MSG"
echo -e "${GREEN}✓ Created commit${NC}\n"

# Ask user if they want to push
read -p "Push to remote? (Y/n): " push_choice
push_choice=${push_choice:-Y}  # Default to Y if empty

if [[ $push_choice =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Pushing to remote...${NC}"
    git push
    echo -e "${GREEN}✓ Pushed to remote${NC}\n"
else
    echo -e "${YELLOW}Skipping push to remote${NC}\n"
fi

# Publish to crates.io
# echo -e "${YELLOW}Publishing to crates.io...${NC}"
cargo publish
# echo -e "${GREEN}✓ Published to crates.io${NC}\n"

echo -e "${GREEN}🎉 Release $NEW_VERSION complete!${NC}"
