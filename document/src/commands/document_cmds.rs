//! Whole-document commands used at serialization boundaries.

use super::EditCommand;
use crate::doc::Document;
use crate::work_mode::WorkMode;

/// Replace the open document as one Setup-mode undo step.
pub struct ReplaceDocument {
    other: Document,
}

impl ReplaceDocument {
    pub fn new(other: Document) -> Self {
        Self { other }
    }
}

impl EditCommand for ReplaceDocument {
    fn apply(&mut self, doc: &mut Document) {
        std::mem::swap(doc, &mut self.other);
    }

    fn revert(&mut self, doc: &mut Document) {
        std::mem::swap(doc, &mut self.other);
    }

    fn label(&self) -> &str {
        "Import Project"
    }

    fn requires_mode(&self) -> Option<WorkMode> {
        Some(WorkMode::Setup)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Set the project's existing frame-grid cadence, without retiming any animation.
/// Prepare against the current document and dispatch only a returned command.
#[derive(Debug)]
pub struct SetProjectFps {
    after: u32,
    before: Option<u32>,
}
impl SetProjectFps {
    /// Existing transport editing bounds; these are not new file-format limits.
    pub const MIN: u32 = 1;
    pub const MAX: u32 = 240;
    pub fn new(doc: &Document, fps: u32) -> Result<Option<Self>, String> {
        if !(Self::MIN..=Self::MAX).contains(&fps) {
            return Err("Project FPS must be a whole number from 1 to 240".into());
        }
        Ok((doc.meta.fps != fps).then_some(Self {
            after: fps,
            before: None,
        }))
    }
}
impl EditCommand for SetProjectFps {
    fn apply(&mut self, doc: &mut Document) {
        if self.before.is_none() {
            self.before = Some(doc.meta.fps);
        }
        doc.meta.fps = self.after;
    }
    fn revert(&mut self, doc: &mut Document) {
        if let Some(before) = self.before {
            doc.meta.fps = before;
        }
    }
    fn label(&self) -> &str {
        "Set Project FPS"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod t828_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edit::Edit;

    #[test]
    fn replacing_a_project_is_one_undo_step() {
        let mut imported = Document::new();
        imported.meta.name = "imported".into();

        let mut edit = Edit::default();
        edit.dispatch(Box::new(ReplaceDocument::new(imported)))
            .expect("setup mode");
        assert_eq!(edit.doc.meta.name, "imported");

        assert!(edit.undo());
        assert_eq!(edit.doc.meta.name, "untitled");
        assert!(!edit.undo(), "the whole import was one command");

        assert!(edit.redo());
        assert_eq!(edit.doc.meta.name, "imported");
    }
}
