#!/bin/bash
# link_android.sh
# Creates symbolic links for both release and debug Android builds

SRC="../../../../../android"

# Array of build types
for BUILD in release debug; do
    DEST="target/dx/mobile/$BUILD/android/app"
    
    # Create parent directory if it doesn't exist
    mkdir -p "$(dirname "$DEST")"
    
    # Remove existing file or symlink if present
    if [ -L "$DEST" ] || [ -e "$DEST" ]; then
        echo "Removing existing file/symlink at $DEST"
        rm -rf "$DEST"
    fi

    # Create the symlink
    ln -s "$SRC" "$DEST"
    echo "Symlink created: $DEST -> $SRC"
done
