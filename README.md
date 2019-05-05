# rw

Like [sponge](https://joeyh.name/code/moreutils/), but without the moreutils kitchen sink.

```bash
cat example.txt | grep -v exclude | rw example.txt
```

`rw` ("rewrite") allows you to redirect the stdout of one command into a file, even if you're using that file to generate the stdout. If you were to try that with standard shell redirection, you would end up truncating the file before you could read it!

```bash
# This causes example.txt to be empty
# Shell redirection would truncate the file, before the cat utility read it
cat example.txt | grep -v exclude > example.txt
```

## Usage

```bash
rw [-a] FILE
```

### Appending to file

Giving the `-a` will append to the output file, instead of overwritting it.