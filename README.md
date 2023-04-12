# pl-abs

[![Version](https://img.shields.io/docker/v/fnndsc/pl-abs?sort=semver)](https://hub.docker.com/r/fnndsc/pl-abs)
[![MIT License](https://img.shields.io/github/license/fnndsc/pl-abs)](https://github.com/FNNDSC/pl-abs/blob/main/LICENSE)
[![ci](https://github.com/FNNDSC/pl-abs/actions/workflows/ci.yml/badge.svg)](https://github.com/FNNDSC/pl-abs/actions/workflows/ci.yml)

`pl-abs` is a [blazingly-fast](#Benchmarks) and [correct](#Correctness) _ChRIS_ _ds_-type plugin which
calculates the **absolute value** of each number in each data file of an input directory,
writing outputs to an output directory.

## Usage

Run `pl-abs` with a directory containing containing .txt and .csv file inputs, and a separate directory for outputs:

```shell
apptainer exec docker://fnndsc/pl-abs:latest abs --input-file .txt,.csv incoming/ outgoing/
```

To write outputs in-place to the same directory, use `--output-suffix` to avoid clobbering files.

```shell
apptainer exec docker://fnndsc/pl-abs:latest abs --input-file .txt --output-suffix abs.txt data/ data/
```

On _ChRIS_, it cn be useful to copy unmodified files to the output directory as well:

```shell
apptainer exec docker://fnndsc/pl-abs:latest abs --copy --output-suffix abs.txt data/ data/
```

## Input Examples

Let `incoming/` be a directory containing input files containing numerical data, e.g.

```
-3
-4
-5.6
7.896
2.3E-4
-5.5E6
```

The values separator can be anything, and non-numerical data is ignored. e.g. a CSV:

```csv
food,price_2019,price_2020,change,tasty
apple,1.1,1.2,0.1,false
cereal,2.0,1.5,-.5,false
peanut butter,3.0,5.0,2.0,true
```

See [`examples/incoming`](./examples/incoming) for examples.

## Correctness?

`pl-abs` does not deserialize numbers. To be more true about its functionality, `pl-abs` removes
any negative sign found in front of anything it thinks is the start of a number, specifically any
character from the set `1234567890.`.

This implementation means `pl-abs` guarantees numerical stability, whereas typical programmatic
implementations of the "absolute value" function can cause a loss of floating point precision.

Some readings on floating point math and numerical stability:

- https://0.30000000000000004.com/
- https://arxiv.org/abs/2112.11508

## Benchmarks

I know, Rust developers are annoying.
`pl-abs` is a very simple program, I wrote it in Rust so that I can personally explore the
inefficiencies of Python for data processing.

`pl-abs` written in Rust is ~5 times faster on a single thread for realistic workloads\*
than other programs with or without multiprocessing, including an equivalent
[Python implementation](./abs.py) and
[`vertstats_math`](https://github.com/BIC-MNI/oobicpl/blob/fc33789c314098607ad81c8e8ea6d1723471da77/src/vertstats_math.cc)
from [CIVET](https://mcin.ca/technology/civet/).

\*Performance, of course, is going to vary with how large your data files are and how many there are.
If your input files are 5 lines long, the startup cost of Python can make it 20 times slower.
On the other hand, for a single input file 10,000,000 lines long:

- `pl-abs` in Rust takes 0.7 seconds (max RSS=2456KB)
- `abs.py` in Python takes 9.8 seconds (max RSS=15680KB)
- `vertstats_math` from CIVET takes 2.2 seconds (max RSS=199936KB)

### Setup Benchmarks

```shell
cargo build --release

pip install chris-plugin==0.2.0a1 numpy

python stress_test/create_data.py
```

### Run Benchmarks

```shell
hyperfine -c 'rm -rf /tmp/outgoing' \
    'target/release/abs stress_test/incoming /tmp/outgoing' \
    'env NUM_THREADS=1 python abs.py stress_test/incoming /tmp/outgoing' \
    'env NUM_THREADS=4 python abs.py stress_test/incoming /tmp/outgoing' \
    'find stress_test/incoming -type f -name "*.txt"  | parallel -j1 "mkdir -p /tmp/outgoing/{}; vertstats_math -old_style_file -abs {} /tmp/outgoing/{}"' \
    'find stress_test/incoming -type f -name "*.txt"  | parallel -j4 "mkdir -p /tmp/outgoing/{}; vertstats_math -old_style_file -abs {} /tmp/outgoing/{}"'
```

TODO: benchmark v.s. Numpy, Codon, PyPy, Julia, ...
