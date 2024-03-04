# Image Compressor

## About

This project is an image compression tool implemented in Rust, leveraging the K-Means clustering algorithm to reduce the color palette of an image. This method can significantly decrease the size of the image file while maintaining visual fidelity to a degree determined by the number of clusters (K) used in the compression process.

## Features

- CLI for easy use and integration into workflows.
- Customizable number of clusters for flexible compression options.
- Utilizes `rayon` for parallel processing, improving performance on multi-core systems.
- Input and output path flexibility.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust and Cargo (Rust's package manager) installed on your machine.
- Basic familiarity with Rust and command-line operations.

## Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/nethriis/image-compressor.git
cd image-compressor
```

Build the project using Cargo:

```bash
cargo build --release
```

The executable will be located in `./target/release/`.

## Usage

To compress an image, run the tool from the command line, specifying the input image path, the output image path, and optionally, the number of clusters (K). The default value for K is 4 if not specified.

```bash
./target/release/image_compressor -i path/to/input.png -o path/to/output.png -k 16
```

### Arguments

- `-i`, `--input`: The path to the input image file.
- `-o`, `--output`: The path where the compressed image will be saved.
- `-k`, `--k`: (Optional) The number of color clusters to use for compression. A higher number retains more detail but reduces compression.

## Contributing

Contributions to this project are welcome. Please adhere to the following guidelines:

- Fork the repository and create your branch from `main`.
- Write clear and concise commit messages.
- Ensure any install or build dependencies are removed before the end of the layer when doing a build.
- Update the README.md with details of changes to the interface, this includes new environment variables, exposed ports, useful file locations, and container parameters.

## License

Distributed under the [GPL-3.0 license](/LICENSE).
