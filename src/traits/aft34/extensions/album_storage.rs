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

/// Extension of [`AFT34`] that allows token holders to destroy their tokens
use crate::traits::aft34::AFT34Error;
use crate::traits::aft34::Id;
use openbrush::traits::String;

pub type ALBUMID = String;

#[openbrush::wrapper]
pub type AFT34ALBUMStorageRef = dyn AFT34ALBUMStorage;

#[openbrush::trait_definition]
pub trait AFT34ALBUMStorage {
    #[ink(message)]
    fn base_album(&self) -> Option<ALBUMID>;
    #[ink(message)]
    fn token_album(&self, token_id: Id) -> Result<Option<ALBUMID>, AFT34Error>;
}
