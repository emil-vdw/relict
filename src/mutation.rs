use rand::Rng;
use rand_distr::{Distribution, Geometric};

use crate::chromosome::ChromosomeBytes;

pub trait Mutate<const CS: usize, const GS: usize> {
    fn mutate<R: Rng>(&self, chromosome_data: &mut ChromosomeBytes<CS, GS>, rng: &mut R);
}

#[derive(Debug)]
pub struct InversionMutation {
    pub mutation_rate: f64,
}

impl InversionMutation {
    pub fn new(mutation_rate: f64) -> Self {
        Self { mutation_rate }
    }
}

impl<const CS: usize, const GS: usize> Mutate<CS, GS> for InversionMutation {
    fn mutate<R: Rng>(&self, chromosome_data: &mut ChromosomeBytes<CS, GS>, rng: &mut R) {
        let mutation_dist = Geometric::new(1.0 - self.mutation_rate).unwrap();

        for chromosome_index in 0..chromosome_data.len() {
            for gene_index in 0..chromosome_data[0].len() {
                let num_mutations: usize = mutation_dist.sample(rng) as usize;

                if num_mutations > 0 {
                    (0..num_mutations).for_each(|_| {
                        let mutation_mask = 0b10000000 >> rng.gen_range(0..8);
                        chromosome_data[chromosome_index][gene_index] ^= mutation_mask;
                    })
                }
            }
        }
    }
}
