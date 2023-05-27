# Building for website
```console
trunk build --release --public-url 10/stem/at2/calculator/
```

Change the url location if building for somewhere else.
Then copy the entire `dist` folder to the desired folder on the webserver

Then run this to get a smaller binary again. (found here https://github.com/WebAssembly/binaryen)
```console
wasm-opt -Os -o output.wasm input.wasm
```