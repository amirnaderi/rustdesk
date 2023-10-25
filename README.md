- windows command

- linux

<p align="center">
  <img src="res/logo-header.svg" alt="DsHelpDesk - Your remote desktop"><br>
  <a href="#free-public-servers">Servers</a> •
  <a href="#raw-steps-to-build">Build</a> •
  <a href="#how-to-build-with-docker">Docker</a> •
  <a href="#file-structure">Structure</a> •
  <a href="#snapshot">Snapshot</a><br>
  [<a href="docs/README-UA.md">Українська</a>] | [<a href="docs/README-CS.md">česky</a>] | [<a href="docs/README-ZH.md">中文</a>] | [<a href="docs/README-HU.md">Magyar</a>] | [<a href="docs/README-ES.md">Español</a>] | [<a href="docs/README-FA.md">فارسی</a>] | [<a href="docs/README-FR.md">Français</a>] | [<a href="docs/README-DE.md">Deutsch</a>] | [<a href="docs/README-PL.md">Polski</a>] | [<a href="docs/README-ID.md">Indonesian</a>] | [<a href="docs/README-FI.md">Suomi</a>] | [<a href="docs/README-ML.md">മലയാളം</a>] | [<a href="docs/README-JP.md">日本語</a>] | [<a href="docs/README-NL.md">Nederlands</a>] | [<a href="docs/README-IT.md">Italiano</a>] | [<a href="docs/README-RU.md">Русский</a>] | [<a href="docs/README-PTBR.md">Português (Brasil)</a>] | [<a href="docs/README-EO.md">Esperanto</a>] | [<a href="docs/README-KR.md">한국어</a>] | [<a href="docs/README-AR.md">العربي</a>] | [<a href="docs/README-VN.md">Tiếng Việt</a>] | [<a href="docs/README-DA.md">Dansk</a>] | [<a href="docs/README-GR.md">Ελληνικά</a>] | [<a href="docs/README-TR.md">Türkçe</a>]<br>
  <b>We need your help to translate this README, <a href="https://github.com/rustdesk/dshelpdesk/tree/master/src/lang">DsHelpDesk UI</a> and <a href="https://github.com/rustdesk/doc.dshelpdesk.com">DsHelpDesk Doc</a> to your native language</b>
</p>

