#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Nicolas Gabriel Cotti

FTDI_LIB="libftd2xx.so"
FTDI_VERSION="1.4.35"
FTDI_PATH="ftdi/${FTDI_LIB}.${FTDI_VERSION}"

## Download .tar file and move library to /usr/local/lib
if ! find "/usr/lib" "/usr/local/lib" -name "${FTDI_LIB}" | grep -q .; then
    printf "Installing FTDI library %s\n" "${FTDI_LIB}.${FTDI_VERSION}"
    sudo cp "${FTDI_PATH}" "/usr/local/lib/"
    sudo ln -s "/usr/local/lib/${FTDI_LIB}.${FTDI_VERSION}" "/usr/local/lib/${FTDI_LIB}"
    sudo chmod 0755 "/usr/local/lib/${FTDI_LIB}.${FTDI_VERSION}"

    # Move header files aswell
    sudo mkdir -p "/usr/local/include"
    sudo cp "ftdi/ftd2xx.h" "/usr/local/include/"
    sudo cp "ftdi/WinTypes.h" "/usr/local/include/"
fi

## If required, you may install again with:
# sudo modprobe ftdi_sio
if lsmod | grep -q "ftdi_sio"; then
    printf "Removing module \"ftdi_sio\".\n"
    sudo rmmod ftdi_sio
fi

if lsmod | grep -q "usbserial"; then
    printf "Removing module \"usbserial\".\n"
    sudo rmmod usbserial
fi








