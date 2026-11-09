# Arachnida - PISCINE CYBERSECURITY DAY 1

Process data from the web in two steps:
1. automatic download
2. analyse and manipulation of metadata

## 1. Spider

Program extracting all the images (.jpg/jpeg, .png, .gif, .bmp) from a website recursively, by provinding a url as a parameter.
Usage :
  ./spider [-rlp] URL

with options:
- -r . recursive download
- -r -l [N] . maximum depth level (N) of the recursive download. By default : 5
- -p [PATH] . where to save the downloaded images. By default : ./data/

## 2. Scorpion

Program displaying EXIF and other metadata over an image file (same types as in spider). Includes basic attributes.

Usage:
  ./scorpion FILE1 [FILE2 ...]


For each,
- --version, display the program's version
- --help, display help about usage of the program

