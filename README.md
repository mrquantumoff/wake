# WAKE
Wake is a rust written "make" alternative.

## Installation from source
Right now you can build the app just from source.
### Installation via ```cargo```
* ```cargo build```
* ```su -c 'install -Dm755 -t /bin/ target/debug/wake && chmod +x /bin/wake'```
### Installation from source with existing ```wake``` binary
* ```wake```
* ```wake -s .wake/install.Wakefile```
### Installation from source via ```make```
* ```make```
* ```make install```
### Installation on Arch Linux/Arch based distros
* Available on [AUR](https://aur.archlinux.org/packages/wake-build-git/).


## Creating a new wake project
* ```wake -n <project_name>```

## Configuring ```wake``` for an existing project
* ```mkdir .wake```
* ```touch WakeFileList```
* ```touch .wake/main.Wakefile```
* fill ```.wake/main.Wakefile``` with your build instructions
* fill ```WakeFileList``` with the .Wakefile(s) you want to build (if the file is in .wake folder, just put the name of the file + .Wakefile)
