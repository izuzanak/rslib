#![allow(dead_code)]

#[macro_use] extern crate err;
#[macro_use] extern crate var;

use var::Var;
use var::Data;
use std::fmt::Write;

const JSON_CREATE_UNSUPPORTED_VARIABLE_TYPE:&str = "json create, unsupported variable type";
const JSON_CREATE_NO_STRING_DICT_KEY:&str = "json create, dictionary key must be string value";
const JSON_PARSE_ERROR_UNRECOGNIZED_TERMINAL:&str = "json parse, unrecognized terminal symbol";
const JSON_PARSE_ERROR_INVALID_SYNTAX:&str = "json parse, invalid json syntax";
const JSON_PARSE_ERROR:&str = "json parse error";

// - parse constants -
const IDX_NOT_EXIST:u32 = std::u32::MAX;

const RULE_CNT:usize = 24;
const RULE_HEAD_IDXS:[u32;RULE_CNT] = [17, 18, 19, 20, 20, 21, 22, 22, 23, 24, 25, 25, 26, 27, 27, 28, 28, 28, 28, 28, 28, 28, 28, 29, ];
const RULE_BODY_LENGTHS:[usize;RULE_CNT] = [1, 2, 2, 1, 2, 1, 3, 1, 3, 2, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];

// - lalr parse table -
const BLANK:u32 = IDX_NOT_EXIST;
macro_rules! shift  { ($e:expr) => ($e) }
macro_rules! reduce { ($e:expr) => (LALR_TABLE_REDUCE_BASE + $e) }
macro_rules! goto   { ($e:expr) => ($e) }

const LALR_TABLE_REDUCE_BASE:u32 = 0x80000000;
const TERMINAL_PLUS_NONTERMINAL_CNT:u32 = 30;
const LALR_STATE_CNT:usize = 32;

const LALR_TABLE:[u32;LALR_STATE_CNT*(TERMINAL_PLUS_NONTERMINAL_CNT as usize)] =
[/*{{{*/
   shift!(11),    shift!(4),    shift!(5),    shift!(8),    shift!(9),   shift!(10),   shift!(16),        BLANK,   shift!(17),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,     goto!(1),     goto!(6),    goto!(12),    goto!(14),        BLANK,        BLANK,     goto!(7),    goto!(13),    goto!(15),        BLANK,     goto!(2),     goto!(3),
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   shift!(18),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(15),        BLANK,  reduce!(15),  reduce!(15),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(15),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(16),        BLANK,  reduce!(16),  reduce!(16),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(16),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(17),        BLANK,  reduce!(17),  reduce!(17),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(17),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(18),        BLANK,  reduce!(18),  reduce!(18),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(18),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(19),        BLANK,  reduce!(19),  reduce!(19),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(19),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(20),        BLANK,  reduce!(20),  reduce!(20),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(20),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(21),        BLANK,  reduce!(21),  reduce!(21),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(21),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(22),        BLANK,  reduce!(22),  reduce!(22),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(22),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(23),        BLANK,  reduce!(23),  reduce!(23),  reduce!(23),        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(23),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   shift!(19),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   shift!(20),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
   shift!(11),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(3),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,    goto!(21),    goto!(22),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,    goto!(23),
   shift!(11),    shift!(4),    shift!(5),    shift!(8),    shift!(9),   shift!(10),   shift!(16),        BLANK,   shift!(17),  reduce!(10),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,     goto!(6),    goto!(12),    goto!(14),        BLANK,        BLANK,     goto!(7),    goto!(13),    goto!(15),    goto!(24),    goto!(25),     goto!(3),
   reduce!(5),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(5),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
  reduce!(12),  reduce!(12),  reduce!(12),  reduce!(12),  reduce!(12),  reduce!(12),  reduce!(12),        BLANK,  reduce!(12),  reduce!(12),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(2),        BLANK,   reduce!(2),   reduce!(2),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(2),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(9),        BLANK,   reduce!(9),   reduce!(9),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(9),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(4),        BLANK,        BLANK,   shift!(26),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(7),        BLANK,        BLANK,   reduce!(7),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   shift!(27),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(11),   shift!(28),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(14),  reduce!(14),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
   shift!(11),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,    goto!(29),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,    goto!(23),
   shift!(11),    shift!(4),    shift!(5),    shift!(8),    shift!(9),   shift!(10),   shift!(16),        BLANK,   shift!(17),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,     goto!(6),    goto!(12),    goto!(14),        BLANK,        BLANK,     goto!(7),    goto!(13),    goto!(15),        BLANK,    goto!(30),     goto!(3),
   shift!(11),    shift!(4),    shift!(5),    shift!(8),    shift!(9),   shift!(10),   shift!(16),        BLANK,   shift!(17),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,     goto!(6),    goto!(12),    goto!(14),        BLANK,        BLANK,     goto!(7),    goto!(13),    goto!(15),        BLANK,    goto!(31),     goto!(3),
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(6),        BLANK,        BLANK,   reduce!(6),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,   reduce!(8),        BLANK,        BLANK,   reduce!(8),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,  reduce!(13),  reduce!(13),        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,        BLANK,
];/*}}}*/

