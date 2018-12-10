use std::cmp;
use std::collections::HashSet;
use std::io::Error;

const INPUT: &str = include_str!("input/10.txt");

fn main() -> Result<(), Error> {
    //let vec = INPUT.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
    let lights = Lights {
        lights: INPUT.lines().map(|n| to_light(&n.to_owned())).collect(),
        ..Default::default()
    };

    //let ans1 = answer1(vec);
    //println!("answer part 1: {}", ans1);
    let ans = find_tightest_grouping(lights);
    println!("answer terminated!: {} {}", ans.max_x, ans.min_x);
    ans.print();
    Ok(())
}

// Turn
// position=< 43975,  22031> velocity=<-4, -2>
//ainto a light
// There's presumeably some nicer way of doing this...
fn to_light(s: &String) -> Light {
    let v = s.split('<').collect::<Vec<&str>>();
    println!("parsing string!: {}", s);
    //
    return Light {
        position: to_tuple(v[1].to_owned()),
        velocity: to_tuple(v[2].to_owned())
    }
}

// turn "43975,  22031>..." into a tuple
fn to_tuple(s: String) -> Tuple {
    let comma_split = s.split(',').collect::<Vec<&str>>();
    println!("to_tuple: comma split 0 is!: {}", comma_split[0]);
    println!("to_tuple: comma split 1 is!: {}", comma_split[1]);
    return Tuple{
    x: (comma_split[0]).trim_start().parse::<i64>().unwrap(),
    y: comma_split[1].trim_start().split('>').collect::<Vec<&str>>()[0].parse().unwrap()
    }
}

struct Tuple {
    x: i64,
    y: i64
}

impl Clone for Tuple {
  fn clone(&self) -> Tuple {
    return  Tuple {
      x: self.x,
      y: self.y
    }
  }
}

struct Light {
    position: Tuple,
    velocity: Tuple,
}

impl Light {
    fn time_step(&self) -> Light {
        return Light{
            position : Tuple{
                x: self.position.x + self.velocity.x,
                y: self.position.y + self.velocity.y
            },
            velocity: self.velocity.clone()
        }
    }
}

impl Lights {
  fn print(&self) -> () {
    let mut lightsOn = HashSet::new();
    for l in self.lights.iter() {
      lightsOn.insert((l.position.x, l.position.y));
    }

    for y in self.min_y..self.max_y+1 {
      for x in self.min_x..self.max_x+1 {
        if lightsOn.contains(&(x,y)) {
          print!("{}", "#");
        } else {
          print!("{}", ".");
        }
      }
      println!("") // linebreak
    }
  }
}

#[derive(Default)]
struct Lights {
    lights: Vec<Light>,
    min_y : i64,
    max_y : i64,
    min_x : i64,
    max_x : i64,
}

//impl fmt::Display for Lights {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    let mut lightsOn = HashSet::new();
//    for l in self.lights {
//      lightsOn.insert((l.position.x, l.position.y));
//    }
//
//    for y in self.min_y..self.max_y {
//      for x in self.min_x..self.max_x {
//        if lightsOn.contains(&(x,y)) {
//          print!("{}", "#");
//        } else {
//          print!("{}", ".");
//        }
//      }
//      println!("\n") // linebreak
//    }
//  }
//}

//impl Default for Lights {
//    fn default() -> Lights {
//        lights: Vec::new(),
//        min_y: i64::max(),
//        max_y: i64::min(),
//        min_x: i64::max(),
//        max_x: i64::min(),
//    }
//}

fn find_tightest_grouping(lights : Lights) -> Lights {
    let mut tightest_x_bound = i64::max_value();
    let mut tightest_y_bound = i64::max_value();
    let mut current_lights = lights;

    let mut timesteps = 0;

    loop {
        let new_lights = time_step(&current_lights);
        let x_bound = new_lights.max_x - new_lights.min_x;
        let y_bound = new_lights.max_y - new_lights.min_y;
        println!("new x bound:  {}", x_bound);
        println!("new y bound:  {}", y_bound);
        if  x_bound < tightest_x_bound {
            tightest_x_bound = x_bound;
            if y_bound < tightest_y_bound {
                tightest_y_bound = y_bound;
            } else {
                println!("tighter in x, but not in y!");
            }
        } else {
            // We're expanding again, so i'm guessing we just saw the message
            // return the old lights which conained the message
            println!("Took {} time steps", timesteps);
            return current_lights
        };

        timesteps += 1;
        current_lights = new_lights
    }
}

fn time_step(lights: &Lights) -> Lights {
    //let mut min_x = i64::max_value();
    //let mut max_x = i64::min_value();
    //let mut min_y = i64::max_value();
    //let mut max_y = i64::min_value();

    let mut new_lights = Lights{lights: Vec::new(), ..Default::default()};

    for l in lights.lights.iter() {
        let new_l = l.time_step();
        new_lights.min_x = cmp::min(new_l.position.x, new_lights.min_x);
        new_lights.max_x = cmp::max(new_l.position.x, new_lights.max_x);
        new_lights.min_y = cmp::min(new_l.position.y, new_lights.min_y);
        new_lights.max_y = cmp::max(new_l.position.y, new_lights.max_y);
        new_lights.lights.push(new_l);
    }

    return new_lights
}
