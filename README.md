# gametimer

Easy program to lock computer after certain time. Made to prevent my brother from playing on my computer more than I allow him.

By the way, this is the first Rust project which is really useful for me.

## How to use

Execute the binary with duration in an abbreviated format as the first command-line argument 

**Example:**
```shell
[binary name] 1h30m45s
```

## Crates used

- [humantime](https://github.com/chronotope/humantime) to format timestamps
- [winrt-notification](https://github.com/allenbenz/winrt-notification) to send notification
