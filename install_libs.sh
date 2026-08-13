#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Nicolas Gabriel Cotti

###############################################################################
# User-modifiable variables
###############################################################################

# Path where the FTD2XX library files is.
D2XX_SRC_PATH="${1}"
D2XX_INC_PATH="${D2XX_SRC_PATH}"

# Path where the MPSSE library is. Can be empty
MPSSE_SRC_PATH="${2}"
MPSSE_INCLUDE_PATH="${MPSSE_SRC_PATH}/../include"

# Paths to where the libraries will be installed.
DST_LIB_PATH="/usr/local/lib"
DST_HEADER_PATH="/usr/local/include"

###############################################################################
# D2XX installation
###############################################################################
# Name of the dynamic and static libraries.
D2XX_DYN_LIB="libftd2xx.so"
D2XX_STATIC_LIB="libftd2xx.a"

if [ ! -f "${D2XX_SRC_PATH}/${D2XX_DYN_LIB}" ]; then
    printf "[Error] Could locate ftd2xx lib files at %s. Please set D2XX_SRC_PATH accordingly.\n" "${D2XX_SRC_PATH}"
    exit 1
fi

# FTD2XX library version.
D2XX_VERSION=$(find "${D2XX_SRC_PATH}/${D2XX_DYN_LIB}".* | sed 's/.*\.so\.//')

## Copy FTD2XX library
if ! find "${DST_LIB_PATH}" -name "${D2XX_DYN_LIB}" | grep -q .; then
    printf "Installing FTD2XX library %s\n" "${D2XX_DYN_LIB}.${D2XX_VERSION}"
    sudo cp "${D2XX_SRC_PATH}/${D2XX_DYN_LIB}.${D2XX_VERSION}" "${DST_LIB_PATH}/"
    sudo cp "${D2XX_SRC_PATH}/${D2XX_STATIC_LIB}" "${DST_LIB_PATH}/"
    sudo ln -s "${DST_LIB_PATH}/${D2XX_DYN_LIB}.${D2XX_VERSION}" "${DST_LIB_PATH}/${D2XX_DYN_LIB}"
    sudo chmod 0755 \
        "${DST_LIB_PATH}/${D2XX_DYN_LIB}" \
        "${DST_LIB_PATH}/${D2XX_DYN_LIB}.${D2XX_VERSION}" \
        "${DST_LIB_PATH}/${D2XX_STATIC_LIB}"

    # Move header files as well
    sudo mkdir -p "/usr/local/include"
    sudo cp "${D2XX_INC_PATH}/ftd2xx.h" "${DST_HEADER_PATH}/"
    sudo cp "${D2XX_INC_PATH}/WinTypes.h" "${DST_HEADER_PATH}/"
else
    printf "Library FTD2XX already installed.\n"
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

###############################################################################
# MPSSE installation
###############################################################################
if [ -n "${MPSSE_SRC_PATH}" ]; then
    MPSSE_DYN_LIB="libmpsse.so"
    MPSSE_STATIC_LIB="libmpsse.a"

    if [ ! -f "${MPSSE_SRC_PATH}/${MPSSE_DYN_LIB}" ]; then
        printf "[Error] Could locate MPSSE lib files at %s. Please set MPSSE_SRC_PATH accordingly.\n" "${MPSSE_SRC_PATH}"
        exit 1
    fi

    # MPSSE library version.
    MPSSE_VERSION=$(find "${MPSSE_SRC_PATH}/${MPSSE_DYN_LIB}".* | sed 's/.*\.so\.//')

    ## Copy MPSSE library
    if ! find "${DST_LIB_PATH}" -name "${MPSSE_DYN_LIB}" | grep -q .; then
        printf "Installing MPSSE library %s\n" "${MPSSE_DYN_LIB}.${MPSSE_VERSION}"
        sudo cp "${MPSSE_SRC_PATH}/${MPSSE_DYN_LIB}.${MPSSE_VERSION}" "${DST_LIB_PATH}/"
        sudo cp "${MPSSE_SRC_PATH}/${MPSSE_STATIC_LIB}" "${DST_LIB_PATH}/"
        sudo ln -s "${DST_LIB_PATH}/${MPSSE_DYN_LIB}.${MPSSE_VERSION}" "${DST_LIB_PATH}/${MPSSE_DYN_LIB}"
        sudo chmod 0755 \
            "${DST_LIB_PATH}/${MPSSE_DYN_LIB}" \
            "${DST_LIB_PATH}/${MPSSE_DYN_LIB}.${MPSSE_VERSION}" \
            "${DST_LIB_PATH}/${MPSSE_STATIC_LIB}"

        # Move header files as well
        sudo mkdir -p "/usr/local/include"
        sudo cp "${MPSSE_INCLUDE_PATH}/libmpsse_i2c.h" "${DST_HEADER_PATH}/"
        sudo cp "${MPSSE_INCLUDE_PATH}/libmpsse_spi.h" "${DST_HEADER_PATH}/"
    else
        printf "Library FTMPSSE already installed.\n"
    fi
fi