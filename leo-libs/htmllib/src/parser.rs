use crate::{
    element::Element,
    html_namespace::{HTML_INTEGRATION_POINT, HTML_NAMESPACE},
    mathml::MATHML_TEXT_INTEGRATION_POINT,
    node::Node,
    tokenzier::Token,
};

pub struct Parser {
    script_nesting_level: usize,
    parser_pause_flag: bool,
    insertion_mode: InsertionMode,
    original_insertion_mode: InsertionMode,
    template_insertion_mode_stack: Vec<InsertionMode>,
    stack_of_open_elements: Vec<Node>,
    active_formatting_elements: Vec<Node>,
    head_element_pointer: Option<()>,
    form_element_pointer: Option<()>,
    scripting_flag: bool,
    frameset_ok_flag: bool,
    foster_parenting: bool,
}

impl Parser {
    // https://html.spec.whatwg.org/multipage/parsing.html#current-template-insertion-mode
    fn current_template_insertion_mode(&self) -> Option<&InsertionMode> {
        self.template_insertion_mode_stack.last()
    }
    // https://html.spec.whatwg.org/multipage/parsing.html#current-node
    fn current_node(&self) -> Option<&Node> {
        self.stack_of_open_elements.last()
    }
    // https://html.spec.whatwg.org/multipage/parsing.html#adjusted-current-node
    fn adjusted_current_node(&self) -> Option<&Node> {
        // TODO: take into account Html fragment parsing algorithm:
        // https://html.spec.whatwg.org/multipage/parsing.html#parsing-html-fragments
        self.current_node()
    }

    // https://html.spec.whatwg.org/multipage/parsing.html#the-stack-of-open-elements
    fn remove_current_node_from_stack_of_open_elements(&mut self) {
        let _current_node = self.stack_of_open_elements.pop();
        // TODO: process internal resource links given the current node's node document:
    }
    // https://html.spec.whatwg.org/multipage/parsing.html#push-onto-the-list-of-active-formatting-elements
    fn push_onto_the_list_of_active_formatting_elements(&mut self, element: Node) {
        self.active_formatting_elements.push(element);
        todo!()
    }
    // https://html.spec.whatwg.org/multipage/parsing.html#reconstruct-the-active-formatting-elements
    fn reconstruct_active_formatting_elements(&mut self) {
        todo!()
    }
    // https://html.spec.whatwg.org/multipage/parsing.html#clear-the-list-of-active-formatting-elements-up-to-the-last-marker
    fn clear_active_formatting_elements_up_to_last_marker(&mut self) {
        todo!()
    }
    // https://html.spec.whatwg.org/multipage/parsing.html#tree-construction-dispatcher
    fn tree_construction_dispatcher(&mut self, next_token: Token) {
        let adj_curr_node = self.adjusted_current_node();

        let foreign_content = if let Some(adj_curr_node) = adj_curr_node {
            if self.stack_of_open_elements.len() == 0 {
                false
            } else if HTML_NAMESPACE.contains(&adj_curr_node.name.as_str()) {
                false
            } else if MATHML_TEXT_INTEGRATION_POINT.contains(&adj_curr_node.name.as_str())
                && next_token.is_start_tag()
                && !next_token.is_tag_with_name("mglyph")
                && !next_token.is_tag_with_name("malignmark")
            {
                false
            } else if MATHML_TEXT_INTEGRATION_POINT.contains(&adj_curr_node.name.as_str())
                && next_token.is_character()
            {
                false
            } else if adj_curr_node.name == "annotation-xml"
                && next_token.is_start_tag()
                && next_token.is_tag_with_name("svg")
            {
                false
            } else if HTML_INTEGRATION_POINT.contains(&adj_curr_node.name.as_str())
                && next_token.is_start_tag()
            {
                false
            } else if HTML_INTEGRATION_POINT.contains(&adj_curr_node.name.as_str())
                && next_token.is_character()
            {
                false
            } else {
                true
            }
        } else {
            true
        };

        if !foreign_content {
            self.process_token(next_token);
        } else {
            self.process_foreign_content(next_token);
        }
    }

    /// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inforeign
    fn process_foreign_content(&mut self, next_token: Token) {}

    fn process_token(&mut self, next_token: Token) {}
}

pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}