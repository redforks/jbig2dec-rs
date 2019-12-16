use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::ptr;
use std::slice;

use jbig2dec_sys::*;

mod errors;

use crate::errors::Error;

/// This struct represents the document structure
#[derive(Debug, Clone)]
pub struct Document {
    images: Vec<Image>,
}

impl Document {
    /// Open a document from a path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut reader = BufReader::new(File::open(path).unwrap());
        Self::from_reader(&mut reader)
    }

    /// Open a document from a `Read`
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let ctx = unsafe {
            jbig2_ctx_new(
                ptr::null_mut(),
                Jbig2Options::JBIG2_OPTIONS_DEFAULT,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
            )
        };
        if ctx.is_null() {
            return Err(Error::CreateContextFailed);
        }
        let mut content = Vec::new();
        let num_bytes = reader.read_to_end(&mut content).unwrap();
        unsafe {
            jbig2_data_in(ctx, content.as_mut_ptr(), num_bytes);
        }
        let code = unsafe { jbig2_complete_page(ctx) };
        if code != 0 {
            return Err(Error::IncompletePage);
        }
        let mut images = Vec::new();
        loop {
            let page = unsafe { jbig2_page_out(ctx) };
            if page.is_null() {
                break;
            }
            let image = unsafe { Image::from_raw(page) };
            images.push(image);
            unsafe { jbig2_release_page(ctx, page) };
        }
        unsafe {
            jbig2_ctx_free(ctx);
        }
        Ok(Self { images })
    }

    /// Get images
    pub fn images(&self) -> &[Image] {
        &self.images
    }

    /// Number of images
    pub fn len(&self) -> usize {
        self.images.len()
    }
}

impl IntoIterator for Document {
    type Item = Image;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.images.into_iter()
    }
}

/// This struct represents a image.
#[derive(Clone, PartialEq)]
pub struct Image {
    width: u32,
    height: u32,
    stride: u32,
    data: Vec<u8>,
}

impl Image {
    unsafe fn from_raw(image: *mut Jbig2Image) -> Self {
        let image = *image;
        let width = image.width;
        let height = image.height;
        let stride = image.stride;
        let length = (height * stride) as usize;
        let data = slice::from_raw_parts(image.data, length).to_vec();
        Self {
            width,
            height,
            stride,
            data,
        }
    }

    /// Get image width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get image height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get image stride
    pub fn stride(&self) -> u32 {
        self.stride
    }

    /// Get image data as bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get image data as mutable bytes
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Document;

    #[test]
    fn test_document_open() {
        let doc = Document::open("annex-h.jbig2").expect("open document failed");
        println!("{:#?}", doc);
    }
}
