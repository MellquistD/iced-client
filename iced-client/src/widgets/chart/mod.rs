mod candle;

pub use candle::*;
use iced::{
    mouse::{self, Cursor},
    widget::canvas::{self, LineDash, Path, Stroke},
    Color, Element, Point, Rectangle, Renderer, Size, Theme,
};

#[derive(Default)]
pub struct DrawingApp;

pub struct State {
    drawings: Vec<Path>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            drawings: Vec::new(),
        }
    }
}

impl State {
    pub fn add_drawing(&mut self, drawing: Path) {
        self.drawings.push(drawing);
    }
}

impl<Message> canvas::Program<Message> for DrawingApp {
    // No internal state
    type State = State;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(v) => match v {
                mouse::Event::ButtonPressed(v) => match v {
                    mouse::Button::Left => {
                        if let Some(pos) = cursor.position() {
                            println!("{:#?}", pos);
                            state.add_drawing(canvas::Path::circle(pos, 5.0));
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        state: &State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`. For now look at this as the canvas as a square
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Set background color
        frame.fill(
            &canvas::Path::rectangle(frame.top_left(), frame.size() * 5.0),
            Color::from_rgb(0.0, 255.0, 0.0),
        );

        // Set background color
        //frame.fill(
        //    &canvas::Path::rectangle(bounds.position(), bounds.size() * 5.0),
        //    Color::WHITE,
        //);

        // Apply drawings
        for drawing in &state.drawings {
            frame.fill(drawing, Color::from_rgb(255.0, 0.0, 0.0));
        }

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

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

#[cfg(test)]
mod tests {
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
