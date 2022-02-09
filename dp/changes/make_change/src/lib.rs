use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec;



fn make_change_bfs(amount:usize, coins: Vec<usize>) -> Option<Vec<usize>>{

    let mut hm = HashMap::new();
    let mut em = vec![];
    hm.insert(0, em);
    let mut deque = VecDeque::new();
    deque.push_back(0);

    while !deque.is_empty(){
        println!("Coming here");
        let paid = deque.pop_front().unwrap();
        let solutions = hm.get(&paid).map(|n| n.clone()).unwrap();
        if paid == amount{
            
            return Some(solutions.iter().map(|n| *n).collect())
        } else {

            for c in &coins{
                let next_paid = c + paid;
                if next_paid > amount{
                    continue
                }
                if !hm.contains_key(&next_paid){
                    let mut new = solutions.iter().map(|n| *n).collect::<Vec<usize>>();
                    new.push(*c);
                    // new.extend_from_slice(&solutions);


                    hm.insert(next_paid, new);
                    deque.push_back(next_paid);
                }
            }
        }
        
    }
    None
}




fn make_change(amount: isize, coins: &Vec<usize>) -> Option<Vec<usize>>{

    let mut optional_change: Option<Vec<usize>> = None;
    if amount == 0 {
        // when amount is 0 that means we have to give 0 transaction
        return Some(Vec::new())
    }
    if amount < 0 {
        //  whnen amount goes less than 0 we cannot make move
        return None
    }
    for coin in coins {
        let partial_result = make_change(amount - *coin as isize, coins);
        if partial_result.is_none(){
            // actually with this coin we will be in negative sate
            continue
        }
        let mut candidate = partial_result.unwrap();
        candidate.push(*coin);
        if optional_change.is_none() || optional_change.as_ref().unwrap().len() > candidate.len(){
            optional_change.replace(candidate);
        }

    }
    optional_change
}


fn make_change_bottom_up(amount: usize, coins: Vec<usize>) -> Vec<Option<Vec<usize>>>{

    let mut solutions: Vec<Option<Vec<usize>>> = vec![None; amount + 1];
    solutions[0] = Some(vec![]);

    let mut paid = 0;
    while paid < amount {

        if solutions[paid].is_some(){
            //  if the coint does not add up then we cannot pay
            for coin in &coins {

            let next_pay = paid + coin;
            if next_pay > amount{
                //  if it is greater than our amount then just move
                continue
            }
            if solutions[next_pay].is_none() || 
                    //  we came to this next pay by coming from paid
                    solutions[next_pay].as_ref().unwrap().len() > 
                    // if next pay len is greater than adding one to paid len then we can add 1 to apid len to change the next_len 
                    solutions[paid].as_ref().unwrap().len() + 1{
                        
                        let paid_sol = solutions[paid].as_ref().unwrap();
                        let mut  new_pay = vec![];
                        for i in paid_sol{
                            new_pay.push(*i);
                        }
                        new_pay.push(*coin);
                        solutions[next_pay] = Some(new_pay);
            }
            
            }
        }
        paid += 1;
        
    }
    solutions
}


fn _count_way_to_pay_combination(amount: isize, 
                                coins: &Vec<usize>,
                                coin_idx: usize,
                                memo: &mut HashMap<(isize,usize),isize>,
                               ) -> isize 
{

    if amount == 0{
        return 1
    }
    if amount < 0 || coin_idx == coins.len(){ return 0}

    if memo.contains_key(&(amount, coin_idx)){
        return *memo.get(&(amount, coin_idx)).as_ref().unwrap().to_owned()
    } else {

        let mut num_ways = 0;
        let coin  = coins[coin_idx];
        for repeats in 0..(amount as usize / coin + 1){

            let payment = repeats * coin;
            num_ways +=  _count_way_to_pay_combination(amount - payment as isize, coins, coin_idx +1, memo);

        }
        memo.insert((amount,coin_idx), num_ways);
        return  num_ways
    }

}


