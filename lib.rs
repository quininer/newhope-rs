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
pub const N2: usize = 2 * N;
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
    pub fn to_bytes(&self) -> [u8; N2] {
        let mut output = [0u8; N2];
        for i in 0..N {
            output[2 * i] = ((self.coeffs[i] >> 8) & 0xff) as u8;
            output[2 * i + 1] = (self.coeffs[i] & 0xff) as u8;
        }
        output
    }

    pub fn from_bytes(bytes: &[u8; N2]) -> Poly {
        let mut poly = Poly::default();
        for i in 0..N {
            poly.coeffs[i] = (u16::from(bytes[2 * i]) << 8) + u16::from(bytes[2 * i + 1]);
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
        write!(f, "{:?}", &self.coeffs.to_vec())
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
