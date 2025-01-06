// a function for turning a 1-d sequence of data into a 2d representation where adjacency is preserved, a la map-of-the-internet

// hypothetical process:
// - expand list to nearest larger even power of 2, N^2
// - create empty NxN matrix
// - for each index:
//   - x=y=0
//   - break value into 2-bit pairs
//   - set adjustment scale to 0
//   - for each pair, going from least significant to most :
//     - x += pair.0*2^scale
//     - y += pair.1*2^scale
//     - scale++
//   - matrix[x,y] = list[index]

use bitvec::vec::BitVec;
use rayon::prelude::*;

fn to_space_filled<T, I: ExactSizeIterator<Item = T>>(iter: I) -> Vec<Vec<T>> {
    let n_side = get_size_of_square(&iter);

    let mut pic = Vec::<Vec<T>>::from_iter((0..n_side).map(|_| Vec::<T>::with_capacity(n_side)));

    iter.enumerate().for_each(|(index, item)| {
        let pos: BitVec<usize> = BitVec::from_element(index);

        // this segment looks like hell, so let me explain
        // each bit is paired with its position in the number
        // then collected into another iterator and iterated through in chunks of 2
        // that is passed into a parallel iterator. If the bit is 1, 2^f(significance), else 0
        // then sum all the even and odd bits to get the x,y positions
        let (x, y) = pos
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, bool)>>()
            .chunks(2)
            .par_bridge()
            .map(|chunk| {
                let (scale_x, bit_x) = chunk[0];
                let (scale_y, bit_y) = chunk[1];
                (
                    if bit_x {
                        2usize.pow((scale_x / 2) as u32)
                    } else {
                        0usize
                    },
                    if bit_y {
                        2usize.pow(((scale_y - 1) / 2) as u32)
                    } else {
                        0usize
                    },
                )
            })
            .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
        pic[x][y] = item;
    });

    pic
}

fn get_size_of_square<T, I: ExactSizeIterator<Item=T>>(iter: &I) -> usize {
    let mut n_size_log2 = iter.len().next_power_of_two().ilog2();
    if n_size_log2 % 2 == 1 {
        n_size_log2 += 1;
    }

    let n_side: usize = 2usize.pow(n_size_log2 / 2);
    n_side
}
