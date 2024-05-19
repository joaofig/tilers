# tilers
A Rust-based map tile generator

## Description

This repository contains the Rust project that implements a map tile generator served through a web application.
The web application uses a single entry point to serve the tiles which contain traffic density data for the 
[Vehicle Energy Dataset](https://github.com/gsoh/VED).

## Usage

To launch the web application use the following command from the project's root:
```shell
cargo run --release
```

From a Jupyter Notebook, use the following code to show the map with overlayed tiles:
```Python
import folium
from folium.raster_layers import TileLayer

html_map = folium.Map(prefer_canvas=True, 
                      tiles="cartodbpositron", 
                      location=(42.274569, -83.733228), 
                      zoom_start=13)
tile_layer = TileLayer(tiles="http://127.0.0.1:8000/density/{x}/{y}/{z}", 
                       overlay=True,
                       attr="(C) JPF")
tile_layer.add_to(html_map)
html_map
```

## Medium Article

[Generating Map Tiles with Rust](https://towardsdatascience.com/generating-map-tiles-with-rust-dbdb0eb09b6b)

## References

[Vehicle Energy Dataset](https://github.com/gsoh/VED)

[Displaying Geographic Information Using Custom Map Tiles](https://towardsdatascience.com/displaying-geographic-information-using-custom-map-tiles-c0e3344909a4)

[Bing Maps Tile System](https://learn.microsoft.com/en-us/bingmaps/articles/bing-maps-tile-system)


## License

MIT License

Copyright (c) 2024 João Paulo Figueira

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
