use std::{
    io::{self, Read, Seek, SeekFrom, Write},
    fs::File,
};

use pyo3::{
    prelude::{Python, PyObject},
};

pub enum Filey {
    RustFile(File),
    PyFileLike(PyObject),
}

impl Filey {
    pub fn from(name_or_file_like: PyObject) -> io::Result<Self> {
        Python::with_gil(|py| {
            if let Ok(path) = name_or_file_like.extract::<&str>(py) {
                Ok(Self::RustFile(File::open(path)?))
            } else {
                Ok(Self::PyFileLike(name_or_file_like))
            }
        })
    }
}

impl Read for Filey {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Filey::RustFile(f) => f.read(buf),
            Filey::PyFileLike(o) => {
                Python::with_gil(|py| {
                    buf.write(o.call_method1(py, "read", (buf.len(),))?
                      .extract(py)?)
                })
            },
        }
    }
}

impl Seek for Filey {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            Filey::RustFile(f) => f.seek(pos),
            Filey::PyFileLike(o) => {
                let args = match pos {
                    SeekFrom::Start(off) => (off as i64, 0),
                    SeekFrom::Current(off) => (off, 1),
                    SeekFrom::End(off) => (off, 2),
                };

                Python::with_gil(|py| {
                    Ok(o.call_method1(py, "seek", args)?.extract(py)?)
                })
            }
        }
    }
}
