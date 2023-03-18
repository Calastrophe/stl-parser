# stl-parser

stl-parser is aiming to be one of the fastest STL parsers available in crates.io while still being understandable and extendable by the average user.

There is support currently only for the binary format, but ASCII is on the roadmap.

Additionally, multi-threading is seen as another goal to allow for quicker read times of STL files, but will be added after ASCII support.

Even currently, the read time for even sizeable files is extremely low in release mode.

It is recommended to convert all STL files to binary format to save space and allow for quicker read times. This option will be provided in later releases.
