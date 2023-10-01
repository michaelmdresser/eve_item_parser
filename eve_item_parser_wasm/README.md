# WASM eve_item_parser

WASM-ready version of eve_item_parser for browser usage.

## Build
``` sh
wasm-pack build --no-typescript --target web
```

## Use direcly via JS

Place the `.js` and `.wasm` files in a directory (e.g. `eve_item_parser`) on the web
server. Then use a snippet like this to load and use:

```html
<script type="module">
import init, { parse } from './eve_item_parser/eve_item_parser_wasm.js';
async function run() {
  await init();
  let {items} = parse("Paladin x5");
  console.log("items:", items);
}
run();
</script>
```

Reference for the above: https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html 
