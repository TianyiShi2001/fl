use crate::alphabet::{Alphabet, Char};
use crate::output::OutputOpts;
use bitflags::bitflags;
use std::cmp::min;
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug)]
pub struct Font {
    opts: FontOpts,
    comment: String,
    alphabet: Alphabet,
}

enum SmushStatus {
    Valid,
    End,
    Invalid,
}

impl Font {
    pub fn parse(data: &str) -> Self {
        let mut lines = data.lines();
        let header = lines.next().unwrap();
        let opts = FontOpts::parse(header);

        // TODO: check for missing data?

        let mut comment = String::new();
        for _ in 0..opts.num_comment_lines {
            comment.push_str(lines.next().unwrap());
            comment.push('\n');
        }

        let mut alphabet = Alphabet::new();

        for c in [
            32u8, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
            53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
            75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
            97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
            115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 196, 214, 220, 228, 246,
            252, 223,
        ]
        .iter()
        {
            let mut letter = Char::with_height(opts.height);
            for l in letter.0.iter_mut() {
                let mut line_i = lines.next().unwrap().as_bytes().to_vec();
                let end_char = line_i.pop().unwrap();
                while line_i.len() > 0 && line_i[line_i.len() - 1] == end_char {
                    line_i.pop().unwrap();
                }
                unsafe { *l = String::from_utf8_unchecked(line_i) }
            }
            alphabet.0.insert(*c, letter);
        }

        Self {
            opts,
            comment,
            alphabet,
        }
    }

    // pub fn render(&self, txt: &str) -> String {
    //     unimplemented!();
    //     let mut res = String::with_capacity(txt.len() * self.opts.height * 10);
    //     for line in txt.lines() {
    //         res.push_str(&self.render_oneline(line));
    //     }
    //     res
    // }

    // pub fn render_oneline(&self, txt: &str, opts: OutputOpts) -> String {
    //     unimplemented!();
    //     let blank = || vec![String::new(); self.opts.height];

    //     let next_fig_chars = (Vec::<u8>::new(), 0);
    //     let outputFigText = blank();
    //     // if opts.width > 0 && opts.white_space_break {}
    //     for (char_index, &character) in txt.as_bytes().iter().enumerate() {
    //         let is_space = (character as char).is_ascii_whitespace();
    //         let fig_char = self.alphabet.0.get(&character).unwrap();

    //         if opts.fittingRules.hlayout != Layout::FullWidth {
    //             let overlap = 10000;
    //             for row in 0..opts.height {
    //                 overlap = std::cmp::min(overlap, )
    //             }
    //         }
    //     }
    //     // TODO: check ascii
    //     let txt = txt.as_bytes();
    //     let mut res = String::new();
    //     for i in 0..self.opts.height {
    //         for c in txt {
    //             res.push_str(&self.alphabet.0.get(&c).unwrap().0[i]);
    //         }
    //         res.push('\n');
    //     }
    //     res
    // }

    // fn horizontal_smush_length(&self, txt1: &[u8], txt2: &[u8]) -> usize {
    //     unimplemented!();
    //     // getHorizontalSmushLength
    //     let (len1, len2) = (txt1.len(), txt2.len());
    //     let max_dist = len1;
    //     let cur_dist = 1;
    //     let break_after = false;
    //     let valid_smush = false;
    //     let seg1;
    //     let seg2;
    //     if len1 == 0 {return 0}
    //     'distCal: while (cur_dist <= max_dist) {
    //         seg1 = &txt1[len1-cur_dist..cur_dist];
    //         seg2 = &txt2[..min(cur_dist, len2)];
    //         for (&c1, &c2) in seg1.iter().zip(seg2.iter()) {
    //             if c1 != b' ' && c2 != b' ' {
    //                 match self.opts.fitting_rules.hlayout {
    //                     Layout::Fitting => {
    //                         cur_dist -= 1;
    //                         break 'distCal;
    //                     }
    //                     Layout::Smushing => {
    //                         if self.use_universal_smushing('h') {
    //                             if c1 == self.opts.hard_blank || ch2 == self.opts.hard_blank {
    //                                 cur_dist -= 1;
    //                             }
    //                             break 'distCal
    //                         } else {
    //                             break_after = true,
    //                             valid_smush = false,
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     0
    // }

    // fn use_universal_smushing(&self, direction: char) -> bool {
    //     match direction {
    //         'h' => self.opts.fitting_rules.hrules.iter().any(|&a| a == true),
    //         'v' => self.opts.fitting_rules.vrules.iter().any(|&a| a == true)
    //     }
    // }

    /// # main vertical smush routines (excluding rules)
    ///
    /// txt1 - A line of text
    /// txt2 - A line of text
    /// opts - FIGlet options array
    /// About: Takes in two lines of text and returns one of the following:
    /// "valid" - These lines can be smushed together given the current smushing rules
    /// "end" - The lines can be smushed, but we're at a stopping point
    /// "invalid" - The two lines cannot be smushed together
    fn can_vertical_smush(&self, txt1: &str, txt2: &str) -> SmushStatus {
        unimplemented!();
        // if self.opts.fitting_rules.vlayout == Layout::FullWidth {
        //     SmushStatus::Invalid
        // }
    }
}

#[derive(Default, Debug)]
pub struct FontOpts {
    pub hard_blank: char,
    pub height: usize,
    pub baseline: usize,
    pub max_len: usize,
    pub num_comment_lines: usize,
    pub print_direction: u8,           // TODO: bool?
    pub code_tag_count: Option<usize>, // unused?
    pub fitting_rules: FittingRules,
}

