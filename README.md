# clipd
A slightly smart clipboard

## Usage Examples

```
clipd copy "something i must remember"
clipd paste

cat notes.txt | clipd c --key notes
clipd p notes 
# contents of notes.txt

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