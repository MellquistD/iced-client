use iced::advanced::layout::{self, Layout, Limits, Node};
use iced::advanced::renderer::{self, Style};
use iced::advanced::widget::tree::State;
use iced::advanced::widget::{self, Tree, Widget};
use iced::mouse::{self, Cursor};
use iced::widget::canvas;
use iced::{border, Renderer, Theme};
use iced::{Color, Element, Length, Rectangle, Size};

pub struct Circle {
    radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 15.0 }
    }
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.radius),
                ..renderer::Quad::default()
            },
            style.text_color,
        );
    }
}

impl<Message, Theme, Renderer> From<Circle> for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(circle: Circle) -> Self {
        Self::new(circle)
    }
}
// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::WHITE);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

// Finally, we simply use our `Circle` to create the `Canvas`!
fn view<'a, Message: 'a>(_state: &'a State) -> iced::Element<'a, Message> {
    canvas(Circle { radius: 50.0 }).into()
}

// Function for accessibility
pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}
