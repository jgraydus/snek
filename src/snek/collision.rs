use crate::engine::{Point2d,Rect};
use crate::snek::game::{Direction,Snek};

pub fn check_collision(snek: &Snek, boundary: &Rect) -> bool {

  let dir = snek.direction;
  let length = snek.path.len();
  let head = snek.path.get(length-1).unwrap();

  // make two points for the corners of the head of the snake
  let (h1, h2) = if dir == Direction::Left || dir == Direction::Right {
    (Point2d { x: head.x, y: head.y-5.0 }, Point2d { x: head.x, y: head.y+5.0 })
  } else {
    (Point2d { x: head.x-5.0, y: head.y }, Point2d { x: head.x+5.0, y: head.y })
  };

  // check for self collisions 
  for window in snek.path[0..length-2].windows(2) {
    let p1 = &window[0];
    let p2 = &window[1];
   
    // create a rect form the path segment
    let (x, y, width, height) = (f64::min(p1.x, p2.x),
                                 f64::min(p1.y, p2.y),
                                 (p1.x-p2.x).abs(),
                                 (p1.y-p2.y).abs());
    let rect = if p1.x == p2.x {
      // vertical segment
      Rect::new(x-5.0, y, 10.0, height)
    } else {
      // horizontal segment
      Rect::new(x, y-5.0, width, 10.0)
    };
   
    // if either of the head points are inside the rect, then a collision has occurred
    if rect.contains(&h1) || rect.contains(&h2) {
      return true;
    }
  }

  // check for bounaary collisions
  let rect = Rect::new(boundary.x+5.0, boundary.y+5.0, boundary.width-10.0, boundary.height-10.0);
  if !rect.contains(&h1) || !rect.contains(&h2) {
    return true;
  }

  false
}

