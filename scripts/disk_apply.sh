#!/bin/bash
# scripts/disk_apply.sh --root <disk> [--home <disk>]

ROOT_DISK=""
HOME_DISK=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --root) ROOT_DISK="$2"; shift 2 ;;
        --home) HOME_DISK="$2"; shift 2 ;;
        *) shift ;;
    esac
done

if [ "$HOME_DISK" == "$ROOT_DISK" ] || [ -z "$HOME_DISK" ]; then
    sleep 2
else
    sleep 2
fi
