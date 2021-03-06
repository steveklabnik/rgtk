// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A "light switch" style toggle

use gtk::cast::GTK_SWITCH;
use gtk::ffi;
/*
* Switch — A "light switch" style toggle
*
* # Availables signals:
* * `activate` : Action
*/
struct_Widget!(Switch)

impl Switch {
    pub fn new() -> Option<Switch> {
        let tmp_pointer = unsafe { ffi::gtk_switch_new() };
        check_pointer!(tmp_pointer, Switch)
    }

    pub fn set_active(&mut self, is_active: bool) -> () {
        match is_active {
            true    => unsafe { ffi::gtk_switch_set_active(GTK_SWITCH(self.pointer), ffi::GTRUE) },
            false   => unsafe { ffi::gtk_switch_set_active(GTK_SWITCH(self.pointer), ffi::GFALSE) }
        }
    }

    pub fn get_active(&self) -> bool {
        match unsafe { ffi::gtk_switch_get_active(GTK_SWITCH(self.pointer)) } {
            ffi::GFALSE => false,
            _ => true
        }
    }
}

impl_drop!(Switch)
impl_TraitWidget!(Switch)

impl_widget_events!(Switch)
