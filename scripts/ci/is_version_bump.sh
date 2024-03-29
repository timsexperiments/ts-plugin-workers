difference=$(($2 - $1))

if [[ $difference -eq 1 || $difference -eq 100000 || $difference -eq 10000000000 ]]; then
    exit 0
else
    exit 1
fi