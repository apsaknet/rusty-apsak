# apsaK WASM SDK

An integration wrapper around [`apsak-wasm`](https://www.npmjs.com/package/apsak-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the apsaK WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/apsaknet/rusty-apsak/releases](https://github.com/apsaknet/rusty-apsak/releases)

## Usage

apsaK NPM module exports include all WASM32 bindings.
```javascript
const apsak = require('apsak');
console.log(apsak.version());
```

## Documentation

Documentation is available at [https://apsak.aspectron.org/docs/](https://apsak.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/apsaknet/rusty-apsak/tree/master/wasm](https://github.com/apsaknet/rusty-apsak/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/apsaknet/rusty-apsak/releases](https://github.com/apsaknet/rusty-apsak/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/apsak-wasm.html](https://aspectron.org/en/projects/apsak-wasm.html)

