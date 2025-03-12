use iced::{widget::canvas, Color, Point};

// TODO: Needs to placed somewhere else
pub trait RectangleExt {
    fn top_left(&self) -> Point;
    fn center(&self) -> Point;
    fn top_right(&self) -> Point {
        Point::new(
            self.center().x + (self.center().x - self.top_left().x),
            self.top_left().y,
        )
    }

    fn bottom_left(&self) -> Point {
        Point::new(
            self.top_left().x,
            self.center().y + (self.center().y - self.top_left().y),
        )
    }

    fn bottom_right(&self) -> Point {
        Point::new(
            self.center().x + (self.center().x - self.top_left().x),
            self.center().y + (self.center().y - self.top_left().y),
        )
    }
}

impl RectangleExt for iced::Rectangle {
    fn top_left(&self) -> Point {
        self.position()
    }
    fn center(&self) -> Point {
        self.center()
    }
}

impl RectangleExt for canvas::Frame {
    fn top_left(&self) -> Point {
        Point::new(
            self.center().x - (self.size().width / 2.0),
            self.center().y - (self.size().height / 2.0),
        )
    }
    fn center(&self) -> Point {
        self.center()
    }
}

pub trait PointExt {
    fn point(&self) -> Point;

    fn transform(&self, x: f32, y: f32) -> Point {
        Point::new(self.point().x + x, self.point().y + y)
    }

    fn transform_x(&self, x: f32) -> Point {
        Point::new(self.point().x + x, self.point().y)
    }

    fn transform_y(&self, y: f32) -> Point {
        Point::new(self.point().x, self.point().y + y)
    }
}

impl PointExt for Point {
    fn point(&self) -> Point {
        *self
    }
}

// Define a trait with your additional colors
pub trait ColorExt {
    const GREEN: Color;
    const RED: Color;
    const BLUE: Color;
    const YELLOW: Color;
    const ORANGE: Color;
    const GRAY: Color;
}

// Implement the trait for the Color type
impl ColorExt for Color {
    const GREEN: Color = Color::from_rgb(0.0, 255.0, 0.0);
    const RED: Color = Color::from_rgb(255.0, 0.0, 0.0);
    const BLUE: Color = Color::from_rgb(0.0, 0.0, 255.0);
    const YELLOW: Color = Color::from_rgb(255.0, 255.0, 0.0);
    const ORANGE: Color = Color::from_rgb(255.0, 165.0, 0.0);
    const GRAY: Color = Color::from_rgb(128.0, 128.0, 128.0);
}

#[cfg(test)]
mod tests {
    use iced::Size;

    use super::*;

    #[test]
    fn test_rectangle_ext() {
        let rect = iced::Rectangle::new(Point::new(0.0, 0.0), Size::new(50.0, 50.0));

        assert_eq!(rect.top_left(), Point::new(0.0, 0.0));
        assert_eq!(rect.center(), Point::new(25.0, 25.0));
        assert_eq!(rect.bottom_left(), Point::new(0.0, 50.0));
        assert_eq!(rect.bottom_right(), Point::new(50.0, 50.0));
    }

    #[test]
    fn test_point_ext() {
        let point = Point::new(10.0, 10.0);

        assert_eq!(point.transform(5.0, 5.0), Point::new(15.0, 15.0));
        assert_eq!(point.transform_x(5.0), Point::new(15.0, 10.0));
        assert_eq!(point.transform_y(5.0), Point::new(10.0, 15.0));
    }
}
