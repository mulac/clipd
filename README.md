# clipd
A slightly smart clipboard using the filesystem under ~/.clipd to persist after shutdown

## Usage Examples

**Simple copy**
```
clipd copy "something i must remember"
clipd paste
# "something i must remember"
```

**Access using number**
```
clipd c "first thing"
clipd c "second thing"
clipd c "third thing"
clipd p -k     # third thing
clipd p -k 1   # second thing
clipd p -k 2   # first thing
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
clipd error paste   # 404 Not Found
```

## Installation
### From Source
```
git clone https://github.com/Mulac/clipd.git
cd clipd
cargo install --path .
```

## Roadmap
0. Improve error handling
1. Implement the `clipd show` command to show a preview of the top ten values in a container
2. Encrypt data stored at rest
3. Add the ability to fetch from X11/Windows/OSX clipboard (`clipd copy` with no value should do this)
4. Create a daemon to constantly listen to system clipboard
