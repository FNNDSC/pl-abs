# pl-abs

[![Version](https://img.shields.io/docker/v/fnndsc/pl-abs?sort=semver)](https://hub.docker.com/r/fnndsc/pl-abs)
[![MIT License](https://img.shields.io/github/license/fnndsc/pl-abs)](https://github.com/FNNDSC/pl-abs/blob/main/LICENSE)
[![ci](https://github.com/FNNDSC/pl-abs/actions/workflows/ci.yml/badge.svg)](https://github.com/FNNDSC/pl-abs/actions/workflows/ci.yml)

`pl-abs` is a [blazingly-fast](#Benchmarks) and correct _ChRIS_ _ds_-type plugin which
calculates the **absolute value** of each number in each data file of an input directory,
writing outputs to an output directory.

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

TODO: benchmark v.s. Numpy, Julia?
