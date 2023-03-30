import os
import subprocess

test_bin = "/Windows/System32/cmd.exe" if os.name == 'nt' else "/bin/bash"
bin_path = "build/Debug/" if os.name == 'nt' else "build/"
extension = ".exe" if os.name == 'nt' else ""

RED = "\033[1;31m"
GREEN = "\033[1;32m"
END = "\033[0m"

def cmd(command: str):
    code, res = subprocess.getstatusoutput(command)
    if code != 0:
        raise Exception(res)
    
def test(command: str):
    try:
        cmd(command)
        cmd("cmake . -B build")
        cmd("cmake --build build --clean-first")
        cmd(bin_path + "writer" + extension)
        cmd("diff " + test_bin + " ./result")
        print(GREEN + "[OK] " + END + command)
    except Exception as e:
        print(RED + "[KO] " + END + command)
        raise Exception(e)

def cleanup():
    cmd("rm -r build")
    cmd("rm asset.h")
    cmd("rm result")


old_path = os.getcwd()
dir_path = os.path.dirname(os.path.realpath(__file__))

def run():
    os.chdir(dir_path)

    test("embin " + test_bin + " --lang=c --name=asset --format=hex -o asset.h")
    test("embin " + test_bin + " --lang=c --name=asset --format=octal -o asset.h")
    test("embin " + test_bin + " --lang=c --name=asset --format=char -o asset.h")

    cleanup()

    os.chdir(old_path)