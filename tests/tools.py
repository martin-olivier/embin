import os
import subprocess

binary = "/Windows/System32/cmd.exe" if os.name == 'nt' else "/bin/bash"
os_extension = ".exe" if os.name == 'nt' else ""

RED = "\033[1;31m"
GREEN = "\033[1;32m"
END = "\033[0m"

def cmd(command: str):
    shell = "powershell" if os.name == 'nt' else "sh"
    arg = "/C" if os.name == 'nt' else "-c"

    code, res = subprocess.getstatusoutput(shell + " " + arg + " \"" + command + "\"")
    if code != 0:
        raise Exception(res)