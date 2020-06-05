use std::time::Duration;

const BASE: usize = 62;

pub trait SmallestReadableString: Sized + Copy {
    fn to_smallest_readable_string(self, result: &mut [u8; 6]) -> &[u8];
}


impl SmallestReadableString for Duration {
    fn to_smallest_readable_string(self, result: &mut [u8; 6]) -> &[u8] {
        let n = (self.as_nanos() % 1000000000) as usize / 1000;
        let mut r = n;
        let mut i = 0;

        while r > BASE {
            r = r / BASE;
            let d = r % BASE;
            let d = if d < 10 { 0x30 + d } else if d < 36 { 0x41 + (d - 10) } else { 0x61 + (d - 36) };
            result[i] = d as u8;
            i += 1;
        }

        result
    }
}