 use std::cmp::Ordering;

 fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
     let mut low = 0;
     let mut high = arr.len();

     while low < high {
         let mid = low + (high - low) / 2;
         
         match arr[mid].cmp(&target) {
             Ordering::Equal => return Some(mid),
             Ordering::Less => low = mid + 1,
             Ordering::Greater => high = mid,
         }
     }
     
     None
 }

 fn main() {
     let arr = [1, 3, 5, 6, 9, 11, 15];

     println!("{:?}", binary_search(&arr, 9));
     println!("{:?}", binary_search(&arr, 4));
 }

