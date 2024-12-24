import platform
import sys
import os
import urllib.request
import stat

github_url = "https://github.com/kernelrop/kctf/releases/latest/download/"
amd64 = "kctf-amd64"
arm64 = "kctf-arm64"

fallback = (
    "https://raw.githubusercontent.com/google/kctf/v1/docker-images/challenge/pow.py"
)

if os.path.isfile(os.path.expanduser("~/.cache/kctf")):
    args = [os.path.expanduser("~/.cache/kctf")] + sys.argv[1:]
    os.execvp(os.path.expanduser("~/.cache/kctf"), args)

elif platform.system() == "Linux":
    if platform.processor() == "i386":
        r = urllib.request.urlopen(github_url + arm64)
    else:
        r = urllib.request.urlopen(github_url + amd64)

    if not os.path.isdir(os.path.expanduser("~/.cache")):
        os.mkdir(os.path.expanduser("~/.cache"))

    with open(os.path.expanduser("~/.cache/kctf"), "wb") as f:
        f.write(r.read())

    os.chmod(os.path.expanduser("~/.cache/kctf"), stat.S_IEXEC)

    args = [os.path.expanduser("~/.cache/kctf")] + sys.argv[1:]
    os.execvp(os.path.expanduser("~/.cache/kctf"), args)

elif os.path.isfile(os.path.expanduser("~/.cache/kctf.py")):
    args = ["python3", os.path.expanduser("~/.cache/kctf.py")] + sys.argv[1:]
    os.execvp("python3", args)

else:
    r = urllib.request.urlopen(fallback)
    with open(os.path.expanduser("~/.cache/kctf.py"), "wb") as f:
        f.write(r.read())

    args = ["python3", os.path.expanduser("~/.cache/kctf.py")] + sys.argv[1:]
    os.execvp("python3", args)
