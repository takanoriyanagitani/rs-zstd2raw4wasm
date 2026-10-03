use std::sync::RwLock;

use ruzstd::io_std::Error as ZError;
use ruzstd::io_std::Read as ZRead;

use ruzstd::decoding::StreamingDecoder;

static DECODED: RwLock<Vec<u8>> = RwLock::new(vec![]);

#[link(wasm_import_module = "env")]
#[allow(unsafe_code)]
unsafe extern "C" {
    /// Copies the data to wasm.
    ///
    /// Should return:
    ///   - 0 on EOF
    ///   - <0 on erorr
    ///   - >0: the number of bytes read
    fn host_to_wasm(ptr: *mut u8, len: usize) -> i32;
}

pub fn zrdr2vec<R>(rdr: R, out: &mut Vec<u8>) -> Result<usize, &'static str>
where
    R: ZRead,
{
    let mut sdec: StreamingDecoder<_, _> =
        StreamingDecoder::new(rdr).map_err(|_| "invalid input")?;
    sdec.read_to_end(out)
        .map_err(|_| "unable to decode the zstd bytes")
}

pub struct ZrdrJspi;

impl ZRead for ZrdrJspi {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ZError> {
        if buf.is_empty() {
            return Ok(0);
        }

        let ptr: *mut u8 = buf.as_mut_ptr();
        let siz: usize = buf.len();

        #[allow(unsafe_code)]
        let ret: i32 = unsafe { host_to_wasm(ptr, siz) };

        if ret < 0 {
            return Err(ZError::other("non-zero error code got"));
        }

        Ok(ret as usize)
    }
}

pub fn optr() -> Result<*const u8, &'static str> {
    let guard = DECODED.try_read().map_err(|_| "unable to read lock")?;
    let s: &[u8] = &guard;
    Ok(s.as_ptr())
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub fn out_pointer() -> *const u8 {
    optr().unwrap_or(std::ptr::null())
}

pub fn osiz() -> Result<usize, &'static str> {
    let guard = DECODED.try_read().map_err(|_| "unable to read lock")?;
    let s: &[u8] = &guard;
    Ok(s.len())
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn osize() -> i32 {
    osiz().ok().and_then(|u| u.try_into().ok()).unwrap_or(-1)
}

pub fn dec_zstd() -> Result<usize, &'static str> {
    let mut guard = DECODED.try_write().map_err(|_| "unable to write lock")?;
    let v: &mut Vec<u8> = &mut guard;
    v.clear();
    let zrdr = ZrdrJspi {};
    zrdr2vec(zrdr, v)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn decode_zstd() -> i32 {
    dec_zstd()
        .ok()
        .and_then(|u| u.try_into().ok())
        .unwrap_or(-1)
}
