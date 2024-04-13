use std::fmt;
use rand::{Rng, thread_rng};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x : i32,
    pub y : i32,
}
#[allow(dead_code)]
impl Vec2 {

    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };

    pub fn new(x:i32, y:i32) -> Self { Self { x, y, } }

    pub fn random_range(lower:Vec2, upper:Vec2) -> Self {
        let mut rng = thread_rng();

        let x = rng.gen_range(lower.x..upper.x);
        let y = rng.gen_range(lower.y..upper.y);

        Self {
            x,
            y,
        }
    }

    pub fn middle_of(lower:Vec2, upper:Vec2) -> Self {
        let fixed = upper - lower;
        Self {
            x : fixed.x / 2,
            y : fixed.y / 2,
        }
    }

    pub fn input_vec(left:bool, right:bool, up:bool, down:bool, allow_diagonal:bool) -> Self {
        let mut back = Vec2::ZERO;

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
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// for *=
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

// for -=
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// for /=
impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

// for -
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Self) -> Self {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

// for *
impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Self) -> Self {
        Vec2::new(self.x * other.x, self.y * other.y)
    }
}

// for +
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Self) -> Self {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

// for /
impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, other: Self) -> Self {
        Vec2::new(self.x / other.x, self.y / other.y)
    }
}

// for printing
impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        if (self.x == other.x) && (self.y == other.y) { return true }
        false
    }
}