Chat with us: [Discord](https://discord.gg/nDceKgxnkV) | [Twitter](https://twitter.com/dshelpdesk) | [Reddit](https://www.reddit.com/r/dshelpdesk)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/I2I04VU09) 

[![Open Bounties](https://img.shields.io/endpoint?url=https%3A%2F%2Fconsole.algora.io%2Fapi%2Fshields%2Fdshelpdesk%2Fbounties%3Fstatus%3Dopen)](https://console.algora.io/org/dshelpdesk/bounties?status=open)

Yet another remote desktop software, written in Rust. Works out of the box, no configuration required. You have full control of your data, with no concerns about security. You can use our rendezvous/relay server, [set up your own](https://dshelpdesk.com/server), or [write your own rendezvous/relay server](https://github.com/rustdesk/dshelpdesk-server-demo).

![image](https://user-images.githubusercontent.com/71636191/171661982-430285f0-2e12-4b1d-9957-4a58e375304d.png)

DsHelpDesk welcomes contribution from everyone. See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for help getting started.

[**FAQ**](https://github.com/rustdesk/dshelpdesk/wiki/FAQ)

[**BINARY DOWNLOAD**](https://github.com/rustdesk/dshelpdesk/releases)

[**NIGHTLY BUILD**](https://github.com/rustdesk/dshelpdesk/releases/tag/nightly)

[<img src="https://fdroid.gitlab.io/artwork/badge/get-it-on.png"
    alt="Get it on F-Droid"
    height="80">](https://f-droid.org/en/packages/com.carriez.flutter_hbb)

## Free Public Servers

Below are the servers you are using for free, they may change over time. If you are not close to one of these, your network may be slow.
| Location | Vendor | Specification |
| --------- | ------------- | ------------------ |
| Germany | [Hetzner](https://www.hetzner.com) | 2 vCPU / 4 GB RAM |
| Ukraine (Kyiv) | [dc.volia](https://dc.volia.com) | 2 vCPU / 4 GB RAM |

## Dev Container

[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Container&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/rustdesk/dshelpdesk)

If you already have VS Code and Docker installed, you can click the badge above to get started. Clicking will cause VS Code to automatically install the Dev Containers extension if needed, clone the source code into a container volume, and spin up a dev container for use.

Go through [DEVCONTAINER.md](docs/DEVCONTAINER.md) for more info.

## Dependencies

Desktop versions use [Sciter](https://sciter.com/) or Flutter for GUI, this tutorial is for Sciter only.

Please download Sciter dynamic library yourself.

[Windows](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.win/x64/sciter.dll) |
[Linux](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so) |
[macOS](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.osx/libsciter.dylib)

## Raw steps to build

- Prepare your Rust development env and C++ build env

- Install [vcpkg](https://github.com/microsoft/vcpkg), and set `VCPKG_ROOT` env variable correctly

  - Windows: vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static
  - Linux/macOS: vcpkg install libvpx libyuv opus aom

- run `cargo run`

## [Build](https://dshelpdesk.com/docs/en/dev/build/)

## How to build on Linux

### Ubuntu 18 (Debian 10)

```sh
sudo apt install -y zip g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev \
        libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake make \
        libclang-dev ninja-build libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
```

### openSUSE Tumbleweed 

```sh
sudo zypper install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libXfixes-devel cmake alsa-lib-devel gstreamer-devel gstreamer-plugins-base-devel xdotool-devel
```
### Fedora 28 (CentOS 8)

```sh
sudo yum -y install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libxdo-devel libXfixes-devel pulseaudio-libs-devel cmake alsa-lib-devel
```

### Arch (Manjaro)

```sh
sudo pacman -Syu --needed unzip git cmake gcc curl wget yasm nasm zip make pkg-config clang gtk3 xdotool libxcb libxfixes alsa-lib pipewire
```

### Install vcpkg

```sh
git clone https://github.com/microsoft/vcpkg
cd vcpkg
git checkout 2023.04.15
cd ..
vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg
vcpkg/vcpkg install libvpx libyuv opus aom
```

### Fix libvpx (For Fedora)

```sh
cd vcpkg/buildtrees/libvpx/src
cd *
./configure
sed -i 's/CFLAGS+=-I/CFLAGS+=-fPIC -I/g' Makefile
sed -i 's/CXXFLAGS+=-I/CXXFLAGS+=-fPIC -I/g' Makefile
make
cp libvpx.a $HOME/vcpkg/installed/x64-linux/lib/
cd
```

### Build

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone https://github.com/rustdesk/dshelpdesk
cd dshelpdesk
mkdir -p target/debug
wget https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so
mv libsciter-gtk.so target/debug
VCPKG_ROOT=$HOME/vcpkg cargo run
```

### Change Wayland to X11 (Xorg)

DsHelpDesk does not support Wayland. Check [this](https://docs.fedoraproject.org/en-US/quick-docs/configuring-xorg-as-default-gnome-session/) to configuring Xorg as the default GNOME session.

## Wayland support

Wayland does not seem to provide any API for sending keypresses to other windows. Therefore, the DsHelpDesk uses an API from a lower level, namely the `/dev/uinput` device (Linux kernel level).

When Wayland is the controlled side, you have to start in the following way:
```bash
# Start uinput service
$ sudo dshelpdesk --service
$ dshelpdesk
```
**Notice**: Wayland screen recording uses different interfaces. DsHelpDesk currently only supports org.freedesktop.portal.ScreenCast.
```bash
$ dbus-send --session --print-reply       \
  --dest=org.freedesktop.portal.Desktop \
  /org/freedesktop/portal/desktop       \
  org.freedesktop.DBus.Properties.Get   \
  string:org.freedesktop.portal.ScreenCast string:version
# Not support
Error org.freedesktop.DBus.Error.InvalidArgs: No such interface “org.freedesktop.portal.ScreenCast”
# Support
method return time=1662544486.931020 sender=:1.54 -> destination=:1.139 serial=257 reply_serial=2
   variant       uint32 4
```

## How to build with Docker

Begin by cloning the repository and building the Docker container:

```sh
git clone https://github.com/rustdesk/dshelpdesk
cd dshelpdesk
docker build -t "dshelpdesk-builder" .
```

Then, each time you need to build the application, run the following command:

```sh
docker run --rm -it -v $PWD:/home/user/dshelpdesk -v dshelpdesk-git-cache:/home/user/.cargo/git -v dshelpdesk-registry-cache:/home/user/.cargo/registry -e PUID="$(id -u)" -e PGID="$(id -g)" dshelpdesk-builder
```

Note that the first build may take longer before dependencies are cached, subsequent builds will be faster. Additionally, if you need to specify different arguments to the build command, you may do so at the end of the command in the `<OPTIONAL-ARGS>` position. For instance, if you wanted to build an optimized release version, you would run the command above followed by `--release`. The resulting executable will be available in the target folder on your system, and can be run with:

```sh
target/debug/dshelpdesk
```

Or, if you're running a release executable:

```sh
target/release/dshelpdesk
```

Please ensure that you are running these commands from the root of the DsHelpDesk repository, otherwise the application might not be able to find the required resources. Also note that other cargo subcommands such as `install` or `run` are not currently supported via this method as they would install or run the program inside the container instead of the host.

## File Structure

- **[libs/hbb_common](https://github.com/rustdesk/dshelpdesk/tree/master/libs/hbb_common)**: video codec, config, tcp/udp wrapper, protobuf, fs functions for file transfer, and some other utility functions
- **[libs/scrap](https://github.com/rustdesk/dshelpdesk/tree/master/libs/scrap)**: screen capture
- **[libs/enigo](https://github.com/rustdesk/dshelpdesk/tree/master/libs/enigo)**: platform specific keyboard/mouse control
- **[src/ui](https://github.com/rustdesk/dshelpdesk/tree/master/src/ui)**: GUI
- **[src/server](https://github.com/rustdesk/dshelpdesk/tree/master/src/server)**: audio/clipboard/input/video services, and network connections
- **[src/client.rs](https://github.com/rustdesk/dshelpdesk/tree/master/src/client.rs)**: start a peer connection
- **[src/rendezvous_mediator.rs](https://github.com/rustdesk/dshelpdesk/tree/master/src/rendezvous_mediator.rs)**: Communicate with [dshelpdesk-server](https://github.com/rustdesk/dshelpdesk-server), wait for remote direct (TCP hole punching) or relayed connection
- **[src/platform](https://github.com/rustdesk/dshelpdesk/tree/master/src/platform)**: platform specific code
- **[flutter](https://github.com/rustdesk/dshelpdesk/tree/master/flutter)**: Flutter code for mobile
- **[flutter/web/js](https://github.com/rustdesk/dshelpdesk/tree/master/flutter/web/js)**: JavaScript for Flutter web client

## Snapshots

![image](https://user-images.githubusercontent.com/71636191/113112362-ae4deb80-923b-11eb-957d-ff88daad4f06.png)

![image](https://user-images.githubusercontent.com/71636191/113112619-f705a480-923b-11eb-911d-97e984ef52b6.png)

![image](https://user-images.githubusercontent.com/71636191/113112857-3fbd5d80-923c-11eb-9836-768325faf906.png)

![image](https://user-images.githubusercontent.com/71636191/135385039-38fdbd72-379a-422d-b97f-33df71fb1cec.png)
