use std::fmt;
use rand::{Rng, thread_rng};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[allow(dead_code)]
#[derive(Clone, Copy, Eq, Hash)]
pub struct Vec2Int {
    pub x : i32,
    pub y : i32,
}
#[allow(dead_code)]
impl Vec2Int {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };

    pub fn new(x:i32, y:i32) -> Self { Self { x, y, } }

    pub fn random_range(lower: Vec2Int, upper: Vec2Int) -> Self {
        let mut rng = thread_rng();

        let x = rng.gen_range(lower.x..upper.x);
        let y = rng.gen_range(lower.y..upper.y);

        Self {
            x,
            y,
        }
    }

    pub fn middle_of(lower: Vec2Int, upper: Vec2Int) -> Self {
        let fixed = upper - lower;
        Self {
            x : fixed.x / 2,
            y : fixed.y / 2,
        }
    }

    pub fn input_vec(left:bool, right:bool, up:bool, down:bool, allow_diagonal:bool) -> Self {
        let mut back = Vec2Int::ZERO;

        if left { back.x = -1; }
        else if right { back.x = 1; }

        if up { back.y = -1; }
        else if down { back.y = 1; }

        if !allow_diagonal && back.x != 0 && back.y != 0 {
            back.y = 0;
        }

        back
    }

}

// for +=
impl AddAssign for Vec2Int {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// for *=
impl MulAssign for Vec2Int {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

// for -=
impl SubAssign for Vec2Int {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// for /=
impl DivAssign for Vec2Int {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

// for -
impl Sub for Vec2Int {
    type Output = Vec2Int;
    fn sub(self, other: Self) -> Self {
        Vec2Int::new(self.x - other.x, self.y - other.y)
    }
}

// for *
impl Mul for Vec2Int {
    type Output = Vec2Int;
    fn mul(self, other: Self) -> Self {
        Vec2Int::new(self.x * other.x, self.y * other.y)
    }
}

// for +
impl Add for Vec2Int {
    type Output = Vec2Int;
    fn add(self, other: Self) -> Self {
        Vec2Int::new(self.x + other.x, self.y + other.y)
    }
}

// for /
impl Div for Vec2Int {
    type Output = Vec2Int;
    fn div(self, other: Self) -> Self {
        Vec2Int::new(self.x / other.x, self.y / other.y)
    }
}

// for printing
impl fmt::Display for Vec2Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Vec2Int {
    fn eq(&self, other: &Self) -> bool {
        if (self.x == other.x) && (self.y == other.y) { return true }
        false
    }
}