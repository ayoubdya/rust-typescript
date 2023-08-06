// fn todo(x: i32) -> bool {
//     if x > 10 {
//         return true;
//     }

//     todo!("less than or equals 10 todo later");
// }

// use std::{f64::consts::PI, ops::Index, os::unix::process};

// use std::collections::{HashMap, HashSet};
mod shapes;
use std::{fmt::Display, str::FromStr};

use shapes::{area::Area, circle::Circle, collisions::Collidable, rect::Rect};

use crate::shapes::collisions::{Contains, PointIter, Points};

fn main() {
  // let y = 11;
  // let ret = todo(y);
  // println!("ret is {}", ret);

  // let x = None;
  // let y: i32 = x.unwrap();
  // println!("{}", y);

  // let mut x = 5;
  // let y = &mut x;
  // *y += 1;
  // print!("{}", y);

  // let arr: Vec<_> = vec![1, 2, 3, 4, 5].iter().map(|x| x + 1).collect();
  // print!("array is {:?}", arr);

  // same thing as .collect()
  // let data = vec![1, 2, 3, 4];
  // let mut foo = data.iter().map(|x| x + 1);
  // let mut new = vec![];
  // while let Some(x) = foo.next() {
  //     new.push(x);
  // }
  // print!("{:?}", new);

  // let string_arr: String = vec!["this", "is", "a", "string"].into_iter().collect();
  // print!("{:?}", string_arr);

  // let foo: HashMap<&str, usize> = vec!["this", "is", "test"]
  //     .into_iter()
  //     .enumerate()
  //     .map(|(idx, item)| (item, idx))
  //     .collect();
  // print!("{:?}", foo);

  // let foo: HashSet<isize> = vec![1, 2, 2, 3, 3].into_iter().collect();
  // print!("{:?}", foo);

  // let sum: usize = vec![1, 2, 3].iter().sum();
  // print!("{:?}", sum);

  // let how_many: usize = vec![1, 2, 3].iter().skip(2).count();
  // print!("{:?}", how_many);

  // vec![1, 2, 5, 9, 4]
  //     .iter()
  //     .skip(2)
  //     .take_while(|x| **x > 4)
  //     .for_each(|x| println!("{}", x));

  // let pair = vec![1, 2, 3].iter().filter(|&&x| x % 2 == 0).count();
  // print!("{}", pair);

  // let map = HashMap::from([("foo", 1), ("bar", 2)]);
  // map.iter()
  //     .for_each(|(&k, &v)| println!("key {} value {}", k, v));

  // // #[derive(Debug)]
  // struct Point {
  //     x: isize,
  //     y: isize,
  // }
  // let arr_points = [Point { x: 1, y: 2 }, Point { x: 11, y: 23 }];
  // // for Point { x, y } in arr_points {
  // //     println!("x : {} and y : {}", x, y);
  // // }
  // for Point { x, y } in &arr_points {
  //     println!("x : {} and y : {}", x, y);
  // }
  // // for point in &arr_points {
  // //     println!("x : {:?}", point);
  // // }

  // let mut p: [Option<Point>; 2] = [None, None]; // Some or None

  // p[0] = Some(Point { x: 1, y: 2 });
  // p[1] = Some(Point { x: 2, y: 3 });

  //read file

  // let file = std::fs::read_to_string("lines");
  // if file.is_err() {
  //     println!("{:?}", file.err().unwrap());
  // } else {
  //     //     // file.unwrap()
  //     //     //     .split("\n")
  //     //     //     .for_each(|line| println!("{}", line));

  //     file.unwrap()
  //         .lines()
  //         .enumerate()
  //         .filter(|(idx, _)| idx % 2 == 0)
  //         .skip(2)
  //         .take(2)
  //         .for_each(|(_, line)| println!("{}", line));
  // }

  // #[derive(PartialEq)]
  // enum Color {
  //   Green,
  //   Blue,
  //   Red,
  //   Yellow,
  // }

  // impl Color {
  //   pub fn is_green(&self) -> bool {
  //     if let Color::Green = self {
  //       return true;
  //     }
  //     return false;
  //   }

  //   pub fn is_green_part(&self) -> bool {
  //     // if *self == Color::Blue || *self == Color::Yellow {
  //     //     return true;
  //     // } else {
  //     //     return false;
  //     // }
  //     match self {
  //       Color::Blue => return true,
  //       Color::Yellow => return true,
  //       _ => return false,
  //     }
  //   }
  // }

  // fn print_color(color: Color) -> () {
  //   match color {
  //     Color::Blue => println!("blue"),
  //     Color::Red => println!("red"),
  //     Color::Green => println!("green"),
  //     Color::Yellow => println!("yellow"),
  //   }
  // }

  // print_color(Color::Red);
  // println!("is this green : {}", Color::Green.is_green());
  // println!("is this green part : {}", Color::Red.is_green_part());

  // #[derive(Debug)]
  // struct Custom {
  //   age: usize,
  //   name: String,
  // }

  // #[derive(Debug)]
  // enum Item {
  //   Num(isize),
  //   Str(String),
  //   Cust(Custom),
  // }

  // // fn append(items: &mut Vec<Item>) -> () {
  // //   items.push(Item::Str("Hello fmt!".into()));
  // // }

  // // let mut test_arr: Vec<Item> = vec![];
  // // test_arr.push(Item::Num(2));
  // // append(&mut test_arr);
  // // test_arr.iter().for_each(|x| println!("{:?}", x));

  // let foo = Item::Num(5);

  // match foo {
  //   Item::Num(num) => println!("this is a number {}", num),
  //   Item::Str(str) => println!("this is a string {}", str),
  //   Item::Cust(Custom { age, .. }) => println!("this is a custom type {}", age),
  // };

  // let foo: Option<i32> = Some(5);
  // if let Some(x) = foo {
  //   println!("test {}", x);
  // }

  // fn test(opt: Option<usize>) -> usize {
  //   // if let Some(x) = opt {
  //   //   return x * 5;
  //   // }
  //   // return 0;
  //   return opt.unwrap_or(0) * 5;
  // }

  // fn test2(opt: Option<usize>) -> Option<usize> {
  //   // return opt.map(|x| x * 5);

  //   let num = opt?;

  //   // same stuff
  //   let num = match opt {
  //     Some(x) => x,
  //     None => return None,
  //   };

  //   println!("doesnt reach here if it is None");
  //   return Some(num * 5);
  // }

  // let x = test2(Some(5));
  // println!("x : {:?}", x)

  // fn practice(nums: Vec<usize>, idx: usize) -> usize {
  //   return nums.get(idx).unwrap_or(&idx) * 10;
  // }
  // println!("pra {}", practice(vec![1, 2, 3], 9)); 1

  // let path = process.argv[2];
  // get 2nd argument

  // let path = std::env::args().nth(1).expect("please give the file path");

  // std::fs::read_to_string(path)
  //   .expect("failed to read file")
  //   .lines()
  //   .for_each(|line| {
  //     if let Ok(parsed) = line.parse::<isize>() {
  //       println!("{}", parsed);
  //     } else {
  //       println!("not a number");
  //     }
  //   });

  // #[derive(Debug)]
  // struct Item {
  //   count: usize,
  // }

  // fn add_one(item: &mut Item) {
  //   item.count += 1;
  // }

  // fn add_two(item: &mut Item) {
  //   item.count += 2;
  // }

  // let mut item = Item { count: 1 };
  // println!("{:?}", item);
  // add_one(&mut item);
  // add_two(&mut item);
  // println!("{:?}", item);

  // fn print_all(items: &Vec<Item>) {
  //   items
  //     .iter()
  //     .for_each(|Item { count }| println!("{:?}", count));
  // }

  // let mut items: Vec<Item> = vec![Item { count: 1 }, Item { count: 2 }];

  // let first = items.get_mut(0).unwrap();
  // // println!("{:?}", first);

  // // print_all(&items);

  // // println!("{:?}", first);

  // let second = items.get_mut(1).unwrap();
  // println!("{:?}", first);

  // let rect = Rect {
  //   height: 2.0,
  //   width: 4.0,
  //   x: 0.0,
  //   y: 0.0,
  // };

  // let rect2 = Rect {
  //   height: 4.0,
  //   width: 2.0,
  //   x: 1.0,
  //   y: 1.0,
  // };

  // let circ = Circle {
  //   radius: 1.0,
  //   x: 0.0,
  //   y: 0.0,
  // };

  // let circ2 = Circle {
  //   radius: 1.0,
  //   x: 10.0,
  //   y: 10.0,
  // };

  // let def_rect = Rect::default();

  // println!("{}", circ.area());
  // println!("{}", rect.area());
  // println!("{}", def_rect);

  // for point in &def_rect {
  //   println!("{:?}", point);
  // }

  // (&def_rect)
  //   .into_iter()
  //   .for_each(|point| println!("{:?}", point));

  // println!("{}", def_rect);

  // println!("{}", circ.collide(&circ2));
  // println!("{}", circ.collide(&rect));
  // println!("{}", rect.collide(&circ2));
  enum Shape {
    Circle(Circle),
    Rect(Rect),
  }

  impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let (shape, data) = s.split_once(" ").unwrap_or(("", ""));
      match shape {
        "rect" => Ok(Shape::Rect(data.parse()?)),
        "circle" => Ok(Shape::Circle(data.parse()?)),
        _ => Err(anyhow::anyhow!("bad shape")),
      }
    }
  }

  impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Shape::Circle(c) => return write!(f, "{}", c),
        Shape::Rect(r) => return write!(f, "{}", r),
      }
    }
  }

  impl Points for &Shape {
    fn points(&self) -> PointIter {
      match self {
        Shape::Circle(c) => return c.points(),
        Shape::Rect(r) => return r.points(),
      }
    }
  }

  impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
      match self {
        Shape::Circle(c) => return c.contains_point(point),
        Shape::Rect(r) => return r.contains_point(point),
      }
    }
  }

  let text = std::fs::read_to_string("shapes").expect("path is not valid");
  let shapes = text
    .lines()
    .filter_map(|shape| shape.parse::<Shape>().ok())
    .collect::<Vec<_>>();

  shapes
    .iter()
    .skip(1)
    .zip(shapes.iter().take(shapes.len() - 1))
    .filter(|(a, b)| a.collide(b))
    .for_each(|(a, b)| println!("{} collides with {}", a, b));
}
