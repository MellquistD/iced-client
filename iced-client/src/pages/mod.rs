mod project;
mod workspace;

pub use project::ProjectPage;
pub use workspace::WorkspacePage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Project(ProjectPage),
    Workspace(WorkspacePage),
}

impl Default for Page {
    fn default() -> Self {
        Page::Project(ProjectPage::default())
    }
}
