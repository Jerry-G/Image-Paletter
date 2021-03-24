# Image Paletter

Image Paletter is a tool to recreate an image using only colors in a defined color palette.

## Usage

```sh
./paletter colors.json in.png out.png
```

## DEMO

Here is an example palette json file and a visual.

```json
{
  "colors": ["e63946", "f1faee", "a8dadc", "457b9d", "1d3557"]
}
```

![color palette image](https://raw.githubusercontent.com/Jerry-G/Image-Paletter/master/demo/palette-00.png)

Using this color palette these are the results.

![image of cows](https://raw.githubusercontent.com/Jerry-G/Image-Paletter/master/demo/cow.png)

Here is another demo.

```json
{
  "colors": ["2b2d42", "8d99ae", "edf2f4", "ef233c", "d90429"]
}
```

![color palette image](https://raw.githubusercontent.com/Jerry-G/Image-Paletter/master/demo/palette-01.png)

![image of dog](https://raw.githubusercontent.com/Jerry-G/Image-Paletter/master/demo/dog.png)

## TODO

- add support for [dithering](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering)
