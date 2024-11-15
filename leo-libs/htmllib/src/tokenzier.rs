pub struct Tokenizer<'a> {
    src: &'a [char],
    pos: usize,
    state: TokenizerState,
    return_state: TokenizerState,
    current_tag_token: TagToken,
    current_comment_token: CommentToken,
    current_doctype_token: DOCTYPEToken,
    temporary_buffer: String,
}
impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a [char]) -> Tokenizer<'a> {
        Tokenizer {
            src,
            pos: 0,
            state: TokenizerState::DataState,
            return_state: TokenizerState::DataState,
            current_tag_token: TagToken::default(),
            current_comment_token: CommentToken::default(),
            current_doctype_token: DOCTYPEToken::default(),
            temporary_buffer: String::new(),
        }
    }
    pub fn cc(&self) -> char {
        self.src[self.pos - 1]
    }
    pub fn nc(&mut self) -> Option<char> {
        let c = self.src.get(self.pos).cloned();
        self.pos += 1;
        c
    }

    fn reconsume_in(&mut self, state: TokenizerState) {
        self.pos -= 1;
        self.state = state;
    }

    pub fn append_char_to_tag_token_name(&mut self, c: char) {
        self.current_tag_token.name.push(c);
    }

    fn new_attrib_in_tag_token(&mut self, name: String, value: String) {
        self.current_tag_token.attributes.push((name, value));
    }

    fn append_char_to_current_attribute_name(&mut self, c: char) {
        self.current_tag_token
            .attributes
            .last_mut()
            .expect("expected attributes to be at leas of length one before changing names")
            .0
            .push(c);
    }

    fn append_char_to_current_attribute_value(&mut self, c: char) {
        self.current_tag_token
            .attributes
            .last_mut()
            .expect("expected attributes to be at leas of length one before changing names")
            .1
            .push(c);
    }
    fn set_self_closing_flag_of_current_tag_token(&mut self, flag: bool) {
        self.current_tag_token.self_closing = flag;
    }

    fn append_char_to_comment_data(&mut self, c: char) {
        self.current_comment_token.0.push(c);
    }

    fn append_char_to_doctype_token_name(&mut self, c: char) {
        self.current_doctype_token.name.push(c);
    }

    fn append_char_to_doctype_token_public_identifier(&mut self, c: char) {
        self.current_doctype_token
            .public_identifier
            .as_mut()
            .expect("public_identifier of current doctype token should not be none")
            .push(c);
    }

    fn append_char_to_doctype_token_system_identifier(&mut self, c: char) {
        self.current_doctype_token
            .system_identifier
            .as_mut()
            .expect("system_identifier of current doctype token should not be none")
            .push(c);
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Vec<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use TokenizerState::*;
            match self.state {
                // https://html.spec.whatwg.org/multipage/parsing.html#data-state
                DataState => match self.nc() {
                    Some('\u{0026}') => {
                        self.return_state = DataState;
                        self.state = CharacterReferenceState;
                    }
                    Some('\u{003C}') => {
                        self.state = TagOpenState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character(self.cc())]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rcdata-state
                RCDATAState => match self.nc() {
                    Some('\u{0026}') => {
                        self.return_state = RCDATAState;
                        self.state = CharacterReferenceState;
                    }
                    Some('\u{003C}') => {
                        self.state = RCDATALessThanSignState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rawtext-state
                RAWTEXTState => match self.nc() {
                    Some('\u{003C}') => {
                        self.state = RAWTEXTLessThanSignState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-state
                ScriptDataState => match self.nc() {
                    Some('\u{003C}') => {
                        self.state = ScriptDataLessThanSignState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#plaintext-state
                PLAINTEXTState => match self.nc() {
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
                TagOpenState => match self.nc() {
                    Some('\u{0021}') => self.state = MarkupDeclarationOpenState,
                    Some('\u{002F}') => self.state = EndTagOpenState,
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.current_tag_token = TagToken {
                            start: true,
                            ..Default::default()
                        };
                        self.reconsume_in(TagNameState);
                    }
                    Some('\u{003F}') => {
                        // ERROR
                        self.current_comment_token = CommentToken(String::new());
                        self.reconsume_in(BogusCommentState);
                    }
                    None => {
                        return None;
                    }
                    Some(_) => {
                        // ERROR
                        self.reconsume_in(DataState);
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#end-tag-open-state
                EndTagOpenState => match self.nc() {
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.current_tag_token = TagToken::default();
                        self.reconsume_in(TagNameState);
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.state = DataState;
                    }
                    None => {
                        return None;
                    }
                    Some(_) => {
                        // ERROR
                        self.current_comment_token = CommentToken(String::new());
                        self.reconsume_in(BogusCommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#tag-name-state
                TagNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BeforeAttributeNameState
                    }
                    Some('\u{002F}') => self.state = SelfClosingStartTagState,
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::Tag(self.current_tag_token.clone())]);
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_tag_token_name(self.cc().to_ascii_lowercase());
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_tag_token_name('\u{FFFD}');
                    }
                    None => {
                        return None;
                    }
                    Some(_) => {
                        self.append_char_to_tag_token_name(self.cc());
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rcdata-less-than-sign-state
                RCDATALessThanSignState => match self.nc() {
                    Some('\u{002F}') => {
                        self.temporary_buffer = String::new();
                        self.state = RCDATAEndTagOpenState;
                    }
                    _ => {
                        self.reconsume_in(RCDATAState);
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rcdata-end-tag-open-state
                RCDATAEndTagOpenState => match self.nc() {
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.current_tag_token = TagToken::default();
                        self.reconsume_in(RCDATAEndTagNameState);
                    }
                    _ => {
                        self.reconsume_in(RCDATAState);
                        return Some(vec![
                            Token::Character('\u{003C}'),
                            Token::Character('\u{002F}'),
                        ]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rcdata-end-tag-name-state
                RCDATAEndTagNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = BeforeAttributeNameState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{002F}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = SelfClosingStartTagState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{003E}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = DataState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_tag_token_name(self.cc().to_ascii_lowercase());
                        self.temporary_buffer.push(self.cc());
                    }
                    Some('\u{0061}'..='\u{007A}') => {
                        self.append_char_to_tag_token_name(self.cc());
                        self.temporary_buffer.push(self.cc());
                    }
                    _ => {
                        let mut res =
                            vec![Token::Character('\u{003C}'), Token::Character('\u{002F}')];
                        for c in self.temporary_buffer.chars() {
                            res.push(Token::Character(c));
                        }
                        self.reconsume_in(RCDATAState);
                        return Some(res);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rawtext-less-than-sign-state
                RAWTEXTLessThanSignState => match self.nc() {
                    Some('\u{002F}') => {
                        self.temporary_buffer = String::new();
                        self.state = RAWTEXTEndTagOpenState
                    }
                    _ => {
                        self.reconsume_in(RAWTEXTState);
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rawtext-end-tag-open-state
                RAWTEXTEndTagOpenState => match self.nc() {
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.current_tag_token = TagToken::default();
                        self.reconsume_in(RAWTEXTEndTagNameState);
                    }
                    _ => {
                        self.reconsume_in(RAWTEXTState);
                        return Some(vec![
                            Token::Character('\u{003C}'),
                            Token::Character('\u{002F}'),
                        ]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#rawtext-end-tag-name-state
                RAWTEXTEndTagNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = BeforeAttributeNameState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{002F}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = SelfClosingStartTagState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{003E}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = DataState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_tag_token_name(self.cc().to_ascii_lowercase());
                        self.temporary_buffer.push(self.cc());
                    }
                    Some('\u{0061}'..='\u{007A}') => {
                        self.append_char_to_tag_token_name(self.cc());
                        self.temporary_buffer.push(self.cc());
                    }
                    _ => {
                        let mut res =
                            vec![Token::Character('\u{003C}'), Token::Character('\u{002F}')];
                        for c in self.temporary_buffer.chars() {
                            res.push(Token::Character(c));
                        }
                        self.reconsume_in(RAWTEXTState);
                        return Some(res);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-less-than-sign-state
                ScriptDataLessThanSignState => match self.nc() {
                    Some('\u{002F}') => {
                        self.temporary_buffer = String::new();
                        self.state = ScriptDataEndTagOpenState;
                    }
                    Some('\u{0021}') => {
                        self.state = ScriptDataEscapeStartState;
                        return Some(vec![
                            Token::Character('\u{003C}'),
                            Token::Character('\u{0021}'),
                        ]);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataState);
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-open-state
                ScriptDataEndTagOpenState => match self.nc() {
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.current_tag_token = TagToken::default();
                        self.reconsume_in(ScriptDataEndTagNameState);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataState);
                        return Some(vec![
                            Token::Character('\u{003C}'),
                            Token::Character('\u{002F}'),
                        ]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-end-tag-name-state
                ScriptDataEndTagNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = BeforeAttributeNameState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{002F}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = SelfClosingStartTagState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{003E}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = DataState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_tag_token_name(self.cc().to_ascii_lowercase());
                        self.temporary_buffer.push(self.cc());
                    }
                    Some('\u{0061}'..='\u{007A}') => {
                        self.append_char_to_tag_token_name(self.cc());
                        self.temporary_buffer.push(self.cc());
                    }
                    _ => {
                        let mut res =
                            vec![Token::Character('\u{003C}'), Token::Character('\u{002F}')];
                        for c in self.temporary_buffer.chars() {
                            res.push(Token::Character(c));
                        }
                        self.reconsume_in(ScriptDataState);
                        return Some(res);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escape-start-state
                ScriptDataEscapeStartState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = ScriptDataEscapeStartDashState;
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escape-start-dash-state
                ScriptDataEscapeStartDashState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = ScriptDataEscapedDashDashState;
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escaped-state
                ScriptDataEscapedState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = ScriptDataEscapedDashState;
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    Some('\u{003C}') => {
                        self.state = ScriptDataEscapedLessThanSignState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => {
                        return Some(vec![Token::Character(c)]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escaped-dash-state
                ScriptDataEscapedDashState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = ScriptDataEscapedDashDashState;
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    Some('\u{003C}') => {
                        self.state = ScriptDataEscapedLessThanSignState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => {
                        self.state = ScriptDataEscapedState;
                        return Some(vec![Token::Character(c)]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escaped-dash-dash-state
                ScriptDataEscapedDashDashState => match self.nc() {
                    Some('\u{002D}') => {
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    Some('\u{003C}') => {
                        self.state = ScriptDataEscapedLessThanSignState;
                    }
                    Some('\u{003E}') => {
                        self.state = ScriptDataState;
                        return Some(vec![Token::Character('\u{003E}')]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.state = ScriptDataEscapedState;
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        return None;
                    }
                    Some(c) => {
                        self.state = ScriptDataEscapedState;
                        return Some(vec![Token::Character(c)]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escaped-less-than-sign-state
                ScriptDataEscapedLessThanSignState => match self.nc() {
                    Some('\u{002F}') => {
                        self.temporary_buffer = String::new();
                        self.state = ScriptDataEscapedEndTagOpenState;
                    }
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.temporary_buffer = String::new();
                        self.reconsume_in(ScriptDataDoubleEscapeStartState);
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataEscapedState);
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-escaped-end-tag-open-state
                ScriptDataEscapedEndTagOpenState => match self.nc() {
                    // ascii alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}' | '\u{0061}'..='\u{007A}') => {
                        self.current_tag_token = TagToken::default();
                        self.reconsume_in(ScriptDataEscapedEndTagNameState);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataEscapedState);
                        return Some(vec![
                            Token::Character('\u{003C}'),
                            Token::Character('\u{002F}'),
                        ]);
                    }
                },
                ScriptDataEscapedEndTagNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = BeforeAttributeNameState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{002F}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = SelfClosingStartTagState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{003E}') => {
                        if
                        /* TODO: appropriate end tag token */
                        true {
                            self.state = DataState;
                        } else {
                            // Same as anything else entry
                        }
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_tag_token_name(self.cc().to_ascii_lowercase());
                        self.temporary_buffer.push(self.cc());
                    }
                    Some('\u{0061}'..='\u{007A}') => {
                        self.append_char_to_tag_token_name(self.cc());
                        self.temporary_buffer.push(self.cc());
                    }
                    _ => {
                        let mut res =
                            vec![Token::Character('\u{003C}'), Token::Character('\u{002F}')];
                        for c in self.temporary_buffer.chars() {
                            res.push(Token::Character(c));
                        }
                        self.reconsume_in(ScriptDataEscapedState);
                        return Some(res);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-double-escape-start-state
                ScriptDataDoubleEscapeStartState => match self.nc() {
                    Some(
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{002F}' | '\u{003E}',
                    ) => {
                        if self.temporary_buffer == "script" {
                            self.state = ScriptDataDoubleEscapedState;
                        } else {
                            self.state = ScriptDataEscapedState;
                            return Some(vec![Token::Character(self.cc())]);
                        }
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_tag_token_name(self.cc().to_ascii_lowercase());
                        self.temporary_buffer.push(self.cc());
                    }
                    Some('\u{0061}'..='\u{007A}') => {
                        self.append_char_to_tag_token_name(self.cc());
                        self.temporary_buffer.push(self.cc());
                    }
                    _ => {
                        self.reconsume_in(ScriptDataEscapedState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-double-escaped-state
                ScriptDataDoubleEscapedState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = ScriptDataDoubleEscapedDashState;
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    Some('\u{003C}') => {
                        self.state = ScriptDataDoubleEscapedLessThanSignState;
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-double-escaped-dash-state
                ScriptDataDoubleEscapedDashState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = ScriptDataDoubleEscapedDashDashState;
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    Some('\u{003C}') => {
                        self.state = ScriptDataDoubleEscapedLessThanSignState;
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.state = ScriptDataDoubleEscapedState;
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => {
                        self.state = ScriptDataDoubleEscapedState;
                        return Some(vec![Token::Character(c)]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-double-escaped-dash-dash-state
                ScriptDataDoubleEscapedDashDashState => match self.nc() {
                    Some('\u{002D}') => {
                        return Some(vec![Token::Character('\u{002D}')]);
                    }
                    Some('\u{003C}') => {
                        self.state = ScriptDataDoubleEscapedLessThanSignState;
                        return Some(vec![Token::Character('\u{003C}')]);
                    }
                    Some('\u{003E}') => {
                        self.state = ScriptDataState;
                        return Some(vec![Token::Character('\u{003E}')]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.state = ScriptDataDoubleEscapedState;
                        return Some(vec![Token::Character('\u{FFFD}')]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => {
                        self.state = ScriptDataDoubleEscapedState;
                        return Some(vec![Token::Character(c)]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-double-escaped-less-than-sign-state
                ScriptDataDoubleEscapedLessThanSignState => match self.nc() {
                    Some('\u{002F}') => {
                        self.temporary_buffer = String::new();
                        self.state = ScriptDataDoubleEscapeEndState;
                        return Some(vec![Token::Character('\u{002F}')]);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataDoubleEscapedState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#script-data-double-escape-end-state
                ScriptDataDoubleEscapeEndState => match self.nc() {
                    Some(
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{002F}' | '\u{003E}',
                    ) => {
                        if self.temporary_buffer == "script" {
                            self.state = ScriptDataEscapedState;
                        } else {
                            self.state = ScriptDataDoubleEscapedState;
                            return Some(vec![Token::Character(self.cc())]);
                        }
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.temporary_buffer.push(self.cc().to_ascii_lowercase());
                        return Some(vec![Token::Character(self.cc())]);
                    }
                    Some('\u{0061}'..='\u{007A}') => {
                        self.temporary_buffer.push(self.cc());
                        return Some(vec![Token::Character(self.cc())]);
                    }
                    _ => {
                        self.reconsume_in(ScriptDataDoubleEscapedState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state
                BeforeAttributeNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{002F}' | '\u{003E}') | None => {
                        self.reconsume_in(AfterAttributeNameState);
                    }
                    Some('\u{003D}') => {
                        // ERROR
                        self.new_attrib_in_tag_token(self.cc().to_string(), String::new());
                        self.state = AttributeNameState;
                    }
                    _ => {
                        self.new_attrib_in_tag_token(String::new(), String::new());
                        self.reconsume_in(AttributeNameState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state
                AttributeNameState => match self.nc() {
                    Some(
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{002F}' | '\u{003E}',
                    )
                    | None => {
                        self.reconsume_in(AfterAttributeNameState);
                    }
                    Some('\u{003D}') => {
                        self.state = BeforeAttributeValueState;
                    }
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_current_attribute_name(self.cc().to_ascii_lowercase());
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_current_attribute_name('\u{FFFD}');
                    }
                    Some('\u{0022}' | '\u{0027}' | '\u{003C}') => {
                        // ERROR treated as "anything else"
                        self.append_char_to_current_attribute_name(self.cc());
                    }
                    Some(c) => {
                        self.append_char_to_current_attribute_name(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-name-state
                AfterAttributeNameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{002F}') => {
                        self.state = SelfClosingStartTagState;
                    }
                    Some('\u{003D}') => {
                        self.state = BeforeAttributeValueState;
                    }
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::Tag(self.current_tag_token.clone())]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(_) => {
                        self.new_attrib_in_tag_token(String::new(), String::new());
                        self.reconsume_in(AttributeNameState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state
                BeforeAttributeValueState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{0022}') => {
                        self.state = AttributeValueDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        self.state = AttributeValueSingleQuotedState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.state = DataState;
                        return Some(vec![Token::Tag(self.current_tag_token.clone())]);
                    }
                    _ => {
                        self.reconsume_in(AttributeValueUnquotedState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state
                AttributeValueDoubleQuotedState => match self.nc() {
                    Some('\u{0022}') => {
                        self.state = AfterAttributeValueQuotedState;
                    }
                    Some('\u{0026}') => {
                        self.return_state = AttributeValueDoubleQuotedState;
                        self.state = CharacterReferenceState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_current_attribute_value('\u{FFFD}');
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => {
                        self.append_char_to_current_attribute_value(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state
                AttributeValueSingleQuotedState => match self.nc() {
                    Some('\u{0027}') => {
                        self.state = AfterAttributeValueQuotedState;
                    }
                    Some('\u{0026}') => {
                        self.return_state = AttributeValueSingleQuotedState;
                        self.state = CharacterReferenceState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_current_attribute_value('\u{FFFD}');
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => {
                        self.append_char_to_current_attribute_value(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(unquoted)-state
                AttributeValueUnquotedState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BeforeAttributeNameState;
                    }
                    Some('\u{0026}') => {
                        self.return_state = AttributeValueUnquotedState;
                        self.state = CharacterReferenceState;
                    }
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::Tag(self.current_tag_token.clone())]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_current_attribute_value('\u{FFFD}');
                    }
                    Some('\u{0022}' | '\u{0027}' | '\u{003C}' | '\u{003D}' | '\u{0060}') => {
                        // ERROR
                        self.append_char_to_current_attribute_value(self.cc());
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => {
                        self.append_char_to_current_attribute_value(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
                AfterAttributeValueQuotedState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BeforeAttributeNameState;
                    }
                    Some('\u{002F}') => {
                        self.state = SelfClosingStartTagState;
                    }
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::Tag(self.current_tag_token.clone())]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(_) => {
                        // ERROR
                        self.reconsume_in(BeforeAttributeNameState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#self-closing-start-tag-state
                SelfClosingStartTagState => match self.nc() {
                    Some('\u{003E}') => {
                        self.set_self_closing_flag_of_current_tag_token(true);
                        self.state = DataState;
                        return Some(vec![Token::Tag(self.current_tag_token.clone())]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(_) => {
                        // ERROR
                        self.reconsume_in(BeforeAttributeNameState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#bogus-comment-state
                BogusCommentState => match self.nc() {
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::Comment(self.current_comment_token.clone())]);
                    }
                    None => {
                        return Some(vec![
                            Token::Comment(self.current_comment_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_comment_data('\u{FFFD}');
                    }
                    Some(c) => {
                        self.append_char_to_comment_data(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#markup-declaration-open-state
                MarkupDeclarationOpenState => {
                    let anything_else = |src: &mut Tokenizer| {
                        // ERROR
                        src.pos -= 1;
                        src.current_comment_token = CommentToken::default();
                        src.state = BogusCommentState;
                    };
                    match self.nc() {
                        Some('\u{002D}') => {
                            if self.nc() == Some('\u{002D}') {
                                self.current_comment_token = CommentToken(String::new());
                                self.state = CommentStartState;
                            } else {
                                anything_else(self);
                            }
                        }
                        Some('D' | 'd') => {
                            for c in "OCTYPE".chars() {
                                let next = self.nc();
                                if next.is_none() || next.unwrap().to_ascii_uppercase() != c {
                                    anything_else(self);
                                }
                            }
                            self.state = DOCTYPEState;
                        }
                        Some('\u{005B}') => {
                            for c in "CDATA[".chars() {
                                let next = self.nc();
                                if next.is_none() || next.unwrap() != c {
                                    anything_else(self);
                                }
                            }
                            if
                            /* TODO: not in html element etc */
                            false {
                                self.state = CDATASectionState;
                            } else {
                                // ERROR
                                self.current_comment_token = CommentToken("[CDATA[".to_owned());
                            }
                        }
                        _ => {
                            anything_else(self);
                        }
                    }
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-start-state
                CommentStartState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = CommentStartDashState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.state = DataState;
                        return Some(vec![Token::Comment(self.current_comment_token.clone())]);
                    }
                    _ => {
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-start-dash-state
                CommentStartDashState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = CommentEndState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.state = DataState;
                        return Some(vec![Token::Comment(self.current_comment_token.clone())]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(_) => {
                        self.append_char_to_comment_data('\u{002D}');
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-state
                CommentState => match self.nc() {
                    Some('\u{003C}') => {
                        self.append_char_to_comment_data(self.cc());
                        self.state = CommentLessThanSignState;
                    }
                    Some('\u{002D}') => {
                        self.state = CommentEndDashState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_comment_data('\u{FFFD}');
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => {
                        self.append_char_to_comment_data(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-less-than-sign-state
                CommentLessThanSignState => match self.nc() {
                    Some('\u{0021}') => {
                        self.append_char_to_comment_data(self.cc());
                        self.state = CommentLessThanSignBangState;
                    }
                    Some('\u{003C}') => {
                        self.append_char_to_comment_data(self.cc());
                    }
                    _ => {
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-less-than-sign-bang-state
                CommentLessThanSignBangState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = CommentLessThanSignBangDashState;
                    }
                    _ => {
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-less-than-sign-bang-dash-state
                CommentLessThanSignBangDashState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = CommentLessThanSignBangDashDashState;
                    }
                    _ => {
                        self.reconsume_in(CommentEndDashState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-less-than-sign-bang-dash-dash-state
                CommentLessThanSignBangDashDashState => match self.nc() {
                    Some('\u{003E}') | None => {
                        self.reconsume_in(CommentEndState);
                    }
                    _ => {
                        // ERROR
                        self.reconsume_in(CommentEndState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-end-dash-state
                CommentEndDashState => match self.nc() {
                    Some('\u{002D}') => {
                        self.state = CommentEndState;
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(_) => {
                        self.append_char_to_comment_data('\u{002D}');
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-end-state
                CommentEndState => match self.nc() {
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::Comment(self.current_comment_token.clone())]);
                    }
                    Some('\u{0021}') => {
                        self.state = CommentEndBangState;
                    }
                    Some('\u{002D}') => {
                        self.append_char_to_comment_data('\u{002D}');
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    _ => {
                        self.append_char_to_comment_data('\u{002D}');
                        self.append_char_to_comment_data('\u{002D}');
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#comment-end-bang-state
                CommentEndBangState => match self.nc() {
                    Some('\u{002D}') => {
                        self.append_char_to_comment_data('\u{002D}');
                        self.append_char_to_comment_data('\u{002D}');
                        self.append_char_to_comment_data('\u{0021}');
                        self.state = CommentEndDashState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.state = DataState;
                        return Some(vec![Token::Comment(self.current_comment_token.clone())]);
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    _ => {
                        self.append_char_to_comment_data('\u{002D}');
                        self.append_char_to_comment_data('\u{002D}');
                        self.append_char_to_comment_data('\u{0021}');
                        self.reconsume_in(CommentState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#doctype-state
                DOCTYPEState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BeforeDOCTYPENameState;
                    }
                    Some('\u{003E}') => {
                        self.reconsume_in(BeforeDOCTYPENameState);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token = DOCTYPEToken {
                            force_quirks: true,
                            ..Default::default()
                        };

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    _ => {
                        // ERROR
                        self.reconsume_in(BeforeDOCTYPENameState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#before-doctype-name-state
                BeforeDOCTYPENameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    // ascii upper alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}') => {
                        self.current_doctype_token = DOCTYPEToken {
                            name: self.cc().to_string(),
                            ..Default::default()
                        };
                        self.state = DOCTYPENameState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.current_doctype_token = DOCTYPEToken {
                            name: '\u{FFFD}'.to_string(),
                            ..Default::default()
                        };
                        self.state = DOCTYPENameState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token = DOCTYPEToken {
                            force_quirks: true,
                            ..Default::default()
                        };
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token = DOCTYPEToken {
                            force_quirks: true,
                            ..Default::default()
                        };

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        self.current_doctype_token = DOCTYPEToken {
                            name: c.to_string(),
                            ..Default::default()
                        };
                        self.state = DOCTYPENameState;
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#doctype-name-state
                DOCTYPENameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = AfterDOCTYPENameState;
                    }
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    // ascii upper alpha code point: https://infra.spec.whatwg.org/#code-points
                    Some('\u{0041}'..='\u{005A}') => {
                        self.append_char_to_doctype_token_name(self.cc().to_ascii_lowercase());
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_doctype_token_name('\u{FFFD}');
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        self.append_char_to_doctype_token_name(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-doctype-name-state
                AfterDOCTYPENameState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        let next5: Vec<_> = (0..5).map(|_| self.nc()).collect();
                        let mut public_flag = true;
                        if c.to_ascii_uppercase() == 'P' {
                            for (c1, c2) in "UBLIC".chars().zip(next5.clone()) {
                                if c2.is_none() || c1 != c2.unwrap().to_ascii_uppercase() {
                                    public_flag = false;
                                }
                            }
                        }
                        if public_flag {
                            self.state = AfterDOCTYPEPublicKeywordState;
                            continue;
                        }
                        let mut system_flag = true;

                        if c.to_ascii_uppercase() == 'S' {
                            for (c1, c2) in "YSTEM".chars().zip(next5) {
                                if c2.is_none() || c1 != c2.unwrap().to_ascii_uppercase() {
                                    system_flag = false;
                                }
                            }
                        }
                        if system_flag {
                            self.state = AfterDOCTYPESystemKeywordState;
                            continue;
                        }
                        // did not exit erlier
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.pos -= 5;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-doctype-public-keyword-state
                AfterDOCTYPEPublicKeywordState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BeforeDOCTYPEPublicIdentifierState;
                    }
                    Some('\u{0022}') => {
                        // ERROR
                        self.current_doctype_token.public_identifier = Some(String::new());
                        self.state = DOCTYPEPublicIdentifierDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        // ERROR
                        self.current_doctype_token.public_identifier = Some(String::new());
                        self.state = DOCTYPEPublicIdentifierSingleQuotedState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#before-doctype-public-identifier-state
                BeforeDOCTYPEPublicIdentifierState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{0022}') => {
                        self.current_doctype_token.public_identifier = Some(String::new());
                        self.state = DOCTYPEPublicIdentifierDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        self.current_doctype_token.public_identifier = Some(String::new());
                        self.state = DOCTYPEPublicIdentifierSingleQuotedState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#doctype-public-identifier-(double-quoted)-state
                DOCTYPEPublicIdentifierDoubleQuotedState => match self.nc() {
                    Some('\u{0022}') => {
                        self.state = AfterDOCTYPEPublicIdentifierState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_doctype_token_public_identifier('\u{FFFD}');
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        self.append_char_to_doctype_token_public_identifier(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#doctype-public-identifier-(single-quoted)-state
                DOCTYPEPublicIdentifierSingleQuotedState => match self.nc() {
                    Some('\u{0027}') => {
                        self.state = AfterDOCTYPEPublicIdentifierState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_doctype_token_public_identifier('\u{FFFD}');
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        self.append_char_to_doctype_token_public_identifier(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-doctype-public-identifier-state
                AfterDOCTYPEPublicIdentifierState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BetweenDOCTYPEPublicAndSystemIdentifiersState;
                    }
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    Some('\u{0022}') => {
                        // ERROR
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        // ERROR
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierSingleQuotedState;
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#between-doctype-public-and-system-identifiers-state
                BetweenDOCTYPEPublicAndSystemIdentifiersState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    Some('\u{0022}') => {
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierSingleQuotedState;
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-doctype-system-keyword-state
                AfterDOCTYPESystemKeywordState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {
                        self.state = BeforeDOCTYPESystemIdentifierState;
                    }
                    Some('\u{0022}') => {
                        // ERROR
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        // ERROR
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierSingleQuotedState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#before-doctype-system-identifier-state
                BeforeDOCTYPESystemIdentifierState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{0022}') => {
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierDoubleQuotedState;
                    }
                    Some('\u{0027}') => {
                        self.current_doctype_token.system_identifier = Some(String::new());
                        self.state = DOCTYPESystemIdentifierSingleQuotedState;
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#doctype-system-identifier-(double-quoted)-state
                DOCTYPESystemIdentifierDoubleQuotedState => match self.nc() {
                    Some('\u{0022}') => {
                        self.state = AfterDOCTYPESystemIdentifierState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_doctype_token_system_identifier('\u{FFFD}');
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        self.append_char_to_doctype_token_system_identifier(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#doctype-system-identifier-(single-quoted)-state
                DOCTYPESystemIdentifierSingleQuotedState => match self.nc() {
                    Some('\u{0027}') => {
                        self.state = AfterDOCTYPESystemIdentifierState;
                    }
                    Some('\u{0000}') => {
                        // ERROR
                        self.append_char_to_doctype_token_system_identifier('\u{FFFD}');
                    }
                    Some('\u{003E}') => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(c) => {
                        self.append_char_to_doctype_token_system_identifier(c);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#after-doctype-system-identifier-state
                AfterDOCTYPESystemIdentifierState => match self.nc() {
                    Some('\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}') => {}
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    None => {
                        // ERROR
                        self.current_doctype_token.force_quirks = true;

                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {
                        // ERROR
                        self.reconsume_in(BogusDOCTYPEState);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#bogus-doctype-state
                BogusDOCTYPEState => match self.nc() {
                    Some('\u{003E}') => {
                        self.state = DataState;
                        return Some(vec![Token::DOCTYPE(self.current_doctype_token.clone())]);
                    }
                    Some('\u{0000}') => {
                        // ERROR
                    }
                    None => {
                        return Some(vec![
                            Token::DOCTYPE(self.current_doctype_token.clone()),
                            Token::EOF,
                        ]);
                    }
                    Some(_) => {}
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#cdata-section-state
                CDATASectionState => match self.nc() {
                    Some('\u{005D}') => {
                        self.state = CDATASectionBracketState;
                    }
                    None => {
                        // ERROR
                        return None;
                    }
                    Some(c) => return Some(vec![Token::Character(c)]),
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#cdata-section-bracket-state
                CDATASectionBracketState => match self.nc() {
                    Some('\u{005D}') => {
                        self.state = CDATASectionEndState;
                    }
                    _ => {
                        self.reconsume_in(CDATASectionState);
                        return Some(vec![Token::Character('\u{005D}')]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#cdata-section-end-state
                CDATASectionEndState => match self.nc() {
                    Some('\u{005D}') => {
                        return Some(vec![Token::Character('\u{005D}')]);
                    }
                    Some('\u{003E}') => {
                        self.state = DataState;
                    }
                    _ => {
                        self.reconsume_in(CDATASectionState);

                        return Some(vec![
                            Token::Character('\u{005D}'),
                            Token::Character('\u{005D}'),
                        ]);
                    }
                },
                // https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
                CharacterReferenceState => {
                    self.temporary_buffer = String::new();
                    self.temporary_buffer.push('\u{0026}');
                    match self.nc() {
                        // ASCII alphanumeric codepoint: https://infra.spec.whatwg.org/#ascii-alphanumeric
                        Some(
                            '\u{0030}'..='\u{0039}'
                            | '\u{0041}'..='\u{005A}'
                            | '\u{0061}'..='\u{007A}',
                        ) => {
                            self.reconsume_in(NamedCharacterReferenceState);
                        }
                        Some('\u{0023}') => {
                            self.temporary_buffer.push(self.cc());
                            self.state = NumericCharacterReferenceState;
                        }
                        _ => {
                            for c in self.temporary_buffer.chars() {
                                return Some(vec![Token::Character(c)]);
                            }
                            self.temporary_buffer = String::new();
                            self.reconsume_in(self.return_state);
                        }
                    }
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#named-character-reference-state
                NamedCharacterReferenceState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#ambiguous-ampersand-state
                AmbiguousAmpersandState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-state
                NumericCharacterReferenceState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#hexadecimal-character-reference-start-state
                HexadecimalCharacterReferenceStartState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#decimal-character-reference-start-state
                DecimalCharacterReferenceStartState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#hexadecimal-character-reference-state
                HexadecimalCharacterReferenceState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#decimal-character-reference-state
                DecimalCharacterReferenceState => {
                    todo!()
                }
                // https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-end-state
                NumericCharacterReferenceEndState => {
                    todo!()
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Character(char),
    EOF,
    Tag(TagToken),
    Comment(CommentToken),
    DOCTYPE(DOCTYPEToken),
}

impl Token {
    #[inline]
    pub fn is_eof(&self) -> bool {
        match self {
            Token::EOF => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_character(&self) -> bool {
        match self {
            Token::Character(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_tag(&self) -> bool {
        match self {
            Token::Tag(_) => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_start_tag(&self) -> bool {
        match self {
            Token::Tag(t) => t.start,
            _ => false,
        }
    }
    #[inline]
    pub fn is_tag_with_name(&self, name: &str) -> bool {
        match self {
            Token::Tag(t) => t.name == name,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DOCTYPEToken {
    name: String,
    force_quirks: bool,
    public_identifier: Option<String>,
    system_identifier: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CommentToken(String);

#[derive(Debug, Clone, Default)]
pub struct TagToken {
    start: bool,
    name: String,
    attributes: Vec<(String, String)>,
    self_closing: bool,
}

#[derive(Debug, Clone, Copy)]
enum TokenizerState {
    DataState,
    CharacterReferenceState,
    TagOpenState,
    RCDATAState,
    RCDATALessThanSignState,
    RAWTEXTState,
    RAWTEXTLessThanSignState,
    ScriptDataState,
    ScriptDataLessThanSignState,
    PLAINTEXTState,
    MarkupDeclarationOpenState,
    EndTagOpenState,
    TagNameState,
    BogusCommentState,
    BeforeAttributeNameState,
    SelfClosingStartTagState,
    RCDATAEndTagOpenState,
    RCDATAEndTagNameState,
    RAWTEXTEndTagOpenState,
    RAWTEXTEndTagNameState,
    ScriptDataEndTagOpenState,
    ScriptDataEndTagNameState,
    ScriptDataEscapeStartState,
    ScriptDataEscapeStartDashState,
    ScriptDataEscapedState,
    ScriptDataEscapedDashState,
    ScriptDataEscapedLessThanSignState,
    ScriptDataEscapedDashDashState,
    ScriptDataEscapedEndTagOpenState,
    ScriptDataDoubleEscapeStartState,
    ScriptDataEscapedEndTagNameState,
    ScriptDataDoubleEscapedState,
    ScriptDataDoubleEscapedDashState,
    ScriptDataDoubleEscapedLessThanSignState,
    ScriptDataDoubleEscapedDashDashState,
    ScriptDataDoubleEscapeEndState,
    AfterAttributeNameState,
    AttributeNameState,
    BeforeAttributeValueState,
    AttributeValueDoubleQuotedState,
    AttributeValueSingleQuotedState,
    AttributeValueUnquotedState,
    AfterAttributeValueQuotedState,
    CommentStartState,
    DOCTYPEState,
    CDATASectionState,
    CommentStartDashState,
    CommentState,
    CommentEndState,
    CommentLessThanSignState,
    CommentLessThanSignBangState,
    CommentLessThanSignBangDashState,
    CommentLessThanSignBangDashDashState,
    CommentEndDashState,
    CommentEndBangState,
    BeforeDOCTYPENameState,
    DOCTYPENameState,
    AfterDOCTYPENameState,
    AfterDOCTYPEPublicKeywordState,
    AfterDOCTYPESystemKeywordState,
    BogusDOCTYPEState,
    BeforeDOCTYPEPublicIdentifierState,
    DOCTYPEPublicIdentifierDoubleQuotedState,
    DOCTYPEPublicIdentifierSingleQuotedState,
    AfterDOCTYPEPublicIdentifierState,
    BetweenDOCTYPEPublicAndSystemIdentifiersState,
    DOCTYPESystemIdentifierDoubleQuotedState,
    DOCTYPESystemIdentifierSingleQuotedState,
    BeforeDOCTYPESystemIdentifierState,
    AfterDOCTYPESystemIdentifierState,
    CDATASectionBracketState,
    CDATASectionEndState,
    NamedCharacterReferenceState,
    NumericCharacterReferenceState,
    AmbiguousAmpersandState,
    HexadecimalCharacterReferenceStartState,
    DecimalCharacterReferenceStartState,
    HexadecimalCharacterReferenceState,
    DecimalCharacterReferenceState,
    NumericCharacterReferenceEndState,
}
