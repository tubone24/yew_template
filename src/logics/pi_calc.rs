use bigdecimal::{BigDecimal, ToPrimitive};
use num_bigint::ToBigInt;

pub fn gauss_legendre(n: i64) -> BigDecimal {
    let scale: i64 = 0;
    let prec = n.to_u64().unwrap();
    let mut a: BigDecimal = BigDecimal::new(1.to_bigint().unwrap(), scale).with_prec(prec);
    let mut b: BigDecimal = (BigDecimal::new(1.to_bigint().unwrap(), scale) / BigDecimal::new(2.to_bigint().unwrap(), scale).sqrt().unwrap().with_prec(prec)).with_prec(prec);
    let mut t: BigDecimal = (BigDecimal::new(1.to_bigint().unwrap(), scale) / BigDecimal::new(4.to_bigint().unwrap(), scale)).with_prec(prec);
    let mut p: BigDecimal = BigDecimal::new(1.to_bigint().unwrap(), scale);
    let mut tmp: BigDecimal;

    for i in 0..n {
        tmp = a.clone();
        a = (tmp.clone() + b.clone()) / BigDecimal::new(2.to_bigint().unwrap(), scale);
        b = (tmp.clone() * b.clone()).sqrt().unwrap();
        t = t - (p.clone() * (a.clone() - tmp.clone()) * (a.clone() - tmp.clone()));
        p = BigDecimal::new(2.to_bigint().unwrap(), scale) * p;
        if i % 10000 == 0 {
            log::info!("loop {:?} times now.", i);
        }
    }
    (a.clone() + b.clone()) * (a.clone() + b.clone()) / (BigDecimal::new(4.to_bigint().unwrap(), scale) * t).with_prec(prec)
}