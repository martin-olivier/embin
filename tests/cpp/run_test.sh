set -e

embin /bin/bash /bin/cat --lang=cpp --format=hex -o asset.hpp
g++ -std=c++17 main.cpp -o writer
./writer
diff /bin/bash ./result

embin /bin/bash /bin/cat --lang=cpp --format=octal -o asset.hpp
g++ -std=c++17 main.cpp -o writer
./writer
diff /bin/bash ./result

embin /bin/bash /bin/cat --lang=cpp --format=char -o asset.hpp
g++ -std=c++17 main.cpp -o writer
./writer
diff /bin/bash ./result

rm asset.hpp
rm writer
rm result