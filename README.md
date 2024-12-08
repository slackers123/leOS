# Leo(s) - a small os/system-dev experimentation project
The leo project is a combination of the LeOS operating system and
programs/libraries written for said operating system.

## Why?
For fun.

## What?
Anything that interest me.

## What exactly?
Currently the project is still extremely immature. Right now the
only things that exist are:
- leos-kernel: a small kernel written in rust based on the bootboot bootloader (should be deprecated)
- leos-libs: some small libraries mainly focused on 2d rendering for the os
  - mathlib: a collection of mathematical primitives and basic functions replacing the ones missing from not using std
  - ttflib: loading glyphs from a TrueType font file
  - drawlib: drawing of shapes onto a generic target (currently only used in gui_lib)
- gui_experiments: native bootstrapping for testing drawlib etc.

[![brainmade.org](https://raw.githubusercontent.com/0atman/BrainMade-org/refs/heads/main/docs/white-logo.svg)](https://brainmade.org)
