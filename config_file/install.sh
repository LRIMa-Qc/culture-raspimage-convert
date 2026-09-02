#!/usr/bin/env bash

set -euo pipefail

sudo iw reg set {{WIFI_COUNTRY}}
sudo raspi-config nonint do_wifi_country {{WIFI_COUNTRY}}
sudo rfkill unblock bluetooth

if [ "$EUID" -ne 0 ]
then
    exec sudo -s "$0" "$@"
fi

WORKSPACE_PATH={{WORKING_DIRECTORY}}
FILENAME={{FILENAME}}

sudo useradd -m {{ACCOUNT_NAME}} 2>/dev/null || true
echo "{{ACCOUNT_NAME}}:{{ACCOUNT_PASSWORD}}" | sudo chpasswd
sudo usermod -aG sudo,video,gpio,i2c,spi {{ACCOUNT_NAME}}

for i in {1..300}; do ping -c1 www.google.com &> /dev/null && break; done

sudo apt-get update
sudo apt-get upgrade -y -q;


sudo apt-get install openssh-server -q -y

sudo systemctl enable --now networkmanager
sudo systemctl enable --now ssh

sudo apt-get install bluez bluetooth python3 bluez-tools python3-pip python3-venv git -q -y
sudo apt-get install python3-gpiozero python3-pigpio python3-picamera2 python3-libcamera -q -y

wget https://github.com/joan2937/pigpio/archive/master.zip
unzip master.zip
cd pigpio-master
make
sudo make install

sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl restart bluetooth
sudo systemctl enable --now bluetooth
sudo systemctl enable --now networkmanager
sudo systemctl enable --now ssh

RASP_VERSION="$(cat /sys/firmware/devicetree/base/model)"
IS_RASP_VERSION_5=false
if [[ $RASP_VERSION == *"Raspberry Pi 5"* ]]; then
	IS_RASP_VERSION_5=true
fi

mkdir -p "$WORKSPACE_PATH"
if [[ ! -d "$WORKSPACE_PATH/$FILENAME/.git" ]]; then
	git clone https://github.com/LRIMa-Qc/iot_obj-sicro-sensor.git "$WORKSPACE_PATH/$FILENAME"
fi
cd "$WORKSPACE_PATH/$FILENAME/code/central"

cp "/var/local/LRIMa-central/config.ini" .

python3 -m venv --system-site-packages venv
if [[ $IS_RASP_VERSION_5 == true ]]; then
	venv/bin/pip install -q -r requirements_pi5.txt
else
	venv/bin/pip install -q -r requirements_pi4.txt
fi
sudo chown -R {{ACCOUNT_NAME}} $WORKSPACE_PATH/$FILENAME

sudo systemctl enable --now LRIMa-camera-setup.service
