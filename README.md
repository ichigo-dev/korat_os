# Korat OS

This is a project to implement a simple OS in Rust.

![marko-ganzaro-NLSjrj5S-pg-unsplash](https://user-images.githubusercontent.com/118079373/202865746-19fb98c9-b38c-4492-a7df-6a5fecddecc4.jpg)


## Overview

It is based on a freestanding kernel based on Rust binaries that can run on 
bare metal (no_std). This OS currently has the following features.

- String output to VGA text mode
- A test framework


## Goal

Implement a safer and faster OS in Rust. I'm thinking of installing a GUI 
mechanism including a display manager here.

[Redox](https://www.redox-os.org/)([github](https://github.com/redox-os/redox)) 
is an ideal form close to what I imagine.

It is basically a project for learning purposes and not targeted for full-scale 
use.


## Development memo

- [memo](/note/memo.md)


## References

- ["Write an OS in Rust", Philipp Oppermann (2022/11/19)](https://os.phil-opp.com/)
