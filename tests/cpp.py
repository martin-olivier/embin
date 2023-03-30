import os

from tools import cmd, binary, os_extension
from tools import GREEN, RED, END

bin_path = "build/Debug/" if os.name == 'nt' else "build/"

def cleanup():
    cmd("rm -r build")
    cmd("rm asset.hpp")
    cmd("rm result")

def test(command: str):
    try:
        cmd(command)
        cmd("cmake . -B build")
        cmd("cmake --build build --clean-first")
        cmd(bin_path + "writer" + os_extension)
        cmd("diff " + binary + " ./result")
        print(GREEN + "[OK] " + END + command)
        cleanup()
    except Exception as e:
        print(RED + "[KO] " + END + command)
        cleanup()
        raise Exception(e)

old_path = os.getcwd()
dir_path = os.path.join(os.path.dirname(os.path.realpath(__file__)), "cpp")

def run_tests():
    os.chdir(dir_path)

    test("embin " + os.path.relpath(binary) + " --lang=cpp --name=asset --format=hex -o asset.hpp")
    test("embin " + os.path.relpath(binary) + " --lang=cpp --name=asset --format=octal -o asset.hpp")
    test("embin " + os.path.relpath(binary) + " --lang=cpp --name=asset --format=char -o asset.hpp")

    os.chdir(old_path)