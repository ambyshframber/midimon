pub struct CircularBuf<T: Sized + Default> {
    ptr: usize,
    buf: Vec<T>
}
impl<T: Default> CircularBuf<T> {
    pub fn new(size: usize) -> CircularBuf<T> {
        let mut buf = Vec::new();
        for i in 0..size {
            buf.push(T::default())
        }
        CircularBuf::<T> {
            ptr: 0,
            buf
        }
    }

    fn signed_idx_to_unsigned(&self, idx: isize) -> usize {
        if idx >= 0 {
            let mut idx = idx as usize;
            if idx > self.buf.len() {
                panic!()
            }
            if idx + self.ptr > self.buf.len() { // do we have to wrap
                idx -= self.buf.len() - self.ptr // remove section between ptr and end
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
                self.buf.len() - (idx - self.ptr)
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
        let idx = self.signed_idx_to_unsigned(idx);
        self.buf[idx] = val
    }
    pub fn push(&mut self, val: T) {
        self.buf[self.ptr] = val;
        self.ptr += 1
    }

    pub fn get_slice(&self, start: isize, mut end: isize) -> Vec<&T> {
        let mut ret = Vec::new();
        if end == 0 {
            end = self.ptr as isize
        }
        for i in start..end {
            ret.push(self.get_ref(i))
        }
        ret
    }
}