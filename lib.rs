// Wire
// Copyright (C) 2018 Wire Swiss GmbH <support@wire.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(test)]

pub const N: usize = 1024;
pub const Q: usize = 12_289;
pub const POLY_BYTES: usize = 1792;
pub const SEEDBYTES: usize = 32;
pub const RECBYTES: usize = 256;
pub const SENDABYTES: usize = POLY_BYTES + SEEDBYTES;
pub const SENDBBYTES: usize = POLY_BYTES + RECBYTES;
pub const SHARED_SECRET_LENGTH: usize = 32;

extern crate test;

use std::fmt::{Debug, Error, Formatter};

extern "C" {
    pub fn newhope_keygen(send: *mut u8, sk: *mut Poly);
    pub fn newhope_sharedb(sharedkey: *mut u8, send: *mut u8, received: *const u8);
    pub fn newhope_shareda(sharedkey: *mut u8, ska: *const Poly, received: *const u8);

    pub fn poly_frombytes(r: &mut Poly, a: *const u8);
    pub fn poly_tobytes(r: *mut u8, p: *const Poly);
}

#[repr(C)]
#[repr(align(32))]
#[derive(Clone)]
pub struct Poly {
    pub coeffs: [u16; N],
}

impl Default for Poly {
    fn default() -> Poly {
        Poly { coeffs: [0; N] }
    }
}

impl Poly {
    pub fn to_bytes(&self) -> [u8; POLY_BYTES] {
        let mut output = [0; POLY_BYTES];

        unsafe {
            poly_tobytes(output.as_mut_ptr(), self);
        }

        output
    }

    pub fn from_bytes(bytes: &[u8; POLY_BYTES]) -> Poly {
        let mut poly = Poly::default();

        unsafe {
            poly_frombytes(&mut poly, bytes.as_ptr());
        }

        poly
    }
}

impl PartialEq for Poly {
    fn eq(&self, other: &Poly) -> bool {
        self.coeffs[..] == other.coeffs[..]
    }
}

impl Eq for Poly {}

impl Debug for Poly {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", &self.coeffs[..])
    }
}

pub struct DerivedSecretAndKey {
    pub shared_secret: [u8; SHARED_SECRET_LENGTH],
    pub public_key: [u8; SENDBBYTES],
}

#[test]
fn test_newhope() {
    let (mut senda, mut sendb) = ([0; SENDABYTES], [0; SENDBBYTES]);
    let (mut keya, mut keyb) = ([0; 32], [0; 32]);
    let mut ska = Poly::default();

    unsafe {
        newhope_keygen(senda.as_mut_ptr(), &mut ska);
        newhope_sharedb(keyb.as_mut_ptr(), sendb.as_mut_ptr(), senda.as_ptr());
        newhope_shareda(keya.as_mut_ptr(), &ska, sendb.as_ptr());
    }

    assert!(keya != [0; 32]);
    assert_eq!(keya, keyb);
}

#[test]
fn test_newhope_bytes() {
    let (mut senda, mut sendb) = ([0; SENDABYTES], [0; SENDBBYTES]);
    let (mut keya, mut keyb) = ([0; 32], [0; 32]);
    let mut ska = Poly::default();

    unsafe {
        newhope_keygen(senda.as_mut_ptr(), &mut ska);
        newhope_sharedb(keyb.as_mut_ptr(), sendb.as_mut_ptr(), senda.as_ptr());
    }

    let ska_bytes = ska.to_bytes();
    let ska = Poly::from_bytes(&ska_bytes);

    unsafe {
        newhope_shareda(keya.as_mut_ptr(), &ska, sendb.as_ptr());
    }

    assert!(keya != [0; 32]);
    assert_eq!(keya, keyb);
}
