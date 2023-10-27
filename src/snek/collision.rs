use crate::engine::{Point2d,Rect};
use crate::snek::entity::{Boundary,Direction,Snek};
use crate::snek::pill::{Pill};

pub trait Collision<Other> {
  type Output;

  fn colliding(&self, other: &Other) -> Self::Output;
}

// self collision
impl Collision<()> for Snek {
  type Output = bool;

  fn colliding(&self, _other: &()) -> Self::Output {
    let dir = self.direction();
    let length = self.path().len();
    let head = self.path().get(length-1).unwrap();

    // make two points for the corners of the head of the snake
    let (h1, h2) = if dir == Direction::Left || dir == Direction::Right {
      (Point2d { x: head.x, y: head.y-5.0 }, Point2d { x: head.x, y: head.y+5.0 })
    } else {
      (Point2d { x: head.x-5.0, y: head.y }, Point2d { x: head.x+5.0, y: head.y })
    };

    // check for self collisions
    for window in self.path()[0..length-2].windows(2) {
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

    false
  }
}

// collision with another snek
impl Collision<Snek> for Snek {
  type Output = bool;

  fn colliding(&self, other: &Snek) -> Self::Output {
    let dir = self.direction();
    let length = self.path().len();
    let head = self.path().get(length-1).unwrap();

    // make two points for the corners of the head of the snake
    let (h1, h2) = if dir == Direction::Left || dir == Direction::Right {
      (Point2d { x: head.x, y: head.y-5.0 }, Point2d { x: head.x, y: head.y+5.0 })
    } else {
      (Point2d { x: head.x-5.0, y: head.y }, Point2d { x: head.x+5.0, y: head.y })
    };

    for window in other.path().windows(2) {
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

    false
  }
}

// collisions with the boundary
impl Collision<Boundary> for Snek {
  type Output = bool;

  fn colliding(&self, boundary: &Boundary) -> Self::Output {
    let dir = self.direction();
    let length = self.path().len();
    let head = self.path().get(length-1).unwrap();

    // make two points for the corners of the head of the snake
    let (h1, h2) = if dir == Direction::Left || dir == Direction::Right {
      (Point2d { x: head.x, y: head.y-5.0 }, Point2d { x: head.x, y: head.y+5.0 })
    } else {
      (Point2d { x: head.x-5.0, y: head.y }, Point2d { x: head.x+5.0, y: head.y })
    };

    // check for bounaary collisions
    let Rect { x, y, width, height } = boundary.rect();
    let rect = Rect::new(x+5.0, y+5.0, width-10.0, height-10.0);
    if !rect.contains(&h1) || !rect.contains(&h2) {
      return true;
    }

    false
  }
}

// check for pill collisions
impl Collision<Vec<Pill>> for Snek {
  type Output = Option<usize>;

  fn colliding(&self, pills: &Vec<Pill>) -> Self::Output {
    let dir = self.direction();
    let length = self.path().len();
    let head = self.path().get(length-1).unwrap();

    // make two points for the corners of the head of the snake
    let (h1, h2) = if dir == Direction::Left || dir == Direction::Right {
      (Point2d { x: head.x, y: head.y-5.0 }, Point2d { x: head.x, y: head.y+5.0 })
    } else {
      (Point2d { x: head.x-5.0, y: head.y }, Point2d { x: head.x+5.0, y: head.y })
    };

    for (i, pill) in pills.iter().enumerate() {
      if pill.contains(&head) || pill.contains(&h1) || pill.contains(&h2) {
        return Some(i);
      }
    }
    None
  }
}

