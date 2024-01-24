// name.rs
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

/// Returns true if `name` only contains characters that are valid in a type name, otherwise false.
pub fn is_valid_name(name: &str) -> bool
{
	if name.is_empty()
	{
		return false;
	}

	let mut first = true;

	let name = String::from(name).to_lowercase();

	for c in name.chars()
	{
		if first
		{
			if (c < 'a' || c > 'z') && c != '_'
			{
				return false;
			}

			first = false;
		}
		else
		{
			if (c < 'a' || c > 'z') && (c < '0' || c > '9') && c != '_'
			{
				return false;
			}
		}
	}

	true
}
/// Returns a string containing `name` with all invalid type name characters replaced with `repl`.
pub fn as_valid_name(name: &str, repl: char) -> String
{
	let mut result = String::from(name.trim());

	if result.is_empty()
	{
		return repl.to_string();
	}

	let mut first = true;
	let mut i: usize = 0;
	let mut indicies: Vec<usize> = Vec::new();
	let mut numstart = false;

	let lo = result.to_lowercase();

	for c in lo.chars()
	{
		if first
		{
			if (c < 'a' || c > 'z') && (c < '0' || c > '9') && c != '_'
			{
				indicies.push(i);
			}
			else
			{
				numstart = c >= '0' && c <= '9';
			}

			first = false;
		}
		else
		{
			if (c < 'a' || c > 'z') && (c < '0' || c > '9') && c != '_'
			{
				indicies.push(i);
			}
		}

		i += 1;
	}

	for ind in indicies
	{
		result.remove(ind);
		result.insert(ind, repl);
	}

	if numstart
	{
		result.insert(0, '_');
	}

	result
}
