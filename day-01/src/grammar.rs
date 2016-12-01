use Turn;
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__turns {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use Turn;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2c_22(&'input str),
        Term_22L_22(&'input str),
        Term_22R_22(&'input str),
        Termr_23_22_28_5c_5cd_2b_29_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt____turns(Vec<Turn>),
        Ntnum(u64),
        Ntturn(Turn),
        Ntturns(Vec<Turn>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 4, 5, 0, 0,
        // State 1
        -6, 0, 0, 0, 0,
        // State 2
        6, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 8, 0,
        // State 4
        0, 0, 0, 8, 0,
        // State 5
        0, 4, 5, 0, 0,
        // State 6
        -3, 0, 0, 0, 0,
        // State 7
        -2, 0, 0, 0, 0,
        // State 8
        -4, 0, 0, 0, 0,
        // State 9
        -5, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -6,
        -1,
        0,
        0,
        0,
        -3,
        -2,
        -4,
        -5,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 3,
        // State 1
        0, 0, 0, 0,
        // State 2
        0, 0, 0, 0,
        // State 3
        0, 7, 0, 0,
        // State 4
        0, 9, 0, 0,
        // State 5
        0, 0, 10, 0,
        // State 6
        0, 0, 0, 0,
        // State 7
        0, 0, 0, 0,
        // State 8
        0, 0, 0, 0,
        // State 9
        0, 0, 0, 0,
    ];
    pub fn parse_turns<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Turn>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 5 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22L_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22R_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_28_5c_5cd_2b_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Turn>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __turns = turns => ActionFn(0);
                let __sym0 = __pop_Ntturns(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // num = r#"(\\d+)"# => ActionFn(5);
                let __sym0 = __pop_Termr_23_22_28_5c_5cd_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnum(__nt), __end));
                1
            }
            3 => {
                // turn = "L", num => ActionFn(3);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22L_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntturn(__nt), __end));
                2
            }
            4 => {
                // turn = "R", num => ActionFn(4);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22R_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntturn(__nt), __end));
                2
            }
            5 => {
                // turns = turns, ",", turn => ActionFn(1);
                let __sym2 = __pop_Ntturn(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_Ntturns(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntturns(__nt), __end));
                3
            }
            6 => {
                // turns = turn => ActionFn(2);
                let __sym0 = __pop_Ntturn(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntturns(__nt), __end));
                3
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22L_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22L_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22R_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22R_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5c_5cd_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5c_5cd_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____turns<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Turn>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____turns(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u64, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntturn<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Turn, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntturn(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntturns<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Turn>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntturns(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__turns::parse_turns;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        44 => /* ',' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        76 => /* 'L' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        82 => /* 'R' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((3, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Turn>, usize),
) -> Vec<Turn>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Vec<Turn>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, Turn, usize),
) -> Vec<Turn>
{
    { let mut l = l; l.push(r); l }
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, t, _): (usize, Turn, usize),
) -> Vec<Turn>
{
    vec![t]
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u64, usize),
) -> Turn
{
    Turn::Left(n)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u64, usize),
) -> Turn
{
    Turn::Right(n)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u64
{
    u64::from_str(__0).unwrap()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
