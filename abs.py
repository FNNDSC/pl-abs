#!/usr/bin/env python

"""
A multi-processing Python implementation of the ``abs`` program
from commit 4aa9863ab5eadc9dcd6b96f30449ea764ce28979
"""

import os
import sys
from pathlib import Path
from argparse import ArgumentParser, Namespace, ArgumentDefaultsHelpFormatter
from concurrent.futures import ProcessPoolExecutor

from chris_plugin import chris_plugin, PathMapper

__version__ = '1.0.0'

DISPLAY_TITLE = r"""
       _             _         
      | |           | |        
 _ __ | |______ __ _| |__  ___ 
| '_ \| |______/ _` | '_ \/ __|
| |_) | |     | (_| | |_) \__ \
| .__/|_|      \__,_|_.__/|___/
| |                            
|_|                            
"""

parser = ArgumentParser(description='A ChRIS plugin to take the absolute value of data files',
                        formatter_class=ArgumentDefaultsHelpFormatter)
parser.add_argument('-p', '--pattern', default='**/*.txt', type=str,
                    help='input file filter glob')
parser.add_argument('-V', '--version', action='version',
                    version=f'%(prog)s {__version__}')


@chris_plugin(
    parser=parser,
    title='Absolute Value',
    category='',                 # ref. https://chrisstore.co/plugins
    min_memory_limit='100Mi',    # supported units: Mi, Gi
    min_cpu_limit='1000m',       # millicores, e.g. "1000m" = 1 CPU core
    min_gpu_limit=0              # set min_gpu_limit=1 to enable GPU
)
def main(options: Namespace, inputdir: Path, outputdir: Path):
    """
    *ChRIS* plugins usually have two positional arguments: an **input directory** containing
    input files and an **output directory** where to write output files. Command-line arguments
    are passed to this main method implicitly when ``main()`` is called below without parameters.

    :param options: non-positional arguments parsed by the parser given to @chris_plugin
    :param inputdir: directory containing (read-only) input files
    :param outputdir: directory where to write output files
    """

    proc = get_workers()
    print(f'Using {proc} threads', flush=True, file=sys.stderr)

    mapper = PathMapper.file_mapper(inputdir, outputdir, glob=options.pattern.split(','))
    input_files, output_files = zip(*mapper)
    with ProcessPoolExecutor(max_workers=proc) as pool:
        results = pool.map(abs_file, input_files, output_files)

    for _ in results:
        pass


def get_workers() -> int:
    if e := os.getenv('NUM_THREADS', None) is not None:
        try:
            return int(e)
        except ValueError:
            print(
                f'WARNING: Environment variable NUM_THREADS={e} '
                f'cannot be parsed as int, ignoring',
                file=sys.stderr,
                flush=True
            )
    return len(os.sched_getaffinity(0))


def abs_file(input_file: Path, output_file: Path):
    """
    A pure-Python implementation of absolute value which removes negative signs from in front of numbers.

    We avoid deserializing the data as floats to avoid loss of precision, and don't do any other kinds of
    processing to preserve whitespace and whatever else.
    """
    with input_file.open('rb') as i:
        with output_file.open('wb') as o:
            prev = b''
            was_negative = False
            cur = None
            while cur := i.read(1):
                if cur == b'-':
                    was_negative = True
                elif was_negative:
                    if cur in b'1234567890.':
                        prev = b''
                    was_negative = False
                o.write(prev)
                prev = cur
            if cur is not None:
                o.write(cur)
    print(f'{input_file} -> {output_file}')


if __name__ == '__main__':
    main()
