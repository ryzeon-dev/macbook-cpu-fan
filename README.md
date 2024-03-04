# macbook-cpu-fan
Macbook fan controller daemon (Rust) and CLI (Python3)

Building the executable requires `cargo` and `make` to be installed on the system 

## Download, build and install
```bash
git clone https://github.com/ryzeon-dev/macbook-cpu-fan && cd macbook-cpu-fan && make && sudo make install
```

## Build 
After cloning the repo, enter the directory and run
```bash
make 
```

## Install 
After building, run
```bash
sudo make install 
```

## Usage
All software activities can be controller via the provided CLI
```bash
$ mbcf --help 
mbcf: MacBook CPU fan cotroller CLI
usage: mbcf [ARGUMENTS] [OPTIONS]

Arguments:
    start | restart | stop | enable | disable    performs the specified command on the service using "systemctl" (requires root)

Options:
    -a | --apply CONFIG_NAME    restarts the daemon applying the specified fan profile (requires root)
    -h | --help                 show this message and exit
```
