#[allow(dead_code)]
pub mod top_pe;

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use crate::top_pe::{
        find_homologies, 
        get_peaks
    };

    #[test]
    fn sinusoid_test() {
        let tst_vec:Vec<f32> = (0..6001)
            .map(|x| ((x as f32 / 1000_f32) * PI).sin())
            .collect();
        let check = tst_vec[10].clone();
        let homologies = find_homologies(&tst_vec);
        let x = get_peaks(&homologies);
        assert_eq!(x.contains(&500), true);
        assert_eq!(x.contains(&2500), true);
        assert_eq!(x.contains(&4500), true);
        assert_eq!(x.contains(&6000), true);
        assert_eq!(x.len(), 4);
        assert_eq!(check, tst_vec[10]);
        println!("{:?}",homologies);
    }
    
    #[test]
    fn sinusoid_double_test() {
        let tst_vec:Vec<f64> = (0..6001)
            .map(|x| ((x as f64 / 1000_f64) * std::f64::consts::PI).sin())
            .collect();
        let check = tst_vec[10].clone();
        let homologies = find_homologies(&tst_vec);
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
        let tst_vec:Vec<i32> = vec![1,2,3,4,5,4,3,6,3,1,1,1,5];
        let homologies = find_homologies(&tst_vec);
        let x = get_peaks(&homologies);
        assert_eq!(x.contains(&4), true);
        assert_eq!(x.contains(&7), true);
        assert_eq!(x.contains(&12), true);
        assert_eq!(x.len(), 3);
    }
}