use super::delta_type::DeltaType;

#[derive(Clone, Copy,Debug)]
pub struct Change {
    dt: DeltaType,
    start_original: isize,
    end_original: isize,
    start_revised: isize,
    end_revised: isize,
}

impl Change {
    pub fn new(
        dt: DeltaType,
        start_original: isize,
        end_original: isize,
        start_revised: isize,
        end_revised: isize,
    ) -> Change {
        Change {
            dt: dt,
            start_original: start_original,
            end_original: end_original,
            start_revised: start_revised,
            end_revised: end_revised,
        }
    }

    pub fn with_end_original(&self, end_original: isize) -> Self {
        Change {
            dt: self.dt,
            start_original: self.start_original,
            end_original: end_original,
            start_revised: self.start_revised,
            end_revised: self.end_revised,
        }
    }

    pub fn with_end_revised(&self, end_revised: isize) -> Self {
        Change {
            dt: self.dt,
            start_original: self.start_original,
            end_original: self.end_original,
            start_revised: self.start_revised,
            end_revised: end_revised,
        }
    }

    pub fn delta_type(&self) -> DeltaType {
        self.dt
    }
    pub fn start_original(&self) -> isize {
        self.start_original
    }
    pub fn end_original(&self) -> isize {
        self.end_original
    }
    pub fn start_revised(&self) -> isize {
        self.start_revised
    }
    pub fn end_revised(&self) -> isize {
        self.end_revised
    }
}
