<p align="center">
  <img src="../res/logo-header.svg" alt="DsHelpDesk - Ваш віддалений робочий стіл"><br>
  <a href="#безкоштовні-загальнодоступні-сервери">Сервери</a> •
  <a href="#первинні-кроки-для-складання">Складання</a> •
  <a href="#як-зібрати-за-допомогою-docker">Docker</a> •
  <a href="#структура-файлів">Структура</a> •
  <a href="#знімки">Знімки</a><br>
  [<a href="../README.md">English</a>] | [<a href="README-CS.md">česky</a>] | [<a href="README-ZH.md">中文</a>] | [<a href="README-HU.md">Magyar</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FA.md">فارسی</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-ID.md">Indonesian</a>] | [<a href="README-FI.md">Suomi</a>] | [<a href="README-ML.md">മലയാളം</a>] | [<a href="README-JP.md">日本語</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-IT.md">Italiano</a>] | [<a href="README-RU.md">Русский</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] | [<a href="README-EO.md">Esperanto</a>] | [<a href="README-KR.md">한국어</a>] | [<a href="README-AR.md">العربي</a>] | [<a href="README-VN.md">Tiếng Việt</a>] | [<a href="README-GR.md">Ελληνικά</a>]<br>
  <b>Нам потрібна ваша допомога для перекладу цього README і <a href="https://github.com/rustdesk/dshelpdesk/tree/master/src/dshelpdesk/tree/master/src/lang">DsHelpDesk UI</a> на вашу рідну мову</B>
</p>

Спілкування з нами: [Discord](https://discord.gg/nDceKgxnkV) | [Twitter](https://twitter.com/dshelpdesk) | [Reddit](https://www.reddit.com/r/dshelpdesk)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/I2I04VU09)

Ще одне програмне забезпечення для віддаленого робочого столу, написане на Rust. Працює з коробки, не потребує налаштування. Ви повністю контролюєте свої дані, не турбуючись про безпеку. Ви можете використовувати наш сервер ретрансляції, [налаштувати свій власний](https://dshelpdesk.com/server), або [написати свій власний сервер ретрансляції](https://github.com/rustdesk/dshelpdesk-server-demo).

![image](https://user-images.githubusercontent.com/71636191/171661982-430285f0-2e12-4b1d-9957-4a58e375304d.png)

DsHelpDesk вітає внесок кожного. Дивіться [`docs/CONTRIBUTING.md`](CONTRIBUTING.md) для допомоги на початку роботи.

[**FAQ**](https://github.com/rustdesk/dshelpdesk/wiki/FAQ)

[**Як працює DsHelpDesk?**](https://github.com/rustdesk/dshelpdesk/wiki/How-does-DsHelpDesk-work%3F)

[**ЗАВАНТАЖИТИ ЗАСТОСУНОК**](https://github.com/rustdesk/dshelpdesk/releases)

[<img src="https://fdroid.gitlab.io/artwork/badge/get-it-on.png"
    alt="Get it on F-Droid"
    height="80">](https://f-droid.org/en/packages/com.carriez.flutter_hbb)

## Безкоштовні загальнодоступні сервери

Нижче наведені сервери, для безкоштовного використання, вони можуть змінюватися з часом. Якщо ви не перебуваєте поруч з одним із них, ваша мережа може працювати повільно.
| Місцезнаходження | Постачальник | Технічні характеристики |
| --------- | ------------- | ------------------ |
| Німеччина | Hetzner | 2 VCPU / 4GB RAM |
| Україна (Київ) | [dc.volia](https://dc.volia.com) | 2 vCPU / 4GB RAM |

## Dev Container

[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Container&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/rustdesk/dshelpdesk)

Якщо у вас уже встановлено VS Code і Docker, ви можете натиснути значок вище, щоб почати. Клацання призведе до того, що VS Code автоматично встановить розширення Dev Containers, якщо це необхідно, клонує виcхідний код у том контейнера та розгорне контейнер dev для використання.

Дивіться [DEVCONTAINER.md](docs/DEVCONTAINER.md) для додаткової інфо.

## Залежності

Настільні версії використовують [sciter](https://sciter.com/) для графічного інтерфейсу, завантажте динамічну бібліотеку sciter самостійно.

[Windows](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.win/x64/sciter.dll) |
[Linux](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so) |
[MacOS](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.osx/libsciter.dylib)

Мобільні версії використовують Flutter. У майбутньому ми перенесемо настільну версію зі Sciter на Flutter.

## Первинні кроки для складання

- Підготуйте середовище розробки Rust і середовище збірки C++.

- Встановіть [vcpkg](https://github.com/microsoft/vcpkg), і правильно встановіть змінну `VCPKG_ROOT`.

  - Windows: vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static
  - Linux/MacOS: vcpkg install libvpx libyuv opus aom

- Запустіть `cargo run`

## Як зібрати на Linux 

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

### Встановлення vcpkg

```sh
git clone https://github.com/microsoft/vcpkg
cd vcpkg
git checkout 2023.04.15
cd ...
vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg
vcpkg/vcpkg install libvpx libyuv opus aom
```

### Виправлення libvpx (для Fedora)

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

### Збірка

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

## Як зібрати за допомогою Docker

Почніть з клонування сховища та створення docker-контейнера:

```sh
git clone https://github.com/rustdesk/dshelpdesk
cd dshelpdesk
docker build -t "dshelpdesk-builder" .
```

Потім кожного разу, коли вам потрібно зібрати додаток, запускайте таку команду:

```sh
docker run --rm -it -v $PWD:/home/user/dshelpdesk -v dshelpdesk-git-cache:/home/user/.cargo/git -v dshelpdesk-registry-cache:/home/user/.cargo/registry -e PUID="$(id -u)" -e PGID="$(id -g)" dshelpdesk-builder
```

Зверніть увагу, що перша збірка може зайняти більше часу, перш ніж залежності будуть кешовані, але наступні збірки будуть виконуватися швидше. Крім того, якщо вам потрібно вказати інші аргументи для команди збірки, ви можете зробити це в кінці команди у змінній `<OPTIONAL-ARGS>`. Наприклад, якщо ви хочете створити оптимізовану версію, ви маєте запустити наведену вище команду і в кінці рядка додати `--release`. Отриманий виконуваний файл буде доступний у цільовій папці вашої системи і може бути запущений за допомогою:

```sh
target/debug/dshelpdesk
```

Або, якщо ви використовуєте виконуваний файл релізу:

```sh
target/release/dshelpdesk
```

Будь ласка, переконайтеся, що ви запускаєте ці команди з кореня сховища DsHelpDesk, інакше додаток не зможе знайти необхідні ресурси. Також зверніть увагу, що інші cargo підкоманди, такі як `install` або `run`, наразі не підтримуються цим методом, оскільки вони будуть встановлювати або запускати програму всередині контейнера, а не на хості.

## Структура файлів

- **[libs/hbb_common](https://github.com/rustdesk/dshelpdesk/tree/master/libs/hbb_common)**: відеокодек, конфіг, обгортка tcp/udp, protobuf, функції fs для передавання файлів і деякі інші службові функції
- **[libs/scrap](https://github.com/rustdesk/dshelpdesk/tree/master/libs/scrap)**: захоплення екрана
- **[libs/enigo](https://github.com/rustdesk/dshelpdesk/tree/master/libs/enigo)**: специфічне для платформи керування клавіатурою/мишею
- **[src/ui](https://github.com/rustdesk/dshelpdesk/tree/master/src/ui)**: графічний інтерфейс користувача
- **[src/server](https://github.com/rustdesk/dshelpdesk/tree/master/src/server)**: сервіси аудіо/буфера обміну/вводу/відео та мережевих підключень
- **[src/client.rs](https://github.com/rustdesk/dshelpdesk/tree/master/src/client.rs)**: однорангове з'єднання
- **[src/rendezvous_mediator.rs](https://github.com/rustdesk/dshelpdesk/tree/master/src/rendezvous_mediator.rs)**: комунікація з [dshelpdesk-server](https://github.com/rustdesk/dshelpdesk-server), очікування віддаленого прямого (обхід TCP NAT) або ретрансльованого з'єднання
- **[src/platform](https://github.com/rustdesk/dshelpdesk/tree/master/src/platform)**: специфічний для платформи код
- **[flutter](https://github.com/rustdesk/dshelpdesk/tree/master/flutter)**: код Flutter для мобільних пристроїв 
- **[flutter/web/js](https://github.com/rustdesk/dshelpdesk/tree/master/flutter/web/js)**: JavaScript для Flutter веб клієнту

## Знімки

![image](https://user-images.githubusercontent.com/71636191/113112362-ae4deb80-923b-11eb-957d-ff88daad4f06.png)

![image](https://user-images.githubusercontent.com/71636191/113112619-f705a480-923b-11eb-911d-97e984ef52b6.png)

![image](https://user-images.githubusercontent.com/71636191/113112857-3fbd5d80-923c-11eb-9836-768325faf906.png)

![image](https://user-images.githubusercontent.com/71636191/135385039-38fdbd72-379a-422d-b97f-33df71fb1cec.png)
