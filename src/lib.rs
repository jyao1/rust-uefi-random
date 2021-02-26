#![no_std]
use x86::random;

pub fn generate(dest: &mut [u8]) -> bool {
    unsafe {
        random::rdrand_slice(dest)
    }
}

#[cfg(test)]
mod tests {
    use super::generate;
    #[test]
    fn it_works() {
        let mut dest = [0u8;32];
        let res = generate(&mut dest[..]);
        assert_eq!(res, true);
    }
}
