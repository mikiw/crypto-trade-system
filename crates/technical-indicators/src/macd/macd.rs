use ta::indicators::MovingAverageConvergenceDivergence as Macd;
use ta::{Close, High, Low, Next, DataItem};

// You can create your own data items.
// You may want it for different purposes, e.g.:
// - you data source don't have volume or other fields.
// - you want to skip validation to avoid performance penalty.
struct Item {
    high: f64,
    low: f64,
    close: f64,
}

impl Low for Item {
    fn low(&self) -> f64 {
        self.low
    }
}

impl High for Item {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Close for Item {
    fn close(&self) -> f64 {
        self.close
    }
}


pub fn simulate(left: usize, right: usize) -> usize {
    let mut macd = Macd::new(3, 6, 4).unwrap();

    // some tests
    assert_eq!(round(macd.next(2.0).into()), (0.0, 0.0, 0.0));
    assert_eq!(round(macd.next(3.0).into()), (0.21, 0.09, 0.13));
    assert_eq!(round(macd.next(4.2).into()), (0.52, 0.26, 0.26));
    assert_eq!(round(macd.next(7.0).into()), (1.15, 0.62, 0.54));
    assert_eq!(round(macd.next(6.7).into()), (1.15, 0.83, 0.32));
    assert_eq!(round(macd.next(6.5).into()), (0.94, 0.87, 0.07));

    fn round(nums: (f64, f64, f64)) -> (f64, f64, f64) {
        let n0 = (nums.0 * 100.0).round() / 100.0;
        let n1 = (nums.1 * 100.0).round() / 100.0;
        let n2 = (nums.2 * 100.0).round() / 100.0;
        (n0, n1, n2)
    }

    let dt1 = DataItem::builder()
        .open(1.0)
        .high(1.0)
        .low(1.0)
        .close(1.0)
        .volume(100.0)
        .build()
        .unwrap();

    let macd_val = macd.next(&dt1);
    println!("dt1: {} = {:?}", macd, macd_val);
    println!("dt1: {} = {:2.2} {:2.2} {:2.2}", macd, macd_val.macd, macd_val.histogram, macd_val.signal);

    let dt2 = DataItem::builder()
        .open(1.0)
        .high(2.0)
        .low(1.0)
        .close(2.0)
        .volume(100.0)
        .build()
        .unwrap();

    let macd_val = macd.next(&dt2);
    println!("dt2: {} = {:?}", macd, macd_val);
    println!("dt2: {} = {:2.2} {:2.2} {:2.2}", macd, macd_val.macd, macd_val.histogram, macd_val.signal);

    let dt3 = DataItem::builder()
        .open(30.0)
        .high(30.0)
        .low(1.0)
        .close(1.0)
        .volume(100.0)
        .build()
        .unwrap();

    let macd_val = macd.next(&dt3);
    println!("dt3: {} = {:?}", macd, macd_val);
    println!("dt3: {} = {:2.2} {:2.2} {:2.2}", macd, macd_val.macd, macd_val.histogram, macd_val.signal);

    left + right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = simulate(2, 2);
        assert_eq!(result, 4);
    }
}
