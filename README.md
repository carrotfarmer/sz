# sz
A pragmatic way to view your file sizes. Think of it as an easier-to-use and more intuitive alternative to the
[du](https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html#du-invocation) coreutil.

This project is still under development and might contains bugs and crappy code! Please open issues/PRs if you find something janky.

## usage 

Let's take a look at how we can use this utility in a directory.

- to find the size of the current directory (excluding gitignored and hidden files/files in gitignored or hidden directories)

```bash
sz . 
```

this will be the output:

![ss1](https://i.imgur.com/ZZYI9ji.png)

```bash
 file/dir name | size    
---------------+---------
 sz            | 41.52KB 


10 files parsed
```

- if you want to list the size of each file along with the file path:

```bash
sz . -l
```

![ss2](https://i.imgur.com/lLPbgmS.png)

- you can include gitignored and hidden flags by including the `-g` and `-i` flag respectively.

```bash
sz . -l -i -g
```

![ss3](https://i.imgur.com/BDOB7ca.png)


- if you run this command in a directory with more than 50 files it will only show the first 20.
  you can either provide the number of items you want to list with the `-n` argument or include the
  `-L` flag to list all the items.

```bash
sz . -l -i -g -n 30 # show me the first 30 items
```

```bash
sz . -l -i -g -L # show me all the files
```

- you can also sort the items based on size using the `-d` and `-a` flags.

```bash
sz . -l -d -n 5 # returns the 5 largest files in the current directory
```

![ss5](https://i.imgur.com/eLrjAUN.png)

```bash
sz . -l -a -n 6 # returns the 6 smallest files in the current directory
```

![ss6](https://i.imgur.com/cSXNcqE.png)

- to view only the root directories in a path, use the `-D` flag.
  this flag will only show directories with the parent same as the path provided.
  it will not list any files.

```bash
sz ~/Documents/rl -l -D
```

![ss7](https://i.imgur.com/NQnnUEz.png)

- you can recursively view all the directories in a path by providing the `-R` flag.

```bash
sz ~/Documents/rl -l -D -R
```

![ss8](https://i.imgur.com/Ct34Uob.png)

note that you can use all the options previously mentioned with the `-D` flag too.

```bash
# shows the 3 largest directories in this path (including gitignored or hidden)
sz ~/Documents/rl -l -D -R -i -g -d -n 3 
```

![ss9](https://i.imgur.com/OFOQtib.png)

- use the `-e` flag to exclude directories

```bash
sz ~/Documents/rl -l -D -R -d -e ~/Documents/rl/public/ ~/Documents/rl/assets/
```

![ss10](https://i.imgur.com/FryPYYU.png)

warning: this arg doesn't *really* work with the `-i` or `-g` 

- use the `-s` flag to show the total line count of all the files parsed. **Run this only with
  UTF-8 encoded files, obviously.**

```bash
sz ./src -l -s
```

![ss11](https://i.imgur.com/9XU9Szm.png)

## options

The options are pretty simple yet powerful:

```bash
Options:
  -d, --sort-files-desc                 Sort items by size in descending order
  -a, --sort-files-asc                  Sort items by size in ascending order
  -i, --include-hidden                  Include hidden files
  -l, --list-files                      List the files in the directory
  -L, --list-all                        List all files in the directory even if directory count exceeds 50
  -g, --include-gitignored              Include gitignored files
  -e, --exclude-dirs <EXCLUDE_DIRS>...  Directories to exclude
  -n, --num-files <NUM_FILES>           Number of files to list
  -D, --only-dirs                       Show only directories
  -R, --recursive-dirs                  Recursively list directories (Only to be used with -D flag)
  -s, --show-lines                      Show total line count for items
  -h, --help                            Print help
```

Run `sz -h` to view this help message.
