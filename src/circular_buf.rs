pub struct CircularBuf<T: Sized, const L: usize> {
    ptr: usize,
    buf: [T; L]
}
impl<T, const L: usize> CircularBuf<T, L> {
    fn signed_idx_to_unsigned(&self, idx: isize) -> usize {
        if idx >= 0 {
            let mut idx = idx as usize;
            if idx > L {
                panic!()
            }
            if idx + self.ptr > L { // do we have to wrap
                idx -= L - self.ptr // remove section between ptr and end
                // S----P--E
                //      ------I
                //     V
                // S----P--E
                // --I
            }
            idx
        }
        else { // idx is negative
            let idx = (idx * -1) as usize;
            if idx > self.ptr { // points to negative buf idx
                L - (idx - self.ptr)
            }
            else {
                self.ptr - idx
            }
        }
    }

    pub fn get_ref(&self, idx: isize) -> &T {
        &self.buf[self.signed_idx_to_unsigned(idx)]
    }
    pub fn set(&mut self, idx: isize, val: T) {
        self.buf[self.signed_idx_to_unsigned(idx)] = val
    }

    pub fn get_slice(&self, start: isize, mut end: isize) -> Vec<&T> {
        let mut ret = Vec::new();
        if end == 0 {
            end = L as isize
        }
        for i in start..end {
            ret.push(self.get_ref(i))
        }
        ret
    }
}