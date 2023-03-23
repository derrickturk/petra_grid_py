/* Copyright (c) 2023 Derrick W. Turk / terminus, LLC
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::PyDateTime,
};

use numpy::ToPyArray;

mod filey;
use filey::Filey;

/// units of measure for a given dimension
///
/// the only known values are:
///   - `UnitOfMeasure.Feet`
///   - `UnitOfMeasure.Meters`
#[pyclass(module="petra_grid")]
#[derive(Copy, Clone, Debug)]
pub enum UnitOfMeasure {
    /// feet
    Feet,
    /// meters
    Meters,
}

impl From<petra_grid::UnitOfMeasure> for UnitOfMeasure {
    fn from(uom: petra_grid::UnitOfMeasure) -> Self {
        match uom {
            petra_grid::UnitOfMeasure::Feet => Self::Feet,
            petra_grid::UnitOfMeasure::Meters => Self::Meters,
        }
    }
}

/// a Petra grid
#[pyclass(module="petra_grid")]
#[derive(Clone, Debug)]
pub struct Grid(petra_grid::Grid);

#[pymethods]
impl Grid {
    /// we think this is the version number; always 2, as far as we can tell
    #[getter]
    fn get_version(&self) -> u32 {
        self.0.version
    }

    /// the grid name
    #[getter]
    fn get_name(&self) -> &str {
        &self.0.name
    }

    /// the "size" (rows x columns) for a rectangular grid; perhaps it's the
    /// pre-triangulation size for triangular grids?
    #[getter]
    fn get_size(&self) -> u32 {
        self.0.size
    }

    /// the number of rows (in the *y* dimension) for a rectangular grid;
    /// perhaps it's the pre-triangulation row count for triangular grids?
    #[getter]
    fn get_rows(&self) -> u32 {
        self.0.rows
    }

    /// the number of columns (in the *x* dimension) for a rectangular grid;
    /// perhaps it's the pre-triangulation column count for triangular grids?
    #[getter]
    fn get_columns(&self) -> u32 {
        self.0.columns
    }

    /// the number of triangles; zero for rectangular grids
    #[getter]
    fn get_n_triangles(&self) -> u32 {
        self.0.n_triangles
    }

    /// minimum bound in the *x* dimension
    #[getter]
    fn get_xmin(&self) -> f64 {
        self.0.xmin
    }

    /// maximum bound in the *x* dimension
    #[getter]
    fn get_xmax(&self) -> f64 {
        self.0.xmax
    }

    /// minimum bound in the *y* dimension
    #[getter]
    fn get_ymin(&self) -> f64 {
        self.0.ymin
    }

    /// maximum bound in the *y* dimension
    #[getter]
    fn get_ymax(&self) -> f64 {
        self.0.ymax
    }

    /// step in the *x* dimension
    #[getter]
    fn get_xstep(&self) -> f64 {
        self.0.xstep
    }

    /// step in the *y* dimension
    #[getter]
    fn get_ystep(&self) -> f64 {
        self.0.ystep
    }

    /// minimum value in the *z* dimension
    #[getter]
    fn get_zmin(&self) -> f64 {
        self.0.zmin
    }

    /// maximum value in the *z* dimension
    #[getter]
    fn get_zmax(&self) -> f64 {
        self.0.zmax
    }

    /// units of measure in the *x* and *y* dimensions
    #[getter]
    fn get_xyunits(&self) -> UnitOfMeasure {
        self.0.xyunits.into()
    }

    /// units of measure in the *z* dimension
    #[getter]
    fn get_zunits(&self) -> UnitOfMeasure {
        self.0.zunits.into()
    }

    /// date of creation (possibily of last modification?) as recorded by Petra
    #[getter]
    fn get_created_date(&self) -> PyResult<Py<PyDateTime>> {
        let dt = self.0.created_date;
        Python::with_gil(|py| {
            Ok(PyDateTime::new(py, dt.year(), dt.month().into(), dt.day(),
              dt.hour(), dt.minute(), dt.second(), dt.microsecond(), None)?
              .into())
        })
    }

    /// we think this is used to describe the source of the data used
    /// in gridding
    #[getter]
    fn get_source_data(&self) -> &str {
        &self.0.source_data
    }

    /// we don't know what this means; as far as we can tell, it's always "C66"
    #[getter]
    fn get_unknown_metadata(&self) -> &str {
        &self.0.unknown_metadata
    }

    /// we think this string describes the map projection (e.g. "TX-27C",
    /// which we're pretty sure corresponds to EPSG:32039)
    #[getter]
    fn get_projection(&self) -> &str {
        &self.0.projection
    }

    /// we think this string describes the map datum (e.g. "NAD27")
    #[getter]
    fn get_datum(&self) -> &str {
        &self.0.datum
    }

    /// we think this number is used to describe the gridding method, but
    /// we're not sure how
    #[getter]
    fn get_grid_method(&self) -> u32 {
        self.0.grid_method
    }

    /// likewise, we think this stores values of an enumerated describing the
    /// map projection, but we don't know how to decode it
    #[getter]
    fn get_projection_code(&self) -> u32 {
        self.0.projection_code
    }

    /// this value is logged by Petra as "CM": "central meridian" perhaps?
    /// (observed values look like plausible longitudes)
    #[getter]
    fn get_cm(&self) -> f64 {
        self.0.cm
    }

    /// this value is logged by Petra as "RLAT": "reference latitude" perhaps?
    /// (observed values look like plausible latitudes)
    #[getter]
    fn get_rlat(&self) -> f64 {
        self.0.rlat
    }

    /// the actual grid data, as a numpy array
    ///
    /// this may be a rectangular or triangular grid; see the `is_rectangular`
    /// and `is_triangular` methods, or check `n_triangles`
    ///
    /// rectangular grids will have shape (rows x columns)
    /// for rectangular grids, each data element is a measurement in the *z*
    /// dimension; the *x* and *y* values are implicit (see the `xmin`, ...)
    /// attributes in `Grid`
    ///
    /// Petra stores the data in row-major form, with the values in each row
    /// proceeding along the *x* dimension from east to west, and the rows
    /// themselves proceeding along the *y* dimension from south to north
    /// (that is, the values of *y* increase for each row in order, and the
    /// "origin" is at the "lower left")
    ///
    /// note that many "computer graphics" applications and libraries expect
    /// the opposite convention for the *y* axis (origin at the upper left);
    /// you may wish to use e.g. `numpy.flip(grid.data, axis=0)` for these
    /// applications, or to use application-specific options (like passing
    /// `origin='lower'` to `matplotlib.pyplot.imshow`)
    ///
    /// triangular grids will have shape
    /// (n_triangles x 3 vertices x 3 dimensions)
    /// each triangle is represented as (*x*, *y*, *z*) triplets; we think
    /// (but haven't verified) that triangles are stored with their vertices in
    /// counterclockwise order, because they seem to work properly
    /// with `matplotlib.tri.Triangulation`
    #[getter]
    fn get_data(&self) -> PyObject {
        Python::with_gil(|py| {
            match &self.0.data {
                petra_grid::GridData::Rectangular(arr) =>
                    arr.to_pyarray(py).into(),
                petra_grid::GridData::Triangular(arr) =>
                    arr.to_pyarray(py).into(),
            }
        })
    }

    /// is this a rectangular grid?
    #[getter]
    fn get_is_rectangular(&self) -> bool {
        match &self.0.data {
            petra_grid::GridData::Rectangular(_) => true,
            _ => false,
        }
    }

    /// is this a triangular grid?
    #[getter]
    fn get_is_triangular(&self) -> bool {
        match &self.0.data {
            petra_grid::GridData::Triangular(_) => true,
            _ => false,
        }
    }

    /// get a debug-friendly representation of the underlying Rust
    /// petra_grid::Grid struct
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// read a Petra grid from a file path or file-like object
#[pyfunction]
fn read_grid(name_or_file_like: PyObject) -> PyResult<Grid> {
    let mut f = Filey::from(name_or_file_like)?;
    Ok(Grid(petra_grid::Grid::read(&mut f).map_err(to_pyerr)?))
}

/// types and functions for retrieving (partial) grid data from Petra GRD files
///
/// this library is based on a lot of time spent in a hex editor examining some
/// Petra grid files, including the WVGS Utica Playbook supplemental Petra grid
/// archive (https://www.wvgs.wvnet.edu/utica/playbook/pb_12.aspx) as well as
/// data provided by helpful collaborators
///
/// educated guesses have been made in some places based on information in Petra
/// documentation or ancillary output
///
/// while we've successfully used it to read real Petra grids, it's had limited
/// testing, and much of the data format remains opaque and mysterious, so
/// expect oddities and perhaps errors, especially when reading grids which use
/// "uncommon" methods or features
#[pymodule]
#[pyo3(name="petra_grid")]
fn petra_grid_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UnitOfMeasure>()?;
    m.add_class::<Grid>()?;
    m.add_function(wrap_pyfunction!(read_grid, m)?)?;
    Ok(())
}

fn to_pyerr(err: petra_grid::Error) -> PyErr {
    match err {
        petra_grid::Error::IOError(e) => e.into(),
        _ => PyValueError::new_err(format!("{}", err)),
    }
}
