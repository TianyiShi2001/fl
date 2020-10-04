use crate::font::FittingRules;

pub struct OutputOpts {
    pub height: usize,
    pub width: usize,
    pub print_direction: u8,
    pub white_space_break: bool,
    pub fittingRules: FittingRules,
}
