# AGI Quest Decoder

This decodes/reads the graphics and levels for Sierra AGI games such as Space Quest 1+2.

To run, clone this repo, install Rust, and run: `make run`. It should generate a bunch of png files in the current folder.

This generates big uncompressed PNGs. To compress them, install `pngquant` then run `make shrink` to make them all a reasonable size.

## See also

![Digging](https://github.com/chrishulbert/digger-decoder/raw/main/readme/digging.png?raw=true)

If you love Lemmings, please check out my [Digger decoder](https://github.com/chrishulbert/digger-decoder) too :)

Or, say, you love Commander Keen, please check out my [Dopefish decoder](https://github.com/chrishulbert/dopefish-decoder).

## References

![Council Member](https://github.com/chrishulbert/dopefish-decoder/blob/main/Council.png?raw=true)

* https://www.agidev.com/articles/agispec/agispecs-5.html
* https://www.agidev.com/articles/agispec/examples/files/agifiles.c
