<p align="center">
    <img src="https://www.citypng.com/public/uploads/small/116404390591jnmiepcahcaeehx3rcg8ctrl4uwsm2qhz65fknnff5liocis69reptmroenzpepwy3polhdtp7kp23tgf7myadlax0xubgkg1dv.png" width="200">
    <br>
    <br>
    <img src="https://img.shields.io/github/v/release/ValuONE/search"
        alt="Release">
    <img src="https://img.shields.io/badge/language-rust-red.svg" alt="Language">
    <img src="https://img.shields.io/github/stars/ValuONE/search" alt="Stars">
    <img src="https://img.shields.io/github/issues/ValuONE/search" alt="Issues">
    <img src="https://img.shields.io/github/forks/ValuONE/search" alt="Forks">
</p>

<p align="center">
    <a href="#key-features">Key Features</a> •
    <a href="#installation">Installation</a> •
    <a href="#usage">Usage</a> •
    <a href="#to-do">To-Do</a> •
    <a href="#license">License</a>
</p>

<p>
    With the intention to learn rust further, i followed some tutorials and one was about creating a little "grep" clone. 
    After a bit of coding I had so much fun, that I expanded the project with some other features...
</p>

## **Key Features**
- Search for a certain string
- Locate a certain file

---

## **Installation**

If you just want to use the tool, you can download the latest release and execute it via cmd etc.

``` bash
# Locate the search.exe and execute it
search.exe --help
```

Alternatively you can download the repository and compile it yourself.

``` bash
# Clone the repository
git clone https://github.com/ValuONE/search

# Build the project
cargo build --release

```
It will create a executable in `target/release`!

Or simply do

`cargo install search`

---
## **Usage**

To get started just type ``search.exe --help`` you should now see this output:

```cmd
Usage: search.exe <COMMAND>

Commands:
  locate    Locate a file on your computer
  find      Find a string in a file
  help      Print this message or the help of the given subcommand(s)      

Options:
  -h, --help     Print help
  -V, --version  Print version
```
---

## **TO-DO**

- [ ] Increase find/locate speed
- [ ] Increase user-friendliness

---

## **License**
[![GitHub license](https://img.shields.io/github/license/pr4k/locate)](https://github.com/pr4k/locate)

- **[MIT license](http://opensource.org/licenses/mit-license.php)**
- Copyright 2023 © ValuONE

---
