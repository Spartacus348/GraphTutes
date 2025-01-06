use std::ops::Index;
use std::vec::IntoIter;
// a function for turning a 1-d sequence of data into a 2d representation where adjacency is preserved, a la map-of-the-internet
use either::Either;

const N_CELL: usize = 3;

type HilbertBranch<'a, T> = Either<T, &'a HilbertCurve<'a, T>>;

struct HilbertCurve<'a, T: 'a> {
    content: [HilbertBranch<'a, T>; N_CELL],
    index: isize,
}
impl<'a, T:'a> Index<usize> for HilbertCurve<'a, T> {
    type Output = HilbertBranch<'a, T>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.content[i]
    }
}

struct HilbertIter<'a, T: 'a> {
    parent: &'a HilbertCurve<'a, T>,
    index: isize,
    diving_iter: Option<&'a HilbertIter<'a, T>>,
}

impl<'a, T: 'a> HilbertIter<'a, T> {
    fn new(parent: &'a HilbertCurve<T>) -> Self {
        Self{
            parent,
            index: 0,
            diving_iter: None,
        }
    }
}

impl<'a, T: 'a> Iterator for HilbertIter<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > (N_CELL - 1) as isize {
            return None
        }

        if self.diving_iter.is_none() {
            self.index += 1;
            let branch = self.parent.content.get(self.index as usize);
            
            
        }
    }
}


