# `rcom` 1.0
> A communication program for accessing serial ports

Simple minicom replacement. Shows timestamp for each line received.

## USAGE:
    rcom [FLAGS] [OPTIONS]

## FLAGS:
    -h, --help             Prints help information
    -n, --no-timestamps    Don't show timestamps
    -V, --version          Prints version information

## OPTIONS:
    -d, --device <device_name>    Serial port name [default: /dev/ttyUSB1]
    -s, --speed <speed>           Communication speed [default: 115200]

## KEYS:
    Use Ctrl+A key to exit rcom
