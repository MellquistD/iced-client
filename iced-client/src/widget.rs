use iced::advanced::layout::{self, Layout, Limits, Node};
use iced::advanced::renderer::{self, Style};
use iced::advanced::widget::{self, Tree, Widget};
use iced::border;
use iced::mouse::{self, Cursor};
use iced::{Color, Element, Length, Rectangle, Size};

#[derive(Default)]
pub struct Circle {
    radius: f32,
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

// Function for accessibility
pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}
