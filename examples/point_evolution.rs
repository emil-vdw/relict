use std::f64::consts::E;

use rand::Rng;
use relict::{
    chromosome::Chromosome,
    crossover::{Crossover, UniformCrossover},
    fitness::Fitness,
    gene::Gene,
    mutation::{InversionMutation, Mutate},
    selection::{Selection, SelectionCount, TournamentSelection, TournamentSize},
};

extern crate relict;

#[derive(Debug, Clone)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        Point::new(rng.gen(), rng.gen())
    }
}

impl Fitness for Point {
    fn fitness(&self) -> f64 {
        let r = ((self
            .x
            .abs_diff(150)
            .saturating_pow(2)
            .saturating_add(self.y.abs_diff(29).saturating_pow(2))) as f64)
            .sqrt();
        E.powf(-0.15 * r) * r.cos()
    }
}

impl Chromosome<2, 1> for Point {
    fn to_genes(&self) -> relict::chromosome::ChromosomeBytes<2, 1> {
        [self.x, self.y].to_genes()
    }

    fn from_genes(genes: relict::chromosome::ChromosomeBytes<2, 1>) -> Self {
        Self {
            x: <u8 as Gene<1>>::from_bytes(genes[0]),
            y: <u8 as Gene<1>>::from_bytes(genes[1]),
        }
    }
}

fn point_life(generations: usize, mut population: Vec<Point>) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let selection_strategy = TournamentSelection::new(
        TournamentSize::Fixed(5),
        0.9,
        true,
        SelectionCount::Fixed(50),
    );
    let crossover_strategy = UniformCrossover {};
    let mut mutation_strategy = InversionMutation::new(0.5);

    for generation in 0..generations {
        if population.iter().any(|point| point.fitness() == 1.0) {
            println!("Found at gen {generation}!");
            print_fitness(&population);
            return population;
        } else if generation % 10 == 0 {
            println!("Population:");
            println!("\tGeneration:{generation}");
            println!("\tSize:{}", population.len());
            print_fitness(&population);
        }

        // Anneal the mutation rate.
        mutation_strategy.mutation_rate =
            0.9_f64.min((1.0 - (generation as f64 / generations as f64)).max(0.1));

        population = selection_strategy.select(population, &mut rng);

        for _breeding_cycles in 0..(2 * population.len()) {
            let partners = rand::seq::index::sample(&mut rng, population.len(), 2);

            let mut offspring_chromosome = crossover_strategy.crossover(
                population[partners.index(0)].to_genes(),
                population[partners.index(1)].to_genes(),
                &mut rng,
            );
            mutation_strategy.mutate(&mut offspring_chromosome, &mut rng);
            population.push(Point::from_genes(offspring_chromosome));
        }
        for _random_point in 0..2 {
            population.push(Point::random(&mut rng));
        }
    }

    println!("Population:");
    println!("\tSize:{}", population.len());
    print_fitness(&population);
    population
}

fn print_fitness(population: &[Point]) {
    println!("Fitness:");
    population.iter().for_each(|point| {
        println!("{:?} -> {}", point, point.fitness());
    });
    println!();
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut pop = (0..200).map(|_| Point::random(&mut rng)).collect();

    pop = point_life(1000, pop);

    println!("{:?}", pop,);
}
