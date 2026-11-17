This project is part of the cybersecurity piscine of 42 Paris.

# FT_OTP (OTP)

## Aim

This project purpose is to learn cryptography. It must create a program (binary or script) that will take a 64 hex-digit key (or more) as a parameter, encrypt it with an input password and will be able from this encrypted key to give a one time password (OTP) base on the HOTP algorithm

## Technical requirements

I have chosen to do this project in Rust. I have used Cargo to manage the dependencies (and the compilation). However, no need to install it, the Makefile includes an init section that install cargo inside a folder in /tmp, builds the project (with cargo build) inside a subfolder of the latter, and copies the binary into this folder.
