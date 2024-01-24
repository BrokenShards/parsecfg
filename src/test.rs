// test.rs
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
#[cfg(test)]
mod tests
{
	use crate::{string_to_tokens, Document, FromTokens, Key, KeyValue, Section};

	const TEST_KEY: &str = "\tOrange= \"Banana\" # Comment";
	const TEST_ARRAY: &str = " Array =[ \"One\", # Comment\n\"Two\", \"Three\" ]";
	const TEST_TABLE: &str = "Language={#Comment\nName=\"C++\",#Comment\nAlias=[\"c++\",\"cpp\",\"\
	                          cplusplus\"]#Comment\n }";
	const TEST_SECTION: &str =
		"[\tTest ]\nFruit = \"Oranges\"# Comment\nElephants = \"No Thanks!\"";
	const TEST_DOCUMENT: &str = "[Size]# Comment\nWidth = \"800\"#Bon\nHeight = \
	                             \"600\"#Lem\n[Position]\nX = \"20\"\nY = \"40\"";

	#[test]
	fn key_test()
	{
		let mut key = Key::new("Banana", KeyValue::String(String::from("BoingBoingBoing")));

		assert_eq!(key.name().as_str(), "Banana");
		assert_eq!(key.value, KeyValue::String(String::from("BoingBoingBoing")));

		let tokens = match string_to_tokens(TEST_KEY)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		let mut index = 0usize;

		key = match Key::from_tokens(&tokens, &mut index)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		assert_eq!(key.name().as_str(), "Orange");
		assert_eq!(key.value, KeyValue::String(String::from("Banana")));

		let tokens = match string_to_tokens(TEST_ARRAY)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		index = 0usize;

		key = match Key::from_tokens(&tokens, &mut index)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		assert_eq!(key.name().as_str(), "Array");
		assert_eq!(
			key.value,
			KeyValue::Array(vec![
				String::from("One"),
				String::from("Two"),
				String::from("Three")
			])
		);

		let tokens = match string_to_tokens(TEST_TABLE)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		index = 0usize;

		key = match Key::from_tokens(&tokens, &mut index)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		assert_eq!(key.name().as_str(), "Language");
		assert_eq!(
			key.value,
			KeyValue::Table(vec![
				Key::new("Name", KeyValue::String(String::from("C++"))),
				Key::new(
					"Alias",
					KeyValue::Array(vec![
						String::from("c++"),
						String::from("cpp"),
						String::from("cplusplus")
					])
				)
			])
		);
	}
	#[test]
	fn section_test()
	{
		let mut sect = Section::new(
			"Settings",
			&[
				Key::new("Width", KeyValue::String(String::from("800"))),
				Key::new("Height", KeyValue::String(String::from("600"))),
			],
		);

		assert_eq!(*sect.name(), String::from("Settings"));
		assert_eq!(
			*sect.get_at(0).unwrap(),
			Key::new("Width", KeyValue::String(String::from("800")))
		);
		assert_eq!(
			*sect.get_at(1).unwrap(),
			Key::new("Height", KeyValue::String(String::from("600")))
		);

		let tokens = match string_to_tokens(TEST_SECTION)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		let mut index = 0usize;

		sect = match Section::from_tokens(&tokens, &mut index)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		assert_eq!(*sect.name(), String::from("Test"));
		assert_eq!(
			*sect.get_at(0).unwrap(),
			Key::new("Fruit", KeyValue::String(String::from("Oranges")))
		);
		assert_eq!(
			*sect.get_at(1).unwrap(),
			Key::new("Elephants", KeyValue::String(String::from("No Thanks!")))
		);
	}
	#[test]
	fn document_test()
	{
		let mut doc = Document::new(&[
			Section::new(
				"Banana",
				&[
					Key::new("Width", KeyValue::String(String::from("800"))),
					Key::new("Height", KeyValue::String(String::from("600"))),
				],
			),
			Section::new(
				"Lemon",
				&[
					Key::new("XPos", KeyValue::String(String::from("40"))),
					Key::new("YPos", KeyValue::String(String::from("60"))),
				],
			),
		]);

		assert_eq!(*doc.get_at(0).unwrap().name(), "Banana");
		assert_eq!(
			doc.get_at(0).unwrap().get("Width").unwrap().value,
			KeyValue::String(String::from("800"))
		);

		let tokens = match string_to_tokens(TEST_DOCUMENT)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		let mut index = 0usize;

		doc = match Document::from_tokens(&tokens, &mut index)
		{
			Ok(k) => k,
			Err(e) =>
			{
				println!("{e}");
				panic!()
			}
		};

		assert_eq!(*doc.get_at(0).unwrap().name(), "Size");
		assert_eq!(
			doc.get_at(0).unwrap().get("Width").unwrap().value,
			KeyValue::String(String::from("800"))
		);
		assert_eq!(
			doc.get_at(0).unwrap().get("Height").unwrap().value,
			KeyValue::String(String::from("600"))
		);

		assert_eq!(*doc.get_at(1).unwrap().name(), "Position");
		assert_eq!(
			doc.get_at(1).unwrap().get("X").unwrap().value,
			KeyValue::String(String::from("20"))
		);
		assert_eq!(
			doc.get_at(1).unwrap().get("Y").unwrap().value,
			KeyValue::String(String::from("40"))
		);
	}
}
