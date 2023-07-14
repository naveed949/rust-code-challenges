

/// Sorts the usernames in the given vector.
///
/// This function is case-insensitive. This means that
/// names that are the same except for capitalization are
/// treated as equal.
///
/// # Examples
///
/// ```
/// use rust_code_challenges::insensitive_sort::sort_usernames;
/// 
/// let mut usernames = vec!["alice", "bob", "charlie"];
/// sort_usernames(&mut usernames);
/// assert_eq!(usernames, vec!["alice", "bob", "charlie"]);
///
/// let mut usernames = vec!["bob", "alice", "Charlie"];
/// sort_usernames(&mut usernames);
/// assert_eq!(usernames, vec!["alice", "bob", "Charlie"]);
/// ```
pub fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by(|a, b| {
        a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase())
    });
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}