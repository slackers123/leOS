# DPI
## Logical
The logical dpi will be 81.28 or more intuitively 32 dpcm. On my hd mac screen this is actually more acurate
than apples 72 dpi, but that doesn't really matter probably.

## Math
We usually get: the width and height in mm and the actual pixel count
cm = mm / 10

physical dpcm = pixel count / cm

scaling factor = physical dpcm / 32

### logical -> physical

logical size * scaling factor = physical size

### physical -> logical

physical size / scaling factor = logical size
