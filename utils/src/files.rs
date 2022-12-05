// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   files.rs                                                                 \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 14:34:22 by Thibault Cheneviere                      \\
//   Updated: 2022/12/05 14:40:37 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::fs;

pub fn read_file(file_path: &str) -> String {
	return fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
}
