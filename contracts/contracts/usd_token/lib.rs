#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod usd_token {
    use dex::impls::usd_token::*;
    use brush::contracts::psp22::extensions::metadata::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::string::String;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
    pub struct UsdTokenContract {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22 for UsdTokenContract {}

    impl PSP22Metadata for UsdTokenContract {}

    impl PSP22Internal for UsdTokenContract {}

    impl UsdTokenContract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut UsdTokenContract| {
                instance.metadata.name = Some(String::from("USD TOKEN"));
                instance.metadata.symbol = Some(String::from("USD"));
                instance.metadata.decimals = 10;
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }
    }
}
