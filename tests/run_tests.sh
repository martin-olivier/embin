set -e
cd "$(dirname "$0")"

GREEN="\033[1;32m"
ENDCOLOR="\033[0m"

cd c
./run_test.sh
cd ..
printf "${GREEN}[OK]${ENDCOLOR} Tests for C lang\n"

cd cpp
./run_test.sh
cd ..
printf "${GREEN}[OK]${ENDCOLOR} Tests for Cpp lang\n"

cd python
./run_test.sh
cd ..
printf "${GREEN}[OK]${ENDCOLOR} Tests for Python lang\n"