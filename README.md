# apen
Append from stdin to a new file.

`Apen` is __7x faster__ than `anewer` which a fast rust version of tomnomnom anew.

### Usage:


```bash
cat 1.txt
one
two
two
one
```

```bash
cat 2.txt
one
line
```

```bash
cat 1.txt | ./apen 2.txt
two
```

```bash
cat 2.txt
one
line
two
```

Inspierd by [anew](https://github.com/tomnomnom/anew).
[anewer](https://github.com/ysf/anewer)
