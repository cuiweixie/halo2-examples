use halo2_proofs::{
    circuit::{Chip, Layouter, SimpleFloorPlanner},
    dev::MockProver,
    pasta::Fp,
    plonk::{Advice, Column, Circuit, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};
use halo2_proofs::circuit::Value;

#[derive(Debug, Clone)]
struct PythagoreanTripleChip {
    config: PythagoreanTripleConfig,
}

#[derive(Debug, Clone)]
struct PythagoreanTripleConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    s: Selector,
}

impl Chip<Fp> for PythagoreanTripleChip {
    type Config = PythagoreanTripleConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl PythagoreanTripleChip {
    fn construct(config: PythagoreanTripleConfig) -> Self {
        Self { config }
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> PythagoreanTripleConfig {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let s = meta.selector();

        meta.create_gate("Pythagorean triple check", |meta| {
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            let s = meta.query_selector(s);

            vec![s * (a.clone() * a.clone() + b.clone() * b.clone() - c.clone() * c)]
        });

        PythagoreanTripleConfig { a, b, c, s }
    }

    fn load(&mut self, _layouter: &mut impl Layouter<Fp>) -> Result<(), Error> {
        Ok(())
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<Fp>,
        a: Fp,
        b: Fp,
        c: Fp,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "load",
            |mut region| {
                let row = 0;
                self.config.s.enable(&mut region, 0)?;
                region.assign_advice(|| "a", self.config.a, row, || Value::known(a))?;
                region.assign_advice(|| "b", self.config.b, row, || Value::known(b))?;
                region.assign_advice(|| "c", self.config.c, row, || Value::known(c))?;
                Ok(())
            },
        )
    }
}

#[derive(Default)]
struct PythagoreanTriple<F> {
    a: F,
    b: F,
    c: F,
}

impl Circuit<Fp> for PythagoreanTriple<Fp> {
    type Config = PythagoreanTripleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        todo!()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        PythagoreanTripleChip::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<Fp>) -> Result<(), Error> {
        let chip = PythagoreanTripleChip::construct(config);
        chip.assign(layouter, self.a, self.b, self.c)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean_triple() {
        let circuit = PythagoreanTriple {
            a: Fp::from(3),
            b: Fp::from(4),
            c: Fp::from(5),
        };

        let prover = MockProver::run(8, &circuit, vec![]).unwrap();

        assert_eq!(prover.verify(), Ok(()));
    }
}
