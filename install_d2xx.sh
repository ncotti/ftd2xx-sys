#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Nicolas Gabriel Cotti

# FTDI library version
FTDI_VERSION="1.4.35"

# PATH where the library files are
FTDI_SRC_PATH="tmp"

# Name of the dynamic and static libraries
FTDI_DYN_LIB="libftd2xx.so"
FTDI_STATIC_LIB="libftd2xx.a"

# Paths where the library will be installed to
DST_LIB_PATH="/usr/local/lib"
DST_HEADER_PATH="/usr/local/include"

if [ ! -f "${FTDI_SRC_PATH}/${FTDI_DYN_LIB}" ]; then
    printf "[Error] Could locate ftd2xx lib files at %s. Please set FTDI_SRC_PATH accordingly.\n" "${FTDI_SRC_PATH}"
    exit 1
fi

## Move library to /usr/local/lib
if ! find "/usr/lib" "/usr/local/lib" -name "${FTDI_DYN_LIB}" | grep -q .; then
    printf "Installing FTDI library %s\n" "${FTDI_DYN_LIB}.${FTDI_VERSION}"
    sudo cp "${FTDI_SRC_PATH}/${FTDI_DYN_LIB}.${FTDI_VERSION}" "${DST_LIB_PATH}/"
    sudo cp "${FTDI_SRC_PATH}/${FTDI_STATIC_LIB}" "${DST_LIB_PATH}/"
    sudo ln -s "/usr/local/lib/${FTDI_DYN_LIB}.${FTDI_VERSION}" "/usr/local/lib/${FTDI_DYN_LIB}"
    sudo chmod 0755 \
        "${DST_LIB_PATH}/${FTDI_DYN_LIB}" \
        "${DST_LIB_PATH}/${FTDI_DYN_LIB}.${FTDI_VERSION}" \
        "${DST_LIB_PATH}/${FTDI_STATIC_LIB}"

    # Move header files as well
    sudo mkdir -p "/usr/local/include"
    sudo cp "${FTDI_SRC_PATH}/ftd2xx.h" "${DST_HEADER_PATH}/"
    sudo cp "${FTDI_SRC_PATH}/WinTypes.h" "${DST_HEADER_PATH}/"
else
    printf "Library already installed.\n"
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
