Name:       dshldesk
Version:    1.2.3
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb1 xdotool libXfixes3 alsa-utils libXtst6 libappindicator-gtk3 libvdpau1 libva2 pam gstreamer-plugins-base gstreamer-plugin-pipewire
Provides:   libdesktop_drop_plugin.so()(64bit), libdesktop_multi_window_plugin.so()(64bit), libflutter_custom_cursor_plugin.so()(64bit), libflutter_linux_gtk.so()(64bit), libscreen_retriever_plugin.so()(64bit), libtray_manager_plugin.so()(64bit), liburl_launcher_linux_plugin.so()(64bit), libwindow_manager_plugin.so()(64bit), libwindow_size_plugin.so()(64bit), libtexture_rgba_renderer_plugin.so()(64bit)

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

# %global __python %{__python3}

%install

mkdir -p "%{buildroot}/usr/lib/dshldesk" && cp -r ${HBB}/flutter/build/linux/x64/release/bundle/* -t "%{buildroot}/usr/lib/dshldesk"
mkdir -p "%{buildroot}/usr/bin"
install -Dm 644 $HBB/res/dshldesk.service -t "%{buildroot}/usr/share/dshldesk/files"
install -Dm 644 $HBB/res/dshldesk.desktop -t "%{buildroot}/usr/share/dshldesk/files"
install -Dm 644 $HBB/res/dshldesk-link.desktop -t "%{buildroot}/usr/share/dshldesk/files"
install -Dm 644 $HBB/res/128x128@2x.png "%{buildroot}/usr/share/icons/hicolor/256x256/apps/dshldesk.png"
install -Dm 644 $HBB/res/scalable.svg "%{buildroot}/usr/share/icons/hicolor/scalable/apps/dshldesk.svg"

%files
/usr/lib/dshldesk/*
/usr/share/dshldesk/files/dshldesk.service
/usr/share/icons/hicolor/256x256/apps/dshldesk.png
/usr/share/icons/hicolor/scalable/apps/dshldesk.svg
/usr/share/dshldesk/files/dshldesk.desktop
/usr/share/dshldesk/files/dshldesk-link.desktop

%changelog
# let's skip this for now

# https://www.cnblogs.com/xingmuxin/p/8990255.html
%pre
# can do something for centos7
case "$1" in
  1)
    # for install
  ;;
  2)
    # for upgrade
    systemctl stop dshldesk || true
  ;;
esac

%post
cp /usr/share/dshldesk/files/dshldesk.service /etc/systemd/system/dshldesk.service
cp /usr/share/dshldesk/files/dshldesk.desktop /usr/share/applications/
cp /usr/share/dshldesk/files/dshldesk-link.desktop /usr/share/applications/
ln -s /usr/lib/dshldesk/dshldesk /usr/bin/dshldesk
systemctl daemon-reload
systemctl enable dshldesk
systemctl start dshldesk
update-desktop-database

%preun
case "$1" in
  0)
    # for uninstall
    systemctl stop dshldesk || true
    systemctl disable dshldesk || true
    rm /etc/systemd/system/dshldesk.service || true
  ;;
  1)
    # for upgrade
  ;;
esac

%postun
case "$1" in
  0)
    # for uninstall
    rm /usr/share/applications/dshldesk.desktop || true
    rm /usr/share/applications/dshldesk-link.desktop || true
    rm /usr/bin/dshldesk || true
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
