# rfstr
Command line utility to search and filter strings.

## Usage
The usage is straightforward, it's designed mainly to be used within a pipe chain to filter string output before sending out to the next utility.

E.g.
```sh
rg -tcpp MyType | fzf | rfstr -q "[[:alpha:]]:(.+?)+\.[[:alpha:]]+" -mf | code
```

This example uses ripgrep to search for MyType in code base, uses fzf to let user filter and select required result. Then rfstr will filter the output of fzf to contain just the file path, which is then sent to vscode to open.

```
$ rfstr -h

Command line utility to search and filter strings

Usage:
    rfstr -q <query> [-p <file path>] [-m <mode>]

EXAMPLE:
$ echo "Hello World" | rfstr -q "[[:alpha:]]+" -m f
$ Hello

OPTIONS:

-q, --query     Required    The query that needs to be searched. It can be any 
                            valid rust expression without any named captures.

-p, --path      Optional    The path to file which needs to be searched.

-m, --mode      Optional    The search mode to be used. By default it will be
                            plain text search.

                            The available modes are -
                            * c - Complete Match      Entire line should match the given regex
                            * s - Substring Match,    Print lines that contain the substring matching query
                            * f - First Substring,    Print only the first substring of a line that matched
                            * l - Last Substring,     Print only the last sustring of a line that matched
                            * a - All Substring,      Print all matched substring of a line
                            * [Default] PlainSearch,  Print lines containing the subsctring - no regex.    
```