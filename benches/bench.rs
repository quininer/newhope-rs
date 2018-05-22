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

extern crate cnewhope;
extern crate test;

use cnewhope::*;
use test::Bencher;

#[bench]
fn bench_cnewhope_keygen(b: &mut Bencher) {
    b.iter(|| unsafe {
        let mut senda = [0; SENDABYTES];
        let mut ska = Poly::default();
        newhope_keygen(senda.as_mut_ptr(), &mut ska);
    });
}

#[bench]
fn bench_cnewhope_sharedb(b: &mut Bencher) {
    let (mut senda, mut sendb) = ([0; SENDABYTES], [0; SENDBBYTES]);
    let mut keyb = [0; 32];
    let mut ska = Poly::default();
    unsafe { newhope_keygen(senda.as_mut_ptr(), &mut ska) };
    drop(ska);
    b.iter(|| unsafe { newhope_sharedb(keyb.as_mut_ptr(), sendb.as_mut_ptr(), senda.as_ptr()) });
}

#[bench]
fn bench_cnewhope_shareda(b: &mut Bencher) {
    let (mut senda, mut sendb) = ([0; SENDABYTES], [0; SENDBBYTES]);
    let (mut keya, mut keyb) = ([0; 32], [0; 32]);
    let mut ska = Poly::default();
    unsafe { newhope_keygen(senda.as_mut_ptr(), &mut ska) };
    unsafe { newhope_sharedb(keyb.as_mut_ptr(), sendb.as_mut_ptr(), senda.as_ptr()) };
    b.iter(|| unsafe { newhope_shareda(keya.as_mut_ptr(), &ska, sendb.as_ptr()) });
}

fn roundtrip() {
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

#[bench]
fn bench_roundtrip(b: &mut test::Bencher) {
    b.iter(|| roundtrip())
}
