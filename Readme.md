# AGI Quest Decoder

This decodes/reads the graphics and levels for Sierra AGI games such as Space Quest 1+2.

To run, clone this repo, install Rust, and run: `make run`. It should generate a bunch of png files in the current folder.

This generates big uncompressed PNGs. To compress them, install `pngquant` and `apngasm` then run `make compress` to make them all a reasonable size.

## Size

The images are quite small, and at half the width the game usually displays them in. To expand:

ffmpeg -i walking.png -vf "scale=iw*8:ih*4:flags=neighbor" -plays 0 walking.apng

## See also

![Digging](https://github.com/chrishulbert/digger-decoder/raw/main/readme/digging.png?raw=true)

If you love Lemmings, please check out my [Digger decoder](https://github.com/chrishulbert/digger-decoder) too :)

Or, say, you love Commander Keen, please check out my [Dopefish decoder](https://github.com/chrishulbert/dopefish-decoder).

## References

![Council Member](https://github.com/chrishulbert/dopefish-decoder/blob/main/Council.png?raw=true)

* https://www.agidev.com/articles/agispec/agispecs-5.html
* https://www.agidev.com/articles/agispec/examples/files/agifiles.c
* https://www.agidev.com/articles/agispec/examples/view/viewview.pas
* https://github.com/lanceewing/agile-gdx/blob/main/core/src/main/java/com/agifans/agile/agilib/View.java
* https://www.agidev.com/games/?page=3
