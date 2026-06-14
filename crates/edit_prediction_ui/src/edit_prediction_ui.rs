mod edit_prediction_button;

use ui::App;

pub use edit_prediction_button::{
    EditPredictionButton, ToggleMenu, get_available_providers, set_completion_provider,
};

pub fn init(_cx: &mut App) {}
