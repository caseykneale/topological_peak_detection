fn sorted_indices<T: PartialOrd>(v: &[T]) -> Vec<usize> {
    let mut original : Vec<usize> = (0..v.len()).collect();
    original.sort_by(
        |a, b| v[*a].partial_cmp(&v[*b])
            .unwrap_or(core::cmp::Ordering::Equal)
    );
    original
}

pub enum Direction {
    Left,
    Right,
    None
}

#[derive(Debug)]
pub struct Homology{
    left_extent: usize,
    right_extent: usize,
    peak: usize,
}

impl Homology{
    /// Determines if an index is adjacent to a topological region.
    /// 
    ///# Arguments
    /// * `idx` - an index location in an array.
    pub fn is_adjacent(&self, idx: usize) -> Direction {
        if (self.left_extent > 0) && (idx == (self.left_extent - 1)) {
            Direction::Left
        } else if idx == self.right_extent + 1 {
            Direction::Right
        }else{
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
    pub fn get_peak_idx(&self) -> usize{
        self.peak
    }
}

pub fn find_homologies<T: PartialOrd>(x: &[T]) -> Vec<Homology> {
    let sorted_idxs = sorted_indices(x).into_iter();
    let mut homologies= Vec::<Homology>::with_capacity(70);
    
    for set_idx in sorted_idxs.into_iter().rev() {
        let mut found_home = false;
        for homology in (&mut homologies).iter_mut() {
            match homology.is_adjacent(set_idx){
                Direction::Left => {
                    homology.extend_left();
                    found_home = true;
                },
                Direction::Right => {
                    homology.extend_right();
                    found_home = true;
                }
                Direction::None => continue,
            }
            if found_home { break }
        }
        if !found_home {
            homologies.push(
                Homology { 
                    left_extent: set_idx, 
                    right_extent: set_idx, 
                    peak: set_idx,
                }
            );
        }
    }
    homologies
}

pub fn get_peaks(x: &[Homology]) -> Vec<usize> {
    x.iter()
        .map(|x| x.get_peak_idx())
        .collect()
}