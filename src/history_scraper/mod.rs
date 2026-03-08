use std::{cmp::Reverse, collections::HashMap, fs};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use walkdir::WalkDir;


#[derive(Clone)]
struct MatchingDirectory<'a> {
    directory_name: &'a str,
    score: i64
} 

/// Turns the bash history lines supplied into a vec of directories, looks for instances of "cd".
fn turn_bash_history_into_directories(bash_history_lines: &str) -> Vec<&str> {
    return bash_history_lines
     .lines()
     .into_iter()
     .filter(|line| line.starts_with("cd"))
     .flat_map(|line| line.strip_prefix("cd "))
     .collect();
}

/// Finds directories that match the input term, using fuzzy matching as well as frequency to determine the most likely
/// directory.
fn find_matching_directories<'a>(directories_list: Vec<&'a str>, input_term: &'a str) -> Result<Vec<MatchingDirectory<'a>>, String> {
    let matcher = SkimMatcherV2::default();

    let mut matching_directories: Vec<MatchingDirectory> = vec![];
    let mut matching_directory_count: HashMap<&str, i32> = HashMap::new();
    
    for directory in directories_list {
        let directory_ref = &directory;
        let result = matcher.fuzzy_match(&directory, input_term);
        
        match result {
            Some(score) => {
                let new_directory = MatchingDirectory { directory_name: directory_ref, score };
                matching_directories.push(new_directory);
            
                matching_directory_count
                .entry(directory)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            },
            None => {
                // Do nothing
            },
        }
    }

    // Now do frequency
    // Multiply score by freqency
    let mut final_dirs : Vec<MatchingDirectory> = matching_directories.iter().map(|directory|  MatchingDirectory {
        directory_name: directory.directory_name,
        score: directory.score * i64::from(matching_directory_count.get(directory.directory_name).unwrap_or(&0).clone())
    })
    .collect();

    final_dirs.sort_by_key(|dir| Reverse(dir.score));
    Ok(final_dirs)

}

fn get_closest_final_dir(final_dir_name: &str) -> String {

    // Find full path to folder on system to cd from anywhere
    let mut final_dir = String::new();

    for entry in WalkDir::new(format!("/home/{}/", std::env::var("USER").unwrap())).max_depth(5) {
        // Searching home dir...        
        if let Ok(ok_entry) = entry {
            if let Some(fname) = ok_entry.path().to_str() {
                if fname.to_string().replace("/", "").ends_with(&final_dir_name.replace("/", ""))  {
                    final_dir = fname.to_string();
                }
            }
        }
    }

    return final_dir;
}

pub fn find_final_dir(input_term: &str, bash_history_fp: &str) -> Result<String, ()> {
    let bash_history_lines = fs::read_to_string(bash_history_fp);
    let final_dir: String;

    match bash_history_lines {
        Ok(string_contents) => {
            let directories: Vec<&str> = turn_bash_history_into_directories(&string_contents);
            let final_dirs_res = find_matching_directories(directories, &input_term);
            match final_dirs_res {
                Ok(final_dirs) => {                    
                    // Find full path to folder on system to cd from anywhere
                    let winner_dir = final_dirs[0].directory_name;
                    final_dir = get_closest_final_dir(winner_dir);
                    
                },
                Err(_) => todo!(),
            }
        }
        Err(e) => {
            todo!("Not implemented!")
        }
    }

    Ok(final_dir)
}

#[cfg(test)]
mod tests;