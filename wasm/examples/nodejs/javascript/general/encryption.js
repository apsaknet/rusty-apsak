const apsak = require('../../../../nodejs/apsak');

apsak.initConsolePanicHook();

(async () => {

    let encrypted = apsak.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = apsak.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
