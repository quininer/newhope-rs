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

extern crate cc;

use cc::Build;
use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("newhope").exists() {
        Command::new("wget")
            .arg("https://cryptojedi.org/crypto/data/newhope-20160815.tar.bz2")
            .arg("-O")
            .arg("newhope-20160815.tar.bz2")
            .output()
            .expect("Failed to download newhope reference implementation.");
        Command::new("tar")
            .arg("xf")
            .arg("newhope-20160815.tar.bz2")
            .output()
            .expect("Failed to extract newhope reference implementation.");
        Command::new("mv")
            .arg("newhope-20160815")
            .arg("newhope")
            .output()
            .expect("Failed to rename directory.");
    }

    let cnh_path = Path::new("newhope").join("ref");
    let mut cfg = Build::new();

    for src in &[
        "crypto_stream_chacha20.c",
        "poly.c",
        "ntt.c",
        "precomp.c",
        "error_correction.c",
        "newhope.c",
        "reduce.c",
        "fips202.c",
        "randombytes.c",
    ] {
        cfg.file(cnh_path.join(src));
    }

    cfg.include(cnh_path)
        .opt_level(3)
        .debug(true)
        .flag("-march=native")
        .compile("libnewhope.a");
}
