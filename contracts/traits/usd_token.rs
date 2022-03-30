pub use brush::contracts::traits::{
    psp22::*,
    psp22::extensions::metadata::*
};

#[brush::wrapper]
pub type UsdTokenRef = dyn PSP22 + PSP22Metadata;

#[brush::trait_definition]
pub trait UsdToken: PSP22 + PSP22Metadata {}