<!--
  FetchX — Fast Neofetch Alternative Written in Rust
  Keywords: neofetch, neofetch alternative, system info, fetch, rust, terminal,
            linux, ascii art, kitty, ricing, unixporn, dotfiles, system fetch,
            fastfetch alternative, screenfetch, pfetch, macchina
-->

<h1 align="center">
  <br>
  ⚡ FetchX
  <br>
</h1>

<h4 align="center">A blazing-fast, feature-rich system information tool written in Rust.<br>The modern neofetch replacement you've been waiting for.</h4>

<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
  <a href="https://github.com/swadhinbiswas/fetchx/releases"><img src="https://img.shields.io/badge/version-0.2.0-green.svg" alt="Version 0.2.0"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust 2021"></a>
  <a href="https://github.com/swadhinbiswas/fetchx/actions"><img src="https://img.shields.io/badge/tests-45%20passing-brightgreen.svg" alt="45 Tests Passing"></a>
  <a href="https://github.com/swadhinbiswas/fetchx/stargazers"><img src="https://img.shields.io/github/stars/swadhinbiswas/fetchx?style=social" alt="GitHub Stars"></a>
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#features">Features</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#gallery">Gallery</a> •
  <a href="#shell-aliases-replace-neofetch">Replace Neofetch</a>
</p>

<!-- Hero Screenshot -->
<p align="center">
  <img src="assets/image.png" alt="FetchX — neofetch alternative in Rust showing system info with ASCII art" width="700">
</p>

---

## Gallery

<table>
  <tr>
    <td align="center" width="50%">
      <img src="assets/image.png" alt="FetchX ASCII art mode with CachyOS" width="100%">
      <br><b>ASCII Art Mode</b>
      <br><sub>Classic distro logo with system info</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/image2.png" alt="FetchX with kitty image backend showing anime image" width="100%">
      <br><b>Kitty Image Backend</b>
      <br><sub>Auto-fetched image via smart caching</sub>
    </td>
  </tr>
  <tr>
    <td align="center" width="50%">
      <img src="assets/randomimage.png" alt="FetchX random image fetching — new image every run" width="100%">
      <br><b>Random Image Every Run</b>
      <br><sub>Background download, zero delay</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/random2.png" alt="FetchX with different random image from API" width="100%">
      <br><b>Smart Image Caching</b>
      <br><sub>Previous image shown instantly</sub>
    </td>
  </tr>
  <tr>
    <td align="center" width="50%">
      <img src="assets/setimage.png" alt="FetchX with custom image set by user" width="100%">
      <br><b>Custom Image</b>
      <br><sub>Use your own wallpaper or photo</sub>
    </td>
    <td align="center" width="50%">
      <img src="assets/image-copy.png" alt="FetchX Nerd Font icons and progress bars" width="100%">
      <br><b>Nerd Fonts + Progress Bars</b>
      <br><sub>Memory & disk usage bars with icons</sub>
    </td>
  </tr>
</table>

---

## Table of Contents

- [Gallery](#gallery)
- [Features](#features)
- [Installation](#installation)
  - [Quick Install (Recommended)](#quick-install-recommended)
  - [From Source (Manual)](#from-source-manual)
  - [With Make](#with-make)
  - [From Cargo](#from-cargo)
  - [Arch Linux / CachyOS / Manjaro (AUR)](#arch-linux--cachyos--manjaro-aur)
- [Shell Aliases (Replace neofetch)](#shell-aliases-replace-neofetch)
- [Configuration](#configuration)
  - [Getting Started](#getting-started)
  - [Config File Location](#config-file-location)
  - [Full Configuration Reference](#full-configuration-reference)
  - [Example Configs](#example-configs)
- [Usage](#usage)
- [Image Backends](#image-backends)
  - [How Smart Image Fetching Works](#how-smart-image-fetching-works)
