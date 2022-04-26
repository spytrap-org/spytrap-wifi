# spytrap

## Setup a device

    # Build from source
    cross build --release --target arm-unknown-linux-gnueabihf
    # TODO: missing setup steps for pi zero
    # on pi zero: install nginx openbsd-netcat gdb lsof dnsmasq tcpdump hostapd sniffglue socat tmux htop
    # Deploy latest version
    ansible-playbook -i inventory site.yml

## Develop

    sudo sniffglue --json enp0s25 | cargo run stream

## Download IOCs

    https://raw.githubusercontent.com/Te-k/stalkerware-indicators/master/ioc.yaml

## License

GPLv3+
