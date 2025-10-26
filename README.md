# WebSocket Reflector X

> [!TIP]
> This library is indeed under maintenance, though the development frequency seems relatively low. The low frequency stems from the fact that as a simple tool, it really has little room for further development. If you encounter any issues during use, feel free to raise them in the issues section, and we will respond as soon as possible.
>
> 这个库确实处于维护状态，看起来开发频率并不高是因为作为一个简单的工具，它确实没有太多可以继续开发的地方，如果在使用过程中遇到问题，请随时在 issue 中提出，我们会尽快响应。

Controlled TCP-over-WebSocket forwarding tunnel.

[Crate Homepage](https://crates.io/crates/wsrx) | [Crate Docs](https://docs.rs/crate/wsrx/latest)

## Usage

Download from [Releases](https://github.com/XDSEC/WebSocketReflectorX/releases) page and run it.

### Desktop App

[#madewithslint](https://slint.dev/showcase.html) !

#### For Mac Users

> [!CAUTION]
> Apple Developer requires a mac and developer account, which I neither have.
>
> so that's required you to do some operations manually before using it, apologize for that.

Run the command before using:

```
sudo xattr -cr ./WebSocketReflectorX.app
```

#### For Arch Linux Users

We do not have desktop app binary packages in release, so `wsrx-bin` is deprecated and package ownership was taken by unknown one, do not use it.

```bash
# Use the appimage version from release package
paru -S wsrx-appimage
# Use git version and build it from scratch
paru -S wsrx-git
```

### Command Line Tools

This release contains the desktop GUI application and the cli daemon, so it's size may too big for some users.

you can just install the command-line tool from Cargo:

```
cargo install wsrx
```

then you can use `wsrx` directly without desktop application.

## **PRs about GitHub CI for other linux distro are welcome!**

## Development

```bash
cargo build --release --bins
```

and find the binary in `target/release`.

## Further Reading

If you want to intergrate `wsrx` in your own server project, you can read the [crate docs](https://docs.rs/crate/wsrx/latest).

Also, `wsrx` is a simple tool that using plain WebSocket protocol to tunnel TCP connections,
so you can implement your own server / client in other languages you like.
You can read the [Protocol Docs](docs/PROTOCOL.md) for more information.

## Desktop Preview

![Home Page](arts/sample-1.png)

![Connections Page](arts/sample-2.png)

![Network Logs Page](arts/sample-3.png)

![Settings Page](arts/sample-4.png)
