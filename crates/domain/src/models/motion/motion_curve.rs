use serde::{Serialize, Deserialize};

/// Type d’interpolation.
/// Inspiré des courbes d’animation :
/// - Linéaire
/// - Ease In
/// - Ease Out
/// - Ease In-Out
/// - Courbe personnalisée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionCurveType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Custom(String),
}

/// Courbe d’interpolation entre keyframes.
///
/// Ce modèle est pur domaine.
/// Il ne calcule pas la courbe réelle.
/// L’adapter motion-control fera le calcul.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionCurve {
    /// Type de courbe.
    pub curve_type: MotionCurveType,

    /// Valeurs optionnelles pour une courbe personnalisée.
    pub params: Option<Vec<f32>>,
}

impl MotionCurve {
    pub fn new(curve_type: MotionCurveType) -> Self {
        Self {
            curve_type,
            params: None,
        }
    }
}
