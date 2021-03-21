use html5ever::tree_builder::{TreeSink};
use html5ever::{Attribute, QualName, ExpandedName};
use html5ever::interface::{NodeOrText, QuirksMode, ElementFlags};
use html5ever::tendril::StrTendril;
use std::borrow::Cow;
use std::alloc::Global;

pub struct DefaultSink {

}

impl TreeSink for DefaultSink {
    type Handle = ();
    type Output = ();

    fn finish(self) -> Self::Output {
        unimplemented!()
    }

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        unimplemented!()
    }

    fn get_document(&mut self) -> Self::Handle {
        unimplemented!()
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> ExpandedName<'a> {
        unimplemented!()
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute, Global>, flags: ElementFlags) -> Self::Handle {
        unimplemented!()
    }

    fn create_comment(&mut self, text: StrTendril) -> Self::Handle {
        unimplemented!()
    }

    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> Self::Handle {
        unimplemented!()
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        unimplemented!()
    }

    fn append_based_on_parent_node(&mut self, element: &Self::Handle, prev_element: &Self::Handle, child: NodeOrText<Self::Handle>) {
        unimplemented!()
    }

    fn append_doctype_to_document(&mut self, name: StrTendril, public_id: StrTendril, system_id: StrTendril) {
        unimplemented!()
    }

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        unimplemented!()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        unimplemented!()
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        unimplemented!()
    }

    fn append_before_sibling(&mut self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>) {
        unimplemented!()
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<Attribute, Global>) {
        unimplemented!()
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        unimplemented!()
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        unimplemented!()
    }
}

impl DefaultSink {
    pub fn new() -> DefaultSink{
        DefaultSink {}
    }
}