fn count_way_to_pay_combination(amount: isize, coins: &Vec<usize>) -> isize {


    let mut hm = HashMap::new();
    _count_way_to_pay_combination(amount, &coins, 0, &mut hm)

}

fn __count_way2(amount: isize, coins: &Vec<usize>) -> Option<Vec<isize>>{
    if amount == 0{
        return Some(vec![]);
    }
    if amount < 0 {
        return None ;
    }
    let mut rv = vec![];
    for c in coins{
        let partial_res = __count_way2(amount - *c as isize, coins);
        if partial_res.is_none(){
                continue
        }
        rv = partial_res.unwrap();
        rv.push(*c as isize);



    }
    Some(rv)
}

fn _count_way2(amount: isize, coins: &Vec<usize>) -> Option<Vec<Vec<isize>>>{

    // if amount == 0{
    //     return Some(vec![]);
    // }
    // if amount < 0 {
    //     return None ;
    // }

    let mut num_ways = vec![];
    let mut r = vec![];

    for i in &*coins{
            let partial_res = _count_way2(amount - *i as isize, coins);
            if partial_res.is_none(){
                continue
            }
            r.push(*i);
            for j in &mut partial_res.unwrap(){
                eprintln!("j --> {:?}", j);
                j.push(*i as isize);
                num_ways.push(j.iter().map(|n| *n).collect());
            }
            // n.push(*i as isize);
    }
        
    return Some(num_ways)

    // if memo.contains_key(&amount){
    //     return *(memo.get(&amount).take().unwrap());
    // } else {
    //     let mut num_ways = 0;
    //     for i in &*coins{
    //         num_ways += _count_way(amount - *i as isize, coins, memo);
            
    //     }
    //     memo.insert(amount, num_ways);
    //     return num_ways
    // }
}


fn _count_way(amount: isize, coins: &Vec<usize>, memo: &mut HashMap<isize,isize>) -> isize{

    if amount == 0{
        return 1;
    }
    if amount < 0 {
        return 0 ;
    }

    if memo.contains_key(&amount){
        return *(memo.get(&amount).take().unwrap());
    } else {
        let mut num_ways = 0;
        for i in &*coins{
            num_ways += _count_way(amount - *i as isize, coins, memo);
            
        }
        memo.insert(amount, num_ways);
        return num_ways
    }
}

fn count_ways_to_pay(amount: isize, coins: Vec<usize>) -> isize {

    let mut hm = HashMap::new();
    _count_way(amount, &coins, &mut hm)
    
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = make_change(9, &vec![5,2,1,4]);
        assert_eq!(result, Some(vec![4,5]));
        println!("{:?}", result);
    }

    #[test]
    fn make_change_bottom_up_test() {
        let result = make_change_bottom_up(9, vec![5,2,1,4]);
        assert_eq!(result[9], Some(vec![4,5]));
        println!("{:?}", result[9]);
    }

    #[test]
    fn make_change_bfs_test() {
        let result = make_change_bfs(9, vec![5,2,1,4]);
        assert_eq!(result, Some(vec![5,4]));
        println!("{:?}", result);
    }


    #[test]
    fn count_way_test() {
        let result = count_ways_to_pay(3, vec![1,2]);
        assert_eq!(result, 3);
        let result1 = count_ways_to_pay(6, vec![1,2,5]);
        println!("count way --> {:?}", result1);


        eprintln!("_count_way2 {:?}", _count_way2(3, &vec![1,2]));
    }

    #[test]
    fn count_way_test_combination() {
        let result1 = count_way_to_pay_combination(6, &vec![1,2,5]);
        let result = count_way_to_pay_combination(3, &vec![1,2]);
        assert_eq!(result, 2);
        assert_eq!(result1, 5);
        println!("{:?}", result);


        // eprintln!("_count_way2 {:?}", _count_way2(3, &vec![1,2]));
    }

    
}
