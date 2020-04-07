A simple cli util to log daily timestamped notes. 

## Usage
```
USAGE:
    logbook [OPTIONS] --message <TEXT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --message <TEXT>         The message to be recorded
    -t, --tag <TEXT>...          Tag associated with this message. Pass this option for each tag you have.
    -u, --utc_offset <NUMBER>    The UTC offset to use for this message's timestamp. Defaults to the current
                                             machine's offset.
```
