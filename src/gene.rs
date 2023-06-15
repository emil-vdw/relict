use std::mem;

pub trait Gene<const GS: usize> {
    fn from_bytes(bytes: [u8; GS]) -> Self;
    fn to_bytes(&self) -> [u8; GS];

    // mutate
    // crossover
}

pub type GeneBytes<const GS: usize> = [u8; GS];

macro_rules! impl_gene {
    ($($t:ty),*) => {
        $(
            impl Gene<{ mem::size_of::<$t>() }> for $t {
                #[inline]

				fn to_bytes(&self) -> GeneBytes<{ mem::size_of::<$t>() }> {
					self.to_be_bytes()
				}

				fn from_bytes(bytes: GeneBytes<{ mem::size_of::<$t>() }>) -> Self {
					Self::from_be_bytes(bytes)
				}
            }
        )*
    }
}

impl_gene!(u8, u16, u32, u64, f32, f64, usize, i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_gene_bytes() {
        let gene = u32::from_str_radix("00000111010110111100110100010101", 2).unwrap();

        assert_eq!(
            gene.to_bytes(),
            [
                u8::from_str_radix("00000111", 2).unwrap(),
                u8::from_str_radix("01011011", 2).unwrap(),
                u8::from_str_radix("11001101", 2).unwrap(),
                u8::from_str_radix("00010101", 2).unwrap(),
            ]
        );
    }
}
