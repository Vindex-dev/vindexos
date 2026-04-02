#!/bin/bash
SORTED=$(lsblk -d -n -o NAME,SIZE,TYPE --sort SIZE 2>/dev/null | grep "disk" | awk '{print $1}')
COUNT=$(echo "$SORTED" | wc -l)
if [ "$COUNT" -eq 0 ]; then
    echo "ERROR"
    exit 1
elif [ "$COUNT" -eq 1 ]; then
    echo "$SORTED"
else
    SMALL=$(echo "$SORTED" | head -n 1)
    BIG=$(echo "$SORTED" | tail -n 1)
    echo "$SMALL"
    echo "$BIG"
fi
