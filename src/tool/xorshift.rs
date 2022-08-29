use byteorder::{BigEndian, ByteOrder};
use sha2::{Digest, Sha256};

pub struct Shift64 {
    seed: i64,
}

impl Shift64 {
    pub fn new(mut seed: i64) -> Self {
        seed += 1;
        seed ^= seed >> 12;
        seed ^= seed << 25;
        seed ^= seed >> 27;

        return Self {
            seed: seed.wrapping_mul(0x2345F4914F6CFD1E),
        };
    }

    pub fn shift(&mut self) -> i64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        self.seed = self.seed.wrapping_mul(0x2345F4914F6CFD1E);

        return self.seed;
    }

    pub fn f32(&mut self, n: f32) -> f32 {
        let x = (self.shift() as f64) / 9223372036854775808.0;
        return ((x * n as f64).abs() - 1.0) as f32;
    }

    pub fn i32(&mut self, n: i32) -> i32 {
        let x = (self.shift() as f64) / 9223372036854775808.0;
        return ((x * n as f64).abs() - 1.0) as i32;
    }

    pub fn usize(&mut self, n: usize) -> usize {
        let x = (self.shift() as f64) / 9223372036854775808.0;
        return ((x * n as f64).abs() - 1.0) as usize;
    }
}

pub fn get_seed(str: String) -> i64 {
    let result = Sha256::new().chain_update(str).finalize();

    let mut res = BigEndian::read_u64(&result[0..8]);
    res = res.wrapping_mul(BigEndian::read_u64(&result[8..16]));
    res = res.wrapping_mul(BigEndian::read_u64(&result[16..24]));
    res = res.wrapping_mul(BigEndian::read_u64(&result[24..32]));

    return res as i64;
}

#[cfg(test)]
mod tests {
    use super::{get_seed, Shift64};

    #[test]
    fn shift64_i32() {
        let mut shift_a = Shift64::new(0);
        assert_eq!(shift_a.seed, 5036377382042008862);

        assert_eq!(shift_a.shift(), -6399782287330682226);
        assert_eq!(shift_a.shift(), 4297237695309840522);
        assert_eq!(shift_a.shift(), 1075437695011947220);
        assert_eq!(shift_a.shift(), -930821246400571898);

        assert_eq!(shift_a.i32(2048), 1025);
        assert_eq!(shift_a.i32(1024), 798);
        assert_eq!(shift_a.i32(512), 235);
        assert_eq!(shift_a.i32(256), 205);

        assert_eq!(shift_a.i32(128), 108);
        assert_eq!(shift_a.i32(64), 42);
        assert_eq!(shift_a.i32(128), 30);

        assert_eq!(shift_a.i32(256), 185);
        assert_eq!(shift_a.i32(512), 237);
        assert_eq!(shift_a.i32(1024), 974);
        assert_eq!(shift_a.i32(2048), 1385);

        assert_eq!(shift_a.shift(), -5828336445164884370);
        assert_eq!(shift_a.shift(), 1599167847083165552);
        assert_eq!(shift_a.shift(), 6218638069927327200);
        assert_eq!(shift_a.shift(), 8232039552211122488);
    }

    #[test]
    fn shift64_get_seed() {
        let a = get_seed("".to_string());
        let b = get_seed("a".to_string());
        let c = get_seed("bb".to_string());
        let d = get_seed("ccc".to_string());
        let e = get_seed("near".to_string());

        assert_eq!(a, 5698237097726351552);
        assert_eq!(b, 3027204654264679692);
        assert_eq!(c, 331832489265128583);
        assert_eq!(d, 1883749424214749104);
        assert_eq!(e, -4661580130154814320);
    }
}
