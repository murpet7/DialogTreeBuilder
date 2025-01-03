mod dialog_tree;
mod input_manager;
mod tree_renderer;
mod state_manager;

use eframe::egui::{self, Context, Ui};
use tree_renderer::TreeRenderer;
use state_manager::StateManager;
use dialog_tree::DialogTree;
use input_manager::InputManager;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Interactable Circle Example",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(DialogTreeBuilder::new(cc)))),
    )
}

#[derive(Default)]
struct DialogTreeBuilder {
    dialog_tree: DialogTree,
    state_manager: StateManager,
    input_manager: InputManager,
    tree_renderer: TreeRenderer,
}

impl eframe::App for DialogTreeBuilder {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Borrow self immutably to call the `run` method
            self.run(ctx, ui);  // `self` is not moved, it's borrowed
        });
    }
}

impl DialogTreeBuilder {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn run(&mut self, ctx: &Context, ui: &Ui) {
        
        self.input_manager.update(&mut self.dialog_tree, &mut self.state_manager, ctx);
        self.tree_renderer.update(&self.dialog_tree, &self.state_manager.state, &self.input_manager, ui);
    }
}