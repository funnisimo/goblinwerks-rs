#!/bin/sh

cargo build --release --target wasm32-unknown-unknown --example $1
wasm-bindgen target/wasm32-unknown-unknown/release/examples/$1.wasm --out-dir wasm --no-modules --no-typescript
cp -r resources wasm/

FILE=wasm/$1.html
if test ! -f "$FILE"; then
cat >$FILE <<EOL
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  </head>
  <body>
    <script src="./$1.js"></script>
    <script>
      window.addEventListener("load", async () => {
        await wasm_bindgen("./$1_bg.wasm");
      });
    </script>
  </body>
</html>
EOL
fi

echo "Run: python3 -m http.server"
