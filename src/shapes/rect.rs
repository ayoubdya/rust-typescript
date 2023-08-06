use std::{fmt::Display, str::FromStr};

use super::{
  area::Area,
  collisions::{Contains, PointIter, Points},
};

pub struct Rect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

impl Points for Rect {
  fn points(&self) -> PointIter {
    return vec![
      (self.x, self.y),
      (self.x + self.width, self.y),
      (self.x, self.y + self.height),
      (self.x + self.width, self.y + self.height),
    ]
    .into();
  }
}

impl Contains for Rect {
  fn contains_point(&self, (x, y): (f64, f64)) -> bool {
    if x >= self.x && x <= self.x + self.height && y >= self.y && y <= self.y + self.height {
      return true;
    }
    return false;
  }
}

impl FromStr for Rect {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.split(" ").collect::<Vec<_>>();
    if parts.len() != 4 {
      return Err(anyhow::anyhow!("Bad rectangle from string"));
    }
    return Ok(Rect {
      x: parts[0].parse()?,
      y: parts[1].parse()?,
      width: parts[2].parse()?,
      height: parts[3].parse()?,
    });
  }
}

impl Area for Rect {
  fn area(&self) -> f64 {
    return self.height * self.width;
  }
}

impl Default for Rect {
  fn default() -> Self {
    //static method cause there is no default(&self)
    return Rect {
      x: 0.0,
      y: 0.0,
      width: 10.0,
      height: 10.0,
    };
  }
}

impl Display for Rect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return write!(
      f,
      "Rectangle ({},{}) {}x{}",
      self.x, self.y, self.width, self.height
    );
  }
}

// impl<T> Collidable<T> for Rect {}

// impl Rect {
//   pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
//     if x >= self.x && x <= self.x + self.height && y >= self.y && y <= self.y + self.height {
//       return true;
//     }
//     return false;
//   }
// }

// impl Collidable<Rect> for Rect {
//   fn collide(&self, other: &Rect) -> bool {
//     for point in other {
//       if self.contains_point(point) {
//         return true;
//       }
//     }
//     return false;
//   }
// }

// impl Collidable<Circle> for Rect {
//   fn collide(&self, other: &Circle) -> bool {
//     return self.contains_point((other.x, other.y));
//   }
// }

// pub struct RectIter {
//   points: Vec<(f64, f64)>,
//   idx: usize,
// }

// impl Iterator for RectIter {
//   type Item = (f64, f64);

//   fn next(&mut self) -> Option<Self::Item> {
//     if self.idx >= self.points.len() {
//       return None;
//     }

//     let point = self.points[self.idx];
//     self.idx += 1;

//     return Some(point);
//   }
// }

// impl From<&Rect> for RectIter {
//   fn from(value: &Rect) -> Self {
//     return RectIter {
//       points: vec![
//         (value.x, value.y),
//         (value.x + value.width, value.y),
//         (value.x, value.y + value.height),
//         (value.x + value.width, value.y + value.height),
//       ],
//       idx: 0,
//     };
//   }
// }

// impl IntoIterator for Rect {
//   type Item = (f64, f64);

//   type IntoIter = RectIter;

//   fn into_iter(self) -> Self::IntoIter {
//     return RectIter::from(&self);
//   }
// }

// impl IntoIterator for &Rect {
//   type Item = (f64, f64);

//   type IntoIter = RectIter;

//   fn into_iter(self) -> Self::IntoIter {
//     return self.into();
//   }
// }
