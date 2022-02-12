#[derive(Debug, Copy, Clone)]
struct ABSqrt {
    a: usize,
    b: usize,
    val: f64,
}

impl ABSqrt {
    fn new(a: usize, b: usize) -> Self {
        Self {
            a,
            b,
            val: (a + b) as f64 * (2 as f64).sqrt(),
        }
    }
}

impl PartialEq for ABSqrt {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for ABSqrt {}

// impl PartialOrd for ABSqrt {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for ABSqrt {

//     fn cmp(&self, other: &Self) -> std::cmp::Ordering{
//         self.val.cmp(&other.val)
//     }
// }

fn gen_first_k_a_b_sqrt(k: usize) -> Vec<f64> {
    let mut res: Vec<ABSqrt> = vec![ABSqrt::new(0, 0)];
    let (mut i, mut j) = (0, 0);

    for _ in 1..k {
        let plus1 = ABSqrt::new(res[i].a + 1, res[i].b);
        let plus_sqrt = ABSqrt::new(res[i].a, res[i].b + 1);

        if plus1.val < plus_sqrt.val {
            res.push(plus1);
        } else {
            res.push(plus_sqrt);
        }
        if res[res.len() - 1] == plus1 {
            i += 1;
        } else {
            j += 1;
        }
    }
    res.into_iter().map(|n| n.val).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_enu_k_seq() {
        let a = gen_first_k_a_b_sqrt(10);

        eprintln!("enumerating sequence == > {:?}", a)
    }
}
