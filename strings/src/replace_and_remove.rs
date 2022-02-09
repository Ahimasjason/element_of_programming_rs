




fn r_and_r(list : &mut Vec<char>, size:usize) ->usize {

    let (mut write_index, mut a_count) = (0,0);

    for i in 0..size{
        if list[i] != 'b'{
            list[write_index] = list[i];
            write_index += 1;
        }
        if list[i] == 'a' {
            a_count += 1;
        }
    }

    println!("curr_idx : {}, write_idx: {}, final_size : {}",write_index - 1 , write_index + a_count - 1, write_index + 1);
    let mut curr_idx = write_index - 1;
    write_index += a_count - 1;
    if write_index + 1 > list.len(){
        return write_index + 1
    }
    while curr_idx >= 0 {

        if list[curr_idx] == 'a' {
            
            println!("{:?} {:?} {:?}",list, write_index,list[(write_index -1)..(write_index + 1)].to_vec());
            &mut list[(write_index -1)..(write_index + 1) ].clone_from_slice(&['d','d']);
            
            if (write_index as isize - 2) >= 0 {
                write_index -= 2;
                
            } else {
                break
            }
            

        } else {
            if !(write_index>0){
                break
            }
            list[write_index] = list[curr_idx];
            write_index -= 1;
        }


        curr_idx -= 1;
        
    }

    write_index + 1
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_r_and_r(){

        let mut list = vec!['a','c','b','a','b','f'];
        let mut len = list.len();
        r_and_r(&mut list, len);
        println!(
            "list is {:?}",
            list
        );

        assert!(!list.contains(&'a'));
        assert!(!list.contains(&'b'));

        let mut list = vec!['a','a','a'];
        len = list.len();
        r_and_r(&mut list, len);
        println!(
            "list is {:?}",
            list
        );
    }
}