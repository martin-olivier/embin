import os
import subprocess

os_extension = ".exe" if os.name == 'nt' else ""
binary = os.path.join(os.path.dirname(os.path.realpath(__file__)), "../target/release/embin" + os_extension)

RED = "\033[1;31m"
GREEN = "\033[1;32m"
END = "\033[0m"

def cmd(command: str):
    shell = "powershell" if os.name == 'nt' else "sh"
    arg = "/C" if os.name == 'nt' else "-c"

    code, res = subprocess.getstatusoutput(shell + " " + arg + " \"" + command + "\"")
    if code != 0:
        raise Exception(res)