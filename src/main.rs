use regex::Regex;

fn is_sub_array(org_arr: &[i32], sub_arr: &[i32]) -> bool {
  let org_arr_length = org_arr.len();
  let sub_arr_length = sub_arr.len();

  if org_arr_length < sub_arr_length {
    return false;
  } else {
    // let org_arr = [1, 2,3,5,6,8 (stop), 10, 11];
    // let sub_arr = [6,8,10];
    let org_max_check_length = org_arr_length - sub_arr_length + 1;

    for org_index in 0..org_max_check_length {
      // println!("org = {}", org_arr[org_index]);
      let mut check = true;
      for sub_index in 0..sub_arr_length {
        // println!("check sub_arr = {}, org_arr = {}", sub_arr[sub_index], org_arr[org_index + sub_index]);
        if sub_arr[sub_index] != org_arr[org_index + sub_index] {
          check = false;
          break;
        }
      }
      if check {
        return true;
      }
    }
  }

  return false;
}

fn exercise1() {
  let org_arr = [1, 2,3,5,6,8, 10, 11];
  let sub_arr = [6,8, 10];
  println!("org_arr = {:?}", org_arr);
  println!("sub_arr = {:?}", sub_arr);

  if is_sub_array(&org_arr, &sub_arr) {
    println!("Là mảng con!");
  } else {
    println!("Không phải là mảng con!");
  }
}

fn count_match_str(str: String, match_str: String) -> usize {
  let regex_format = format!(r"(?i){}", match_str);

  let regex = Regex::new(&regex_format).unwrap();
  let result = regex.find_iter(&str).count();

  // for cap in Regex::new(&regex_format).unwrap().find_iter(&str) {
  //   println!("{:#?}", cap);
  // }

  return result;
}

fn exercise2() {
  let str = "ThiS is a regular paragraph with the default style of Normal. This Is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
  println!("{}", str);
  println!();

  println!("Nhập chuỗi cần tìm kiếm:");
  let mut match_str = String::new();
  std::io::stdin().read_line(&mut match_str).unwrap();

  let count = count_match_str(str.to_string(), match_str.trim().to_string());
  println!("Số lượng từ xuất hiện = {}", count);
}

fn main() {
  println!("Bài tập 1: ");
  exercise1();

  println!("-------------------");

  println!("Bài tập 2: cho chuỗi string:");
  exercise2();
}
