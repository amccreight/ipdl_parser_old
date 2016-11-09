use ast::{IncludeType, Node};
extern crate lalrpop_util as __lalrpop_util;

mod __parse__IncludeStmt {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::{IncludeType, Node};
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22include_22(&'input str),
        Term_22protocol_22(&'input str),
        Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        NtID(String),
        NtIncludeStmt(Node),
        Nt____IncludeStmt(Node),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, // on "include", goto 2
        0, // on "protocol", error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, error
        // State 1
        0, // on "include", error
        0, // on "protocol", error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, error
        // State 2
        0, // on "include", error
        5, // on "protocol", goto 4
        6, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, goto 5
        // State 3
        0, // on "include", error
        0, // on "protocol", error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, error
        // State 4
        0, // on "include", error
        0, // on "protocol", error
        6, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, goto 5
        // State 5
        0, // on "include", error
        0, // on "protocol", error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, error
        // State 6
        0, // on "include", error
        0, // on "protocol", error
        0, // on r#"[a-zA-Z_][a-zA-Z0-9_]*"#, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0, // on EOF, error
        -4, // on EOF, reduce `__IncludeStmt = IncludeStmt => ActionFn(0);`
        0, // on EOF, error
        -3, // on EOF, reduce `IncludeStmt = "include", ID => ActionFn(2);`
        0, // on EOF, error
        -1, // on EOF, reduce `ID = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(3);`
        -2, // on EOF, reduce `IncludeStmt = "include", "protocol", ID => ActionFn(1);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on ID, error
        2, // on IncludeStmt, goto 1
        0, // on __IncludeStmt, error
        // State 1
        0, // on ID, error
        0, // on IncludeStmt, error
        0, // on __IncludeStmt, error
        // State 2
        4, // on ID, goto 3
        0, // on IncludeStmt, error
        0, // on __IncludeStmt, error
        // State 3
        0, // on ID, error
        0, // on IncludeStmt, error
        0, // on __IncludeStmt, error
        // State 4
        7, // on ID, goto 6
        0, // on IncludeStmt, error
        0, // on __IncludeStmt, error
        // State 5
        0, // on ID, error
        0, // on IncludeStmt, error
        0, // on __IncludeStmt, error
        // State 6
        0, // on ID, error
        0, // on IncludeStmt, error
        0, // on __IncludeStmt, error
    ];
    pub fn parse_IncludeStmt<
        'input,
    >(
        input: &'input str,
    ) -> Result<Node, __lalrpop_util::ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            let __integer = match __lookahead {
                (_, (0, _), _) if true => 0,
                (_, (1, _), _) if true => 1,
                (_, (2, _), _) if true => 2,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 3 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22include_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22protocol_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__tok0),
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
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
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
    ) -> Option<Result<Node,__lalrpop_util::ParseError<usize,(usize, &'input str),()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ID = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtID(__nt), __end));
                0
            }
            2 => {
                // IncludeStmt = "include", "protocol", ID => ActionFn(1);
                let __sym2 = __pop_NtID(__symbols);
                let __sym1 = __pop_Term_22protocol_22(__symbols);
                let __sym0 = __pop_Term_22include_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtIncludeStmt(__nt), __end));
                1
            }
            3 => {
                // IncludeStmt = "include", ID => ActionFn(2);
                let __sym1 = __pop_NtID(__symbols);
                let __sym0 = __pop_Term_22include_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtIncludeStmt(__nt), __end));
                1
            }
            4 => {
                // __IncludeStmt = IncludeStmt => ActionFn(0);
                let __sym0 = __pop_NtIncludeStmt(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 3 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22include_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22include_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22protocol_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22protocol_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIncludeStmt<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIncludeStmt(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____IncludeStmt<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____IncludeStmt(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__IncludeStmt::parse_IncludeStmt;
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
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        97 ... 104 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        105 => /* 'i' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        106 ... 111 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 1;
                            continue;
                        }
                        112 => /* 'p' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        113 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
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
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 110 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        111 => /* 'o' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        112 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
                        48 ... 57 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((2, __index + __ch.len_utf8()));
                            __current_state = 5;
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
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, String, usize),
) -> Node
{
    Node::Include(IncludeType::Protocol, id) /* resolveIncludePath etc. */
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, String, usize),
) -> Node
{
    Node::Include(IncludeType::Header, id)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    String::from(s)
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
