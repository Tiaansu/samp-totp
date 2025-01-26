# samp-totp

A plugin for generating/verifying 2FA authentication tokens per TOTP for samp/open.mp in Rust. Made using [totp-rs](https://docs.rs/totp-rs/latest/totp_rs).

## Installation
- Download the suitable binary files from releases for your operating system.
- Add it to your `plugins` folder
- Add `samp_totp` to server.cfg (config.json if you're using open.mp) or `samp_totp.so` for linux.
- Add [samp_totp.inc](./include/samp_totp.inc) in includes folder.

## Building
- Clone the repo
```bash
git clone https://github.com/Tiaansu/samp-cron.git
```
- Install Rust
```bash
rustup update stable-i686 --no-self-update && rustup default stable-i686
```
- Build using `cargo build`
```bash
cargo build --release
```

## API
* ### totp_generate_secret(const output[], size = sizeof output)
    * `output` - the secret will be stored
    * `size` - the maximum size `output` can hold

    **Returns**   
    * true/false

    **Example**
    ```pawn
    main()
    {
        new output[MAX_TOTP_SECRET_LENGTH];
        totp_generate_secret(output, sizeof(output));
    }
    ```

* ### totp_verify(const secret[], const otp[])
    * `secret` - the output of `totp_generate_secret`.
    * `otp` - the OTP (from your authenticator app)

    **Returns**   
    * true if valid, false if invalid

    **Example**
    ```pawn
    main()
    {
        new output[MAX_TOTP_SECRET_LENGTH];
        totp_generate_secret(output, sizeof(output));
        
        // replace the 123456 to the actual OTP
        printf("OTP is %s", totp_verify(output, "123456") ? "Valid" : "Invalid"); 
    }
    ```