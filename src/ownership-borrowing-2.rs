//====================================
// BÀI 2:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

//=====================================

fn main() {
    // let objetive = 3126.59;

    // 27
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();
    // let values_index_max = values_number - 1;

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while &additions.len() > &0usize {
        let mut addition: f64 = 0.0;
        // let mut saltar: i32 = 0;

        // Sumar valores en additions
        for element_index in additions.clone() {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }
    }
}