impl FontOpts {
    fn parse(header: &str) -> Self {
        let mut header = header.split(' ');
        let hard_blank = header.next().unwrap().chars().nth(5).unwrap();
        let height = header.next().map(|n| n.parse::<usize>().unwrap()).unwrap();
        let baseline = header.next().map(|n| n.parse::<usize>().unwrap()).unwrap();
        let max_len = header.next().map(|n| n.parse::<usize>().unwrap()).unwrap();
        let old_layout = header.next().map(|n| n.parse::<i8>().unwrap()).unwrap();
        let num_comment_lines = header.next().map(|n| n.parse::<usize>().unwrap()).unwrap();
        let print_direction = header.next().map(|n| n.parse::<u8>().unwrap()).unwrap_or(0);
        let full_layout = header
            .next()
            .map(|n| RuleFlags::from_bits(n.parse::<u16>().unwrap()).unwrap());
        let code_tag_count = header.next().map(|n| n.parse::<usize>().unwrap());
        let fitting_rules = FittingRules::from_flags(old_layout, full_layout);
        Self {
            hard_blank,
            height,
            baseline,
            max_len,
            num_comment_lines,
            print_direction,
            code_tag_count,
            fitting_rules,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Layout {
    FullWidth,
    Fitting,
    Smushing,
    // ControlledSmushing,
}

impl Default for Layout {
    fn default() -> Self {
        Self::FullWidth
    }
}

#[derive(Default, Debug)]
pub struct FittingRules {
    hlayout: Layout,
    vlayout: Layout,
    hrules: [bool; 6],
    vrules: [bool; 5],
}

#[rustfmt::skip]
bitflags! {
    struct RuleFlags: u16 {
        const NULL           = 0b0000000000000000;
        const HRULE1         = 0b0000000000000001;
        const HRULE2         = 0b0000000000000010;
        const HRULE3         = 0b0000000000000100;
        const HRULE4         = 0b0000000000001000;
        const HRULE5         = 0b0000000000010000;
        const HRULE6         = 0b0000000000100000;
        const HLAYOUT_FIT    = 0b0000000001000000;
        const HLAYOUT_SMUSH  = 0b0000000010000000;
        const VRULE1         = 0b0000000100000000;
        const VRULE2         = 0b0000001000000000;
        const VRULE3         = 0b0000010000000000;
        const VRULE4         = 0b0000100000000000;
        const VRULE5         = 0b0001000000000000;
        const VLAYOUT_FIT    = 0b0010000000000000;
        const VLAYOUT_SMUSH  = 0b0100000000000000;
    }
}

// 0b0000000000000001 hRule1         0
// 0b0000000000000010 hRule2         1
// 0b0000000000000100 hRule3         2
// 0b0000000000001000 hRule4         3
// 0b0000000000010000 hRule5         4
// 0b0000000000100000 hRule6         5
// 0b0000000001000000 hLayout/Fit    6
// 0b0000000010000000 hLayout/Smush  7
// 0b0000000100000000 vRule1         8
// 0b0000001000000000 vRule2         9
// 0b0000010000000000 vRule3         10
// 0b0000100000000000 vRule4         11
// 0b0001000000000000 vRule5         12
// 0b0010000000000000 vLayout/Fit    13
// 0b0100000000000000 vLayout/Smush  14
// 0b1000000000000000                15

impl FittingRules {
    // Original getSmushingRules
    fn from_flags(oldflags: i8, newflags: Option<RuleFlags>) -> Self {
        let mut hlayout: Layout;
        let mut vlayout: Layout;
        let mut hrules = [false; 6];
        let mut vrules = [false; 5];
        let mut flags: RuleFlags;

        let parse_rules = |flags: RuleFlags| {
            (
                [
                    flags.contains(RuleFlags::HRULE1),
                    flags.contains(RuleFlags::HRULE2),
                    flags.contains(RuleFlags::HRULE3),
                    flags.contains(RuleFlags::HRULE4),
                    flags.contains(RuleFlags::HRULE5),
                    flags.contains(RuleFlags::HRULE6),
                ],
                [
                    flags.contains(RuleFlags::VRULE1),
                    flags.contains(RuleFlags::VRULE2),
                    flags.contains(RuleFlags::VRULE3),
                    flags.contains(RuleFlags::VRULE4),
                    flags.contains(RuleFlags::VRULE5),
                ],
            )
        };

        flags = if let Some(flags) = newflags {
            hlayout = if flags.contains(RuleFlags::HLAYOUT_SMUSH) {
                Layout::Smushing
            } else if flags.contains(RuleFlags::HLAYOUT_FIT) {
                Layout::Fitting
            } else {
                Layout::FullWidth
            };
            vlayout = if flags.contains(RuleFlags::VLAYOUT_SMUSH) {
                Layout::Smushing
            } else if flags.contains(RuleFlags::VLAYOUT_FIT) {
                Layout::Fitting
            } else {
                Layout::FullWidth
            };
            flags
        } else {
            match oldflags {
                -1 => {
                    hlayout = Layout::FullWidth;
                    vlayout = Layout::FullWidth;
                    RuleFlags::empty()
                }
                0 => {
                    hlayout = Layout::Fitting;
                    vlayout = Layout::FullWidth;
                    RuleFlags::empty()
                }
                _ => {
                    hlayout = Layout::Smushing;
                    vlayout = Layout::FullWidth;
                    RuleFlags::from_bits(oldflags as u16).unwrap()
                }
            }
        };
        let (hrules, vrules) = parse_rules(flags);

        Self {
            hlayout,
            vlayout,
            hrules,
            vrules,
        }
    }
}
