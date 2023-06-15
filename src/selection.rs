use rand::Rng;
use rand_distr::{Distribution, Geometric};

use crate::{chromosome::Chromosome, fitness::Fitness};

pub enum TournamentSize {
    Fixed(usize),
    Max,
}

pub enum SelectionCount {
    Ratio(f64),
    Fixed(usize),
}

pub struct TournamentSelection {
    // Tournament size
    pub tournament_size: TournamentSize,
    pub probability: f64,
    pub remove_winners: bool,
    pub selection_count: SelectionCount,
}

impl TournamentSelection {
    pub fn new(
        tournament_size: TournamentSize,
        probability: f64,
        remove_winners: bool,
        selection_count: SelectionCount,
    ) -> Self {
        Self {
            tournament_size,
            probability,
            remove_winners,
            selection_count,
        }
    }

    // Choose `self.tournament_size` competitors at random from `population`.
    fn choose_competitors<T, R>(&self, population: &mut Vec<T>, rng: &mut R) -> Vec<usize>
    where
        T: Fitness,
        R: Rng,
    {
        let tournament_size = match self.tournament_size {
            TournamentSize::Fixed(size) => size.min(population.len()),
            TournamentSize::Max => population.len(),
        };

        let mut competitors: Vec<usize> =
            rand::seq::index::sample(rng, population.len(), tournament_size)
                .iter()
                .collect();

        competitors.sort_by(|competitor_index, challenger_index| {
            population[*challenger_index]
                .fitness()
                .partial_cmp(&population[*competitor_index].fitness())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        competitors
    }
}

pub trait Selection<const CS: usize, const GS: usize> {
    fn select<T, R>(&self, population: Vec<T>, rng: &mut R) -> Vec<T>
    where
        T: Chromosome<CS, GS> + Fitness + Clone,
        R: Rng;
}

impl<const CS: usize, const GS: usize> Selection<CS, GS> for TournamentSelection {
    fn select<T, R>(&self, mut population: Vec<T>, rng: &mut R) -> Vec<T>
    where
        T: Chromosome<CS, GS> + Fitness + Clone,
        R: Rng,
    {
        let selected_population_size = match self.selection_count {
            SelectionCount::Fixed(size) => size,
            SelectionCount::Ratio(population_ratio) => {
                (population.len() as f64 * population_ratio).ceil() as usize
            }
        };
        let winner_dist = Geometric::new(self.probability).unwrap();
        let mut winners: Vec<T> = Vec::with_capacity(selected_population_size);

        while winners.len() < selected_population_size || population.is_empty() {
            let competitors = self.choose_competitors(&mut population, rng);
            let winner_index: usize =
                competitors[(winner_dist.sample(rng) as usize).min(competitors.len() - 1)];

            if self.remove_winners {
                winners.push(population.swap_remove(winner_index));
            } else {
                winners.push(population[winner_index].clone());
            }
        }

        winners
    }
}
