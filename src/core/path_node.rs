use std::rc::Rc;

pub struct PathNode {
    i: isize,
    j: isize,
    snake: bool,
    bootstrap: bool,
    prev: Option<Rc<PathNode>>,
}

impl PathNode {
    pub fn new(
        i: isize,
        j: isize,
        snake: bool,
        bootstrap: bool,
        prev: Option<Rc<PathNode>>,
    ) -> PathNode {
        if snake {
            PathNode {
                i: i,
                j: j,
                snake: snake,
                bootstrap: bootstrap,
                prev: prev,
            }
        } else {
            if prev.as_ref().is_none() {
                PathNode {
                    i: i,
                    j: j,
                    snake: snake,
                    bootstrap: bootstrap,
                    prev: None,
                }
            } else {
                PathNode {
                    i: i,
                    j: j,
                    snake: snake,
                    bootstrap: bootstrap,
                    prev: prev.unwrap().previous_snake(),
                }
            }
        }
    }
    pub fn i(&self) -> isize {
        self.i
    }

    pub fn j(&self) -> isize {
        self.j
    }

    pub fn is_snake(&self) -> bool {
        self.snake
    }

    pub fn is_bootstrap(&self) -> bool {
        self.bootstrap
    }

    pub fn prev(&self) -> Option<Rc<PathNode>> {
        self.prev.as_ref().map(|rc| rc.clone())
    }

    pub fn previous_snake(&self) -> Option<Rc<PathNode>> {
        if self.bootstrap {
            return None;
        }
        if !self.snake && self.prev.is_some() {
            return self.prev.as_ref().unwrap().previous_snake();
        }

        if self.prev.is_none() {
            Some(Rc::new(PathNode {
                i: self.i,
                j: self.j,
                snake: self.snake,
                bootstrap: self.bootstrap,
                prev: None,
            }))
        } else {
            Some(Rc::new(PathNode {
                i: self.i,
                j: self.j,
                snake: self.snake,
                bootstrap: self.bootstrap,
                prev: Some(self.prev.as_ref().unwrap().clone()),
            }))
        }
    }
}

impl Drop for PathNode {
    fn drop(&mut self) {
        let mut prev = self.prev.take();
        while let Some(rc) = prev {
            if let Ok(mut pathnode) = Rc::try_unwrap(rc) {
                prev = pathnode.prev.take();
            } else {
                break;
            }
        }
    }
}
