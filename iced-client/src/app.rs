use crate::{ComponentPage, Page, ProjectPage, SingleStrictSelectionList, WorkspacePage};

type PagesList = SingleStrictSelectionList<Box<dyn Page>>;

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    GoTo(String),
}
/// Represents the holding og all apps state. This is the outer-most layer you have control over. title(), update(), view() will be directly put into application.run()
pub struct App {
    pages: PagesList,
}

// Picks Workspacepage as default page
impl Default for App {
    fn default() -> Self {
        Self {
            // Create selectable list of pages
            pages: SingleStrictSelectionList::new(
                vec![
                    Box::new(WorkspacePage::default()),
                    Box::new(ProjectPage::default()),
                    Box::new(ComponentPage::default()),
                ],
                2,
            ),
        }
    }
}

// GETTER / SETTERS
impl App {
    fn get_active_page(&self) -> &dyn Page {
        self.pages.get_selected().as_ref()
    }

    fn select_page(&mut self, index: usize) {
        self.pages.toggle_by_index(index);
    }
    fn select_by_title(&mut self, title: String) {
        let index = self
            .pages
            .iter()
            .position(|x| x.title() == title)
            .expect("Page not found");

        self.select_page(index);
    }
}

// ICED / RENDERING
impl App {
    // Get title of each page
    pub fn title(&self) -> String {
        self.get_active_page().title().into()
    }
    // Update values based on events
    pub fn update(&mut self, event: AppEvent) {
        match event {
            AppEvent::GoTo(title) => self.select_by_title(title),
        }
    }
    // Render the right page
    pub fn view(&self) -> iced::Element<AppEvent> {
        self.get_active_page().show()
    }
}
