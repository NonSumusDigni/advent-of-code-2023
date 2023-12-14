#[derive(Debug, Default)]
pub struct GearBuilder {
    ratio_factors: Vec<u64>,
    state: GearState,
}

impl GearBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, factor: u64) -> GearState {
        self.ratio_factors.push(factor);
        self.state = match self.ratio_factors.len() {
            1 => GearState::Incomplete,
            2 => GearState::Buildable,
            _ => GearState::Seized,
        };
        self.state
    }

    pub fn get_state(&self) -> GearState {
        self.state
    }

    pub fn build(&self) -> u64 {
        if self.state != GearState::Buildable {
            panic!("Cannot build gear")
        }

        self.ratio_factors[0] * self.ratio_factors[1]
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GearState {
    #[default]
    Incomplete,
    Buildable,
    Seized,
}
