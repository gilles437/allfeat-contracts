// Copyright (c) 2022-2023 Allfeat labs
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    aft34,
    aft34::extensions::album_storage,
    traits::aft34::{extensions::album_storage::*, *},
};
pub use aft34::{
    AFT34Impl, BalancesManager as _, Internal as _, InternalImpl as _, Operator, Owner,
};
use openbrush::storage::Mapping;
use openbrush::traits::{AccountId, Storage};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub base_album: Option<ALBUMID>,
    pub token_albums: Mapping<Id, ALBUMID>,
}

pub trait AFT34ALBUMStorageImpl: aft34::Internal + Storage<Data> {
    fn base_album(&self) -> Option<ALBUMID> {
        self.data().base_album.clone()
    }
    fn token_album(&self, token_id: Id) -> Result<Option<ALBUMID>, AFT34Error> {
        aft34::Internal::_owner_of(self, &token_id).ok_or(AFT34Error::TokenNotExists)?;
        let token_album = self.data().token_albums.get(&token_id);
        match token_album {
            None => Ok(None),
            Some(album) => {
                let base_album = self.data().base_album.clone();

                match base_album {
                    // If both are set, concatenate the baseALBUMID and tokenALBUMID.
                    Some(base) => Ok(Some(base + &album)),
                    // If there is no base ALBUMID, return the token ALBUMID.
                    None => Ok(Some(album)),
                }
            }
        }
    }
}

pub trait Internal: aft34::Internal {
    /// Event is emitted when an ALBUMID is set for a token.
    fn _emit_attribute_set_event(&self, token_id: Id, token_album: ALBUMID);
    /// Event is emitted when the base ALBUMID is updated.
    fn _emit_attribute_set_base_event(&self, base_album: Option<ALBUMID>);

    /// Sets `token_album` as the tokenALBUMID of `token_id`.
    ///
    /// `token_id` must exist.
    fn _set_token_album(&mut self, token_id: Id, token_album: ALBUMID) -> Result<(), AFT34Error>;

    /// Sets `base_album`.
    fn _set_base_album(&mut self, base_album: Option<ALBUMID>);

    /// This override additionally checks to see if
    /// token-specific ALBUMID was set for the token, and if so, it deletes the token ALBUMID from
    ///  the storage mapping.
    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), AFT34Error>;
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_attribute_set_event(&self, _token_id: Id, _token_album: ALBUMID) {}
    fn _emit_attribute_set_base_event(&self, _base_album: Option<ALBUMID>) {}

    fn _set_token_album(&mut self, token_id: Id, token_album: ALBUMID) -> Result<(), AFT34Error> {
        aft34::Internal::_owner_of(self, &token_id).ok_or(AFT34Error::TokenNotExists)?;
        self.data().token_albums.insert(&token_id, &token_album);
        Internal::_emit_attribute_set_event(self, token_id, token_album);
        Ok(())
    }

    fn _set_base_album(&mut self, base_album: Option<ALBUMID>) {
        self.data().base_album = base_album.clone();
        Internal::_emit_attribute_set_base_event(self, base_album)
    }

    fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), AFT34Error> {
        self.data().token_albums.remove(&id);
        aft34::Internal::_burn_from(self, from, id)
    }
}
