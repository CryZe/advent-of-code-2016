use Operation;
use std::str::FromStr;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__operation {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use Operation;
    use std::str::FromStr;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22move_20position_22(&'input str),
        Term_22reverse_20positions_22(&'input str),
        Term_22rotate_20based_20on_20position_20of_20letter_22(&'input str),
        Term_22rotate_20left_22(&'input str),
        Term_22rotate_20right_22(&'input str),
        Term_22swap_20letter_22(&'input str),
        Term_22swap_20position_22(&'input str),
        Term_22through_22(&'input str),
        Term_22to_20position_22(&'input str),
        Term_22with_20letter_22(&'input str),
        Term_22with_20position_22(&'input str),
        Termr_23_22_28_5ba_2dh_5d_29_22_23(&'input str),
        Termr_23_22_28_5c_5cd_2b_29_22_23(&'input str),
        Termr_23_22steps_3f_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt____operation(Operation),
        Ntletter(u8),
        Ntnum(usize),
        Ntoperation(Operation),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -8,
        -2,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -6,
        -7,
        0,
        0,
        -10,
        -3,
        -9,
        -5,
        -4,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 2,
        // State 1
        0, 0, 0, 0,
        // State 2
        0, 0, 10, 0,
        // State 3
        0, 0, 12, 0,
        // State 4
        0, 14, 0, 0,
        // State 5
        0, 0, 16, 0,
        // State 6
        0, 0, 18, 0,
        // State 7
        0, 19, 0, 0,
        // State 8
        0, 0, 21, 0,
        // State 9
        0, 0, 0, 0,
        // State 10
        0, 0, 0, 0,
        // State 11
        0, 0, 0, 0,
        // State 12
        0, 0, 0, 0,
        // State 13
        0, 0, 0, 0,
        // State 14
        0, 0, 0, 0,
        // State 15
        0, 0, 0, 0,
        // State 16
        0, 0, 0, 0,
        // State 17
        0, 0, 0, 0,
        // State 18
        0, 0, 0, 0,
        // State 19
        0, 0, 0, 0,
        // State 20
        0, 0, 0, 0,
        // State 21
        0, 0, 0, 0,
        // State 22
        0, 0, 29, 0,
        // State 23
        0, 0, 31, 0,
        // State 24
        0, 0, 0, 0,
        // State 25
        0, 0, 0, 0,
        // State 26
        0, 32, 0, 0,
        // State 27
        0, 0, 33, 0,
        // State 28
        0, 0, 0, 0,
        // State 29
        0, 0, 0, 0,
        // State 30
        0, 0, 0, 0,
        // State 31
        0, 0, 0, 0,
        // State 32
        0, 0, 0, 0,
    ];
    pub fn parse_operation<
        'input,
    >(
        input: &'input str,
    ) -> Result<Operation, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 15 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22move_20position_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22reverse_20positions_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22rotate_20based_20on_20position_20of_20letter_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22rotate_20left_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22rotate_20right_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22swap_20letter_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22swap_20position_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22through_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22to_20position_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22with_20letter_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22with_20position_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Termr_23_22_28_5ba_2dh_5d_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Termr_23_22_28_5c_5cd_2b_29_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Termr_23_22steps_3f_22_23(__tok0),
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
    ) -> Option<Result<Operation,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __operation = operation => ActionFn(0);
                let __sym0 = __pop_Ntoperation(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // letter = r#"([a-h])"# => ActionFn(9);
                let __sym0 = __pop_Termr_23_22_28_5ba_2dh_5d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntletter(__nt), __end));
                1
            }
            3 => {
                // num = r#"(\\d+)"# => ActionFn(8);
                let __sym0 = __pop_Termr_23_22_28_5c_5cd_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnum(__nt), __end));
                2
            }
            4 => {
                // operation = "swap position", num, "with position", num => ActionFn(1);
                let __sym3 = __pop_Ntnum(__symbols);
                let __sym2 = __pop_Term_22with_20position_22(__symbols);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22swap_20position_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            5 => {
                // operation = "swap letter", letter, "with letter", letter => ActionFn(2);
                let __sym3 = __pop_Ntletter(__symbols);
                let __sym2 = __pop_Term_22with_20letter_22(__symbols);
                let __sym1 = __pop_Ntletter(__symbols);
                let __sym0 = __pop_Term_22swap_20letter_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            6 => {
                // operation = "rotate left", num, r#"steps?"# => ActionFn(3);
                let __sym2 = __pop_Termr_23_22steps_3f_22_23(__symbols);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22rotate_20left_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            7 => {
                // operation = "rotate right", num, r#"steps?"# => ActionFn(4);
                let __sym2 = __pop_Termr_23_22steps_3f_22_23(__symbols);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22rotate_20right_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            8 => {
                // operation = "rotate based on position of letter", letter => ActionFn(5);
                let __sym1 = __pop_Ntletter(__symbols);
                let __sym0 = __pop_Term_22rotate_20based_20on_20position_20of_20letter_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            9 => {
                // operation = "reverse positions", num, "through", num => ActionFn(6);
                let __sym3 = __pop_Ntnum(__symbols);
                let __sym2 = __pop_Term_22through_22(__symbols);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22reverse_20positions_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            10 => {
                // operation = "move position", num, "to position", num => ActionFn(7);
                let __sym3 = __pop_Ntnum(__symbols);
                let __sym2 = __pop_Term_22to_20position_22(__symbols);
                let __sym1 = __pop_Ntnum(__symbols);
                let __sym0 = __pop_Term_22move_20position_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::Ntoperation(__nt), __end));
                3
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 4 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22move_20position_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22move_20position_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22reverse_20positions_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22reverse_20positions_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22rotate_20based_20on_20position_20of_20letter_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22rotate_20based_20on_20position_20of_20letter_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22rotate_20left_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22rotate_20left_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22rotate_20right_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22rotate_20right_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22swap_20letter_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22swap_20letter_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22swap_20position_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22swap_20position_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22through_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22through_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22to_20position_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22to_20position_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22with_20letter_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22with_20letter_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22with_20position_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22with_20position_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5ba_2dh_5d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5ba_2dh_5d_29_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_Termr_23_22steps_3f_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22steps_3f_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_Nt____operation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____operation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntletter<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntletter(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntoperation<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operation, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntoperation(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__operation::parse_operation;
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
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((11, __index + __ch.len_utf8()));
                            __current_state = 2;
                            continue;
                        }
                        109 => /* 'm' */ {
                            __current_state = 3;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 4;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_state = 5;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_state = 6;
                            continue;
                        }
                        119 => /* 'w' */ {
                            __current_state = 7;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 1;
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
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 10;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 11;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 13;
                            continue;
                        }
                        119 => /* 'w' */ {
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        104 => /* 'h' */ {
                            __current_state = 15;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_state = 16;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 17;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1632 ... 1641 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1776 ... 1785 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        1984 ... 1993 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2406 ... 2415 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2534 ... 2543 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2662 ... 2671 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2790 ... 2799 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        2918 ... 2927 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3046 ... 3055 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3174 ... 3183 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3302 ... 3311 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3430 ... 3439 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3558 ... 3567 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3664 ... 3673 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3792 ... 3801 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        3872 ... 3881 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4160 ... 4169 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        4240 ... 4249 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6112 ... 6121 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6160 ... 6169 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6470 ... 6479 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6608 ... 6617 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6784 ... 6793 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6800 ... 6809 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        6992 ... 7001 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7088 ... 7097 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7232 ... 7241 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        7248 ... 7257 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        42528 ... 42537 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43216 ... 43225 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43264 ... 43273 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43472 ... 43481 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43504 ... 43513 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        43600 ... 43609 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        44016 ... 44025 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        65296 ... 65305 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        66720 ... 66729 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69734 ... 69743 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69872 ... 69881 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        69942 ... 69951 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70096 ... 70105 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70384 ... 70393 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        70864 ... 70873 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71248 ... 71257 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71360 ... 71369 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71472 ... 71481 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        71904 ... 71913 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        92768 ... 92777 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        93008 ... 93017 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        120782 ... 120831 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 9;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        118 => /* 'v' */ {
                            __current_state = 18;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        118 => /* 'v' */ {
                            __current_state = 19;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 20;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 21;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        97 => /* 'a' */ {
                            __current_state = 22;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        114 => /* 'r' */ {
                            __current_state = 23;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 24;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 25;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 26;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 27;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        97 => /* 'a' */ {
                            __current_state = 28;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_state = 30;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 31;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_state = 32;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        104 => /* 'h' */ {
                            __current_state = 33;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 34;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        114 => /* 'r' */ {
                            __current_state = 35;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 36;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_match = Some((13, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 38;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        117 => /* 'u' */ {
                            __current_state = 39;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 40;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 41;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_state = 42;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 43;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 44;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        108 => /* 'l' */ {
                            __current_state = 45;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_state = 46;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        103 => /* 'g' */ {
                            __current_state = 47;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                40 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 48;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                41 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        108 => /* 'l' */ {
                            __current_state = 49;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_state = 50;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                42 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 51;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                43 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 52;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                44 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 53;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                45 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 54;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                46 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 55;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                47 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        104 => /* 'h' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 56;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                48 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 57;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                49 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 58;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                50 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 59;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                51 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 60;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                52 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 61;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                53 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        98 => /* 'b' */ {
                            __current_state = 62;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_state = 63;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_state = 64;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                54 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 65;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                55 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 66;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                56 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                57 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 67;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                58 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 68;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                59 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 69;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                60 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 70;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                61 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_state = 71;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                62 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        97 => /* 'a' */ {
                            __current_state = 72;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                63 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 73;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                64 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 74;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                65 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 75;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                66 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 76;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                67 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 77;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                68 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 78;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                69 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 79;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                70 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 80;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                71 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 81;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                72 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 82;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                73 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        102 => /* 'f' */ {
                            __current_state = 83;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                74 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        103 => /* 'g' */ {
                            __current_state = 84;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                75 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 85;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                76 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 86;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                77 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 87;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                78 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 88;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                79 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 89;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                80 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 90;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                81 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 91;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                82 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 92;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                83 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 93;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                84 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        104 => /* 'h' */ {
                            __current_state = 94;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                85 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        114 => /* 'r' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 95;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                86 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 96;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                87 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 97;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                88 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        114 => /* 'r' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 98;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                89 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 99;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                90 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 100;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                91 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 101;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                92 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        100 => /* 'd' */ {
                            __current_state = 102;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                93 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                94 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 103;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                95 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                96 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 104;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                97 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                98 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                99 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 105;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                100 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 106;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                101 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 107;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                102 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 108;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                103 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                104 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 109;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                105 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 110;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                106 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                107 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 111;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                108 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 112;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                109 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                110 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                111 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 113;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                112 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_state = 114;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                113 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_state = 115;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                114 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 116;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                115 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 117;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                116 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        112 => /* 'p' */ {
                            __current_state = 118;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                117 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                118 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 119;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                119 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        115 => /* 's' */ {
                            __current_state = 120;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                120 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 121;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                121 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 122;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                122 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        105 => /* 'i' */ {
                            __current_state = 123;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                123 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 124;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                124 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        110 => /* 'n' */ {
                            __current_state = 125;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                125 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 126;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                126 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        111 => /* 'o' */ {
                            __current_state = 127;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                127 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        102 => /* 'f' */ {
                            __current_state = 128;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                128 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        32 => /* ' ' */ {
                            __current_state = 129;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                129 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        108 => /* 'l' */ {
                            __current_state = 130;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                130 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 131;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                131 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 132;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                132 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        116 => /* 't' */ {
                            __current_state = 133;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                133 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        101 => /* 'e' */ {
                            __current_state = 134;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                134 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        114 => /* 'r' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 135;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                135 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
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
    (_, __0, _): (usize, Operation, usize),
) -> Operation
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, usize, usize),
) -> Operation
{
    Operation::SwapPosition(__0, __1)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, u8, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, u8, usize),
) -> Operation
{
    Operation::SwapLetter(__0, __1)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Operation
{
    Operation::RotateLeft(__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Operation
{
    Operation::RotateRight(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, u8, usize),
) -> Operation
{
    Operation::RotatePosition(__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, usize, usize),
) -> Operation
{
    Operation::ReversePositions(__0, __1)
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, usize, usize),
) -> Operation
{
    Operation::Move(__0, __1)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> usize
{
    usize::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> u8
{
    __0.as_bytes()[0]
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
