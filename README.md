# wslpath
Extend wslpath to support non-existent Linux path

```
Usage
$ wslpath -w -f /mnt/c/no-such-directory
C:\no-such-directory
$ wslpath -w -f /home/no-such-directory
\\wsl$\<distro>\home\no-such-directory
```