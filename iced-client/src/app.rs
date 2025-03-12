use crate::{
    ComponentPage, ComponentPageEvent, Page, ProjectPage, ProjectPageEvent,
    SingleStrictSelectionList, WorkspacePage, WorkspacePageEvent,
};

type PageList = SingleStrictSelectionList<Box<dyn Page>>;

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
    // Global Events
    GoTo(String),

    // Pages Events
    Workspace(WorkspacePageEvent),
    Project(ProjectPageEvent),
    Component(ComponentPageEvent),
}

/// Represents the holding og all apps state. This is the outer-most layer you have control over. title(), update(), view() will be directly put into application.run()
pub struct App {
    pages: PageList,
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
    fn get_active_page(&self) -> &Box<dyn Page> {
        self.pages.get_selected()
    }
    fn toggle_by_index(&mut self, index: usize) {
        self.pages.toggle_by_index(index);
    }
    fn toggle_by_title(&mut self, title: String) {
        let index = self
            .pages
            .iter()
            .position(|x| x.title() == title)
            .expect("Page not found");

        self.toggle_by_index(index);
    }
    fn get_page_by_title(&self, title: &str) -> &Box<dyn Page> {
        let index = self
            .pages
            .iter()
            .position(|x| x.title() == title)
            .expect("Page not found");

        self.pages.get(index)
    }
    fn get_mut_page_by_title(&mut self, title: &str) -> &mut Box<dyn Page> {
        let index = self
            .pages
            .iter()
            .position(|x| x.title() == title)
            .expect("Page not found");

        self.pages.get_mut(index)
    }
}

// ICED / RENDERING
impl App {
    // Get title of each page
    pub fn title(&self) -> String {
        self.get_active_page().title().into()
    }
    // Update values based on events
    pub fn update(&mut self, app_event: AppEvent) {
        match app_event {
            AppEvent::GoTo(title) => self.toggle_by_title(title),
            AppEvent::Workspace(_) => self
                .get_mut_page_by_title("Workspace".into())
                .run_event(app_event),
            AppEvent::Component(_) => self
                .get_mut_page_by_title("Component".into())
                .run_event(app_event),
        }
    }
    // Render the right page
    pub fn view(&self) -> iced::Element<AppEvent> {
        self.get_active_page().show()
    }
}
