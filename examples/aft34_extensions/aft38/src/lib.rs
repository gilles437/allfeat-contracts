#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT34, Ownable, AFT34Burnable, AFT34Mintable, AFT34Enumerable, AFT34URIStorage, AFT34ALBUMStorage)]
#[allfeat_contracts::contract]
pub mod my_aft38 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        enumerable: enumerable::Data,
        #[storage_field]
        uris: uri_storage::Data,
        #[storage_field]
        albums: album_storage::Data,
    }

    impl Contract {
        // #[ink(constructor)]
        // pub fn new() -> Self {
        //     Self::default()
        // }

        /// A constructor which set the base uri of the collection.
        #[ink(constructor)]
        pub fn new(base_uri: Option<URI>, base_album: Option<ALBUMID>) -> Self {
            let mut instance = Default::default();
            uri_storage::Internal::_set_base_uri(&mut instance, base_uri);
            album_storage::Internal::_set_base_album(&mut instance, base_album);
            instance
        }
        #[ink(message)]
        pub fn set_token_uri(&mut self, token_id: Id, token_uri: URI) -> Result<(), AFT34Error> {
            uri_storage::Internal::_set_token_uri(self, token_id, token_uri)?;
            Ok(())
        }
        #[ink(message)]
        pub fn set_token_album(&mut self, token_id: Id, token_album: ALBUMID) -> Result<(), AFT34Error> {
            album_storage::Internal::_set_token_album(self, token_id, token_album)?;
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft34::extensions::{
            burnable::aft34burnable_external::AFT34Burnable,
            enumerable::aft34enumerable_external::AFT34Enumerable,
            mintable::aft34mintable_external::AFT34Mintable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::address_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn enumerable_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "my_aft38",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let aft34_id1 = Id::U8(1u8);
            let aft34_id2 = Id::U8(2u8);

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), aft34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), aft34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(aft34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Ok(aft34_id2.clone()));

            let token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(aft34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(aft34_id2.clone()));

            Ok(())
        }

        #[ink_e2e::test]
        async fn enumerable_works_after_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate(
                    "my_aft38",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let aft34_id1 = Id::U8(1u8);
            let aft34_id2 = Id::U8(2u8);

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(alice), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(owners_token_by_index_1, Err(_)));
            assert!(matches!(owners_token_by_index_2, Err(_)));

            let mint_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), aft34_id1.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            let mint_result_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), aft34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(mint_result_1, Ok(()));
            assert_eq!(mint_result_2, Ok(()));

            let token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_by_index(1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(token_by_index_1, Ok(aft34_id1.clone()));
            assert_eq!(token_by_index_2, Ok(aft34_id2.clone()));

            let burn_result_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(bob), aft34_id2.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(burn_result_1, Ok(()));

            let owners_token_by_index_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 0));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            let owners_token_by_index_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.owners_token_by_index(address_of!(bob), 1));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(owners_token_by_index_1, Ok(aft34_id1.clone()));
            assert_eq!(owners_token_by_index_2, Err(AFT34Error::TokenNotExists));

            Ok(())
        }
    }
}
