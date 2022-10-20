#![deny(clippy::all)]

fn main() {
    let org_arr = [1, 2, 3, 6, 6, 10, 3, 3, 6, 6, 11];
    let sub_arr = [3, 6, 6, 11];
    // let sub_arr = [1, 2, 3, 6, 6, 10, 3, 3, 6, 6, 11, 12];

    println!("{}", is_sub_arr(&org_arr, &sub_arr).to_string());
    println!("{}", org_arr.is_contain(&sub_arr).to_string());
}

trait IsContain {
    fn is_contain(&self, sub_arr: &[i32]) -> bool;
}

impl IsContain for [i32] {
    fn is_contain(&self, sub_arr: &[i32]) -> bool {
        is_sub_arr(self, sub_arr)
    }
}

fn is_sub_arr(org_arr: &[i32], sub_arr: &[i32]) -> bool {
    let mut identical_count = 0;
    let org_arr_len = org_arr.len();
    let sub_arr_len = sub_arr.len();

    if org_arr_len < sub_arr_len {
        return false;
    } else {
        let mut index = 0;
        while index < org_arr_len {
            let num = &org_arr[index];
            if index > org_arr_len - sub_arr_len && identical_count == 0 {
                break;
            }
            println!("Num / Identical: {} {}", num, identical_count);
            if num.to_owned() == sub_arr[identical_count].to_owned() {
                identical_count += 1;
                if identical_count == sub_arr_len {
                    break;
                }
            } else {
                if num == &sub_arr[0] {
                    identical_count = 1;
                } else {
                    identical_count = 0;
                }
            }
            index += 1;
        }
        identical_count == sub_arr_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sub_arr() {
        let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
        let sub_arr = [6, 8, 10];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), true);

        let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
        let sub_arr = [1, 2, 3];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), true);

        let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
        let sub_arr = [8, 10, 11];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), true);

        let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
        let sub_arr = [6, 10, 11];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), false);

        let org_arr = [1, 2, 3, 5, 8, 8, 10, 11];
        let sub_arr = [8, 10, 11];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), true);

        let org_arr = [1, 1, 3, 5, 1, 2, 3, 11];
        let sub_arr = [1, 2, 3];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), true);

        let org_arr = [1, 1, 3, 5, 5, 5, 3, 11];
        let sub_arr = [5, 5, 5];
        assert_eq!(is_sub_arr(&org_arr, &sub_arr), true);
    }
}
