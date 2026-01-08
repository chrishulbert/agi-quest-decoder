# AGI Quest Decoder

![Root Monster](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/RootMonster.png)

This decodes/reads the graphics and levels for Sierra AGI games such as Space Quest 1+2.

![Root Trapped](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/RootTrapped.png)

To run, clone this repo, install Rust, and run: `make run`. It should generate a bunch of png files in the current folder.

![Crawl](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/Crawl.png)

This generates big uncompressed PNGs. To compress them, install `pngquant` and `apngasm` then run `make compress` to make them all a reasonable size.

## See also

![Digging](https://github.com/chrishulbert/digger-decoder/raw/main/readme/digging.png)

If you love Lemmings, please check out my [Digger decoder](https://github.com/chrishulbert/digger-decoder) too :)

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/raw/main/Dopefish.png)

Or, say, you love Commander Keen, please check out my [Dopefish decoder](https://github.com/chrishulbert/dopefish-decoder).

## References

![Labion Terror Beast](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/LabionTerrorBeast.png)

* https://www.agidev.com/articles/agispec/agispecs-5.html
* https://www.agidev.com/articles/agispec/examples/files/agifiles.c
* https://www.agidev.com/articles/agispec/examples/view/viewview.pas
* https://github.com/lanceewing/agile-gdx/blob/main/core/src/main/java/com/agifans/agile/agilib/View.java
* https://www.agidev.com/games/?page=3

![Cleaner](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/Cleaner.png)
