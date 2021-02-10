#include <iostream>
#include "CImg.cpp"

using namespace cimg_library;

/*
  TODO:
    add support for palettes from pngs and from txt files
    add support for dithering ( https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering )
*/

float dist(uint8_t red1, uint8_t grn1, uint8_t blu1, uint8_t red2, uint8_t grn2, uint8_t blu2)
{
  float r1 = float(red1);
  float g1 = float(grn1);
  float b1 = float(blu1);
  float r2 = float(red2);
  float g2 = float(grn2);
  float b2 = float(blu2);

  return sqrt((r2 - r1) * (r2 - r1) + (g2 - g1) * (g2 - g1) + (b2 - b1) * (b2 - b1));
}

void nibbletrgb(uint8_t (&color)[3], u_int8_t nibble)
{
  /*
      0  black   #00 00 00
      1  drkgry  #44 44 44
      2  grey    #88 88 88
      3  ltgrey  #bb bb bb
      4  ltbrwn  #99 66 33
      5  brown   #66 33 00
      6  green   #00 66 00
      7  lime    #00 aa 00
      8  cyan    #00 99 ff
      9  blue    #00 00 cc
      a  violet  #33 00 99
      b  magenta #ff 00 99
      c  red     #dd 00 00
      d  orange  #ff 66 00
      e  yellow  #ff ff 00
      f  white   #ff ff ff
  */
  nibble = nibble & 0xF;
  uint8_t red[16] = {0x00, 0x44, 0x88, 0xbb, 0x99, 0x66, 0x00, 0x00, 0x00, 0x00, 0x33, 0xff, 0xdd, 0xff, 0xff, 0xff};
  uint8_t grn[16] = {0x00, 0x44, 0x88, 0xbb, 0x66, 0x33, 0x66, 0xaa, 0x99, 0x00, 0x00, 0x00, 0x00, 0x66, 0xff, 0xff};
  uint8_t blu[16] = {0x00, 0x44, 0x88, 0xbb, 0x33, 0x00, 0x00, 0x00, 0xff, 0xcc, 0x99, 0x99, 0x00, 0x00, 0x00, 0xff};
  color[0] = red[nibble];
  color[1] = grn[nibble];
  color[2] = blu[nibble];
}

int main(int argc, char *argv[])
{
  if (argc != 3)
  {
    printf("Usage ./main [PICTURE FILE] [OUTPUT FILE]\n");
    return 1;
  }

  CImg<int> image(argv[1]);
  int width = image.width();
  int height = image.height();
  CImg<int> result(width, height, 1, 3, 0);

  for (int j = 0; j < image.height(); ++j)
  {
    for (int i = 0; i < image.width(); ++i)
    {
      uint8_t red = image(i, j, 0, 0);
      uint8_t grn = image(i, j, 0, 1);
      uint8_t blu = image(i, j, 0, 2);

      // find closest color in the palette
      float minima = 1e16;
      float minima_color = 0;
      uint8_t color[3];
      for (int col = 0; col < 0xF; col++)
      {
        nibbletrgb(color, col);
        float d = (dist(red, grn, blu, color[0], color[1], color[2]));
        if (d < minima)
        {
          minima = d;
          minima_color = col;
        }
      }

      // set color
      nibbletrgb(color, minima_color);
      result(i, j, 0, 0) = color[0];
      result(i, j, 0, 1) = color[1];
      result(i, j, 0, 2) = color[2];
    }
  }
  result.save(argv[2]);
  return 0;
}
