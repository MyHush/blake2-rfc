// Copyright (c) 2015 Cesar Eduardo Barros
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::mem;
use std::slice;

pub trait AsMutBytes {
    fn as_mut_bytes(&mut self) -> &mut [u8];
}

macro_rules! as_mut_bytes_impl {
    ($t:ty) => {
        impl AsMutBytes for $t {
            #[inline]
            fn as_mut_bytes(&mut self) -> &mut [u8] {
                unsafe {
                    slice::from_raw_parts_mut(
                        self.as_mut_ptr() as *mut u8,
                        mem::size_of::<Self>())
                }
            }
        }
    }
}

as_mut_bytes_impl!([u32; 16]);
as_mut_bytes_impl!([u64; 16]);
