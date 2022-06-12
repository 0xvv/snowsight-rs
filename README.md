# snowsight-rs
Basic app to connect to snowsight mempool streaming service in Rust


You will need to set the env var `BOT_PWD` with a 32 chars long password to encrypt and decrypt the private key.

And run
```shell
snowsight-rs <private-key>
```

This will store your encrypted private key in the home directory for later use

You can then run the example
```shell
snowsight-rs 
```