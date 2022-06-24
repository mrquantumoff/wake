# WAKE
Wake is a rust written "make" alternative.

## Installation from source
Right now you can build the app just from source.
### Installation via ```cargo``` (git)
* clone the repo and ```cd``` in it
* ```cargo build -r``` and ```su -c 'install -Dm755 -t /bin/ target/release/wake && chmod +x /bin/wake'``` for global installation on GNU+Linux
* ```cargo install --path .``` for local installation on Windows/GNU+Linux

### Installation via ```cargo``` (release)
* ```cargo install wake``` for local installation on Windows/GNU+Linux
### Installation from source with existing ```wake``` binary on GNU+Linux
* ```wake```
* ```wake -s .wake/install.Wakefile```
### Installation from source via ```make``` on GNU+Linux
* ```make```
* ```make install```
### Installation on Arch Linux/Arch based distros
* Available on [AUR](https://aur.archlinux.org/packages/wake-build-git/).


## Creating a new wake project
* ```wake new -n <project_name> -l <language>```

## Configuring ```wake``` for an existing project
* ```mkdir .wake```
* ```touch WakeFileList```
* ```touch .wake/main.Wakefile```
* fill ```.wake/main.Wakefile``` with your build instructions
* fill ```WakeFileList``` with the .Wakefile(s) you want to build (if the file is in .wake folder, just put the name of the file + .Wakefile)
