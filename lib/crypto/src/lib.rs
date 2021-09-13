#![allow(dead_code,non_camel_case_types)]

#[macro_use] extern crate err;

static CRYPTO_INVALID_BASE16_DATA_SIZE:&str = "invalid base16 data size";
static CRYPTO_DIGEST_INVALID_ALGORITHM_NAME:&str = "invalid name of digest algorithm";
static CRYPTO_DIGEST_CREATE_INIT_ERROR:&str = "error while initializing message digest context";
static CRYPTO_DIGEST_UPDATE_ERROR:&str = "error while updating message digest";
static CRYPTO_DIGEST_VALUE_ERROR:&str = "error while retrieving message digest value";
static CRYPTO_CIPHER_INVALID_ALGORITHM_NAME:&str = "invalid name of cipher algorithm";
static CRYPTO_CIPHER_INVALID_KEY_LENGTH:&str = "invalid length of cipher key";
static CRYPTO_CIPHER_INVALID_INIT_VECTOR_LENGTH:&str = "invalid length of cipher initialisation vector";
static CRYPTO_CIPHER_NEW_INIT_ERROR:&str = "error while initializing cipher context";
static CRYPTO_CIPHER_UPDATE_ERROR:&str = "error while updating cipher context";
static CRYPTO_CIPHER_FINALIZE_ERROR:&str = "error while finalizing cipher context";

const OPENSSL_INIT_LOAD_CRYPTO_STRINGS:u64 = 0x02;
const OPENSSL_INIT_ADD_ALL_CIPHERS    :u64 = 0x04;
const OPENSSL_INIT_ADD_ALL_DIGESTS    :u64 = 0x08;

const BASE16_MAP:[u8;16] =
[//{{{
    '0' as u8,
    '1' as u8,
    '2' as u8,
    '3' as u8,
    '4' as u8,
    '5' as u8,
    '6' as u8,
    '7' as u8,
    '8' as u8,
    '9' as u8,
    'a' as u8,
    'b' as u8,
    'c' as u8,
    'd' as u8,
    'e' as u8,
    'f' as u8
];//}}}

use std::os::raw::{c_char,c_int,c_uint};

#[repr(C)] struct EVP_MD { dummy:c_int }
#[repr(C)] struct EVP_MD_CTX { dummy:c_int }
#[repr(C)] struct EVP_CIPHER { dummy:c_int }
#[repr(C)] struct EVP_CIPHER_CTX { dummy:c_int }
#[repr(C)] struct ENGINE { dummy:c_int }

