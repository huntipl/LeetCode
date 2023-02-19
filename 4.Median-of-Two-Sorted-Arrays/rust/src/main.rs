fn main() {
    println!("Hello, world!");
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // merge both arrays
        let mut merged: Vec<i32> = vec![];
        let mut n1;
        let mut n2;

        n1 = nums1.iter();
        n2 = nums2.iter();
        let mut t1 = n1.next();
        let mut t2 = n2.next();

        let mut v1;
        let mut v2;

        loop {
            if t1.is_some() && t2.is_some() {
                v1 = *t1.unwrap();
                v2 = *t2.unwrap();
                match v1.cmp(&v2) {
                    std::cmp::Ordering::Less => {
                        merged.push(v1);
                        t1 = n1.next();
                    }
                    std::cmp::Ordering::Equal => {
                        merged.push(v1);
                        t1 = n1.next();
                        merged.push(v2);
                        t2 = n2.next();
                    }
                    std::cmp::Ordering::Greater => {
                        merged.push(v2);
                        t2 = n2.next();
                    }
                }
            } else if t1.is_some() {
                merged.push(*t1.unwrap());
                t1 = n1.next();
            } else if t2.is_some() {
                merged.push(*t2.unwrap());
                t2 = n2.next();
            } else {
                break;
            }
        }
        dbg!(&merged);

        let merged_len = merged.len();
        let ind = merged_len / 2;
        if merged_len % 2 != 0 {
            return merged[ind] as f64;
        } else {
            return ((merged[ind] + merged[ind - 1]) as f64 / 2.0) as f64;
        }
    }
}
