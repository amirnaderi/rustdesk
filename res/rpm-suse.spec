Name:       dshldesk
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
mkdir -p %{buildroot}/usr/lib/dshldesk/
mkdir -p %{buildroot}/usr/share/dshldesk/files/
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps/
mkdir -p %{buildroot}/usr/share/icons/hicolor/scalable/apps/
install -m 755 $HBB/target/release/dshldesk %{buildroot}/usr/bin/dshldesk
install $HBB/libsciter-gtk.so %{buildroot}/usr/lib/dshldesk/libsciter-gtk.so
install $HBB/res/dshldesk.service %{buildroot}/usr/share/dshldesk/files/
install $HBB/res/128x128@2x.png %{buildroot}/usr/share/icons/hicolor/256x256/apps/dshldesk.png
install $HBB/res/scalable.svg %{buildroot}/usr/share/icons/hicolor/scalable/apps/dshldesk.svg
install $HBB/res/dshldesk.desktop %{buildroot}/usr/share/dshldesk/files/
install $HBB/res/dshldesk-link.desktop %{buildroot}/usr/share/dshldesk/files/

%files
/usr/bin/dshldesk
/usr/lib/dshldesk/libsciter-gtk.so
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
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
