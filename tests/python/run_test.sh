set -e

embin /bin/bash /bin/cat --lang=python --format=hex -o asset.py
python3 main.py
diff /bin/bash ./result
rm -rf __pycache__

embin /bin/bash /bin/cat --lang=python --format=octal -o asset.py
python3 main.py
diff /bin/bash ./result
rm -rf __pycache__

embin /bin/bash /bin/cat --lang=python --format=char -o asset.py
python3 main.py
diff /bin/bash ./result
rm -rf __pycache__

rm -f asset.py
rm -f result
rm -rf __pycache__