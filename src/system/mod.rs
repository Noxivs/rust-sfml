/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
 * Base module of SFML, defining various utilities.
 *
 * It provides vector classes, unicode strings and conversion functions, threads and mutexes, timing classes.
 */

pub use system::vector2::{Vector2u, Vector2i, Vector2f, ToVec};
pub use system::vector3::Vector3f;
pub use system::msleep::sleep;
pub use system::time::Time;
pub use system::clock::Clock;

#[doc(hidden)]
#[cfg(any(target_os="macos", target_os="linux", target_os="windows"))]
mod platform {
    #[link(name = "csfml-system")]
    extern {}
}

pub mod vector2;
pub mod vector3;
mod time;
mod clock;
#[path = "sleep.rs"]
mod msleep;
