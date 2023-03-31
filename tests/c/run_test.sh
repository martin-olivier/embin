set -e

embin /bin/bash /bin/cat --lang=c --format=hex -o asset.h
gcc main.c -o writer
./writer
diff /bin/bash ./result

embin /bin/bash /bin/cat --lang=c --format=octal -o asset.h
gcc main.c -o writer
./writer
diff /bin/bash ./result

embin /bin/bash /bin/cat --lang=c --format=char -o asset.h
gcc main.c -o writer
./writer
diff /bin/bash ./result

rm asset.h
rm writer
rm result