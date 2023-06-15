use std::mem::MaybeUninit;

use rand::Rng;

use crate::{chromosome::ChromosomeBytes, gene::GeneBytes};

pub trait Crossover<const CS: usize, const GS: usize> {
    fn crossover<R>(
        &self,
        mother_chromosome: ChromosomeBytes<CS, GS>,
        father_chromosome: ChromosomeBytes<CS, GS>,
        rng: &mut R,
    ) -> ChromosomeBytes<CS, GS>
    where
        R: Rng;
}

pub struct UniformCrossover {}

impl<const CS: usize, const GS: usize> Crossover<CS, GS> for UniformCrossover {
    fn crossover<R>(
        &self,
        mother_chromosome: ChromosomeBytes<CS, GS>,
        father_chromosome: ChromosomeBytes<CS, GS>,
        rng: &mut R,
    ) -> ChromosomeBytes<CS, GS>
    where
        R: Rng,
    {
        let mut offspring_chromosome: MaybeUninit<ChromosomeBytes<CS, GS>> = MaybeUninit::uninit();
        let chromosome_start = offspring_chromosome.as_mut_ptr() as *mut GeneBytes<GS>;

        // Use unsafe to avoid initializing twice and allocating into Vec.
        unsafe {
            for chromosome_index in 0..CS {
                let mut offspring_gene: MaybeUninit<[u8; GS]> = MaybeUninit::uninit();
                let gene_start = offspring_gene.as_mut_ptr() as *mut u8;

                for gene_index in 0..GS {
                    let crossover_bitmask: u8 = rng.gen();
                    let offspring_gene = (mother_chromosome[chromosome_index][gene_index]
                        & crossover_bitmask)
                        + (father_chromosome[chromosome_index][gene_index] & !crossover_bitmask);
                    gene_start.add(gene_index).write(offspring_gene);
                }
                chromosome_start
                    .add(chromosome_index)
                    .write(offspring_gene.assume_init());
            }
            offspring_chromosome.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Error, RngCore};

    use crate::gene::Gene;

    use super::*;

    struct MockRng {
        values: Vec<u64>,
        index: usize,
    }

    impl MockRng {
        pub fn new(values: Vec<u64>) -> Self {
            MockRng { values, index: 0 }
        }
    }

    impl RngCore for MockRng {
        #[inline]
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        #[inline]
        fn next_u64(&mut self) -> u64 {
            let result = self.values[self.index];
            self.index = (self.index + 1) % self.values.len();
            result
        }

        #[inline]
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let mut left = dest;
            while left.len() >= 8 {
                let (l, r) = { left }.split_at_mut(8);
                left = r;
                let chunk: [u8; 8] = self.next_u64().to_le_bytes();
                l.copy_from_slice(&chunk);
            }
            let n = left.len();
            if n > 4 {
                let chunk: [u8; 8] = self.next_u64().to_le_bytes();
                left.copy_from_slice(&chunk[..n]);
            } else if n > 0 {
                let chunk: [u8; 4] = self.next_u32().to_le_bytes();
                left.copy_from_slice(&chunk[..n]);
            }
        }

        #[inline]
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    #[test]
    fn u32_uniform_crossover() {
        let mother_gene: u32 = 0b11101001010010110010110110010110;
        let father_gene: u32 = 0b00110101100101010101110010010101;
        let mut rng = MockRng::new(vec![
            0b00001111_u64, // First 4 bits from the father's gene and the last 4 from the mother's
            0b10101010_u64,
            0b00000000_u64,
            0b11111111_u64,
        ]);

        let strategy = UniformCrossover {};

        let offspring_gene = u32::from_bytes(
            strategy.crossover([mother_gene.to_bytes()], [father_gene.to_bytes()], &mut rng)[0],
        );

        assert_eq!(offspring_gene, 0b00111001000111110101110010010110_u32);
    }
}
