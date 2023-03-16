use crate::core::snake::Snake;
use crate::core::{Change, DeltaType};

pub struct MeyerDiffLineSpace {}

impl MeyerDiffLineSpace {
    pub fn new() -> MeyerDiffLineSpace {
        MeyerDiffLineSpace {}
    }

    pub fn compute_diff(&self, source: &Vec<String>, target: &Vec<String>) -> Vec<Change> {
        let mut changes = Vec::new();
        let size = source.len() + target.len() + 2;

        let mut down: Vec<isize> = Vec::with_capacity(size);
        let mut up: Vec<isize> = Vec::with_capacity(size);

        for _ in 0..size {
            down.push(0);
            up.push(0);
        }

        self.build_script(
            source,
            target,
            &mut changes,
            &mut down,
            &mut up,
            0,
            source.len() as isize,
            0,
            target.len() as isize,
        );
        changes
    }

    fn build_script(
        &self,
        source: &Vec<String>,
        target: &Vec<String>,
        changes: &mut Vec<Change>,
        down: &mut Vec<isize>,
        up: &mut Vec<isize>,
        start1: isize,
        end1: isize,
        start2: isize,
        end2: isize,
    ) {
        let middle = self.get_middle_snake(
            source, target, down, up, start1, end1, start2, end2,
        );
        if middle.is_none()
            || (middle.unwrap().start() == end1 && middle.unwrap().diag() == end1 - end2)
            || (middle.unwrap().end() == start1 && middle.unwrap().diag() == start1 - start2)
        {
            let mut i = start1;
            let mut j = start2;

            while i < end1 || j < end2 {
                if i < end1 && j < end2 && source[i as usize] == target[j as usize] {
                    i += 1;
                    j += 1;
                } else {
                    if end1 - start1 > end2 - start2 {
                        let idx = changes.len();
                        if changes.is_empty()
                            || changes[idx - 1].end_original() != i
                            || changes[idx - 1].delta_type() != DeltaType::Delete
                        {
                            changes.push(Change::new(DeltaType::Delete, i, i + 1, j, j));
                        } else {
                            changes[idx - 1] = changes[idx - 1].with_end_original(i + 1);

                        }
                        i += 1;
                    } else {
                        let idx = changes.len();
                        if changes.is_empty()
                            || changes[idx - 1].end_revised() != j
                            || changes[idx - 1].delta_type() != DeltaType::Insert
                        {
                            changes.push(Change::new(DeltaType::Insert, i, i, j, j + 1));
                        } else {
                            changes[idx - 1] = changes[idx - 1].with_end_revised(j + 1);
                        }
                        j += 1;
                    }
                }
            }
        } else {
            self.build_script(
                source,
                target,
                changes,
                down,
                up,
                start1,
                middle.unwrap().start(),
                start2,
                middle.unwrap().start() - middle.unwrap().diag(),
            );


            self.build_script(
                source,
                target,
                changes,
                down,
                up,
                middle.unwrap().end(),
                end1,
                middle.unwrap().end() - middle.unwrap().diag(),
                end2,
            );
        }
    }

    fn get_middle_snake(
        &self,
        source: &Vec<String>,
        target: &Vec<String>,
        down: &mut Vec<isize>,
        up: &mut Vec<isize>,
        start1: isize,
        end1: isize,
        start2: isize,
        end2: isize,
    ) -> Option<Snake> {
        let m = end1 - start1;
        let n = end2 - start2;

        if m == 0 || n == 0 {
            return None;
        }

        let delta = m - n;
        let sum = m + n;
        let offset = if sum % 2 == 0 { sum / 2 } else { (sum + 1) / 2 };

        down[(1 + offset) as usize] = start1;
        up[(1 + offset) as usize] = end1 + 1;

        for d in 0..=offset {
            for k in (-d..=d).step_by(2) {
                let i = (k + offset) as usize;
                if k == -d || k != d && down[i - 1] < down[i + 1] {
                    down[i] = down[i + 1];
                } else {
                    down[i] = down[i - 1] + 1;
                }

                let mut x = down[i];
                let mut y = x - start1 + start2 - k;

                while x < end1 && y < end2 && source[x as usize] == target[y as usize] {
                    x += 1;
                    down[i] = x;
                    y += 1;
                }

                if delta % 2 != 0 && delta - d <= k && k <= delta + d {
                    let idx = (i as isize - delta) as usize;
                    if up[idx] <= down[i] {
                        return Some(self.build_snake(
                            source,
                            target,
                            up[idx],
                            k + start1 - start2,
                            end1,
                            end2,
                        ));
                    }
                }
            }

            for k in (delta - d..=delta + d).step_by(2) {
                let i = (k + offset - delta) as usize;
                if k == delta - d || k != delta + d && up[i + 1] <= up[i - 1] {
                    up[i] = up[i + 1] - 1;
                } else {
                    up[i] = up[i - 1];
                }

                let mut x = up[i] - 1;
                let mut y = x - start1 + start2 - k;

                while x >= start1 && y >= start2 && source[x as usize] == target[y as usize] {
                    up[i] = x;
                    x -= 1;
                    y -= 1;
                }

                if delta % 2 == 0 && -d <= k && k <= d {
                    if up[i] <= down[(i as isize + delta) as usize] {
                        return Some(self.build_snake(
                            source,
                            target,
                            up[i],
                            k + start1 - start2,
                            end1,
                            end2,
                        ));
                    }
                }
            }
        }
        None
    }

    fn build_snake(
        &self,
        source: &Vec<String>,
        target: &Vec<String>,
        start: isize,
        diag: isize,
        end1: isize,
        end2: isize,
    ) -> Snake {
        let mut end = start;
        while end - diag < end2
            && end < end1
            && source[end as usize] == target[(end - diag) as usize]
        {
            end += 1;
        }
        Snake::new(start, end, diag)
    }
}
