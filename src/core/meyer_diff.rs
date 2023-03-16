use super::Change;
use super::DeltaType;
use super::PathNode;
use std::rc::Rc;

pub struct MeyerDiff {}

impl MeyerDiff {
    pub fn new() -> MeyerDiff {
        MeyerDiff {}
    }

    pub fn compute_diff(&self, source: &Vec<String>, target: &Vec<String>) -> Vec<Change> {
        let node = self.build_path(source, target);
        match node {
            Some(rc) => self.build_revision(rc),
            None => Vec::new(),
        }
    }

    fn build_path(&self, original: &Vec<String>, revised: &Vec<String>) -> Option<Rc<PathNode>> {
        let n = original.len();
        let m = revised.len();

        let max = n + m + 1;
        let size = 1 + 2 * max;
        let middle = size / 2;

        let mut diagonal: Vec<Option<Rc<PathNode>>> = Vec::with_capacity(size);
        for _ in 0..size {
            diagonal.push(None);
        }

        diagonal[middle + 1] = Some(Rc::new(PathNode::new(0, -1, true, true, None)));

        for d in 0..max as isize {
            for k in (-d..=d).step_by(2) {
                let kmiddle = (middle as isize + k) as usize;
                let kplus = kmiddle + 1;
                let kminus = kmiddle - 1;

                let prev: Option<Rc<PathNode>>;
                let mut i ;

                if k == -d
                    || (k != d
                        && diagonal[kminus].as_ref().unwrap().i()
                            < diagonal[kplus].as_ref().unwrap().i())
                {
                    i = diagonal[kplus].as_ref().unwrap().i();
                    prev = diagonal[kplus].as_ref().map(|rc| rc.clone());
                } else {
                    i = diagonal[kminus].as_ref().unwrap().i() + 1;
                    prev = diagonal[kminus].as_ref().map(|rc| rc.clone())
                }

                diagonal[kminus] = None;
                let mut j = i - k;

                let mut node = PathNode::new(i, j, false, false, prev);

                while i < n as isize
                    && j < m as isize
                    && original[i as usize] == revised[j as usize]
                {
                    i += 1;
                    j += 1;
                }

                if i != node.i() {
                    node = PathNode::new(i, j, true, false, Some(Rc::new(node)));
                }
                diagonal[kmiddle] = Some(Rc::new(node));

                if i >= n as isize && j >= m as isize {
                    return diagonal.into_iter().skip(kmiddle).next().unwrap();
                }
            }
            diagonal[middle + d as usize - 1] = None;
        }
        None
    }

    fn build_revision(&self, pathnode: Rc<PathNode>) -> Vec<Change> {
        let mut path: Option<Rc<PathNode>> = Some(pathnode);

        let mut changes = Vec::new();
        if path.as_ref().unwrap().is_snake() {
            path = path.as_ref().unwrap().prev();
        }

        while self.meet_cond(&path) {
            let prev = path.as_ref().unwrap().prev();
            let i = path.as_ref().unwrap().i();
            let j = path.as_ref().unwrap().j();
            path = prev;

            let ianchor = path.as_ref().unwrap().i();
            let janchor = path.as_ref().unwrap().j();

            if ianchor == i && janchor != j {
                changes.push(Change::new(DeltaType::Insert, ianchor, i, janchor, j));
            } else if ianchor != i && janchor == j {
                changes.push(Change::new(DeltaType::Delete, ianchor, i, janchor, j));
            } else {
                changes.push(Change::new(DeltaType::Change, ianchor, i, janchor, j));
            }

            if path.as_ref().unwrap().is_snake() {
                path = path.as_ref().unwrap().prev();
            }
        }
        changes
    }

    fn meet_cond(&self, path: &Option<Rc<PathNode>>) -> bool {
        if path.is_none() {
            return false;
        }
        let prev = path.as_ref().unwrap().prev();
        if prev.is_none() {
            return false;
        }
        if prev.as_ref().unwrap().j() < 0 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn compute() {
        let f1 = File::open("1.txt");
        let reader1 = BufReader::new(f1.unwrap());
        let mut v1: Vec<String> = Vec::new();

        for line in reader1.lines() {
            v1.push(line.unwrap());
        }
        let f2 = File::open("2.txt");
        let reader2 = BufReader::new(f2.unwrap());
        let mut v2: Vec<String> = Vec::new();

        for line in reader2.lines() {
            v2.push(line.unwrap());
        }

        let meyer_diff = MeyerDiff::new();
        let changes = meyer_diff.compute_diff(&v1, &v2);

        let mut m = 0;
        for c in &changes {
            m += c.end_original() - c.start_original() + c.end_revised() - c.start_revised();
        }

        let sim = 1.0f32 - (m as f32 + 0.0) / ((v1.len() + v2.len()) as f32 + 1e-8);
        println!("{}", sim);
        assert!(sim > 0.0f32);
    }
}
