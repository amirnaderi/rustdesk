Name:       dshelpdesk
Version:    1.1.9
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb1 xdotool libXfixes3 alsa-utils libXtst6 libayatana-appindicator3-1 libvdpau1 libva2 pam gstreamer-plugins-base gstreamer-plugin-pipewire

%description
The best open-source remote desktop client software, written in Rust.

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

%global __python %{__python3}

%install
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/usr/lib/dshelpdesk/
mkdir -p %{buildroot}/usr/share/dshelpdesk/files/
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps/
mkdir -p %{buildroot}/usr/share/icons/hicolor/scalable/apps/
install -m 755 $HBB/target/release/dshelpdesk %{buildroot}/usr/bin/dshelpdesk
install $HBB/libsciter-gtk.so %{buildroot}/usr/lib/dshelpdesk/libsciter-gtk.so
install $HBB/res/dshelpdesk.service %{buildroot}/usr/share/dshelpdesk/files/
install $HBB/res/128x128@2x.png %{buildroot}/usr/share/icons/hicolor/256x256/apps/dshelpdesk.png
install $HBB/res/scalable.svg %{buildroot}/usr/share/icons/hicolor/scalable/apps/dshelpdesk.svg
install $HBB/res/dshelpdesk.desktop %{buildroot}/usr/share/dshelpdesk/files/
install $HBB/res/dshelpdesk-link.desktop %{buildroot}/usr/share/dshelpdesk/files/

%files
/usr/bin/dshelpdesk
/usr/lib/dshelpdesk/libsciter-gtk.so
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
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
