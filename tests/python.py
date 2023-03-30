import os

from tools import cmd, binary
from tools import GREEN, RED, END

def cleanup():
    cmd("rm -rf __pycache__")
    cmd("rm asset.py")
    cmd("rm result")

def test(command: str):
    try:
        cmd(command)
        cmd("python3 main.py")
        cmd("diff " + binary + " ./result")
        print(GREEN + "[OK] " + END + command)
        cleanup()
    except Exception as e:
        print(RED + "[KO] " + END + command)
        cleanup()
        raise Exception(e)

old_path = os.getcwd()
dir_path = os.path.join(os.path.dirname(os.path.realpath(__file__)), "python")

def run_tests():
    os.chdir(dir_path)

    test("embin " + binary + " --lang=python --name=asset --format=hex -o asset.py")
    test("embin " + binary + " --lang=python --name=asset --format=octal -o asset.py")
    test("embin " + binary + " --lang=python --name=asset --format=char -o asset.py")

    os.chdir(old_path)