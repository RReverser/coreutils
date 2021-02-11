/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Jordi Boggiano <j.boggiano@seld.be>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

// spell-checker:ignore (ToDO) getusername

#[cfg(unix)]
#[path = "unix.rs"]
mod imp;

#[cfg(windows)]
#[path = "windows.rs"]
mod imp;

#[cfg(not(any(unix, windows)))]
mod imp {
	pub unsafe fn getusername() -> std::io::Result<String> {
		Ok("unknown".into())
	}
}

pub use self::imp::getusername;
