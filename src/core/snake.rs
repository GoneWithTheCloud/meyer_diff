#[derive(Copy, Clone)]
pub struct Snake {
    start: isize,
    end: isize,
    diag: isize,
}

impl Snake {
    pub fn new(start: isize, end: isize, diag: isize) -> Snake {
        Snake {
            start: start,
            end: end,
            diag: diag,
        }
    }

    pub fn start(&self) -> isize {
        self.start
    }
    pub fn end(&self) -> isize {
        self.end
    }
    pub fn diag(&self) -> isize {
        self.diag
    }
}
