
# Introduction

gchemol-view is a simple molecule viewer based on [gchemol](https://github.com/gchemol/gchemol) and [bevy](https://bevyengine.org/).

![img](data/72/9b0609-04b6-40c0-93db-5674f85b0738/2023-04-09_09-52-36_screenshot.png)


# Features

-   [X] visualization of atom, bond and lattice
-   [X] mouse control for view like in GaussView
-   [X] view molecules sent from remote client side in terminal
-   [X] visualization of molecule trajectory (Left/Right key for next/previous frame, currently broken in v0.2.0)


# Install


## Binary install

Download prebuilt binary for Linux on the [release](https://github.com/ybyygu/gchemol-view/releases) page.


## Compile from sources

-   follow [bevy setup guide](https://bevyengine.org/learn/book/getting-started/setup/)


# How to use

Visualization of molecules from common file formats supported by [gchemol](https://github.com/gchemol/gchemol-readwrite/tree/master/src/formats)

    gchemol-view foo.xyz
    gchemol-view POSCAR
    gchemol-view foo.cif


# Todo List

-   [ ] fix visualiation of lattice in trajectory animation
-   [ ] set atom freezing codes for optimization
-   [ ] select atoms by atom serial numbers


# Credits

Codes that inspired me:

-   [Plonq/bevy\_panorbit\_camera: A simple pan and orbit camera for the Bevy game engine](https://github.com/Plonq/bevy_panorbit_camera)
-   [NightsWatchGames/rubiks-cube: Rubik's cube made with bevy engine.](https://github.com/NightsWatchGames/rubiks-cube)

