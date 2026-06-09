#!/usr/bin/env bash

set -euo pipefail

if [ "$EUID" -ne 0 ]
then
    exec sudo -s "$0" "$@"
fi

WORKSPACE_PATH={{WORKING_DIRECTORY}}
FILENAME={{FILENAME}}

sudo useradd --system --no-create-home lrima 2>/dev/null || true

sudo apt-get update
sudo apt-get upgrade -y -q;

sudo apt-get install bluez bluetooth python3 bluez-tools python3-pip python3-venv git -q -y

sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl restart bluetooth
sudo systemctl enable bluetooth

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

cp /tmp/config.ini .

python3 -m venv venv
if [[ $IS_RASP_VERSION_5 == true ]]; then
	venv/bin/pip install -r requirements_pi5.txt
else
	venv/bin/pip install -r requirements_pi4.txt
fi
sudo systemctl disable LRIMa-centrale-install-runonce.service
sudo -k
