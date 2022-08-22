# spytrap-wifi

Test a phone for stalkerware using a wifi hotspot and deep packet inspection.

Based on [stalkerware-indicators] data provided by [Echap].

[stalkerware-indicators]: https://github.com/AssoEchap/stalkerware-indicators
[Echap]: https://github.com/AssoEchap

## Configuration: Wifi

Modern phones test for internet connectivity and might not stay connected to
the network if it doesn't complete. This check uses TLS in some cases and can't
be spoofed easily, so for a reliable setup a working internet connection is
required. This can be a tethered connection with a different phone.

This is the only configuration required.

Edit `roles/spytrap/files/wpa_supplicant.conf`:

```
ctrl_interface=/run/wpa_supplicant
update_config=1

network={
    ssid="iPhone"
    psk="changeme"
}
```

## Setup a device

This was developed using [Arch Linux ARM](https://archlinuxarm.org/):

    # Build from source
    cross build --release --target arm-unknown-linux-gnueabihf
    # TODO: missing setup steps for pi zero
    # on pi zero: install nginx openbsd-netcat gdb lsof dnsmasq tcpdump hostapd sniffglue socat tmux htop

    # Edit the inventory with the ip of the pi zero
    vim inventory

    # Deploy
    ansible-playbook -i inventory site.yml

## Develop

    sudo sniffglue --json enp0s25 | cargo run stream

## Download IOCs

    https://raw.githubusercontent.com/AssoEchap/stalkerware-indicators/master/ioc.yaml

## Similar work

- [TinyCheck](https://github.com/KasperskyLab/TinyCheck)

## License

GPLv3+
