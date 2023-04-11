import numpy as np
from pathlib import Path

input_dir = fname = Path(__file__).parent / 'incoming'
input_dir.mkdir()

for i, data in enumerate(np.random.rand(50, 50000)):
    fname = input_dir / f'{i}.txt'
    np.savetxt(fname, data, fmt='%f')
    print(fname, flush=True)
