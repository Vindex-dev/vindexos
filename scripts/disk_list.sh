lsblk -d -n -o NAME,SIZE,TYPE --sort SIZE 2>/dev/null | grep "disk" | awk '{print $1, $2}'
