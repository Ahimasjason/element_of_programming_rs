


fn is_valid(s: &String) -> bool {

    // 00, 000, 01 are not valid but 0 is valid 
    if s.len() == 1 {
        return true
    }

    let char_vec = s.chars().collect::<Vec<char>>();
    if char_vec[0] == '0'{
        return false
    }

    s.parse::<i32>().unwrap() <= 255
}

fn get_valid_ip_addres(s: String) -> Vec<String> {


    let char_vec = s.chars().collect::<Vec<char>>();

    let mut result = Vec::new();
    let mut parts = vec![String::new();4];
    for i in 1..(4.min(s.len())){
        parts[0] = char_vec[..i].iter().collect::<String>();
        if is_valid(&parts[0]){

            for j in 1..(4.min(s.len() -i)){
                parts[1] = char_vec[i..(i+j)].iter().collect::<String>();
                if is_valid(&parts[1]){

                    for k in 1..(4.min(s.len() -i - j)){
                        parts[2] = char_vec[(i+j)..(i+j+k)].iter().collect::<String>();
                        parts[3] = char_vec[(i+j+k)..].iter().collect::<String>();
                        if is_valid(&parts[3]) && is_valid(&parts[2]){
                            let mut valid  = String::new();
                            let mut is_zero = true;
                            for pt in &parts{
                                if is_zero {

                                    valid.push_str(pt);
                                    is_zero = false;
                                } else {
                                    valid.push_str(&(".".to_owned() + pt));
                                }
                            }
                            result.push(valid);
                        }
                    }
                }        
            }
        }
    }
    result
}





#[test]
fn test_valid_ip(){
    let ip = "19216811".to_owned();
    println!("{:?}",get_valid_ip_addres(ip));
}