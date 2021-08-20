#![allow(clippy::many_single_char_names)]
use std::cmp::Ordering;

const R: usize = 256; // extended ASCII alphabet size
const CUTOFF: usize = 15; // cutoff to insertion sort

pub struct MSD;

impl MSD {
    pub fn sort<T: AsRef<str> + Copy>(a: &mut [T]) {
        let n = a.len();
        let mut aux = vec![a[0]; n];
        Self::do_sort(a, 0, n - 1, 0, &mut aux);
    }

    fn do_sort<T: AsRef<str> + Copy>(a: &mut [T], lo: usize, hi: usize, d: usize, aux: &mut [T]) {
        // cutoff to insertion sort for small subarrays
        if hi <= lo + CUTOFF {
            Self::insertion(a, lo, hi, d);
            return;
        }

        // compute frequency counts
        let mut count = [0; R + 2];
        for it in a.iter().take(hi + 1).skip(lo) {
            let c = Self::char_at(*it, d);
            count[(c + 2) as usize] += 1;
        }

        // transform counts to indicies
        for r in 0..R + 1 {
            count[r + 1] += count[r];
        }

        // distribute
        for it in a.iter().take(hi + 1).skip(lo) {
            let c = Self::char_at(*it, d);
            aux[count[(c + 1) as usize]] = *it;
            count[(c + 1) as usize] += 1;
        }

        // copy back
        a[lo..(hi + 1)].clone_from_slice(&aux[..((hi - lo) + 1)]);

        // recursively sort for each character (excludes sentinel -1)
        for r in 0..R {
            Self::do_sort(
                a,
                lo + count[r],
                (lo + count[r + 1]).saturating_sub(1),
                d + 1,
                aux,
            );
        }
    }

    fn char_at<T: AsRef<str> + Copy>(s: T, d: usize) -> i32 {
        let s = s.as_ref();
        let len = s.len();
        if d >= len {
            -1
        } else {
            s.as_bytes()[d] as i32
        }
    }

    fn insertion<T: AsRef<str> + Copy>(a: &mut [T], lo: usize, hi: usize, d: usize) {
        for i in lo..=hi {
            let mut j = i;
            while j > lo && Self::less(a[j], a[j - 1], d) {
                a.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn less<T: AsRef<str> + Copy>(v: T, w: T, d: usize) -> bool {
        let v = v.as_ref();
        let w = w.as_ref();
        for i in d..std::cmp::min(v.len(), w.len()) {
            let a = v.as_bytes()[i];
            let b = w.as_bytes()[i];
            match a.cmp(&b) {
                Ordering::Less => return true,
                Ordering::Equal => (),
                Ordering::Greater => return false,
            }
        }
        v.len() < w.len()
    }
}
