## Usage
```bash
$ ./encrypter [action] [target] [keyfile]
```

## Arguments
`[Action]`: 
 * `-d`: Decrypt file 
 * `-e`: Encrypt file
 * `-c`: Generate public and private keys

`[Target]`:
 * **For Encryption and Decryption:** File to encrypt/decrypt directory (Output goes to `./`)
 * **For Key Generation:** Output directory (if same directory use `.`)

`[Keyfile]`:  
 * **Public/Private Key** directory _(Unused in key generation)_

## Notes
 * Encryption method is `RSA`.
 * Keyfiles must be `.pem` files, following the `pkcs1` standard.
 * Keyfiles follow the `LF` line ending pattern.
 * Encrypted output is a `.bin` file, but the program theoretically decrypt any type of binary file. 

## Examples

* Key Generation
    ```bash
    $ ./encrypter -c . # Creates keyfiles in same directory
    ```

* Encryption
    ```bash
    $ ./encrypter -e ./text.txt ./public_key.pem
    ```

* Decryption
    ```bash
    $ ./encrypter -d ./encrypted_text.bin ./private_key.pem
    ```