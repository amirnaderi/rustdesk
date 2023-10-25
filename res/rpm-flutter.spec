Name:       dshelpdesk
Version:    1.2.4
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb libxdo libXfixes alsa-lib libappindicator-gtk3 libvdpau libva pam gstreamer1-plugins-base
Provides:   libdesktop_drop_plugin.so()(64bit), libdesktop_multi_window_plugin.so()(64bit), libflutter_custom_cursor_plugin.so()(64bit), libflutter_linux_gtk.so()(64bit), libscreen_retriever_plugin.so()(64bit), libtray_manager_plugin.so()(64bit), liburl_launcher_linux_plugin.so()(64bit), libwindow_manager_plugin.so()(64bit), libwindow_size_plugin.so()(64bit), libtexture_rgba_renderer_plugin.so()(64bit)

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

# %global __python %{__python3}

%install

mkdir -p "%{buildroot}/usr/lib/dshelpdesk" && cp -r ${HBB}/flutter/build/linux/x64/release/bundle/* -t "%{buildroot}/usr/lib/dshelpdesk"
mkdir -p "%{buildroot}/usr/bin"
install -Dm 644 $HBB/res/dshelpdesk.service -t "%{buildroot}/usr/share/dshelpdesk/files"
install -Dm 644 $HBB/res/dshelpdesk.desktop -t "%{buildroot}/usr/share/dshelpdesk/files"
install -Dm 644 $HBB/res/dshelpdesk-link.desktop -t "%{buildroot}/usr/share/dshelpdesk/files"
install -Dm 644 $HBB/res/128x128@2x.png "%{buildroot}/usr/share/icons/hicolor/256x256/apps/dshelpdesk.png"
install -Dm 644 $HBB/res/scalable.svg "%{buildroot}/usr/share/icons/hicolor/scalable/apps/dshelpdesk.svg"

%files
/usr/lib/dshelpdesk/*
/usr/share/dshelpdesk/files/dshelpdesk.service
/usr/share/icons/hicolor/256x256/apps/dshelpdesk.png
/usr/share/icons/hicolor/scalable/apps/dshelpdesk.svg
/usr/share/dshelpdesk/files/dshelpdesk.desktop
/usr/share/dshelpdesk/files/dshelpdesk-link.desktop

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
    systemctl stop dshelpdesk || true
  ;;
esac

%post
cp /usr/share/dshelpdesk/files/dshelpdesk.service /etc/systemd/system/dshelpdesk.service
cp /usr/share/dshelpdesk/files/dshelpdesk.desktop /usr/share/applications/
cp /usr/share/dshelpdesk/files/dshelpdesk-link.desktop /usr/share/applications/
ln -s /usr/lib/dshelpdesk/dshelpdesk /usr/bin/dshelpdesk
systemctl daemon-reload
systemctl enable dshelpdesk
systemctl start dshelpdesk
update-desktop-database

%preun
case "$1" in
  0)
    # for uninstall
    systemctl stop dshelpdesk || true
    systemctl disable dshelpdesk || true
    rm /etc/systemd/system/dshelpdesk.service || true
  ;;
  1)
    # for upgrade
  ;;
esac

%postun
case "$1" in
  0)
    # for uninstall
    rm /usr/share/applications/dshelpdesk.desktop || true
    rm /usr/share/applications/dshelpdesk-link.desktop || true
    rm /usr/bin/dshelpdesk || true
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
