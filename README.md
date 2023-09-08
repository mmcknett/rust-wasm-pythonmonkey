# rust-wasm-pythonmonkey
Following along with a tutorial for calling rust code compiled to WASM from Python. See [YouTube: Call Rust code from Python](https://www.youtube.com/watch?v=jYCD0I-gdo8)

## Building
To build the javascript bindings:

```
wasm-pack build --target nodejs
```

### Limitations
PythonMonkey is missing support for `fs` and `path` packages. In the video, Will hand-modifies the generated `pkg` files like this:

```javascript
// Modifications by hand...
//const path = require('path').join(__dirname, 'adder_bg.wasm');
const path = [__dirname, 'adder_bg.wasm'].join('/');

// Modifications by hand...
//const bytes = require('fs').readFileSync(path);
function pyReadFileSync(path) {
    python.exec(`
def getBytes(filename):
    with open(filename, "rb") as f:
        return bytearray(f.read())
`);
    return python.eval(`getBytes`)(path);
}

const bytes = pyReadFileSync(path);
```

He also provides the [build.sh](./build.sh) script that automatically does these modifications.