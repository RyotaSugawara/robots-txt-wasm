# robots-txt-wasm

The robots.txt checker based https://github.com/Folyd/robotstxt

## Pre-requirements

```bash
$ cargo install wasm-pack
```

## Build

```bash
# for web
$ wasm-pack build --target web
# for nodejs
$ wasm-pack build --target nodejs
```

## Local Development

### Web

```bash
$ npx serve -l 3000
$ open http://localhost:3000
```

### Node.js

TBD
