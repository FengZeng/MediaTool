use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Read, Seek};

extern "C" {
    fn mi_Init() -> *const c_void;
    fn mi_Open_Buffer_Init(mi: *const c_void, size: u64, offset: u64) -> i32;
    fn mi_Open_Buffer_Continue(mi: *const c_void, buffer: *const c_char, size: i32) -> i32;
    fn mi_Open_Buffer_Continue_GoTo_Get(mi: *const c_void) -> i32;
    fn mi_Open_Buffer_Finalize(mi: *const c_void) -> i32;
    fn mi_Info(mi: *const c_void) -> *const c_char;
    fn mi_Close(mi: *const c_void, buffer: *const c_char);
}

pub fn init() -> *const c_void {
    unsafe { mi_Init() }
}

pub fn open(url: &str) -> Result<String, io::Error> {
    let mut file = File::open(url)?;

    let file_size = file.metadata()?.len();
    let mut buffer = [0u8; 1024 * 256];

    unsafe {
        // Initialize the library
        let mi_handle = mi_Init();
        if mi_handle.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "mi_Init failed"));
        }

        // Initialize buffer processing
        mi_Open_Buffer_Init(mi_handle, file_size, 0);

        let result = (|| -> io::Result<()> {
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    // End of file
                    break;
                }

                let state = mi_Open_Buffer_Continue(
                    mi_handle,
                    buffer.as_ptr() as *const c_char,
                    bytes_read as i32,
                );

                if state & 0x08 != 0 {
                    // Buffer processing is complete
                    break;
                }

                let goto_offset = mi_Open_Buffer_Continue_GoTo_Get(mi_handle);
                if goto_offset > 0 {
                    let offset = file.seek(io::SeekFrom::Start(goto_offset as u64))?;
                    mi_Open_Buffer_Init(mi_handle, file_size, offset);
                }
            }
            Ok(())
        })();

        // Propagate any errors from the loop
        result?;

        // Retrieve the information
        let c_buffer = mi_Info(mi_handle);
        if c_buffer.is_null() {
            mi_Close(mi_handle, c_buffer);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "mi_Info returned null",
            ));
        }

        let c_str = CStr::from_ptr(c_buffer);
        let info = c_str.to_string_lossy().to_string(); // Convert to Rust String

        // Finalize and cleanup
        mi_Open_Buffer_Finalize(mi_handle);
        mi_Close(mi_handle, c_buffer);

        Ok(info) // Return the string as part of the Result
    }
}
