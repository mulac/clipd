# clipd
A slightly smart clipboard, persistent after shutdown

## Usage Examples

```
clipd copy "something i must remember"
clipd paste

echo "bad things happened" > /tmp/error.log
cat /tmp/error.log | clipd c --key error
clipd p notes 
# bad things happend

clipd p 1
# something i must remember
```

## Installation
### From Source
```
git clone https://github.com/Mulac/clipd.git
cd clipd
cargo install --path .
```

## Roadmap
2. Encrypt data stored at rest
3. Add the ability to fetch from X11/Windows/OSX clipboard (`clipd copy` with no value should do this)
