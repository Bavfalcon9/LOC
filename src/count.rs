// Counts all the files
pub fn count_all_lines(contents: String) -> usize {
     let contents = contents.chars();
     let mut lines = 0;
     for c in contents {
          if c == '\n' { lines = lines + 1 };
     }
     return lines;
}