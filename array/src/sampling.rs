use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::ops::Range;

pub fn rand_offline_data(k: usize, arr: &mut Vec<usize>) {
    let mut rng = thread_rng();

    for i in 0..k {
        let r: usize = rng.gen_range(i..arr.len());
        arr.swap(i, r)
    }

    eprintln!("sample offline arr {:?}", arr);
}

pub fn rand_online_sampling(k: usize, mut stream: Range<usize>) -> Vec<usize> {
    let mut rng = thread_rng();

    let mut res = {
        let mut i: usize = 0;

        stream
            .take_while_ref(|_| {
                if i < k {
                    i += 1;
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<usize>>()
    };

    let mut fetched_so_far: usize = k;

    for i in stream {
        fetched_so_far += 1;

        let idx_to_replace = rng.gen_range(0..=fetched_so_far);

        if idx_to_replace < k {
            res[idx_to_replace] = i;
        }
    }
    eprintln!("Online sampling ==> {:?}", res);
    res
}

pub fn random_subset<'a>(n: usize, k: usize) -> Result<Vec<usize>, &'a str> {
    let mut H = std::collections::HashMap::<usize, usize>::new();
    let mut rng = thread_rng();

    for i in 0..k {
        let r: usize = rng.gen_range(i..n);
        let i_mapped = match H.get(&i) {
            None => i,
            Some(ref el) => (*el).clone(),
        };
        let rand_mapped = match H.get(&r) {
            None => r,
            Some(ref el) => (*el).clone(),
        };

        H.insert(i, rand_mapped);
        H.insert(r, i_mapped);
    }
    Ok(H.into_values().collect())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_offline_data() {
        let mut a = vec![3, 4, 1, 2, 5, 6];
        rand_offline_data(2, &mut a);

        assert!(true);
    }

    #[test]
    fn test_online_data() {
        // let mut a = vec![3, 4, 1, 2, 5, 6];
        rand_online_sampling(2, Range { start: 0, end: 10 });

        assert!(true);
    }
}
