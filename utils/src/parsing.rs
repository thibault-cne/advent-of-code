// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   parsing.rs                                                               \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 14:35:33 by Thibault Cheneviere                      \\
//   Updated: 2022/12/05 15:10:39 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

pub fn parse_lines(content: String) -> Vec<String> {
	let split: Vec<&str> = content.split('\n').collect::<Vec<&str>>();

	return split.iter().map(|_str| _str.to_string()).collect::<Vec<String>>();
}
