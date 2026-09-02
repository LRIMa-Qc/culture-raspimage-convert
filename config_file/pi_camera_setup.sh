#!/bin/bash
# Raspberry Pi 4 setup


# setup camera firmware
sudo tee /boot/firmware/config.txt > /dev/null <<'EOF'
# For more options and information see
# http://rptl.io/configtxt
# Some settings may impact device functionality. See link above for details

# Uncomment some or all of these to enable the optional hardware interfaces
#dtparam=i2c_arm=on
#dtparam=i2s=on
#dtparam=spi=on

# Enable audio (loads snd_bcm2835)
dtparam=audio=on

# Additional overlays and parameters are documented
# /boot/firmware/overlays/README

# Automatically load overlays for detected cameras
camera_auto_detect=0

# Automatically load overlays for detected DSI displays
display_auto_detect=1

# Automatically load initramfs files, if found
auto_initramfs=1

# Enable DRM VC4 V3D driver
dtoverlay=vc4-kms-v3d
max_framebuffers=2

# Don't have the firmware create an initial video= setting in cmdline.txt.
# Use the kernel's default instead.
disable_fw_kms_setup=1

# Run in 64-bit mode
arm_64bit=1

# Disable compensation for displays with overscan
disable_overscan=1

# Run as fast as firmware / board allows
arm_boost=1

[cm4]
# Enable host mode on the 2711 built-in XHCI USB controller.
# This line should be removed if the legacy DWC2 controller is required
# (e.g. for USB device mode) or if USB support is not required.
otg_mode=1

[cm5]
dtoverlay=dwc2,dr_mode=host

[all]
enable_uart=1
dtoverlay=imx708
EOF


#install camera firmware and modules

#required libs fot compiling
sudo apt install -y libboost-dev
sudo apt install -y libgnutls28-dev openssl libtiff-dev pybind11-dev
sudo apt install -y qtbase5-dev libqt5core5a libqt5widgets5
sudo apt install -y meson cmake
sudo apt install -y python3-yaml python3-ply
sudo apt install -y libglib2.0-dev libgstreamer-plugins-base1.0-dev

#Install libcamera to interface with arduicam camera modules
git clone https://github.com/raspberrypi/libcamera.git
cd libcamera
meson setup build --buildtype=release -Dgstreamer=enabled -Dpycamera=enabled
ninja -C build install

# Detect Config File
CONFIG_FILE="/boot/firmware/config.txt"
if [ ! -f "$CONFIG_FILE" ]; then
    CONFIG_FILE="/boot/config.txt"
fi
echo "Using $CONFIG_FILE"

echo "Camera setup complete! Reboot for changes to fully apply."
