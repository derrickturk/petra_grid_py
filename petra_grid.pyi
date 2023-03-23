import datetime

import numpy as np
import numpy.typing as npt

from typing import BinaryIO, ClassVar, Union


class UnitOfMeasure:
    Feet: ClassVar['UnitOfMeasure']
    '''feet'''

    Meters: ClassVar['UnitOfMeasure']
    '''meters'''


class Grid:
    @property
    def version(self) -> int: ...

    @property
    def version(self) -> int: ...

    @property
    def name(self) -> str: ...

    @property
    def size(self) -> int: ...

    @property
    def rows(self) -> int: ...

    @property
    def columns(self) -> int: ...

    @property
    def n_triangles(self) -> int: ...

    @property
    def xmin(self) -> float: ...

    @property
    def xmax(self) -> float: ...

    @property
    def ymin(self) -> float: ...

    @property
    def ymax(self) -> float: ...

    @property
    def xstep(self) -> float: ...

    @property
    def ystep(self) -> float: ...

    @property
    def zmin(self) -> float: ...

    @property
    def zmax(self) -> float: ...

    @property
    def xyunits(self) -> UnitOfMeasure: ...

    @property
    def zunits(self) -> UnitOfMeasure: ...

    @property
    def created_date(self) -> datetime.datetime: ...

    @property
    def source_data(self) -> str: ...

    @property
    def unknown_metadata(self) -> str: ...

    @property
    def projection(self) -> str: ...

    @property
    def datum(self) -> str: ...

    @property
    def grid_method(self) -> int: ...

    @property
    def projection_code(self) -> int: ...

    @property
    def cm(self) -> float: ...

    @property
    def rlat(self) -> float: ...

    @property
    def data(self) -> npt.NDArray[np.float64]: ...

    @property
    def is_rectangular(self) -> bool: ...

    @property
    def is_triangular(self) -> bool: ...

    def __repr__(self) -> str: ...


def read_grid(name_or_file_like: Union[str, BinaryIO]) -> Grid: ...
