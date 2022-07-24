# clipd
A slightly smart clipboard using the filesystem under ~/.clipd to persist after shutdown.

```
cowsay "clipd is great" | clipd copy
clipd paste
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

This will install 2 binaries:  `clipd` and `clipdaemon`.  
 - `clipdaemon` will listen to your system clipboard and save everything in "sys" [container](#containers).
 - `clipd` is a command line tool to interface with the "~/.clipd" directory, see [examples](#examples).

### Systemd
⚠️ Encryption is not yet implemented - please use with care especially if you are coping sensitive data.

To automatically launch the clipdaemon on login the systemd service manager can be used.  Simply setup the user [service file](etc/clipd.service) for your system or use the make target:
```
make systemd
```

## Containers
Containers are namespaces used to organize your clippings.
In the literal sense, a container is a directory within "~/.clipd" and is where all your clippings are stored.

When no container is provided to `clipd`, it will use the "default" container.

`clipdaemon` sends all clippings to the "sys" container.

## Examples

**Copy from system clipboard (ctrl-c)**

```clipd copy```

```clipd c```


**Paste the last thing you copied**

```clipd paste```

```clipd p```

```clipd```

**Paste using an index key**
```
clipd c "first thing"
clipd c "second thing"
clipd c "third thing"
clipd 0               # third thing
clipd 1               # second thing
clipd 2               # first thing
```

**Use a custom key**
```
clipd phone c "+44789564264" 
clipd phone
```

**Use custom containers**
```
clipd --container gcp c $PROJECT_ID
clipd --container gcp                 # your-project-id
```

**Sneak peek a container**
```
clipd show
```

```
clipd --container gcp show
```


## Roadmap
- Improve `clipd show`
    - show the container name in the table view
    - only show information about one clipping if a specific key is given
- Encrypt clippings stored at rest
- Add support for Windows, OSX and Wayland
