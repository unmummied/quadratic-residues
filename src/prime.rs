pub type Num = u32;

pub trait Prime<T> {
    fn is_prime(&self) -> bool;
    fn is_square(&self) -> bool;
    fn floor_sqrt(&self) -> Self;
    fn prime_factorize(&self) -> Result<Vec<(T, T)>, String>;
    fn mod_pow(&self, exp: Self, modulo: Self) -> Result<T, String>;
}
pub trait QR<T> {
    fn qr_p(&self) -> Result<Vec<T>, String>;
    fn qr_2pow(&self) -> Result<Vec<T>, String>;
    fn qr_odd_pp(&self) -> Result<Vec<T>, String>;
    fn qr_pp(&self) -> Result<Vec<T>, String>;
    fn fast_ord_qr_pp(&self) -> Result<T, String>;
    fn fast_ord_qr_pp_v2(&self) -> Result<T, String>;
}

impl Prime<Num> for Num {
    fn is_prime(&self) -> bool {
        if *self < 3 {
            if *self == 2 {
                return true;
            }
            return false;
        }
        if self & 1 == 0 {
            return false;
        }
        let mut odd = 3;
        while odd * odd <= *self {
            if self % odd == 0 {
                return false;
            }
            odd += 2;
        }
        true
    }
    fn is_square(&self) -> bool {
        let mut sqrt = 0;
        while sqrt * sqrt <= *self {
            if sqrt * sqrt == *self {
                return true;
            }
            sqrt += 1;
        }
        false
    }
    fn floor_sqrt(&self) -> Self {
        let mut i = 0;
        while i * i <= *self {
            if i * i == *self {
                return i;
            }
            i += 1;
        }
        i - 1
    }
    fn prime_factorize(&self) -> Result<Vec<(Self, Self)>, String> {
        if self.is_prime() {
            return Ok(vec![(*self, 1)]);
        }
        if *self == 0 {
            return Err("not a positive integer...".to_string());
        }
        if *self == 1 {
            return Ok(vec![]);
        }
        let mut res = Vec::new();
        let mut n = *self;
        let mut p = 2;
        while p * p <= n {
            let mut e = 0;
            if n % p == 0 {
                while n % p == 0 {
                    n /= p;
                    e += 1;
                }
                res.push((p, e));
            }
            p += 1;
        }
        if n != 1 {
            res.push((n, 1));
        }
        Ok(res)
    }
    fn mod_pow(&self, exp: Self, modulo: Self) -> Result<Self, String> {
        if modulo < 1 {
            return Err("not a prime...".to_string());
        }
        Ok(match (self, exp, modulo) {
            (_, _, 1) => 0,
            (_, 0, _) => 1,
            (0, _, _) => 0,
            _ => {
                let mut res = self % modulo;
                for _ in 1..exp {
                    res *= self;
                    res %= modulo;
                }
                res
            }
        })
    }
}
impl QR<Num> for Num {
    fn qr_p(&self) -> Result<Vec<Self>, String> {
        if !self.is_prime() {
            return Err("not a prime...".to_string());
        }
        let mut qr = vec![1];
        for i in 2..*self {
            if i.mod_pow((self - 1) / 2, *self)? == 1 {
                qr.push(i);
            }
        }
        Ok(qr)
    }
    fn qr_2pow(&self) -> Result<Vec<Num>, String> {
        let pf = self.prime_factorize()?;
        if pf.len() != 1 {
            return Err("not a prime pow...".to_string());
        }
        if pf.last().unwrap().0 != 2 {
            return Err("not a 2-pow".to_string());
        }
        let exp = pf.last().unwrap().1;
        let mut qr_2p = vec![1];
        for e in 1..exp {
            let mut new = Vec::new();
            if e & 1 == 0 {
                new.push(1 << e);
            }
            new.extend(
                qr_2p
                    .iter()
                    .filter(|&n| *n != 1 << (2 * (((e + 1) >> 1) - 1)))
                    .map(|qr| *qr + (1 << e)),
            );
            qr_2p.extend(new);
        }
        Ok(qr_2p)
    }
    fn qr_odd_pp(&self) -> Result<Vec<Self>, String> {
        let pf = self.prime_factorize()?;
        if pf.len() != 1 {
            return Err("not a prime pow...".to_string());
        }
        let (prime, exp) = pf.last().unwrap();
        let qr_p = prime.qr_p()?;
        let mut qr_pp = qr_p.clone();
        let dangerous = qr_p
            .into_iter()
            .filter(|&qr| !qr.is_square())
            .collect::<Vec<_>>();
        let mut res = qr_pp.clone();
        for e in 1..*exp {
            for i in 1..*prime {
                if e & 1 == 0 && (i.is_square() || dangerous.contains(&i)) {
                    res.push(i * prime.pow(e as _));
                }
                res.extend(qr_pp.iter().map(|qr| qr + i * prime.pow(e as _)));
            }
            qr_pp = res.clone();
        }
        Ok(res)
    }
    fn qr_pp(&self) -> Result<Vec<Num>, String> {
        let pf = self.prime_factorize()?;
        if pf.len() != 1 {
            return Err("not a prime pow".to_string());
        }
        if pf.last().unwrap().0 == 2 {
            return self.qr_2pow();
        }
        self.qr_odd_pp()
    }
    fn fast_ord_qr_pp(&self) -> Result<Self, String> {
        let factor = self.prime_factorize()?;
        if factor.len() != 1 {
            return Err("not a prime pow...".to_string());
        }
        let (prime, exp) = factor.last().unwrap();
        let coef = prime
            .qr_p()?
            .into_iter()
            .filter(|&qr| !qr.is_square())
            .count() as Self
            + (prime - 1).floor_sqrt();
        let mut res = (prime - 1) / 2;
        for e in 2..exp + 1 {
            res *= prime;
            if e & 1 == 1 {
                res += coef;
            }
        }
        Ok(res)
    }
    fn fast_ord_qr_pp_v2(&self) -> Result<Self, String> {
        let factor = self.prime_factorize()?;
        if factor.len() != 1 {
            return Err("not a prime pow...".to_string());
        }
        let (prime, exp) = factor.last().unwrap();
        let coef = prime
            .qr_p()?
            .into_iter()
            .filter(|&qr| !qr.is_square())
            .count() as Self
            + (prime - 1).floor_sqrt();
        let mut res = prime.pow(exp - 1) * (prime - 1) / 2;
        res += if exp & 1 == 1 {
            coef * (prime.pow(exp - 1) / (prime * prime - 1))
        } else {
            coef * (prime.pow(*exp) - 1) / (prime * prime - 1) / prime
        };
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prime_factorize() {
        assert_eq!(vec![(2, 1)], 2.prime_factorize().unwrap());
        assert_eq!(vec![(3, 1)], 3.prime_factorize().unwrap());
        assert_eq!(vec![(97, 1)], 97.prime_factorize().unwrap());
        assert_eq!(
            vec![(2, 1), (5, 1), (7, 3), (13, 1)],
            44590.prime_factorize().unwrap(),
        );
        assert!(0.prime_factorize().is_err());
        assert!(1.prime_factorize().unwrap().is_empty());
    }
    #[test]
    fn test_qr_p() {
        assert_eq!(vec![1], 2.qr_p().unwrap());
        assert_eq!(vec![1], 3.qr_p().unwrap());
        assert_eq!(vec![1, 4], 5.qr_p().unwrap());
        assert_eq!(vec![1, 2, 4], 7.qr_p().unwrap());
        assert_eq!(vec![1, 3, 4, 5, 9], 11.qr_p().unwrap());
        assert_eq!(
            vec![
                1, 2, 3, 4, 6, 8, 9, 11, 12, 16, 18, 22, 24, 25, 27, 31, 32, 33, 35, 36, 43, 44,
                47, 48, 49, 50, 53, 54, 61, 62, 64, 65, 66, 70, 72, 73, 75, 79, 81, 85, 86, 88, 89,
                91, 93, 94, 95, 96,
            ],
            97.qr_p().unwrap(),
        );
        assert!(0.qr_p().is_err());
        assert!(1.qr_p().is_err());
        assert!(100.qr_p().is_err());
        assert!(4.qr_p().is_err());
    }
    #[test]
    fn test_qr_pp() {
        assert_eq!(vec![1], 2.qr_odd_pp().unwrap());
        assert_eq!(vec![1], 3.qr_odd_pp().unwrap());
        assert_eq!(vec![1, 4], 5.qr_odd_pp().unwrap());
        assert_eq!(vec![1, 2, 4], 7.qr_odd_pp().unwrap());
        assert_eq!(vec![1, 3, 4, 5, 9], 11.qr_odd_pp().unwrap());
        assert_eq!(
            vec![
                1, 2, 3, 4, 6, 8, 9, 11, 12, 16, 18, 22, 24, 25, 27, 31, 32, 33, 35, 36, 43, 44,
                47, 48, 49, 50, 53, 54, 61, 62, 64, 65, 66, 70, 72, 73, 75, 79, 81, 85, 86, 88, 89,
                91, 93, 94, 95, 96,
            ],
            97.qr_odd_pp().unwrap(),
        );
        assert!(0.qr_odd_pp().is_err());
        assert!(1.qr_odd_pp().is_err());
        assert!(100.qr_odd_pp().is_err());
        assert_eq!(vec![1, 4, 7], 9.qr_odd_pp().unwrap());
        assert_eq!(
            vec![
                1, 2, 3, 4, 6, 8, 9, 12, 13, 16, 18, 24, 25, 26, 27, 29, 31, 32, 35, 36, 39, 41,
                47, 48, 49, 50, 52, 54, 55, 58, 59, 62, 64, 70, 71, 72, 73, 75, 77, 78, 81, 82, 85,
                87, 93, 94, 95, 96, 98, 100, 101, 104, 105, 108, 110, 116, 117, 118, 119, 121, 123,
                124, 127, 128, 131, 133, 139, 140, 141, 142, 144, 146, 147, 150, 151, 154, 156,
                162, 163, 164, 165, 167, 169, 170, 173, 174, 177, 179, 185, 186, 187, 188, 190,
                192, 193, 196, 197, 200, 202, 208, 209, 210, 211, 213, 215, 216, 219, 220, 223,
                225, 231, 232, 233, 234, 236, 238, 239, 242, 243, 246, 248, 254, 255, 256, 257,
                259, 261, 262, 265, 266, 269, 271, 277, 278, 279, 280, 282, 284, 285, 288, 289,
                292, 294, 300, 301, 302, 303, 305, 307, 308, 311, 312, 315, 317, 323, 324, 325,
                326, 328, 330, 331, 334, 335, 338, 340, 346, 347, 348, 349, 351, 353, 354, 357,
                358, 361, 363, 369, 370, 371, 372, 374, 376, 377, 380, 381, 384, 386, 392, 393,
                394, 395, 397, 399, 400, 403, 404, 407, 409, 415, 416, 417, 418, 420, 422, 423,
                426, 427, 430, 432, 438, 439, 440, 441, 443, 445, 446, 449, 450, 453, 455, 461,
                462, 463, 464, 466, 468, 469, 472, 473, 476, 478, 484, 485, 486, 487, 489, 491,
                492, 495, 496, 499, 501, 507, 508, 509, 510, 512, 514, 515, 518, 519, 522, 524,
            ],
            529.qr_odd_pp().unwrap(), // 529 = 23 * 23
        );
    }
    #[test]
    fn test_qr_2pow() {
        assert_eq!(vec![1], 2.qr_2pow().unwrap());
        assert_eq!(vec![1], 4.qr_2pow().unwrap());
        assert_eq!(vec![1, 4], 8.qr_2pow().unwrap());
        assert_eq!(vec![1, 4, 9], 16.qr_2pow().unwrap());
        assert_eq!(vec![1, 4, 9, 16, 17, 25], 32.qr_2pow().unwrap());
        assert_eq!(
            vec![1, 4, 9, 16, 17, 25, 33, 36, 41, 49, 57],
            64.qr_2pow().unwrap(),
        );
        assert_eq!(
            vec![
                1, 4, 9, 16, 17, 25, 33, 36, 41, 49, 57, 64, 65, 68, 73, 81, 89, 97, 100, 105, 113,
                121,
            ],
            128.qr_2pow().unwrap(),
        );
        assert_eq!(
            vec![
                1, 4, 9, 16, 17, 25, 33, 36, 41, 49, 57, 64, 65, 68, 73, 81, 89, 97, 100, 105, 113,
                121, 129, 132, 137, 144, 145, 153, 161, 164, 169, 177, 185, 193, 196, 201, 209,
                217, 225, 228, 233, 241, 249,
            ],
            256.qr_2pow().unwrap(),
        );
        assert_eq!(
            vec![
                1, 4, 9, 16, 17, 25, 33, 36, 41, 49, 57, 64, 65, 68, 73, 81, 89, 97, 100, 105, 113,
                121, 129, 132, 137, 144, 145, 153, 161, 164, 169, 177, 185, 193, 196, 201, 209,
                217, 225, 228, 233, 241, 249, 256, 257, 260, 265, 272, 273, 281, 289, 292, 297,
                305, 313, 321, 324, 329, 337, 345, 353, 356, 361, 369, 377, 385, 388, 393, 400,
                401, 409, 417, 420, 425, 433, 441, 449, 452, 457, 465, 473, 481, 484, 489, 497,
                505,
            ],
            512.qr_2pow().unwrap(),
        );
        assert_eq!(
            vec![
                1, 4, 9, 16, 17, 25, 33, 36, 41, 49, 57, 64, 65, 68, 73, 81, 89, 97, 100, 105, 113,
                121, 129, 132, 137, 144, 145, 153, 161, 164, 169, 177, 185, 193, 196, 201, 209,
                217, 225, 228, 233, 241, 249, 256, 257, 260, 265, 272, 273, 281, 289, 292, 297,
                305, 313, 321, 324, 329, 337, 345, 353, 356, 361, 369, 377, 385, 388, 393, 400,
                401, 409, 417, 420, 425, 433, 441, 449, 452, 457, 465, 473, 481, 484, 489, 497,
                505, 513, 516, 521, 528, 529, 537, 545, 548, 553, 561, 569, 576, 577, 580, 585,
                593, 601, 609, 612, 617, 625, 633, 641, 644, 649, 656, 657, 665, 673, 676, 681,
                689, 697, 705, 708, 713, 721, 729, 737, 740, 745, 753, 761, 769, 772, 777, 784,
                785, 793, 801, 804, 809, 817, 825, 833, 836, 841, 849, 857, 865, 868, 873, 881,
                889, 897, 900, 905, 912, 913, 921, 929, 932, 937, 945, 953, 961, 964, 969, 977,
                985, 993, 996, 1001, 1009, 1017,
            ],
            1024.qr_2pow().unwrap(),
        );
    }
}
