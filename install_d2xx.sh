#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Nicolas Gabriel Cotti

FTDI_VERSION="1.4.35"
FTDI_SRC_PATH="tmp/linux-x86_64"

FTDI_DYN_LIB="libftd2xx.so"
FTDI_STATIC_LIB="libftd2xx.a"

DST_LIB_PATH="/usr/local/lib"
DST_HEADER_PATH="/usr/local/include"

## Move library to /usr/local/lib
if ! find "/usr/lib" "/usr/local/lib" -name "${FTDI_DYN_LIB}" | grep -q .; then
    printf "Installing FTDI library %s\n" "${FTDI_DYN_LIB}.${FTDI_VERSION}"
    sudo cp "${FTDI_SRC_PATH}/${FTDI_DYN_LIB}" "${DST_LIB_PATH}/"
    sudo cp "${FTDI_SRC_PATH}/${FTDI_DYN_LIB}.${FTDI_VERSION}" "${DST_LIB_PATH}/"
    sudo cp "${FTDI_SRC_PATH}/${FTDI_STATIC_LIB}" "${DST_LIB_PATH}/"
    sudo chmod 0755 \
        "${DST_LIB_PATH}/${FTDI_DYN_LIB}" \
        "${DST_LIB_PATH}/${FTDI_DYN_LIB}.${FTDI_VERSION}" \
        "${DST_LIB_PATH}/${FTDI_STATIC_LIB}"

    # Move header files as well
    sudo mkdir -p "/usr/local/include"
    sudo cp "${FTDI_SRC_PATH}/ftd2xx.h" "${DST_HEADER_PATH}/"
    sudo cp "${FTDI_SRC_PATH}/WinTypes.h" "${DST_HEADER_PATH}/"
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
