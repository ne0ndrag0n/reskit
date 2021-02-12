reskit
======

***INCOMPLETE***

Resource kit for the Sega Genesis/Megadrive.

# Convert PNG to tile + palette set
```
reskit tileset --input file.png --output file.bin --format bin
```
The file format will contain the palette required by the image, followed by the image.

## Caveats
* Image dimensions that are not a multiple of 8 will be rounded up to the nearest multiple of 8.
* Input images containing more than 15 colours will not be converted and will throw an error.