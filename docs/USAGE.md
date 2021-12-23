# OPM's usage 

## Table of Contents
- [Basic Usage](#basic-usage)
    - [Package Installation](#package-installation)
    - [Package Removing](#package-removing)
    - [Package Search](#package-search)
    - [Package Listing](#package-listing)
- [Advanced Usage](#advanced-usage)
    - [Package building](#package-building)
    - [Package inspect](#package-inspect)

## Basic Usage
This all involves the high-level api

### Package Installation
**NOTE:** It does not store the package in a database for now due testing phase. We're working on that
It'll require superuser privileges to perform this action

You can search and install by the name
```
$ opm install <package_name>
```
Or if you have a `.deb` package you can give the path to it as an argument and then the package will be installed
```
$ opm install <package_name>.deb
```

### Package Removing
**NOTE:** Still under development
A package can be removed only by the name it was installed before
```
$ opm remove <package_name>
```

### Package Search
You can search for a package by it's name
```
$ opm search <package_name>
```
This will bring you can a list with names that are `contains` the name you provide.

E.g.: `tmux` can bring `python3-tmux`

### Package Listing
All installed packages can be listed by using the `--list/-l` option
```
$ opm --list
```
Or
```
$ opm -l
```

## Advanced Usage
This all involves the low-level api
None are ready, it's just for "preview"

### Package Building
To build a package you need to have the `fmt` defined on the config file.

You can build a package through the `--build` option. It'll build a package following the defined on the `fmt` field of the `config.json` file.

As input you pass the directory where the files are. Make sure that your following the instructions on how to build thet package you want.
```
$ opm build --build <mypackage>/
```

### Package Inspecting
You want to have a look at a package before installing? Use the `--inspect` on the `.deb` package. It'll bring informations about the package.
```
$ opm --inspect <package_name>.deb
```