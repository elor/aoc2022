use std::fs;

#[allow(dead_code)] // WHYYYY is this necessary?
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let grid = input_to_grid(input);
    return count_visible(&grid);
}

fn part2(input: &str) -> i32 {
    let grid = input_to_grid(input);
    return highest_scenic_score(&grid) as i32;
}

fn input_to_grid(input: &str) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Make sure the grid is rectangular (no empty or uneven lines)
    let width = grid[0].len();
    for line in &mut grid {
        assert_eq!(line.len(), width);
    }

    return grid;
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let height = grid[y][x];

    return (0..x).all(|x| grid[y][x] < height)
        || (x + 1..grid.len()).all(|x| grid[y][x] < height)
        || (0..y).all(|y| grid[y][x] < height)
        || (y + 1..grid.len()).all(|y| grid[y][x] < height);
}

fn count_visible(grid: &Vec<Vec<u32>>) -> i32 {
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            if is_visible(grid, x, y) {
                count += 1;
            }
        }
    }
    return count;
}

fn score_meta(line: Vec<u32>) -> u32 {
    assert!(!line.is_empty());

    let ref_height = line.first().unwrap();
    let mut score = 0;

    for (id, height) in line.iter().enumerate() {
        if id == 0 {
            continue;
        }

        score += 1;
        if height >= ref_height {
            break;
        }
    }

    return score;
}

fn score_left(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    return score_meta((0..=x).rev().map(|x| grid[y][x]).collect());
}

fn score_up(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    return score_meta((0..=y).rev().map(|y| grid[y][x]).collect());
}

fn score_right(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    return score_meta((x..grid.len()).map(|x| grid[y][x]).collect());
}

fn score_down(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    return score_meta((y..grid.len()).map(|y| grid[y][x]).collect());
}

fn scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    return score_up(grid, x, y)
        * score_down(grid, x, y)
        * score_left(grid, x, y)
        * score_right(grid, x, y);
}

fn highest_scenic_score(grid: &Vec<Vec<u32>>) -> u32 {
    let mut highest_score = 0;

    for x in 0..grid.len() {
        for y in 0..grid.len() {
            let score = scenic_score(grid, x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    return highest_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    // 5x5 grid with ints
    #[test]
    fn test_part1() {
        let input = "30373
25512
65332
33549
35390";

        let grid = input_to_grid(input);

        assert_eq!(grid[0][0], 3);
        assert_eq!(grid[0][1], 0);
        assert_eq!(grid[4][3], 9);

        assert_eq!(is_visible(&grid, 0, 0), true);

        assert_eq!(count_visible(&grid), 21);
    }

    #[test]
    fn test_part2() {
        let input = "30373
25512
65332
33549
35390";

        let grid = input_to_grid(input);

        let x = 2;
        let y = 1;
        assert_eq!(grid[y][x], 5);
        assert_eq!(score_up(&grid, x, y), 1);
        assert_eq!(score_left(&grid, x, y), 1);
        assert_eq!(score_right(&grid, x, y), 2);
        assert_eq!(score_down(&grid, x, y), 2);
        assert_eq!(scenic_score(&grid, x, y), 4);

        let x = 2;
        let y = 3;
        assert_eq!(grid[y][x], 5);
        assert_eq!(score_up(&grid, x, y), 2);
        assert_eq!(score_left(&grid, x, y), 2);
        assert_eq!(score_right(&grid, x, y), 2);
        assert_eq!(score_down(&grid, x, y), 1);
        assert_eq!(scenic_score(&grid, x, y), 8);

        assert_eq!(highest_scenic_score(&grid), 8);
    }
}
