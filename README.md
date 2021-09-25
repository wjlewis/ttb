# Text-to-Bin

This is a poor-man's hex editor.
In particular, it converts sequences of ASCII bytes `'0'` and `'1'` to the bits they represent.

For instance, if _test.txt_ contains

```
00000001
00000010
00000011
00001111
```

then executing

```shell
$ ttb < test.txt > test.bin
```

results in a file named _test.bin_ containing bytes `0x01 0x02 0x03 0x0f`:

```shell
$ xxd -p test.bin
0102030f
```

In addition, all other bytes are skipped, and if the byte `';'` is encountered, all bytes until the next `CR` or `LF` are skipped.
This allows for convenient separation of bit-groups, along with comments containing the bytes `'0'` and `'1'`.
Given a file named _quux.txt_ with the following contents,

```
;; quux.txt
0001_0001 ;; This line contains 6 0's and 2 1's
```

executing

```shell
$ ttb < quux.txt > quux.bin
```

produces a file named _quux.bin_ containing the byte `0x11`.

## Usage

At the moment, `ttb` only reads from `stdin` and writes to `stdout`.
Shell redirection allows for transforming text files, and also authoring binary files "in-situ" (just remember to use `Ctrl+D` to send `EOF`):

```shell
$ ttb > out.bin
;; `ttb` is reading from stdin at the moment
;; This works OK, but doesn't allow editing previously entered lines
0000_1111 ;; First line (the byte 0x0f)
1111_1011 ;; Second line (the byte 0xfb)
;; etc.
$ xxd -p out.bin
0ffb
```