#[link(name = "crypto")]
extern
{//{{{
    fn OPENSSL_init_crypto(opts:u64,settings:*const u8) -> c_int;

    fn RAND_bytes(buf:*mut u8,num:c_int) -> c_int;

    fn OBJ_nid2sn(n:c_int) -> *const c_char;

    fn EVP_MD_type(md:*const EVP_MD) -> c_int;
    fn EVP_MD_size(md:*const EVP_MD) -> c_int;

    fn EVP_CIPHER_nid(cipher:*const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_block_size(cipher:*const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_key_length(cipher:*const EVP_CIPHER) -> c_int;
    fn EVP_CIPHER_iv_length(cipher:*const EVP_CIPHER) -> c_int;

    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_free(ctx:*mut EVP_MD_CTX);
    fn EVP_MD_CTX_copy_ex(out:*mut EVP_MD_CTX,_in:*const EVP_MD_CTX) -> c_int;
    fn EVP_MD_CTX_md(ctx:*const EVP_MD_CTX) -> *const EVP_MD;

    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    fn EVP_CIPHER_CTX_free(ctx:*mut EVP_CIPHER_CTX);
    fn EVP_CIPHER_CTX_copy(out:*mut EVP_CIPHER_CTX,_in:*const EVP_CIPHER_CTX) -> c_int;
    fn EVP_CIPHER_CTX_cipher(ctx:*const EVP_CIPHER_CTX) -> *const EVP_CIPHER;
    fn EVP_CIPHER_CTX_block_size(ctx:*const EVP_CIPHER_CTX) -> c_int;

    fn EVP_DigestInit_ex(ctx:*mut EVP_MD_CTX,_type:*const EVP_MD,_impl:*mut ENGINE) -> c_int;
    fn EVP_DigestUpdate(ctx:*mut EVP_MD_CTX,d:*const u8,cnt:usize) -> c_int;
    fn EVP_DigestFinal_ex(ctx:*mut EVP_MD_CTX,md:*mut u8,s:*mut c_uint) -> c_int;

    fn EVP_EncryptInit_ex(ctx:*mut EVP_CIPHER_CTX,cipher:*const EVP_CIPHER,_impl:*mut ENGINE,key:*const u8,iv:*const u8) -> c_int;
    fn EVP_EncryptUpdate(ctx:*mut EVP_CIPHER_CTX,out:*mut u8,outl:*mut c_int,_in:*const u8,inl:c_int) -> c_int;
    fn EVP_EncryptFinal_ex(ctx:*mut EVP_CIPHER_CTX,out:*mut u8,outl:*mut c_int) -> c_int;

    fn EVP_DecryptInit_ex(ctx:*mut EVP_CIPHER_CTX,cipher:*const EVP_CIPHER,_impl:*mut ENGINE,key:*const u8,iv:*const u8) -> c_int;
    fn EVP_DecryptUpdate(ctx:*mut EVP_CIPHER_CTX,out:*mut u8,outl:*mut c_int,_in:*const u8,inl:c_int) -> c_int;
    fn EVP_DecryptFinal_ex(ctx:*mut EVP_CIPHER_CTX,out:*mut u8,outl:*mut c_int) -> c_int;

    fn EVP_EncodeBlock(t:*mut u8,f:*const u8,n:c_int) -> c_int;
    fn EVP_DecodeBlock(t:*mut u8,f:*const u8,n:c_int) -> c_int;

    fn EVP_get_digestbyname(name:*const c_char) -> *const EVP_MD;
    fn EVP_get_cipherbyname(name:*const c_char) -> *const EVP_CIPHER;
}//}}}

pub fn init()
{//{{{
    unsafe {
        OPENSSL_init_crypto(
            OPENSSL_INIT_LOAD_CRYPTO_STRINGS |
            OPENSSL_INIT_ADD_ALL_CIPHERS |
            OPENSSL_INIT_ADD_ALL_DIGESTS,
            std::ptr::null());
    }
}//}}}

pub fn random(a_size:usize) -> Vec<u8>
{//{{{
    let mut result = vec![0u8;a_size];

    unsafe {
        RAND_bytes(result.as_mut_ptr(),a_size as c_int);
    }

    result
}//}}}

pub struct Base16 {}
pub struct Base64 {}

pub struct DigestInfo {
    mdt:*const EVP_MD
}

pub struct Digest {
    ctx:*mut EVP_MD_CTX
}

pub struct CipherInfo {
    ct:*const EVP_CIPHER
}

pub struct Encrypt {
    ctx:*mut EVP_CIPHER_CTX
}

pub struct Decrypt {
    ctx:*mut EVP_CIPHER_CTX
}

impl Base16 {
    pub fn encode(a_data:&[u8]) -> Vec<u8>
    {//{{{
        let mut result:Vec<u8> = Vec::with_capacity(a_data.len() << 1);

        for byte in a_data {
            result.push(BASE16_MAP[(byte >> 4) as usize]);
            result.push(BASE16_MAP[(byte & 0x0f) as usize]);
        }

        result
    }//}}}

    pub fn decode(a_data:&[u8]) -> Result<Vec<u8>,err::Error>
    {//{{{
        if a_data.len() & 0x01 != 0 {
            return err!(CRYPTO_INVALID_BASE16_DATA_SIZE);
        }

        let mut result:Vec<u8> = Vec::with_capacity(a_data.len() >> 1);

        let mut ch_idx = 0;
        while ch_idx < a_data.len() {
            let mut ch;

            let first = a_data[ch_idx];
            match first {
                48 ..=  57 => ch = first - 48,
                97 ..= 102 => ch = 10 + (first - 97),
                65 ..=  70 => ch = 10 + (first - 65),
                _ => return err!(CRYPTO_INVALID_BASE16_DATA_SIZE)
            }

            ch <<= 4;

            let second = a_data[ch_idx + 1];
            match second {
                48 ..=  57 => ch += second - 48,
                97 ..= 102 => ch += 10 + (second - 97),
                65 ..=  70 => ch += 10 + (second - 65),
                _ => return err!(CRYPTO_INVALID_BASE16_DATA_SIZE)
            }

            result.push(ch);

            ch_idx += 2;
        }

        Ok(result)
    }//}}}
}

impl Base64 {
    pub fn encode(a_data:&[u8]) -> Vec<u8>
    {//{{{
        let mut result = vec![0u8;((a_data.len()/3 + 1) << 2) + 1];

        unsafe {
            let length = EVP_EncodeBlock(result.as_mut_ptr(),a_data.as_ptr(),a_data.len() as c_int);
            result.truncate(length as usize);
        }

        result
    }//}}}

    pub fn decode(a_data:&[u8]) -> Vec<u8>
    {//{{{
        let mut result = vec![0u8;((a_data.len() >> 2) * 3) + 1];

        unsafe {
            let length = EVP_DecodeBlock(result.as_mut_ptr(),a_data.as_ptr(),a_data.len() as c_int);
            result.truncate(length as usize);
        }

        result
    }//}}}
}

impl DigestInfo {
    pub fn by_name(a_name:&str) -> Result<DigestInfo,err::Error>
    {//{{{
        if a_name.as_bytes().last() != Some(&0u8) {
            return err!(err::CSTRING_MISSING_TERMINATING_ZERO);
        }

        unsafe {
            let mdt = EVP_get_digestbyname(std::ffi::CStr::from_bytes_with_nul_unchecked(a_name.as_bytes()).as_ptr());

            if mdt.is_null() {
                err!(CRYPTO_DIGEST_INVALID_ALGORITHM_NAME)
            }
            else {
                Ok(DigestInfo{mdt:mdt})
            }
        }
    }//}}}

    pub fn name(&self) -> String
    {//{{{
        unsafe {
            String::from(std::ffi::CStr::from_ptr(OBJ_nid2sn(EVP_MD_type(self.mdt))).to_str().unwrap())
        }
    }//}}}

    pub fn digest(&self) -> Result<Digest,err::Error>
    {//{{{
        Digest::from_info(self)
    }//}}}
}

impl Digest {
    pub fn from_info(a_info:&DigestInfo) -> Result<Digest,err::Error>
    {//{{{
        unsafe {
            let ctx = EVP_MD_CTX_new();

            if ctx.is_null() {
                return err!(CRYPTO_DIGEST_CREATE_INIT_ERROR);
            }

            let digest = Digest{ctx:ctx};

            if EVP_DigestInit_ex(digest.ctx,a_info.mdt,0 as *mut ENGINE) != 1 {
                return err!(CRYPTO_DIGEST_CREATE_INIT_ERROR);
            }

            Ok(digest)
        }
    }//}}}

    pub fn update(&mut self,a_data:&[u8]) -> Result<(),err::Error>
    {//{{{
        unsafe {
            match EVP_DigestUpdate(self.ctx,a_data.as_ptr(),a_data.len()) {
                1 => Ok(()),
                _ => err!(CRYPTO_DIGEST_UPDATE_ERROR)
            }
        }
    }//}}}

    pub fn value(&mut self) -> Result<Vec<u8>,err::Error>
    {//{{{
        unsafe {
            let copy_ctx = EVP_MD_CTX_new();

            if copy_ctx.is_null() {
                return err!(CRYPTO_DIGEST_VALUE_ERROR);
            }

            let copy = Digest{ctx:copy_ctx};

            let mdt = EVP_MD_CTX_md(self.ctx);
            if EVP_DigestInit_ex(copy.ctx,mdt,0 as *mut ENGINE) != 1 {
                return err!(CRYPTO_DIGEST_VALUE_ERROR);
            }

            if EVP_MD_CTX_copy_ex(copy.ctx,self.ctx) != 1 {
                return err!(CRYPTO_DIGEST_VALUE_ERROR);
            }

            let mut result = vec![0u8;EVP_MD_size(mdt) as usize];

            let mut length:c_uint = 0;
            if EVP_DigestFinal_ex(copy.ctx,result.as_mut_ptr(),&mut length as *mut c_uint) != 1 {
                return err!(CRYPTO_DIGEST_VALUE_ERROR);
            }

            Ok(result)
        }
    }//}}}
}

impl Drop for Digest {
    fn drop(&mut self)
    {//{{{
        unsafe {
            EVP_MD_CTX_free(self.ctx);
        }
    }//}}}
}

impl CipherInfo {
    pub fn by_name(a_name:&str) -> Result<CipherInfo,err::Error>
    {//{{{
        if a_name.as_bytes().last() != Some(&0u8) {
            return err!(err::CSTRING_MISSING_TERMINATING_ZERO);
        }

        unsafe {
            let ct = EVP_get_cipherbyname(std::ffi::CStr::from_bytes_with_nul_unchecked(a_name.as_bytes()).as_ptr());

            if ct.is_null() {
                err!(CRYPTO_CIPHER_INVALID_ALGORITHM_NAME)
            }
            else {
                Ok(CipherInfo{ct:ct})
            }
        }
    }//}}}

    pub fn name(&self) -> String
    {//{{{
        unsafe {
            String::from(std::ffi::CStr::from_ptr(OBJ_nid2sn(EVP_CIPHER_nid(self.ct))).to_str().unwrap())
        }
    }//}}}

    pub fn block_size(&self) -> usize
    {//{{{
        unsafe { EVP_CIPHER_block_size(self.ct) as usize }
    }//}}}

    pub fn key_length(&self) -> usize
    {//{{{
        unsafe { EVP_CIPHER_key_length(self.ct) as usize }
    }//}}}

    pub fn iv_length(&self) -> usize
    {//{{{
        unsafe { EVP_CIPHER_iv_length(self.ct) as usize }
    }//}}}

    pub fn check_key_iv(&self,a_key:&[u8],a_iv:&[u8]) -> Result<(),err::Error>
    {//{{{
        unsafe {
            if a_key.len() != EVP_CIPHER_key_length(self.ct) as usize {
                return err!(CRYPTO_CIPHER_INVALID_KEY_LENGTH);
            }

            if a_iv.len() != EVP_CIPHER_iv_length(self.ct) as usize {
                return err!(CRYPTO_CIPHER_INVALID_INIT_VECTOR_LENGTH);
            }
        }

        Ok(())
    }//}}}

    pub fn encrypt(&self,a_key:&[u8],a_iv:&[u8]) -> Result<Encrypt,err::Error>
    {//{{{
        Encrypt::from_info(self,a_key,a_iv)
    }//}}}

    pub fn decrypt(&self,a_key:&[u8],a_iv:&[u8]) -> Result<Decrypt,err::Error>
    {//{{{
        Decrypt::from_info(self,a_key,a_iv)
    }//}}}
}

macro_rules! cipher_from_info {
    ($struct:tt,$init:tt) => {
        pub fn from_info(a_info:&CipherInfo,a_key:&[u8],a_iv:&[u8]) -> Result<$struct,err::Error>
        {//{{{
            unsafe {
                match a_info.check_key_iv(a_key,a_iv) {
                    Err(err) => return Err(err),
                    Ok(()) => {}
                }

                let ctx = EVP_CIPHER_CTX_new();

                if ctx.is_null() {
                    return err!(CRYPTO_CIPHER_NEW_INIT_ERROR);
                }

                let cipher = $struct{ctx:ctx};

                if $init(cipher.ctx,a_info.ct,0 as *mut ENGINE,a_key.as_ptr(),a_iv.as_ptr()) != 1 {
                    return err!(CRYPTO_CIPHER_NEW_INIT_ERROR);
                }

                Ok(cipher)
            }
        }//}}}
    }
}

macro_rules! cipher_update {
    ($update:tt) => {
        pub fn update(&mut self,a_data:&[u8]) -> Result<Vec<u8>,err::Error>
        {//{{{
            unsafe {
                let data_length = a_data.len();
                let block_size = EVP_CIPHER_CTX_block_size(self.ctx) as usize;

                let mut result = vec![0;data_length + (block_size - data_length % block_size)];

                let mut length:c_int = 0;
                if $update(self.ctx,result.as_mut_ptr(),&mut length as *mut c_int,a_data.as_ptr(),a_data.len() as c_int) != 1 {
                    return err!(CRYPTO_CIPHER_UPDATE_ERROR);
                }

                result.truncate(length as usize);

                Ok(result)
            }
        }//}}}
    }
}

macro_rules! cipher_finalize {
    ($struct:tt,$init:tt,$final:tt) => {
        pub fn finalize(&mut self) -> Result<Vec<u8>,err::Error>
        {//{{{
            unsafe {
                let copy_ctx = EVP_CIPHER_CTX_new();

                if copy_ctx.is_null() {
                    return err!(CRYPTO_CIPHER_FINALIZE_ERROR);
                }

                let copy = $struct{ctx:copy_ctx};

                let ct = EVP_CIPHER_CTX_cipher(self.ctx);
                if $init(copy.ctx,ct,0 as *mut ENGINE,0 as *const u8,0 as *const u8) != 1 {
                    return err!(CRYPTO_CIPHER_FINALIZE_ERROR);
                }

                if EVP_CIPHER_CTX_copy(copy.ctx,self.ctx) != 1 {
                    return err!(CRYPTO_CIPHER_FINALIZE_ERROR);
                }

                let mut result = vec![0u8;EVP_CIPHER_CTX_block_size(copy.ctx) as usize];

                let mut length:c_int = 0;
                if $final(copy.ctx,result.as_mut_ptr(),&mut length as *mut c_int) != 1 {
                    return err!(CRYPTO_CIPHER_FINALIZE_ERROR);
                }

                result.truncate(length as usize);

                Ok(result)
            }
        }//}}}
    }
}

impl Encrypt {
    cipher_from_info!(Encrypt,EVP_EncryptInit_ex);
    cipher_update!(EVP_EncryptUpdate);
    cipher_finalize!(Encrypt,EVP_EncryptInit_ex,EVP_EncryptFinal_ex);
}

impl Drop for Encrypt {
    fn drop(&mut self)
    {//{{{
        unsafe {
            EVP_CIPHER_CTX_free(self.ctx);
        }
    }//}}}
}

impl Decrypt {
    cipher_from_info!(Decrypt,EVP_DecryptInit_ex);
    cipher_update!(EVP_DecryptUpdate);
    cipher_finalize!(Decrypt,EVP_DecryptInit_ex,EVP_DecryptFinal_ex);
}

impl Drop for Decrypt {
    fn drop(&mut self)
    {//{{{
        unsafe {
            EVP_CIPHER_CTX_free(self.ctx);
        }
    }//}}}
}

#[cfg(test)]
mod tests {
use super::*;

static ERROR_TEST_FAILED:&str = "Test failed";

#[test]
fn init_t0()
{//{{{
    init();
}//}}}

#[test]
fn base16_t0()
{//{{{
    assert_eq!(Base16::decode(&vec![0u8;3]),test_err!(CRYPTO_INVALID_BASE16_DATA_SIZE));

    let data = "Hello world!".as_bytes();
    let base16 = Base16::encode(data);

    println!("{}",std::str::from_utf8(data).unwrap());
    println!("{}",std::str::from_utf8(base16.as_slice()).unwrap());

    let decoded = Base16::decode(base16.as_slice()).unwrap();
    assert_eq!(data,decoded.as_slice());

    println!("{}",std::str::from_utf8(decoded.as_slice()).unwrap());

    let mut length = 0;
    while length < 64 {
        let data = random(length);
        let decoded = Base16::decode(Base16::encode(data.as_slice()).as_slice()).unwrap();
        assert_eq!(data,decoded);

        length += 1;
    }
}//}}}

#[test]
fn base64_t0()
{//{{{
    let data = "Hello world!".as_bytes();
    let base64 = Base64::encode(data);

    println!("{}",std::str::from_utf8(data).unwrap());
    println!("{}",std::str::from_utf8(base64.as_slice()).unwrap());

    let decoded = Base64::decode(base64.as_slice());
    assert_eq!(data,decoded.as_slice());

    println!("{}",std::str::from_utf8(decoded.as_slice()).unwrap());

    let mut length = 0;
    while length < 64 {
        let data = random(length);
        let decoded = Base64::decode(Base64::encode(data.as_slice()).as_slice());
        assert_eq!(data,decoded);

        length += 3;
    }
}//}}}

#[test]
fn digest_t0()
{//{{{
    init();

    match DigestInfo::by_name("") { Err(err) => { assert_eq!(err.descr,err::CSTRING_MISSING_TERMINATING_ZERO) } _ => panic!(ERROR_TEST_FAILED) }
    match DigestInfo::by_name("sha256") { Err(err) => { assert_eq!(err.descr,err::CSTRING_MISSING_TERMINATING_ZERO) } _ => panic!(ERROR_TEST_FAILED) }
    match DigestInfo::by_name("sha255\0") { Err(err) => { assert_eq!(err.descr,CRYPTO_DIGEST_INVALID_ALGORITHM_NAME) } _ => panic!(ERROR_TEST_FAILED) }
    match DigestInfo::by_name("sha256\0") { Ok(_) => {} _ => panic!(ERROR_TEST_FAILED) }

    let digest_info = DigestInfo::by_name("sha256\0").unwrap();
    assert_eq!(digest_info.name(),"SHA256");

    match Digest::from_info(&digest_info) { Ok(Digest{ctx:_}) => {} _ => panic!(ERROR_TEST_FAILED) }
    match digest_info.digest() { Ok(Digest{ctx:_}) => {} _ => panic!(ERROR_TEST_FAILED) }

    let mut digest = digest_info.digest().unwrap();
    digest.update("Hello world!".as_bytes()).unwrap();
    let value = Base64::encode(digest.value().unwrap().as_slice());
    println!("{}",std::str::from_utf8(value.as_slice()).unwrap());
}//}}}

#[test]
fn encrypt_t0()
{//{{{
    init();

    match CipherInfo::by_name("") { Err(err) => { assert_eq!(err.descr,err::CSTRING_MISSING_TERMINATING_ZERO) } _ => panic!(ERROR_TEST_FAILED) }
    match CipherInfo::by_name("AES-256-CBC") { Err(err) => { assert_eq!(err.descr,err::CSTRING_MISSING_TERMINATING_ZERO) } _ => panic!(ERROR_TEST_FAILED) }
    match CipherInfo::by_name("AES-256-CBB\0") { Err(err) => { assert_eq!(err.descr,CRYPTO_CIPHER_INVALID_ALGORITHM_NAME) } _ => panic!(ERROR_TEST_FAILED) }
    match CipherInfo::by_name("AES-256-CBC\0") { Ok(_) => {} _ => panic!(ERROR_TEST_FAILED) }

    let cipher_info = CipherInfo::by_name("AES-256-CBC\0").unwrap();

    assert_eq!(cipher_info.name(),"AES-256-CBC");
    assert_eq!(cipher_info.block_size(),16);
    assert_eq!(cipher_info.key_length(),32);
    assert_eq!(cipher_info.iv_length(),16);

    let key = random(cipher_info.key_length());
    let iv = random(cipher_info.iv_length());

    match Encrypt::from_info(&cipher_info,key.as_slice(),iv.as_slice()) { Ok(Encrypt{ctx:_}) => {} _ => panic!(ERROR_TEST_FAILED) }
    match cipher_info.encrypt(key.as_slice(),iv.as_slice()) { Ok(Encrypt{ctx:_}) => {} _ => panic!(ERROR_TEST_FAILED) }

    let mut encrypt = cipher_info.encrypt(key.as_slice(),iv.as_slice()).unwrap();
    let mut encrypted_lst:Vec<Vec<u8>> = vec![];

    let mut message:Vec<u8> = vec![];

    // - encrypt message block by block -
    for _ in 0..100 {
        let mut msg_block = random(256);

        encrypted_lst.push(encrypt.update(msg_block.as_slice()).unwrap());
        message.append(&mut msg_block);
    }

    encrypted_lst.push(encrypt.finalize().unwrap());

    // - join encrypted list -
    let mut encrypted:Vec<u8> = vec![];
    for idx in 0..encrypted_lst.len() {
        encrypted.append(&mut encrypted_lst[idx].clone());
    }

    // - decrypt whole message at once -
    let mut decrypt = cipher_info.decrypt(key.as_slice(),iv.as_slice()).unwrap();
    let mut decrypted:Vec<u8> = vec![];

    decrypted.append(&mut decrypt.update(encrypted.as_slice()).unwrap());
    decrypted.append(&mut decrypt.finalize().unwrap());

    assert_eq!(decrypted,message);

    // - decrypt message block by block -
    let mut decrypt = cipher_info.decrypt(key.as_slice(),iv.as_slice()).unwrap();
    let mut decrypted:Vec<u8> = vec![];

    for block in &encrypted_lst {
        decrypted.append(&mut decrypt.update(block.as_slice()).unwrap());
    }
    decrypted.append(&mut decrypt.finalize().unwrap());

    assert_eq!(decrypted,message);
}//}}}

}

