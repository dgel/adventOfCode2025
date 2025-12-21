pub mod template;
use num_integer::gcd;
use rand::{rng, rngs::ThreadRng, seq::IteratorRandom};
use std::collections::HashSet;
use std::f32;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::rc::Rc;

pub type Point3d = [u64; 3];

pub type Point2d = [u64; 2];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Area {
    bottom_left: Point2d,
    top_right: Point2d,
}

impl Area {
    pub fn from_points(p1: Point2d, p2: Point2d) -> Self {
        Area {
            bottom_left: [p1[1].min(p2[1]), p1[0].min(p2[0])],
            top_right: [p1[1].max(p2[1]), p1[0].max(p2[0])],
        }
    }

    pub fn bottom(&self) -> u64 {
        self.bottom_left[0]
    }
    pub fn left(&self) -> u64 {
        self.bottom_left[1]
    }
    pub fn top(&self) -> u64 {
        self.top_right[0]
    }
    pub fn right(&self) -> u64 {
        self.top_right[1]
    }

    pub fn size(&self) -> u64 {
        (self.top_right[0] - self.bottom_left[0] + 1)
            * (self.top_right[1] - self.bottom_left[1] + 1)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() > other.bottom()
            && self.bottom() < other.top()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AreaKDTree {
    Node {
        left: Rc<AreaKDTree>,
        right: Rc<AreaKDTree>,
        pivot: u64,
        axis: u32,
    },
    Leaf {
        areas: Vec<Area>,
    },
}

impl AreaKDTree {
    pub fn new(areas: &[Area]) -> Rc<AreaKDTree> {
        let mut rng = rng();
        AreaKDTree::construct_recur(areas, &mut rng, 0)
    }
    fn construct_recur(areas: &[Area], rng: &mut ThreadRng, mut axis: u32) -> Rc<AreaKDTree> {
        // it's faster to iterate over a small number of elements than to create a deep tree
        if areas.len() <= 8 {
            Rc::new(AreaKDTree::Leaf {
                areas: areas.into(),
            })
        } else {
            // try splitting on either axis,
            for _ in 0..2 {
                let axis_idx = axis as usize;
                let pivot = {
                    let mut sample: Vec<Point2d> = areas
                        .iter()
                        .choose_multiple(rng, std::cmp::min(7, areas.len() / 2))
                        .iter()
                        .flat_map(|area| [area.bottom_left, area.top_right])
                        .collect();
                    sample.sort_by_key(|point| point[axis_idx]);
                    sample[sample.len() / 2][axis_idx]
                };
                let left: Vec<Area> = areas
                    .iter()
                    .cloned()
                    .filter(|area| {
                        let v1 = area.bottom_left[axis_idx];
                        v1 < pivot
                    })
                    .collect();
                let right: Vec<Area> = areas
                    .iter()
                    .cloned()
                    .filter(|area| {
                        let v1 = area.top_right[axis_idx];
                        v1 >= pivot
                    })
                    .collect();
                let new_axis = (axis + 1) % 2;
                if left.len() != areas.len() && right.len() != areas.len() {
                    return Rc::new(AreaKDTree::Node {
                        left: AreaKDTree::construct_recur(&left, rng, new_axis),
                        right: AreaKDTree::construct_recur(&right, rng, new_axis),
                        pivot,
                        axis,
                    });
                }
                axis = new_axis;
            }
            Rc::new(AreaKDTree::Leaf {
                areas: areas.into(),
            })
        }
    }

    pub fn any_overlapping(&self, area: &Area) -> bool {
        match self {
            Self::Node {
                left,
                right,
                pivot,
                axis,
            } => {
                if area.bottom_left[*axis as usize] < *pivot && Self::any_overlapping(left, area) {
                    return true;
                }
                if area.top_right[*axis as usize] >= *pivot {
                    return Self::any_overlapping(right, area);
                }
                false
            }
            Self::Leaf { areas } => {
                for &overlapping_area in areas {
                    if area.overlaps(&overlapping_area) {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn get_overlapping(&self, area: Area) -> HashSet<Area> {
        let mut overlapping_areas = HashSet::new();
        self.get_overlapping_recur(&mut overlapping_areas, area);
        overlapping_areas
    }

    fn get_overlapping_recur(&self, overlapping_areas: &mut HashSet<Area>, area: Area) {
        match self {
            Self::Node {
                left,
                right,
                pivot,
                axis,
            } => {
                if area.bottom_left[*axis as usize] < *pivot {
                    Self::get_overlapping_recur(left, overlapping_areas, area);
                }
                if area.top_right[*axis as usize] >= *pivot {
                    Self::get_overlapping_recur(right, overlapping_areas, area);
                }
            }
            Self::Leaf { areas } => {
                for &overlapping_area in areas.iter().filter(|a| area.overlaps(a)) {
                    overlapping_areas.insert(overlapping_area);
                }
            }
        }
    }

    pub fn count_overlapping(&self, area: Area) -> usize {
        self.get_overlapping(area).len()
    }

    pub fn print(&self) {
        AreaKDTree::print_recur(self, 0);
    }

    fn print_recur(kdtree: &AreaKDTree, indent: usize) {
        match kdtree {
            Self::Node {
                left,
                right,
                pivot,
                axis,
            } => {
                println!(
                    "{} Node, pivot: {}, axis: {}",
                    " ".repeat(indent),
                    pivot,
                    axis
                );
                AreaKDTree::print_recur(left, indent + 2);
                AreaKDTree::print_recur(right, indent + 2);
            }
            Self::Leaf { areas } => {
                println!("{} Leaf {:?}", " ".repeat(indent), areas);
            }
        }
    }
}

pub trait ZeroExt {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait AbsExt {
    fn abs(self) -> Self;
}

pub trait NumExt:
    Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + ZeroExt
    + AbsExt
    + Ord
    + Copy
{
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Mat<T> {
    data: Box<[T]>,
    cols: usize,
    rows: usize,
}

impl<T> Mat<T>
where
    T: NumExt,
    T: Display,
{
    fn div_row_by(&mut self, row: usize, value: T) {
        for item in &mut self.data[row * self.cols..(row + 1) * self.cols] {
            *item /= value;
        }
    }

    fn sub_n_rows_from_row(&mut self, row: usize, n: T, row_to_sub: usize) {
        let row_start = row * self.cols;
        let row_to_sub_start = row_to_sub * self.cols;
        for col in 0..self.cols {
            self.data[row_start + col] -= n * self.data[row_to_sub_start + col];
        }
    }

    fn swap_rows(&mut self, i: usize, j: usize) {
        let lower = std::cmp::min(i, j);
        let higher = std::cmp::max(i, j);
        if higher < self.rows {
            let (first, second) = self.data.split_at_mut(higher * self.cols);
            first[lower * self.cols..(lower + 1) * self.cols]
                .swap_with_slice(&mut second[0..self.cols]);
        }
    }

    fn highest_row_below(&self, start_row: usize, column: usize) -> Option<usize> {
        let mut cur_val: Option<T> = None;
        let mut best_row = None;
        for row in start_row..self.rows {
            let new_val = self[(row, column)];
            if !new_val.is_zero() && cur_val.map(|v| new_val.abs() > v.abs()).unwrap_or(true) {
                cur_val = Some(new_val);
                best_row = Some(row);
            }
        }
        best_row
    }

    pub fn to_rref(&mut self, augmented_mat: bool) {
        let mut non_zero_col = 0;
        let max_cols = if augmented_mat {
            self.cols.saturating_sub(1)
        } else {
            self.cols
        };
        // bring each row to ref
        for i in 0..self.rows {
            while non_zero_col < max_cols {
                // if we found some non-zero row
                if let Some(row) = self.highest_row_below(i, non_zero_col) {
                    if row != i {
                        // println!("swapping: row {} and {}:\n {:6.2}", i, row, self);
                        self.swap_rows(i, row);
                    }
                    // println!("normalizing:\n{:6.2}", self);
                    // normalize the row
                    let val = self[(i, non_zero_col)];
                    self.div_row_by(i, val);
                    // println!("normalized:\n{:6.2}", self);

                    // eliminate all other rows
                    for other_row in 0..self.rows {
                        if other_row != i {
                            let val = self[(other_row, non_zero_col)];
                            if !val.is_zero() {
                                self.sub_n_rows_from_row(other_row, val, i);
                            }
                        }
                    }
                    // println!("subtracted:\n{:6.2}", self);

                    // normalize rows below
                    non_zero_col += 1;
                    break;
                }
                non_zero_col += 1;
            }
        }
        // bring ref to rref
    }
}

impl<T: Default> Mat<T> {
    pub fn new(cols: usize, rows: usize) -> Mat<T> {
        let mut data = Vec::new();
        data.resize_with(cols * rows, T::default);
        Mat {
            data: data.into_boxed_slice(),
            cols,
            rows,
        }
    }
}

impl<T> Mat<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn row(&self, row: usize) -> &[T] {
        if row >= self.rows {
            panic!("index out of bounds");
        }
        &self.data[row * self.cols..(row + 1) * self.cols]
    }
}

impl<T: Clone> Mat<T> {
    pub fn from_array<const R: usize, const C: usize>(array: &[[T; C]; R]) -> Mat<T> {
        let mut data = Vec::with_capacity(R * C);
        for row in array {
            data.extend_from_slice(row);
        }
        Mat {
            data: data.into_boxed_slice(),
            cols: C,
            rows: R,
        }
    }
    pub fn with_value(cols: usize, rows: usize, val: T) -> Mat<T> {
        Mat {
            data: vec![val; cols * rows].into_boxed_slice(),
            cols,
            rows,
        }
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        if index.0 < self.rows && index.1 < self.cols {
            &self.data[index.0 * self.cols + index.1]
        } else {
            panic!("out of bounds!");
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        if index.0 < self.rows && index.1 < self.cols {
            &mut self.data[index.0 * self.cols + index.1]
        } else {
            panic!("out of bounds!");
        }
    }
}

impl<T> Display for Mat<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rows > 0 {
            for row in 0..self.rows {
                f.write_str(if row == 0 { "[" } else { " " })?;
                for col in 0..self.cols {
                    if col > 0 {
                        write!(f, ", ")?;
                    }
                    self[(row, col)].fmt(f)?;
                }

                f.write_str(if row == self.rows - 1 { "]" } else { "\n" })?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rational {
    numerator: i32,
    denominator: i32,
}

impl Rational {
    pub fn new<T: Into<i32>>(numerator: T, denominator: T) -> Rational {
        Rational {
            numerator: numerator.into(),
            denominator: denominator.into(),
        }
    }

    pub fn from_int<T: Into<i32>>(i: T) -> Rational {
        Rational {
            numerator: i.into(),
            denominator: 1,
        }
    }

    pub fn to_int(self) -> Option<i32> {
        if self.denominator == 1 {
            Some(self.numerator)
        } else {
            None
        }
    }

    fn normalize(&mut self) {
        if self.denominator == 0 {
            self.numerator = 0;
            self.denominator = 1;
            return;
        }
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }

        let gcd = gcd(self.numerator.abs(), self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
    }
}

impl From<Rational> for f64 {
    fn from(value: Rational) -> Self {
        value.numerator as f64 / value.denominator as f64
    }
}

impl From<Rational> for f32 {
    fn from(value: Rational) -> Self {
        value.numerator as f32 / value.denominator as f32
    }
}

impl From<i32> for Rational {
    fn from(value: i32) -> Self {
        Rational {
            numerator: value,
            denominator: 1,
        }
    }
}

impl Default for Rational {
    fn default() -> Self {
        Rational {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        self.numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        self.denominator *= rhs.denominator;
        self.normalize();
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        self.numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        self.denominator *= rhs.denominator;
        self.normalize();
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.normalize();
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self *= other;
        self
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.denominator;
        self.denominator *= rhs.numerator;
        self.normalize();
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(mut self, other: Self) -> Self {
        self /= other;
        self
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ZeroExt for Rational {
    fn zero() -> Self {
        Rational {
            numerator: 0,
            denominator: 1,
        }
    }
    fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

impl AbsExt for Rational {
    fn abs(self) -> Self {
        Rational {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }
}

impl NumExt for Rational {}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.numerator as f64 / self.denominator as f64;
        f64::fmt(&val, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test_partition_by_axis() {
    //         let mut points = [
    //             [8, 5, 4],
    //             [2, 0, 9],
    //             [0, 0, 0],
    //             [1, 1, 1],
    //             [4, 2, 3],
    //             [1, 1, 6],
    //         ];
    //         let pivot = [1, 1, 1];
    //         let partition_point = KDTree::partition_by_axis(&mut points, pivot, 1);
    //         println!("{:?}", points);
    //         assert!(partition_point == 3);
    //         assert!(points[partition_point] == [1, 1, 1]);
    //     }

    #[test]
    fn test_mat_to_rref_1() {
        let mut mat = Mat::from_array(&[
            [Rational::from_int(-1), Rational::from_int(1)],
            [Rational::from_int(-1), Rational::from_int(0)],
            [Rational::from_int(0), Rational::from_int(-1)],
            [Rational::from_int(-1), Rational::from_int(-2)],
        ]);
        mat.to_rref(false);

        let expected = Mat::from_array(&[
            [Rational::from_int(1), Rational::from_int(0)],
            [Rational::from_int(0), Rational::from_int(1)],
            [Rational::from_int(0), Rational::from_int(0)],
            [Rational::from_int(0), Rational::from_int(0)],
        ]);
        assert!(mat == expected)
    }

    #[test]
    fn test_mat_to_rref_2() {
        let mut mat = Mat::from_array(&[
            [Rational::from_int(-1), Rational::from_int(1)],
            [Rational::from_int(-1), Rational::from_int(2)],
            [Rational::from_int(-3), Rational::from_int(2)],
        ]);
        mat.to_rref(false);

        let expected = Mat::from_array(&[
            [Rational::from_int(1), Rational::from_int(0)],
            [Rational::from_int(0), Rational::from_int(1)],
            [Rational::from_int(0), Rational::from_int(0)],
        ]);

        assert!(mat == expected)
    }

    #[test]
    fn test_mat_to_rref_3() {
        let mut mat = Mat::from_array(&[
            [
                Rational::from_int(1),
                Rational::from_int(3),
                Rational::from_int(2),
                Rational::from_int(1),
            ],
            [
                Rational::from_int(0),
                Rational::from_int(-9),
                Rational::from_int(-4),
                Rational::from_int(-4),
            ],
        ]);
        mat.to_rref(false);

        let expected = Mat::from_array(&[
            [
                Rational::from_int(1),
                Rational::from_int(0),
                Rational::new(2, 3),
                Rational::new(-1, 3),
            ],
            [
                Rational::from_int(0),
                Rational::from_int(1),
                Rational::new(4, 9),
                Rational::new(4, 9),
            ],
        ]);
        assert!(mat == expected)
    }

    #[test]
    fn test_mat_to_rref_augmented_1() {
        let mut mat = Mat::from_array(&[
            [
                Rational::from_int(1),
                Rational::from_int(1),
                Rational::from_int(-1),
                Rational::from_int(7),
            ],
            [
                Rational::from_int(1),
                Rational::from_int(-1),
                Rational::from_int(2),
                Rational::from_int(3),
            ],
            [
                Rational::from_int(2),
                Rational::from_int(1),
                Rational::from_int(1),
                Rational::from_int(9),
            ],
        ]);
        mat.to_rref(true);

        let expected = Mat::from_array(&[
            [
                Rational::from_int(1),
                Rational::from_int(0),
                Rational::from_int(0),
                Rational::from_int(6),
            ],
            [
                Rational::from_int(0),
                Rational::from_int(1),
                Rational::from_int(0),
                Rational::from_int(-1),
            ],
            [
                Rational::from_int(0),
                Rational::from_int(0),
                Rational::from_int(1),
                Rational::from_int(-2),
            ],
        ]);
        assert!(mat == expected)
    }

    #[test]
    fn test_mat_to_rref_augmented_2() {
        let mut mat = Mat::from_array(&[
            [
                Rational::from_int(1),
                Rational::from_int(1),
                Rational::from_int(3),
                Rational::from_int(0),
            ],
            [
                Rational::from_int(1),
                Rational::from_int(3),
                Rational::from_int(5),
                Rational::from_int(0),
            ],
            [
                Rational::from_int(2),
                Rational::from_int(0),
                Rational::from_int(4),
                Rational::from_int(1),
            ],
        ]);
        mat.to_rref(true);
        println!("{:5.2}", mat);

        let expected = Mat::from_array(&[
            [
                Rational::from_int(1),
                Rational::from_int(0),
                Rational::from_int(2),
                Rational::new(1, 2),
            ],
            [
                Rational::from_int(0),
                Rational::from_int(1),
                Rational::from_int(1),
                Rational::new(-1, 6),
            ],
            [
                Rational::from_int(0),
                Rational::from_int(0),
                Rational::from_int(0),
                Rational::new(-1, 3),
            ],
        ]);
        assert!(mat == expected)
    }
}
