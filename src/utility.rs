// utility.rs
//
// ParseCfg - A simple cfg file parser.
// Copyright(C) 2024 Michael Furlong.
//
// This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either version 3
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.
//

/// Indents a string with a given amount of tabs.
pub fn indent(string: &str, amount: usize) -> String
{
	let mut tabs = String::new();
	let mut i = 0;

	while i < amount
	{
		tabs.push('\t');
		i += 1;
	}

	tabs.clone() + &string.replace('\n', &(String::from("\n") + &tabs))
}
