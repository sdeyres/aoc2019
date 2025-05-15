use num::integer;

use crate::vec3d::Vec3D;

pub fn solve_part_one(test: bool) {
    let mut positions = load_data(test);
    let mut velocities: Vec<Vec3D> = (0..positions.len()).map(|_| Vec3D::default()).collect();

    let limit = if test { 10 } else { 1000 };
    for _ in 0..limit {
        simulate_motion(&mut positions, &mut velocities);
    }

    let total_energy: i64 = (0..positions.len())
        .map(|i| positions[i].energy() * velocities[i].energy())
        .sum();
    println!("Part 1: {}", total_energy);
}

pub fn solve_part_two(test: bool) {
    let mut positions = load_data(test);
    let mut velocities: Vec<Vec3D> = (0..positions.len()).map(|_| Vec3D::default()).collect();

    let origins = positions.clone();
    let mut x_cycle = 0;
    let mut y_cycle = 0;
    let mut z_cycle = 0;

    let mut cycle: usize = 0;
    while x_cycle == 0 || y_cycle == 0 || z_cycle == 0 {
        simulate_motion(&mut positions, &mut velocities);
        cycle += 1;

        if positions
            .iter()
            .enumerate()
            .all(|(idx, p)| p.x == origins[idx].x)
            && velocities.iter().all(|v| v.x == 0)
            && x_cycle == 0
        {
            x_cycle = cycle;
        }
        if positions
            .iter()
            .enumerate()
            .all(|(idx, p)| p.y == origins[idx].y)
            && velocities.iter().all(|v| v.y == 0)
            && y_cycle == 0
        {
            y_cycle = cycle;
        }
        if positions
            .iter()
            .enumerate()
            .all(|(idx, p)| p.z == origins[idx].z)
            && velocities.iter().all(|v| v.z == 0)
            && z_cycle == 0
        {
            z_cycle = cycle;
        }
    }

    let mut lcm = integer::lcm(x_cycle, y_cycle);
    lcm = integer::lcm(lcm, z_cycle);
    println!("Part 2: {}", lcm);
}

fn simulate_motion(moons: &mut [Vec3D], velocities: &mut [Vec3D]) {
    apply_gravity(moons, velocities);
    apply_velocity(moons, velocities);
}

fn apply_gravity(moons: &[Vec3D], velocities: &mut [Vec3D]) {
    for (i, moon) in moons.iter().enumerate() {
        for (j, other) in moons.iter().enumerate() {
            if i == j {
                continue;
            } else {
                let gravity = (other - moon).signum();
                velocities[i] += gravity;
            }
        }
    }
}

fn apply_velocity(moons: &mut [Vec3D], velocities: &[Vec3D]) {
    for (i, moon) in moons.iter_mut().enumerate() {
        *moon += velocities[i];
    }
}

fn load_data(test: bool) -> Vec<Vec3D> {
    aoc2019::load_data(12, test)
        .lines()
        .map(Vec3D::from)
        .collect()
}
