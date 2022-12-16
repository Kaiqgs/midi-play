use ggez::graphics::Color;

const ALPHA: f32 = 0.065_484_546;
const BETA: f32 = 0.228_171_81;
const CRETA: f32 = 0.478_171_83;
const ETAT: f32 = 0.771_828_2;
const DELTA: f32 = 0.978_171_8;
pub(crate) const DARK: Color = Color::new(ALPHA, ALPHA, ALPHA, 1.0);
pub(crate) const DARKER_DARK: Color = Color::new(BETA, BETA, BETA, 1.0);
pub(crate) const LIGHTER_DARK: Color = Color::new(CRETA, CRETA, CRETA, 1.0);
pub(crate) const DARKER_LIGHT: Color = Color::new(CRETA, CRETA, CRETA, 1.0);
pub(crate) const LIGHTER_LIGHT: Color = Color::new(ETAT, ETAT, ETAT, 1.0);
pub(crate) const LIGHT: Color = Color::new(DELTA, DELTA, DELTA, 1.0);
