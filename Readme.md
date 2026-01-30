# AGI Quest Decoder

![Root Monster](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/RootMonster.png)

This decodes/reads the graphics and levels for Sierra AGI games such as Space Quest 1+2.

![Root Trapped](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/RootTrapped.png)

To run, clone this repo, install Rust, and run: `make run`. It should generate a bunch of png files in the current folder.

To run against Space Quest 2 or Leisure Suit Larry 1, put them in `data/SQ2` and `data/LSL1`, then run `make run-sq2` or `make run-lsl1`.

![Broom](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/Broom.png)

Images are upscaled to suit the original aspect ratio of the game, using XBRZ to give a smoother-but-still-pixely effect. If you prefer nearest-neighbour, uncomment it at the bottom of `renderer.rs`.

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

![Leisure Suit Larry](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/Larry.png)

## Space Quest 2

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.1.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.10.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.2.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.20.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.21.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.30.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.40.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.50.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.60.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/a.70.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.10.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.100.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.110.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.120.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.130.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.140.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.150.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.160.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.170.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.180.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.190.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.20.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.200.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.21.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.210.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.220.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.230.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.240.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.250.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.260.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.30.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.40.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.50.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.60.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.70.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.80.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.81.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.82.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/b.90.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.10.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.100.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.110.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.120.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.130.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.140.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.150.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.160.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.170.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.180.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.190.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.20.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.200.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.210.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.220.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.230.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.240.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.30.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.40.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.41.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.50.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.60.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.70.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.80.png)

![Picture](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/sq2/c.90.png)

![Cleaner](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/Cleaner.png)
