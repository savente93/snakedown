use super::class::ClassDocumentation;
use super::function::FunctionDocumentation;
use super::module::ModuleDocumentation;

#[derive(Debug)]
pub enum ObjectDocumentation {
    Module(ModuleDocumentation),
    Class(ClassDocumentation),
    Function(FunctionDocumentation),
}
