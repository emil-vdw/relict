use std::mem::MaybeUninit;

use crate::gene::{Gene, GeneBytes};

pub type ChromosomeBytes<const CS: usize, const GS: usize> = [GeneBytes<GS>; CS];

pub trait Chromosome<const CS: usize, const GS: usize> {
    fn to_genes(&self) -> ChromosomeBytes<CS, GS>;
    fn from_genes(genes: ChromosomeBytes<CS, GS>) -> Self;
}

impl<T, const CS: usize, const GS: usize> Chromosome<CS, GS> for [T; CS]
where
    T: Gene<GS>,
{
    fn to_genes(&self) -> ChromosomeBytes<CS, GS> {
        let mut gene_bytes: MaybeUninit<ChromosomeBytes<CS, GS>> = MaybeUninit::uninit();
        let first_byte = gene_bytes.as_mut_ptr() as *mut GeneBytes<GS>;

        unsafe {
            self.iter().enumerate().for_each(|(gene_index, gene)| {
                first_byte.add(gene_index).write(gene.to_bytes());
            });
            gene_bytes.assume_init()
        }
    }

    fn from_genes(genes: ChromosomeBytes<CS, GS>) -> Self {
        let mut array_data: MaybeUninit<[T; CS]> = MaybeUninit::uninit();
        let first_byte = array_data.as_mut_ptr() as *mut T;

        unsafe {
            genes
                .iter()
                .enumerate()
                .for_each(|(gene_index, gene_bytes)| {
                    first_byte.add(gene_index).write(T::from_bytes(*gene_bytes));
                });
            array_data.assume_init()
        }
    }
}