struct LalrStackElement {
    lalr_state:u32,
    terminal_start:usize,
    terminal_end:usize,
}

impl LalrStackElement {
    fn new(state:u32,term_start:usize,term_end:usize) -> LalrStackElement {
        LalrStackElement{lalr_state:state,terminal_start:term_start,terminal_end:term_end}
    }

    fn new_state(state:u32) -> LalrStackElement {
        LalrStackElement{lalr_state:state,terminal_start:0,terminal_end:0}
    }
}

struct TermSource<'a> {
    input_idx:usize,
    source:&'a[u8],
    in_char:u8,
    start:bool,
}

macro_rules! get_next_char {
    ($s:expr) => (
        $s.in_char = if $s.input_idx < $s.source.len() { $s.source[$s.input_idx] } else { 0 }
    )
}

macro_rules! close_char {
    ($s:expr,$e:expr) => (
        if $s.in_char == 0 {
            return $e;
        }

        $s.input_idx += 1;
    )
}

impl<'a> TermSource<'a>
{//{{{
    fn new(source:&[u8]) -> TermSource {
        TermSource{input_idx:0,source:source,in_char:0,start:true}
    }

    fn recognize_terminal(&mut self) -> u32 {
        self.start = true;
        self.state_0()
    }

    // - STATE 0 -
    fn state_0(&mut self) -> u32
    {//{{{
        if !self.start {
            close_char!(self,IDX_NOT_EXIST);
        }
        else {
            self.start = false;
        }

        get_next_char!(self);

        match self.in_char {
            0x00          => self.state_1(),
            0x08 ..= 0x0d => self.state_17(),
            0x20          => self.state_17(),
            0x22          => self.state_2(),
            0x23          => self.state_3(),
            0x2c          => self.state_4(),
            0x2d          => self.state_5(),
            0x2f          => self.state_6(),
            0x30          => self.state_7(),
            0x31 ..= 0x39 => self.state_8(),
            0x3a          => self.state_9(),
            0x5b          => self.state_10(),
            0x5d          => self.state_11(),
            0x66          => self.state_12(),
            0x6e          => self.state_13(),
            0x74          => self.state_14(),
            0x7b          => self.state_15(),
            0x7d          => self.state_16(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 1 -
    fn state_1(&mut self) -> u32
    {//{{{
        close_char!(self,16);

        16
    }//}}}

    // - STATE 2 -
    fn state_2(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x00 ..= 0x21 => self.state_2(),
            0x22          => self.state_18(),
            0x23 ..= 0x5b => self.state_2(),
            0x5c          => self.state_19(),
            0x5d ..= 0xff => self.state_2(),
        }
    }//}}}

    // - STATE 3 -
    fn state_3(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x00 ..= 0x09 => self.state_3(),
            0x0a          => self.state_20(),
            0x0b ..= 0xff => self.state_3(),
        }
    }//}}}

    // - STATE 4 -
    fn state_4(&mut self) -> u32
    {//{{{
        close_char!(self,10);

        10
    }//}}}

    // - STATE 5 -
    fn state_5(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x30          => self.state_7(),
            0x31 ..= 0x39 => self.state_8(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 6 -
    fn state_6(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x2a          => self.state_21(),
            0x2f          => self.state_22(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 7 -
    fn state_7(&mut self) -> u32
    {//{{{
        close_char!(self,1);
        get_next_char!(self);

        match self.in_char {
            0x2e          => self.state_23(),
            0x45          => self.state_24(),
            0x65          => self.state_24(),
              _           => 1
        }
    }//}}}

    // - STATE 8 -
    fn state_8(&mut self) -> u32
    {//{{{
        close_char!(self,1);
        get_next_char!(self);

        match self.in_char {
            0x2e          => self.state_23(),
            0x30 ..= 0x39 => self.state_8(),
            0x45          => self.state_24(),
            0x65          => self.state_24(),
              _           => 1
        }
    }//}}}

    // - STATE 9 -
    fn state_9(&mut self) -> u32
    {//{{{
        close_char!(self,11);

        11
    }//}}}

    // - STATE 10 -
    fn state_10(&mut self) -> u32
    {//{{{
        close_char!(self,8);

        8
    }//}}}

    // - STATE 11 -
    fn state_11(&mut self) -> u32
    {//{{{
        close_char!(self,9);

        9
    }//}}}

    // - STATE 12 -
    fn state_12(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x61          => self.state_25(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 13 -
    fn state_13(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x75          => self.state_26(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 14 -
    fn state_14(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x72          => self.state_27(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 15 -
    fn state_15(&mut self) -> u32
    {//{{{
        close_char!(self,6);

        6
    }//}}}

    // - STATE 16 -
    fn state_16(&mut self) -> u32
    {//{{{
        close_char!(self,7);

        7
    }//}}}

    // - STATE 17 -
    fn state_17(&mut self) -> u32
    {//{{{
        close_char!(self,12);
        get_next_char!(self);

        match self.in_char {
            0x08 ..= 0x0d => self.state_17(),
            0x20          => self.state_17(),
              _           => 12
        }
    }//}}}

    // - STATE 18 -
    fn state_18(&mut self) -> u32
    {//{{{
        close_char!(self,0);

        0
    }//}}}

    // - STATE 19 -
    fn state_19(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x22          => self.state_2(),
            0x2f          => self.state_2(),
            0x5c          => self.state_2(),
            0x62          => self.state_2(),
            0x66          => self.state_2(),
            0x6e          => self.state_2(),
            0x72          => self.state_2(),
            0x74          => self.state_2(),
            0x75          => self.state_28(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 20 -
    fn state_20(&mut self) -> u32
    {//{{{
        close_char!(self,13);

        13
    }//}}}

    // - STATE 21 -
    fn state_21(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x00 ..= 0x29 => self.state_21(),
            0x2a          => self.state_29(),
            0x2b ..= 0xff => self.state_21(),
        }
    }//}}}

    // - STATE 22 -
    fn state_22(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x00 ..= 0x09 => self.state_22(),
            0x0a          => self.state_30(),
            0x0b ..= 0xff => self.state_22(),
        }
    }//}}}

    // - STATE 23 -
    fn state_23(&mut self) -> u32
    {//{{{
        close_char!(self,2);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_23(),
            0x45          => self.state_24(),
            0x65          => self.state_24(),
              _           => 2
        }
    }//}}}

    // - STATE 24 -
    fn state_24(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x2b          => self.state_31(),
            0x2d          => self.state_31(),
            0x30 ..= 0x39 => self.state_32(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 25 -
    fn state_25(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x6c          => self.state_33(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 26 -
    fn state_26(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x6c          => self.state_34(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 27 -
    fn state_27(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x75          => self.state_35(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 28 -
    fn state_28(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_36(),
            0x41 ..= 0x46 => self.state_36(),
            0x61 ..= 0x66 => self.state_36(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 29 -
    fn state_29(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x00 ..= 0x2e => self.state_21(),
            0x2f          => self.state_37(),
            0x30 ..= 0xff => self.state_21(),
        }
    }//}}}

    // - STATE 30 -
    fn state_30(&mut self) -> u32
    {//{{{
        close_char!(self,14);

        14
    }//}}}

    // - STATE 31 -
    fn state_31(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_32(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 32 -
    fn state_32(&mut self) -> u32
    {//{{{
        close_char!(self,2);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_32(),
              _           => 2
        }
    }//}}}

    // - STATE 33 -
    fn state_33(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x73          => self.state_38(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 34 -
    fn state_34(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x6c          => self.state_39(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 35 -
    fn state_35(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x65          => self.state_40(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 36 -
    fn state_36(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_41(),
            0x41 ..= 0x46 => self.state_41(),
            0x61 ..= 0x66 => self.state_41(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 37 -
    fn state_37(&mut self) -> u32
    {//{{{
        close_char!(self,15);

        15
    }//}}}

    // - STATE 38 -
    fn state_38(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x65          => self.state_42(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 39 -
    fn state_39(&mut self) -> u32
    {//{{{
        close_char!(self,5);

        5
    }//}}}

    // - STATE 40 -
    fn state_40(&mut self) -> u32
    {//{{{
        close_char!(self,3);

        3
    }//}}}

    // - STATE 41 -
    fn state_41(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_43(),
            0x41 ..= 0x46 => self.state_43(),
            0x61 ..= 0x66 => self.state_43(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

    // - STATE 42 -
    fn state_42(&mut self) -> u32
    {//{{{
        close_char!(self,4);

        4
    }//}}}

    // - STATE 43 -
    fn state_43(&mut self) -> u32
    {//{{{
        close_char!(self,IDX_NOT_EXIST);
        get_next_char!(self);

        match self.in_char {
            0x30 ..= 0x39 => self.state_2(),
            0x41 ..= 0x46 => self.state_2(),
            0x61 ..= 0x66 => self.state_2(),
              _           => IDX_NOT_EXIST
        }
    }//}}}

}//}}}

#[derive(PartialEq,PartialOrd)]
struct F64 {
    value:f64,
}

impl Eq for F64 {}
impl Ord for F64 {
    fn cmp(&self,other:&Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Parser<'a> {
    source:&'a[u8],
    lalr_stack:Vec<LalrStackElement>,

    var_null:Var,
    var_true:Var,
    var_false:Var,

    integer_map:std::collections::BTreeMap<i64,Var>,
    double_map:std::collections::BTreeMap<F64,Var>,
    string_map:std::collections::BTreeMap<&'a str,Var>,

    strings:Vec<Var>,
    values:Vec<Var>,
    arrays:Vec<Var>,
    objects:Vec<Var>,
}

impl<'a> Parser<'a>
{//{{{
    fn new(source:&[u8]) -> Parser {
        Parser{
            source:source,
            lalr_stack:vec![],
            var_null:var!(blank),
            var_true:var!(true),
            var_false:var!(false),

            integer_map:std::collections::BTreeMap::new(),
            double_map:std::collections::BTreeMap::new(),
            string_map:std::collections::BTreeMap::new(),

            strings:vec![],
            values:vec![],
            arrays:vec![],
            objects:vec![],
        }
    }

    fn parse_source(&mut self) -> Result<Var,err::Error>
    {/*{{{*/
        let mut term_source = TermSource::new(&self.source);

        self.lalr_stack.clear();
        self.lalr_stack.push(LalrStackElement::new_state(0));

        let mut old_input_idx:usize = 0;
        let mut ret_term:u32 = IDX_NOT_EXIST;

        loop {

            while ret_term == IDX_NOT_EXIST {
                old_input_idx = term_source.input_idx;

                ret_term = term_source.recognize_terminal();

                // - ERROR -
                if ret_term == IDX_NOT_EXIST {
                    return err!(JSON_PARSE_ERROR_UNRECOGNIZED_TERMINAL);
                }

                // - skipping of _SKIP_ terminals -
                if ret_term >= 12 && ret_term <= 15 {
                    ret_term = IDX_NOT_EXIST;
                }
            }

            // - retrieve action from table of actions -
            let mut parse_action = LALR_TABLE[(self.lalr_stack.last().unwrap().lalr_state*TERMINAL_PLUS_NONTERMINAL_CNT + ret_term) as usize];

            // - ERROR -
            if parse_action == IDX_NOT_EXIST {
                return err!(JSON_PARSE_ERROR_INVALID_SYNTAX);
            }

            // - action SHIFT -
            if parse_action < LALR_TABLE_REDUCE_BASE {

                // - end on _END_ terminal -
                if ret_term == 16 {
                    break;
                }

                // - push new state to lalr stack -
                self.lalr_stack.push(LalrStackElement::new(parse_action,old_input_idx,term_source.input_idx));
                ret_term = IDX_NOT_EXIST;
            }

            // - action REDUCE -
            else {
                parse_action -= LALR_TABLE_REDUCE_BASE;

                // - call parse action function -
                if ! match parse_action {
                       0 => true,
                       1 => true,
                       2 => true,
                       3 => true,
                       4 => true,
                       5 => self.pa_object_begin(),
                       6 => true,
                       7 => true,
                       8 => self.pa_object_pair(),
                       9 => true,
                      10 => true,
                      11 => true,
                      12 => self.pa_array_begin(),
                      13 => self.pa_array_value(),
                      14 => self.pa_array_value(),
                      15 => self.pa_val_string(),
                      16 => self.pa_val_integer(),
                      17 => self.pa_val_float(),
                      18 => self.pa_val_object(),
                      19 => self.pa_val_array(),
                      20 => self.pa_val_true(),
                      21 => self.pa_val_false(),
                      22 => self.pa_val_null(),
                      23 => self.pa_string(),
                       _ => unreachable!(),
                }

                // - ERROR -
                {
                    return err!(JSON_PARSE_ERROR);
                }

                // - remove rule body from stack -
                let truncate_size = self.lalr_stack.len() - RULE_BODY_LENGTHS[parse_action as usize];
                self.lalr_stack.truncate(truncate_size);

                // - push new state to lalr stack -
                let goto_val:u32 = LALR_TABLE[(self.lalr_stack.last().unwrap().lalr_state*TERMINAL_PLUS_NONTERMINAL_CNT + RULE_HEAD_IDXS[parse_action as usize]) as usize];
                self.lalr_stack.push(LalrStackElement::new_state(goto_val));
            }

        }

        Ok(var!(v(self.values.pop().unwrap())))
    }/*}}}*/
}//}}}

impl<'a> Parser<'a>
{//{{{
    fn pa_null(&mut self) -> bool {
        panic!()
    }

    fn pa_object_begin(&mut self) -> bool {
        self.objects.push(var!({}));
        true
    }

    fn pa_object_pair(&mut self) -> bool {
        self.objects.last().unwrap().to_dict().unwrap().insert(
            self.strings.pop().unwrap(),
            self.values.pop().unwrap());

        true
    }

    fn pa_array_begin(&mut self) -> bool {
        self.arrays.push(var!([]));
        true
    }

    fn pa_array_value(&mut self) -> bool {
        self.arrays.last().unwrap().to_array().unwrap().push(self.values.pop().unwrap());
        true
    }

    fn pa_val_string(&mut self) -> bool {
        self.values.push(self.strings.pop().unwrap());
        true
    }

    fn pa_val_integer(&mut self) -> bool {
        let ref lse = self.lalr_stack.last().unwrap();

        unsafe {
            let const_int = std::str::from_utf8_unchecked(
                &self.source[lse.terminal_start .. lse.terminal_end]).parse::<i64>().unwrap();

            let value = self.integer_map.entry(const_int).or_insert(var!(i(const_int)));
            self.values.push(var!(v(value)));
        }

        true
    }

    fn pa_val_float(&mut self) -> bool {
        let ref lse = self.lalr_stack.last().unwrap();

        unsafe {
            let const_float = std::str::from_utf8_unchecked(
                &self.source[lse.terminal_start .. lse.terminal_end]).parse::<f64>().unwrap();

            let value = self.double_map.entry(F64{value:const_float}).or_insert(var!(f(const_float)));
            self.values.push(var!(v(value)));
        }

        true
    }

    fn pa_val_object(&mut self) -> bool {
        self.values.push(self.objects.pop().unwrap());
        true
    }

    fn pa_val_array(&mut self) -> bool {
        self.values.push(self.arrays.pop().unwrap());
        true
    }

    fn pa_val_true(&mut self) -> bool {
        self.values.push(var!(v(self.var_true)));
        true
    }

    fn pa_val_false(&mut self) -> bool {
        self.values.push(var!(v(self.var_false)));
        true
    }

    fn pa_val_null(&mut self) -> bool {
        self.values.push(var!(v(self.var_null)));
        true
    }

    fn pa_string(&mut self) -> bool
    {//{{{
        let ref lse = self.lalr_stack.last().unwrap();

        let mut idx = lse.terminal_start + 1;
        let idx_end = lse.terminal_end - 1;

        unsafe {
            let slice_str = std::str::from_utf8_unchecked(
                    &self.source[idx .. idx_end]);

            let value = self.string_map.entry(slice_str).or_insert({
                let mut const_str:Vec<u8> = vec![];

                while idx < idx_end {
                    if self.source[idx] == '\\' as u8 {
                        idx += 1;

                        // - process character represented by unicode number -
                        if self.source[idx] == 'u' as u8 {
                            idx += 1;

                            let mut value:u32 = 0;

                            // - retrieve character value -
                            let idx_end = idx + 4;
                            loop {
                                value <<= 4;

                                if self.source[idx] >= '0' as u8 && self.source[idx] <= '9' as u8 {
                                    value += (self.source[idx] - '0' as u8) as u32;
                                }
                                else if self.source[idx] >= 'a' as u8 && self.source[idx] <= 'f' as u8 {
                                    value += (10 + (self.source[idx] - 'a' as u8)) as u32;
                                }
                                else if self.source[idx] >= 'A' as u8 && self.source[idx] <= 'F' as u8 {
                                    value += (10 + (self.source[idx] - 'A' as u8)) as u32;
                                }
                                else
                                {
                                    unreachable!();
                                }

                                idx += 1;
                                if idx >= idx_end {
                                    break;
                                }
                            }

                            // - convert utf16/32 value to utf8 character string -
                            if value <= 0x7f {
                                const_str.push(value as u8);
                            }
                            else if value <= 0x7ff {
                                const_str.push((0xc0 | value >> 6) as u8);
                                const_str.push((0x80 | (value & 0x3f)) as u8);
                            }
                            else if value <= 0xffff {
                                const_str.push((0xe0 |   value >> 12) as u8);
                                const_str.push((0x80 | ((value >>  6)  & 0x3f)) as u8);
                                const_str.push((0x80 |  (value         & 0x3f)) as u8);
                            }
                        }
                        else
                        {
                            const_str.push(match self.source[idx] {
                                 34 =>    '"' as u8,
                                 92 =>   '\\' as u8,
                                 98 => '\x08' as u8,
                                102 => '\x0c' as u8,
                                110 =>   '\n' as u8,
                                114 =>   '\r' as u8,
                                116 =>   '\t' as u8,
                                  _ => unreachable!(),
                            });

                            idx += 1;
                        }
                    }
                    else
                    {
                        const_str.push(self.source[idx]);
                        idx += 1;
                    }
                }

                var!(s(std::str::from_utf8_unchecked(&const_str)))
            });

            self.strings.push(var!(v(value)));
        }

        true
    }//}}}
}//}}}

pub fn parse(source:&str) -> Result<Var,err::Error> {
    Parser::new(source.as_bytes()).parse_source()
}

struct Creator {
    result:String,
    tab_str:String,
    indent_buffer:String,
    indent_size:usize,
}

impl Creator {
    fn new() -> Creator
    {//{{{
        Creator{
            result:String::new(),
            tab_str:String::new(),
            indent_buffer: String::new(),
            indent_size: 0,
        }
    }//}}}

    fn new_nice(tab_str:&str) -> Creator
    {//{{{
        Creator{
            result:String::new(),
            tab_str: String::from(tab_str),
            indent_buffer: String::new(),
            indent_size: 0,
        }
    }//}}}

    fn append_string(&mut self,string:&String)
    {//{{{
        for ch in string.chars() {
            match ch {
                 '"' => { self.result.push('\\'); self.result.push('"'); },
                '\\' => { self.result.push('\\'); self.result.push('\\'); },
              '\x08' => { self.result.push('\\'); self.result.push('b'); },
              '\x0c' => { self.result.push('\\'); self.result.push('f'); },
                '\n' => { self.result.push('\\'); self.result.push('n'); },
                '\r' => { self.result.push('\\'); self.result.push('r'); },
                '\t' => { self.result.push('\\'); self.result.push('t'); },
                   _ => self.result.push(ch),
            }
        }
    }//}}}

    fn create(&mut self,var:&Var) -> Result<&mut Creator,err::Error>
    {//{{{
        match var.data() {
            &Data::Blank => { self.result.push_str("null"); },
            &Data::Bool(value) => { write!(&mut self.result,"{}",value).unwrap(); },
            &Data::Int(value) => { write!(&mut self.result,"{}",value).unwrap(); },
            &Data::Float(value) => { write!(&mut self.result,"{}",value).unwrap(); },
            &Data::String(ref value) => {
                self.result.push('"');
                self.append_string(&value);
                self.result.push('"');
            },

            &Data::Array(ref value) => {
                self.result.push('[');
                
                let mut first = true;
                for item in value {

                    if !first { self.result.push(','); }
                    first = false;

                    match self.create(&item) {
                        Err(err) => return Err(err),
                        _ => {},
                    }
                }

                self.result.push(']');
            },
            &Data::Dict(ref value) => {
                self.result.push('{');

                let mut first = true;
                for (key, item) in value.iter() {

                    if !first { self.result.push(','); }
                    first = false;

                    match key.data() {
                        &Data::String(ref value) => {
                            self.result.push('"');
                            self.append_string(&value);
                            self.result.push('"');
                            self.result.push(':');
                        }
                        _ => return err!(JSON_CREATE_NO_STRING_DICT_KEY),
                    }

                    match self.create(&item) {
                        Err(err) => return Err(err),
                        _ => {},
                    }
                }

                self.result.push('}');
            }
        }

        Ok(self)
    }//}}}

    fn cn_push_tab(&mut self)
    {//{{{
        self.indent_size += self.tab_str.len();
        if self.indent_size > self.indent_buffer.len() {
            self.indent_buffer.push_str(&self.tab_str);
        }
    }//}}}

    fn cn_pop_tab(&mut self) {
        self.indent_size -= self.tab_str.len()
    }

    fn cn_nice_indent(&mut self) {
        self.result.push('\n');
        self.result.push_str(self.indent_buffer.get(0..self.indent_size).unwrap());
    }

    fn create_nice(&mut self,var:&Var) -> Result<&mut Creator,err::Error>
    {//{{{
        match var.data() {
            &Data::Blank => { self.result.push_str("null"); },
            &Data::Bool(value) => { write!(&mut self.result,"{}",value).unwrap(); },
            &Data::Int(value) => { write!(&mut self.result,"{}",value).unwrap(); },
            &Data::Float(value) => { write!(&mut self.result,"{}",value).unwrap(); },
            &Data::String(ref value) => {
                self.result.push('"');
                self.append_string(&value);
                self.result.push('"');
            },

            &Data::Array(ref value) => {
                self.result.push('[');
    
                if !value.is_empty() {
                    self.cn_push_tab();
                    self.cn_nice_indent();

                    let mut first = true;
                    for item in value {

                        if !first {
                            self.result.push(',');
                            self.cn_nice_indent();
                        }
                        first = false;

                        match self.create_nice(&item) {
                            Err(err) => return Err(err),
                            _ => {},
                        }
                    }

                    self.cn_pop_tab();
                    self.cn_nice_indent();
                }

                self.result.push(']');
            },
            &Data::Dict(ref value) => {
                self.result.push('{');

                if !value.is_empty() {
                    self.cn_push_tab();
                    self.cn_nice_indent();

                    let mut first = true;
                    for (key, item) in value.iter() {

                        if !first {
                            self.result.push(',');
                            self.cn_nice_indent();
                        }
                        first = false;

                        match key.data() {
                            &Data::String(ref value) => {
                                self.result.push('"');
                                self.append_string(&value);
                                self.result.push('"');
                                self.result.push(':');
                            }
                            _ => return err!(JSON_CREATE_NO_STRING_DICT_KEY),
                        }

                        match self.create_nice(&item) {
                            Err(err) => return Err(err),
                            _ => {},
                        }
                    }

                    self.cn_pop_tab();
                    self.cn_nice_indent();
                }

                self.result.push('}');
            }
        }

        Ok(self)
    }//}}}

    fn result(&mut self) -> String {
        let mut value = String::new();
        std::mem::swap(&mut self.result,&mut value);
        value
    }
}

pub fn create(var:&Var) -> Result<String,err::Error> {
    match Creator::new().create(var) {
        Ok(ref mut creator) => Ok(creator.result()),
        Err(err) => Err(err),
    }
}

pub fn create_nice(var:&Var,tab_str:&str) -> Result<String,err::Error> {
    match Creator::new_nice(tab_str).create_nice(var) {
        Ok(ref mut creator) => Ok(creator.result()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn parse_t0()
{//{{{
    assert_eq!(parse("null null"),test_err!(JSON_PARSE_ERROR_INVALID_SYNTAX));
    assert_eq!(parse("'Hello'"),test_err!(JSON_PARSE_ERROR_UNRECOGNIZED_TERMINAL));

    println!("{}",parse("{\"value\":null}").unwrap());
    println!("{}",parse("{\"value\":false}").unwrap());
    println!("{}",parse("{\"value\":true}").unwrap());
    println!("{}",parse("{\"value\":123}").unwrap());
    println!("{}",parse("{\"value\":123.45}").unwrap());
    println!("{}",parse("{\"value\":\"Hello world\"}").unwrap());
    println!("{}",parse("{\"value\":[1,2,3]}").unwrap());
    println!("{}",parse("{\"value\":{\"one\":1,\"two\":2,\"three\":3}}").unwrap());

    assert_eq!(parse("null"),Ok(var!(blank)));
    assert_eq!(parse("false"),Ok(var!(false)));
    assert_eq!(parse("true"),Ok(var!(true)));
    assert_eq!(parse("123"),Ok(var!(i(123))));
    assert_eq!(parse("123.45"),Ok(var!(f(123.45))));
    assert_eq!(parse("\"Hello world\""),Ok(var!(s("Hello world"))));
    assert_eq!(parse("[1,2,3]"),Ok(var!([i(1),i(2),i(3)])));
    assert_eq!(parse("{\"one\":1,\"two\":2,\"three\":3}"),
        Ok(var!({s("one"):i(1),s("two"):i(2),s("three"):i(3)})));

    assert_eq!(parse("{\"value\":null}"),Ok(var!({s("value"):blank})));
    assert_eq!(parse("{\"value\":false}"),Ok(var!({s("value"):false})));
    assert_eq!(parse("{\"value\":true}"),Ok(var!({s("value"):true})));
    assert_eq!(parse("{\"value\":123}"),Ok(var!({s("value"):i(123)})));
    assert_eq!(parse("{\"value\":123.45}"),Ok(var!({s("value"):f(123.45)})));
    assert_eq!(parse("{\"value\":\"Hello world\"}"),Ok(var!({s("value"):s("Hello world")})));
    assert_eq!(parse("{\"value\":[1,2,3]}"),Ok(var!({s("value"):[i(1),i(2),i(3)]})));
    assert_eq!(parse("{\"value\":{\"one\":1,\"two\":2,\"three\":3}}"),
        Ok(var!({s("value"):{s("one"):i(1),s("two"):i(2),s("three"):i(3)}})));
}//}}}

#[test]
fn parse_t1()
{//{{{
    let json_str =r#"
{
    "null":null,
    "false":false,
    "true":true,
    "integer":123,
    "float":12.345,
    "string":"Hello world",
    "array":[1,2,3],
    "object":{}
}"#;

    assert_eq!(parse(json_str),Ok(var!(
    {
        s("null"):blank,
        s("false"):false,
        s("true"):true,
        s("integer"):i(123),
        s("float"):f(12.345),
        s("string"):s("Hello world"),
        s("array"):[i(1),i(2),i(3)],
        s("object"):{},
    }
    )));
}//}}}

#[test]
fn create_t0()
{//{{{
    println!("create: {}",create(&var!(blank)).unwrap());
    println!("create: {}",create(&var!(false)).unwrap());
    println!("create: {}",create(&var!(true)).unwrap());
    println!("create: {}",create(&var!(i(123))).unwrap());
    println!("create: {}",create(&var!(f(12.345))).unwrap());
    println!("create: {}",create(&var!(s("Hello world"))).unwrap());
    println!("create: {}",create(&var!([i(1),i(2),i(3),[i(1),i(2),i(3)]])).unwrap());
    println!("create: {}",create(&var!({s("one"):i(1),s("two"):i(2),s("three"):i(3)})).unwrap());

    assert_eq!(create(&var!(blank)).unwrap(),"null");
    assert_eq!(create(&var!(false)).unwrap(),"false");
    assert_eq!(create(&var!(true)).unwrap(),"true");
    assert_eq!(create(&var!(i(123))).unwrap(),"123");
    assert_eq!(create(&var!(f(12.345))).unwrap(),"12.345");
    assert_eq!(create(&var!(s("Hello world"))).unwrap(),"\"Hello world\"");
    assert_eq!(create(&var!([i(1),i(2),i(3),[i(1),i(2),i(3)]])).unwrap(),"[1,2,3,[1,2,3]]");
    assert_eq!(create(&var!({s("one"):i(1),s("two"):i(2),s("three"):i(3)})).unwrap(),"{\"one\":1,\"three\":3,\"two\":2}");
}//}}}

#[test]
fn create_t1()
{//{{{
    assert_eq!(create(&var!({i(1):s("one")})),test_err!(JSON_CREATE_NO_STRING_DICT_KEY));
}//}}}

#[test]
fn create_t2()
{//{{{
    println!("create: {}",create(&var!(s("Hello\tworld\n"))).unwrap());

    assert_eq!(create(&var!(s("Hello\tworld\n"))).unwrap(),"\"Hello\\tworld\\n\"");
}//}}}

#[test]
fn create_nice_t0()
{//{{{
    let tab_str = "  ";
    println!("create nice: {}",create_nice(&var!(blank),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!(false),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!(true),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!(i(123)),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!(f(12.345)),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!(s("Hello world")),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!([i(1),i(2),i(3),[i(1),i(2),i(3)]]),tab_str).unwrap());
    println!("create nice: {}",create_nice(&var!({s("one"):i(1),s("two"):i(2),s("three"):i(3)}),tab_str).unwrap());

    assert_eq!(create_nice(&var!(blank),tab_str).unwrap(),"null");
    assert_eq!(create_nice(&var!(false),tab_str).unwrap(),"false");
    assert_eq!(create_nice(&var!(true),tab_str).unwrap(),"true");
    assert_eq!(create_nice(&var!(i(123)),tab_str).unwrap(),"123");
    assert_eq!(create_nice(&var!(f(12.345)),tab_str).unwrap(),"12.345");
    assert_eq!(create_nice(&var!(s("Hello world")),tab_str).unwrap(),"\"Hello world\"");

    assert_eq!(create_nice(&var!([i(1),i(2),i(3),[i(1),i(2),i(3)]]),tab_str).unwrap(),
            format!("[\n{0}1,\n{0}2,\n{0}3,\n{0}[\n{0}{0}1,\n{0}{0}2,\n{0}{0}3\n{0}]\n]",tab_str));

    assert_eq!(create_nice(&var!({s("one"):i(1),s("two"):i(2),s("three"):i(3)}),tab_str).unwrap(),
            format!("{{\n{0}\"one\":1,\n{0}\"three\":3,\n{0}\"two\":2\n}}",tab_str));

    println!("create nice: {}",create_nice(&var!(
{
    s("one"):i(1),
    s("two"):i(2),
    s("three"):i(3),
    s("array"):[i(1),i(2),i(3)],
    s("object"):{s("one"):i(1),s("two"):i(2),s("three"):i(3)},
}),tab_str).unwrap());

    assert_eq!(create_nice(&var!(
{
    s("one"):i(1),
    s("two"):i(2),
    s("three"):i(3),
    s("array"):[i(1),i(2),i(3)],
    s("object"):{s("one"):i(1),s("two"):i(2),s("three"):i(3)},
}),tab_str).unwrap(),
format!("{{
{0}\"array\":[
{0}{0}1,
{0}{0}2,
{0}{0}3
{0}],
{0}\"object\":{{
{0}{0}\"one\":1,
{0}{0}\"three\":3,
{0}{0}\"two\":2
{0}}},
{0}\"one\":1,
{0}\"three\":3,
{0}\"two\":2
}}",tab_str));
}//}}}

}

