use crate::{Page, SingleStrictSelectionList};

#[derive(Debug, Clone)]
pub enum Command {
    GoTo(Page),
}

pub struct App {
    pages: SingleStrictSelectionList<Page>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            pages: SingleStrictSelectionList::new(
                vec![
                    Page::Workspace(crate::WorkspacePage::default()),
                    Page::Project(crate::ProjectPage::default()),
                ],
                0,
            ),
        }
    }
}

// GETTER / SETTERS
impl App {
    fn get_active_page(&self) -> &Page {
        &self.pages.get_selected()
    }

    fn select_active_page(&mut self, page: &Page) {
        // First find the index without holding onto the borrow
        let index = self.pages.iter().position(|p| p == page);

        // Now use the index (if found) with a fresh mutable borrow
        if let Some(idx) = index {
            self.pages.toggle_by_index(idx);
        }
    }
}

// ICED / RENDERING
impl App {
    // Get title of each page
    pub fn title(&self) -> String {
        let screen = match self.get_active_page() {
            Page::Workspace(_) => "Workspace",
            Page::Project(_) => "Projects View",
        };

        format!("{} - Iced", screen)
    }
    // Update values based on events
    pub fn update(&mut self, event: Command) {
        match event {
            Command::GoTo(page) => self.select_active_page(&page),
        }
    }
    // Render the right page
    pub fn view(&self) -> iced::Element<Command> {
        match &self.get_active_page() {
            Page::Workspace(v) => v.show(),
            Page::Project(v) => v.show(),
        }
    }
}
