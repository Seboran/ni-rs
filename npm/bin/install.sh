#!/bin/bash

# Détecter l'architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64) ARCH="x86";;
    arm64) ARCH="arm64";;
    *) echo "Architecture inconnue"; exit 1 ;;
esac

# Détecter le système d'exploitation
OS=$(uname -s)
case "$OS" in
    Linux) OS="linux";;
    Darwin) OS="macos";;
    MINGW*|MSYS*|CYGWIN*) OS="windows";;
    *) echo "Système d'exploitation inconnu"; exit 1 ;;
esac

# Installer les dépendances
npm install @seboran/ni-$ARCH-$OS

