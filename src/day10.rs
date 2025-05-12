use std::{
    collections::HashSet, f64::consts::{FRAC_PI_2, PI}, fmt::Display, ops::{Add, Div, Mul, Sub}
};

use num::integer;

pub fn solve_part_one(test: bool) {
    let field = load_data(test);
    let best_position = field.best_position().1;
    println!("Part 1: {}", best_position);
}

pub fn solve_part_two(test: bool) {
    let field = load_data(test);
    let best_position = field.best_position().0;
    let nth_position = field.vaporize(&best_position, 200);
    println!("Part 2: {}", nth_position.x * 100 + nth_position.y);
}

struct AsteroidField {
    height: usize,
    width: usize,
    asteroids: HashSet<Coordinates>,
}

impl Display for AsteroidField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.asteroids.contains(&Coordinates {
                    x: x as i32,
                    y: y as i32,
                }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl AsteroidField {
    fn best_position(&self) -> (Coordinates, usize) {
        let mut best = 0;
        let mut best_position = Coordinates::new(0, 0);

        for source in self.asteroids.iter() {
            let count = self.count_reachable_aseroids(source);
            if count > best {
                best = count;
                best_position = *source;
            }
        }

        (best_position, best)
    }

    fn count_reachable_aseroids(&self, source: &Coordinates) -> usize {
        let mut normalized_diffs = HashSet::new();

        for asteroid in self.asteroids.iter() {
            if asteroid == source {
                continue;
            }

            let normalized_diff = (asteroid - source).normalize();
            normalized_diffs.insert(normalized_diff);
        }

        normalized_diffs.len()
    }

    fn vaporize(&self, base_position: &Coordinates, count: usize) -> Coordinates {
        let mut list = Vec::new();
        for asteroid in self.asteroids.iter() {
            if asteroid == base_position {
                continue;
            }

            let diff = asteroid - base_position;
            list.push((diff.angle(), diff.magnitude(), *asteroid));
        }

        list.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap().reverse().then(a.1.cmp(&b.1))
        });

        let mut removed = Vec::with_capacity(count);
        for (idx, v) in list.iter().enumerate() {
            if v.0 <= FRAC_PI_2 {
                removed.push(list.remove(idx));
                break;
            }
        }
        if removed.last().is_none() {
            removed.push(list.remove(0));
        }

        'outer: for _ in 1..count {
            for (idx, v) in list.iter().enumerate() {
                if v.0 < removed.last().unwrap().0 {
                    removed.push(list.remove(idx));
                    continue 'outer;
                }
            }
            removed.push(list.remove(0));
        }

        removed.last().unwrap().2
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Add<&Coordinates> for &Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &Coordinates) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&Coordinates> for &Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &Coordinates) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: rhs.y - self.y,
        }
    }
}

impl Mul<&Coordinates> for i32 {
    type Output = Coordinates;

    fn mul(self, rhs: &Coordinates) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<i32> for &Coordinates {
    type Output = Coordinates;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn normalize(&self) -> Coordinates {
        let gcd = integer::gcd(self.x, self.y);
        self / gcd
    }

    fn angle(&self) -> f64 {
        let result = (self.y as f64).atan2(self.x as f64);
        if result < 0.0 {
            result + 2.0 * PI
        } else {
            result
        }
    }

    fn magnitude(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

fn load_data(test: bool) -> AsteroidField {
    let mut height = 0;
    let mut width = 0;

    let asteroids = aoc2019::load_data(10, test)
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            height += 1;
            width = line.len();
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Coordinates::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    AsteroidField {
        height,
        width,
        asteroids,
    }
}

#[cfg(test)]
mod test {
    use super::Coordinates;

    #[test]
    fn test_mul() {
        let c = Coordinates::new(4, 8);
        let n = c.normalize();
        let three_times_n = 3 * &n;
        let half_c = &c / 2;

        assert_eq!(Coordinates { x: 1, y: 2 }, n);
        assert_eq!(Coordinates { x: 3, y: 6 }, three_times_n);
        assert_eq!(Coordinates { x: 2, y: 4 }, half_c);
    }
}
