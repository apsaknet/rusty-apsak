/*!
# `rusty-apsak WASM32 bindings`

[<img alt="github" src="https://img.shields.io/badge/github-apsaknet/rusty--apsak-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/apsaknet/rusty-apsak/tree/master/wasm)
[<img alt="crates.io" src="https://img.shields.io/crates/v/apsak-wasm.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/apsak-wasm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-apsak--wasm-56c2a5?maxAge=2592000&style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/apsak-wasm)
<img alt="license" src="https://img.shields.io/crates/l/apsak-wasm.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">

<br>

Rusty-apsaK WASM32 bindings offer direct integration of Rust code and Rusty-apsaK
codebase within JavaScript environments such as Node.js and Web Browsers.

## Documentation

- [**integrating with apsaK** guide](https://apsak.aspectron.org/)
- [**Rustdoc** documentation](https://docs.rs/apsak-wasm/latest/apsak-wasm)
- [**JSDoc** documentation](https://apsak.aspectron.org/jsdoc/)

Please note that while WASM directly binds JavaScript and Rust resources, their names on JavaScript side
are different from their name in Rust as they conform to the 'camelCase' convention in JavaScript and
to the 'snake_case' convention in Rust.

## Interfaces

The APIs are currently separated into the following groups (this will be expanded in the future):

- **Transaction API** — Bindings for primitives related to transactions.
- **RPC API** — [RPC interface bindings](rpc) for the apsaK node using WebSocket (wRPC) connections.
- **Wallet API** — API for async core wallet processing tasks.

## NPM Modules

For JavaScript / TypeScript environments, there are two
available NPM modules:

- <https://www.npmjs.com/package/apsak>
- <https://www.npmjs.com/package/apsak-wasm>

The `apsak-wasm` module is a pure WASM32 module that includes
the entire wallet framework, but does not support RPC due to an absence
of a native WebSocket in NodeJs environment, while
the `apsak` module includes `websocket` package dependency simulating
the W3C WebSocket and due to this supports RPC.

## Examples

JavaScript examples for using this framework can be found at:
<https://github.com/apsaknet/rusty-apsak/tree/master/wasm/nodejs>

## WASM32 Binaries

For pre-built browser-compatible WASM32 redistributables of this
framework please see the releases section of the Rusty apsaK
repository at <https://github.com/apsaknet/rusty-apsak/releases>.

## Using RPC

**NODEJS:** If you are building from source, to use WASM RPC client
in the NodeJS environment, you need to introduce a global W3C WebSocket
object before loading the WASM32 library (to simulate the browser behavior).
You can the [WebSocket](https://www.npmjs.com/package/websocket)
module that offers W3C WebSocket compatibility and is compatible
with apsaK RPC implementation.

You can use the following shims:

```js
// WebSocket
globalThis.WebSocket = require('websocket').w3cwebsocket;
```

## Loading in a Web App

```html
<html>
    <head>
        <script type="module">
            import * as apsak_wasm from './apsak/apsak-wasm.js';
            (async () => {
                const apsak = await apsak_wasm.default('./apsak/apsak-wasm_bg.wasm');
                // ...
            })();
        </script>
    </head>
    <body></body>
</html>
```

## Loading in a Node.js App

```javascript
// W3C WebSocket module shim
// this is provided by NPM `apsak` module and is only needed
// if you are building WASM libraries for NodeJS from source
// globalThis.WebSocket = require('websocket').w3cwebsocket;

let {RpcClient,Encoding,initConsolePanicHook} = require('./apsak-rpc');

// enabling console panic hooks allows WASM to print panic details to console
// initConsolePanicHook();
// enabling browser panic hooks will create a full-page DIV with panic details
// this is useful for mobile devices where console is not available
// initBrowserPanicHook();

// if port is not specified, it will use the default port for the specified network
const rpc = new RpcClient("127.0.0.1", Encoding.Borsh, "testnet-10");
const rpc = new RpcClient({
    url : "127.0.0.1",
    encoding : Encoding.Borsh,
    networkId : "testnet-10"
});


(async () => {
    try {
        await rpc.connect();
        let info = await rpc.getInfo();
        console.log(info);
    } finally {
        await rpc.disconnect();
    }
})();
```

For more details, please follow the [**integrating with apsaK**](https://apsak.aspectron.org/) guide.

*/

#![allow(unused_imports)]

#[cfg(all(
    any(feature = "wasm32-sdk", feature = "wasm32-rpc", feature = "wasm32-core", feature = "wasm32-keygen"),
    not(target_arch = "wasm32")
))]
compile_error!("`apsak-wasm` crate for WASM32 target must be built with `--features wasm32-sdk|wasm32-rpc|wasm32-core|wasm32-keygen`");

mod version;
pub use version::*;

cfg_if::cfg_if! {

    if #[cfg(feature = "wasm32-sdk")] {

        pub use apsak_addresses::{Address, Version as AddressVersion};
        pub use apsak_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use apsak_pow::wasm::*;

        pub mod rpc {
            //! apsaK RPC interface
            //!

            pub mod messages {
                //! apsaK RPC messages
                pub use apsak_rpc_core::model::message::*;
            }
            pub use apsak_rpc_core::api::rpc::RpcApi;
            pub use apsak_rpc_core::wasm::message::*;

            pub use apsak_wrpc_wasm::client::*;
            pub use apsak_wrpc_wasm::resolver::*;
            pub use apsak_wrpc_wasm::notify::*;
        }

        pub use apsak_consensus_wasm::*;
        pub use apsak_wallet_keys::prelude::*;
        pub use apsak_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-core")] {

        pub use apsak_addresses::{Address, Version as AddressVersion};
        pub use apsak_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use apsak_pow::wasm::*;

        pub mod rpc {
            //! apsaK RPC interface
            //!

            pub mod messages {
                //! apsaK RPC messages
                pub use apsak_rpc_core::model::message::*;
            }
            pub use apsak_rpc_core::api::rpc::RpcApi;
            pub use apsak_rpc_core::wasm::message::*;

            pub use apsak_wrpc_wasm::client::*;
            pub use apsak_wrpc_wasm::resolver::*;
            pub use apsak_wrpc_wasm::notify::*;
        }

        pub use apsak_consensus_wasm::*;
        pub use apsak_wallet_keys::prelude::*;
        pub use apsak_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-rpc")] {

        pub use apsak_rpc_core::api::rpc::RpcApi;
        pub use apsak_rpc_core::wasm::message::*;
        pub use apsak_rpc_core::wasm::message::IPingRequest;
        pub use apsak_wrpc_wasm::client::*;
        pub use apsak_wrpc_wasm::resolver::*;
        pub use apsak_wrpc_wasm::notify::*;
        pub use apsak_wasm_core::types::*;

    } else if #[cfg(feature = "wasm32-keygen")] {

        pub use apsak_addresses::{Address, Version as AddressVersion};
        pub use apsak_wallet_keys::prelude::*;
        pub use apsak_wasm_core::types::*;

    }
}
