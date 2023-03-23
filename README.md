# petra_grid

**`petra_grid` is a Python library for reading grid data in the `.GRD` file
format produced by the Petra™ geological interpretation application**

`petra_grid` is implemented as a set of bindings to the [corresponding Rust
crate](https://crates.io/crates/petra_grid)

---

This library is based on a lot of time spent in a hex editor looking at
grid files and poring over publicly-available documentation. It is by necessity
incomplete and incorrect, and will remain so until the file format is properly
and publicly specified.

However, the library is able to successfully read rectangular and triangulated
grids and a good portion of their metadata. Please open an issue if you have
trouble reading a grid and are able to share the grid (in GRD and exported form)
with the developer.

---

### Example usage

This library can be used to read `.GRD` grid data from a file or file-like
object.  Here's a short program for dumping "debug" representations of grid
files provided on the command line:
```python
import sys

import petra_grid


def main(argv: list[str]) -> int:
    if len(argv) <= 1:
        print(f'Usage: {argv[0] if argv else "read_grid"} <grd-files>',
          file=sys.stderr)
        return 2

    for path in argv[1:]:
        with open(path, 'rb') as f:
            grid = petra_grid.read_grid(f)
            if grid.is_rectangular:
                print(f'{grid.name}: {grid.rows} x {grid.columns} rectangular grid')
            elif grid.is_triangular:
                print(f'{grid.name}: {grid.n_triangles}-triangle triangular grid')
            print(f'Details: {grid}')

    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
```

As another example, we can use `matplotlib` to render images of rectangular or
triangular grids.
```python
import sys

import petra_grid

import numpy as np

from matplotlib.tri import Triangulation # type: ignore
import matplotlib.pyplot as plt # type: ignore


def plot_rectangular(grid: petra_grid.Grid) -> None:
    plt.imshow(grid.data, extent=(grid.xmin, grid.xmax, grid.ymin, grid.ymax),
      origin='lower')
    plt.title(f'{grid.name}')
    plt.colorbar()
    plt.show()


def plot_triangular(grid: petra_grid.Grid) -> None:
    triangles = grid.data
    xs = np.ravel(triangles[:, :, 0])
    ys = np.ravel(triangles[:, :, 1])
    ixs = np.reshape(np.indices(xs.shape)[0], (triangles.shape[0], 3))
    triangulation = Triangulation(xs, ys, ixs)
    zs = np.ravel(triangles[:, :, 2])
    fig, ax = plt.subplots()
    ax.set_aspect('equal')
    tc = ax.tripcolor(triangulation, zs)
    ax.set_title(f'{grid.name}')
    fig.colorbar(tc)
    plt.show()


def main(argv: list[str]) -> int:
    if len(argv) <= 1:
        print(f'Usage: {argv[0] if argv else "render_grid"} <grd-files>',
          file=sys.stderr)
        return 2

    for path in argv[1:]:
        with open(path, 'rb') as f:
            grid = petra_grid.read_grid(f)
            if grid.is_rectangular:
                plot_rectangular(grid)
            elif grid.is_triangular:
                plot_triangular(grid)
            print(f'Details: {grid}')

    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
```

---

### Documentation

Documentation strings and type annotations are provided for all public types,
functions, and methods. We recommend viewing "nice" documentation pages using
[`pdoc`](https://pdoc.dev/docs/pdoc.html); e.g. in the same environment as the
`petra_grid` package is installed, install `pdoc` with `pip install pdoc`, then
run `pdoc petra_grid`.

#### Available under the [MIT license](LICENSE)

#### (c) 2023 [dwt](https://www.github.com/derrickturk) | [terminus, LLC](https://terminusdatascience.com)
