set -e

embin /bin/bash --lang=c --name=asset --format=hex -o asset.h
gcc main.c -o writer
./writer
diff /bin/bash ./result

embin /bin/bash --lang=c --name=asset --format=octal -o asset.h
gcc main.c -o writer
./writer
diff /bin/bash ./result

embin /bin/bash --lang=c --name=asset --format=char -o asset.h
gcc main.c -o writer
./writer
diff /bin/bash ./result

rm asset.h
rm writer
rm result