IFS=. read -r major minor patch <<< "$1"

minor=$(printf "%05d" "$minor")
patch=$(printf "%05d" "$patch")

echo $major$minor$patch;
