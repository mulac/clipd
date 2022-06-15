# clipd
A slightly smart clipboard using the filesystem under ~/.clipd to persist after shutdown.

```
$ cowsay "clipd is great" | clipd copy
$ clipd paste
 _________________
< clipd is great! >
 -----------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

Currently clipd ("clipped") is only implemented for Linux.
It can listen to your system clipboard so that you never loose a clipping
and since it uses the filesystem it allows you to easily sync your clippings across 
different machines using something like [syncthing](https://github.com/syncthing/syncthing).

## Installation
### From Source
```
git clone https://github.com/Mulac/clipd.git
cd clipd
cargo install --path .
```

### Systemd
To automatically launch the clipdaemon on login the systemd service manager can be used.  Simply setup the user [service file](systemd/clipd.service) for your system or use make as below.
```
make systemd
```


## Usage Examples
**Copy from system clipboard**
```
clipd copy
clipd paste
```

**Mannually start the clipdaemon**
```
clipdaemon
```

**Access using number**
```
clipd c "first thing"
clipd c "second thing"
clipd c "third thing"
clipd p -k                # third thing
clipd p -k 1              # second thing
clipd p -k 2              # first thing
```

**Use a custom key**
```
clipd c "+44789564264" --key phone
clipd p --key phone
```

**Use containers**
```
clipd error c "404 Not Found"
clipd debug c "Some Warning"
clipd error paste                 # 404 Not Found
```

**Sneak peek a container**
```
clipd show
```


## Roadmap
- Improve `clipd show`
    - show only the first $n$ clippings
    - truncate values if they are too long
    - show the container name in the table view
    - only show information about one clipping if a specific key is given
- Encrypt clippings stored at rest
- Add support for Windows, OSX and Wayland
