#[allow(dead_code)]
#[derive(Debug)]
#[non_exhaustive]
pub struct Unsortable;

fn sorted_indices<T: PartialOrd>(v: &[T]) -> Result<Vec<usize>, Unsortable> {
    let mut original: Vec<usize> = (0..v.len()).collect();
    let mut errored = false;
    original.sort_by(|a, b| match v[*a].partial_cmp(&v[*b]) {
        Some(ordering) => ordering,
        None => {
            errored = true;
            core::cmp::Ordering::Equal
        }
    });
    if errored {
        return Err(Unsortable);
    }
    Ok(original)
}

pub enum Direction {
    Left,
    Right,
    None,
}

#[derive(Debug)]
pub struct Homology {
    left_extent: usize,
    right_extent: usize,
    peak: usize,
}

impl Homology {
    /// Determines if an index is adjacent to a topological region.
    ///
    ///# Arguments
    /// * `idx` - an index location in an array.
    pub fn is_adjacent(&self, idx: usize) -> Direction {
        if (self.left_extent > 0) && (idx == (self.left_extent - 1)) {
            Direction::Left
        } else if idx == self.right_extent + 1 {
            Direction::Right
        } else {
            Direction::None
        }
    }

    /// extend's a region one hop to the left.
    pub fn extend_left(&mut self) {
        self.left_extent -= 1;
    }

    /// extend's a region one hop to the right.
    pub fn extend_right(&mut self) {
        self.right_extent += 1;
    }

    /// accessor method for aquiring the peak index associated with this region.
    pub fn get_peak_idx(&self) -> usize {
        self.peak
    }
}

pub fn find_homologies<T: PartialOrd>(x: &[T]) -> Result<Vec<Homology>, Unsortable> {
    if x.len() == 0 {
        return Err(Unsortable);
    }
    let sorted_idxs = sorted_indices(x)?.into_iter();
    let mut homologies = Vec::<Homology>::with_capacity(70);

    for set_idx in sorted_idxs.into_iter().rev() {
        let mut found_home = false;
        for homology in (&mut homologies).iter_mut() {
            match homology.is_adjacent(set_idx) {
                Direction::Left => {
                    homology.extend_left();
                    found_home = true;
                }
                Direction::Right => {
                    homology.extend_right();
                    found_home = true;
                }
                Direction::None => continue,
            }
            if found_home {
                break;
            }
        }
        if !found_home {
            homologies.push(Homology {
                left_extent: set_idx,
                right_extent: set_idx,
                peak: set_idx,
            });
        }
    }
    Ok(homologies)
}

pub fn get_peaks(x: &[Homology]) -> Vec<usize> {
    x.iter().map(|x| x.get_peak_idx()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{find_homologies, get_peaks};
    use std::f32::consts::PI;

    #[test]
    fn sinusoid_test() {
        let tst_vec: Vec<f32> = (0..6001)
            .map(|x| ((x as f32 / 1000_f32) * PI).sin())
            .collect();
        let check = tst_vec[10].clone();
        let homologies = find_homologies(&tst_vec).unwrap();
        let x = get_peaks(&homologies);
        assert_eq!(x.contains(&500), true);
        assert_eq!(x.contains(&2500), true);
        assert_eq!(x.contains(&4500), true);
        assert_eq!(x.contains(&6000), true);
        assert_eq!(x.len(), 4);
        assert_eq!(check, tst_vec[10]);
        println!("{:?}", homologies);
    }

    #[test]
    fn sinusoid_double_test() {
        let tst_vec: Vec<f64> = (0..6001)
            .map(|x| ((x as f64 / 1000_f64) * std::f64::consts::PI).sin())
            .collect();
        let check = tst_vec[10].clone();
        let homologies = find_homologies(&tst_vec).unwrap();
        let x = get_peaks(&homologies);
        assert_eq!(x.contains(&500), true);
        assert_eq!(x.contains(&2500), true);
        assert_eq!(x.contains(&4500), true);
        assert_eq!(x.contains(&6000), true);
        assert_eq!(x.len(), 4);
        assert_eq!(check, tst_vec[10]);
    }

    #[test]
    fn integer_test() {
        let tst_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 4, 3, 6, 3, 1, 1, 1, 5];
        let homologies = find_homologies(&tst_vec).unwrap();
        let x = get_peaks(&homologies);
        assert_eq!(x.contains(&4), true);
        assert_eq!(x.contains(&7), true);
        assert_eq!(x.contains(&12), true);
        assert_eq!(x.len(), 3);
    }
    #[test]
    fn failure_test() {
        let tst_vec: Vec<f32> = vec![1., 2., 3., 4., 5., std::f32::NAN, 1.0];
        let failed = match find_homologies(&tst_vec){
            Ok(_) => false,
            Err(_) => true,
        };
        assert_eq!(failed, true);
    }
}
