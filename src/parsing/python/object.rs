use crate::indexing::object_ref::ObjectRef;

use super::class::ClassDocumentation;
use super::function::FunctionDocumentation;
use super::module::ModuleDocumentation;

#[derive(Debug)]
pub enum ObjectDocumentation {
    Module(ModuleDocumentation),
    Class(ClassDocumentation),
    Function(FunctionDocumentation),
}

impl ObjectDocumentation {
    pub fn extract_used_references(&self) -> Vec<ObjectRef> {
        match self {
            ObjectDocumentation::Module(module_documentation) => {
                module_documentation.extract_used_references()
            }
            ObjectDocumentation::Class(class_documentation) => {
                class_documentation.extract_used_references()
            }
            ObjectDocumentation::Function(function_documentation) => {
                function_documentation.extract_used_references()
            }
        }
    }
}
