set -e

embin /bin/bash --lang=python --name=asset --format=hex -o asset.py
python3 main.py
diff /bin/bash ./result
rm -rf __pycache__

embin /bin/bash --lang=python --name=asset --format=octal -o asset.py
python3 main.py
diff /bin/bash ./result
rm -rf __pycache__

embin /bin/bash --lang=python --name=asset --format=char -o asset.py
python3 main.py
diff /bin/bash ./result
rm -rf __pycache__

rm -f asset.py
rm -f result
rm -rf __pycache__