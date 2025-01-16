## About
 MolViz v0.1.0 by Adrien Pelfresne <adrien.pelfresne@gmail.com>
    A simple OpenGL molecular visualization, capable of reading protein data bank files using imposter based rendering.
This application has been written for the Scientific visualization course of the MIRI master in the Facultat d'Informàtica de Barcelona.

You must have rust installed on your computer to start this program, see [rust website](https://www.rust-lang.org/).
A version of OpenGL 4.1 + must also be available, this application has only been tested on an apple silicon machine with OpenGL 4.1 Metal and GLSL 410 core.
    
## Usage
You can either test the program with the provided pdb files (see [pdb folder](./resources/pdb/), or provide your own file.
The files need to be placed inside the pdb folder, see above.

The program is not capable of deducing the bonds of pdb files without `CONECT` records.

```sh
    Usage: cargo run --release -- --file <FILE>
    Options:
  -f, --file <FILE>
  -h, --help         Print help
  -V, --version      Print version
```

## Showcase
![Methane Molecule](./methane.png)

![Large Molecule with silhouete](./zoom_capability.png)
