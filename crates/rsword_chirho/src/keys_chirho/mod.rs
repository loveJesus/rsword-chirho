// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Key types for navigating SWORD modules.
//!
//! Keys are used to identify and navigate to specific entries in modules.
//! Different module types use different key implementations:
//!
//! | Key Type | Module Types | Example |
//! |----------|--------------|---------|
//! | [`VerseKeyChirho`] | Bibles, Commentaries | "John 3:16" |
//! | [`TreeKeyChirho`] | General Books | "/Chapter1/Section2" |
//! | [`StrKeyChirho`] | Lexicons, Dictionaries | "G26" (Strong's number) |
//! | [`DateKeyChirho`] | Daily Devotionals | "01.25" (Jan 25) |
//! | [`ListKeyChirho`] | Search Results | Collection of keys |
//!
//! ## Verse Navigation
//!
//! ```rust,ignore
//! use rsword_chirho::VerseKeyChirho;
//!
//! // Parse a verse reference
//! let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();
//!
//! // Navigate through verses
//! key_chirho.increment_chirho(1);  // Move to Genesis 1:2
//! key_chirho.increment_chirho(30); // Move to Genesis 2:1 (crosses chapter boundary)
//!
//! // Get components
//! println!("Book: {}", key_chirho.get_book_name_chirho());
//! ```
//!
//! ## Verse Ranges
//!
//! ```rust,ignore
//! use rsword_chirho::VerseKeyChirho;
//!
//! // Set bounds for iteration
//! let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();
//! key_chirho.set_lower_bound_chirho(&VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap());
//! key_chirho.set_upper_bound_chirho(&VerseKeyChirho::from_str_chirho("Genesis 1:31").unwrap());
//!
//! // Iterate through the range
//! while !key_chirho.pop_error_chirho() {
//!     println!("{}", key_chirho);
//!     key_chirho.increment_chirho(1);
//! }
//! ```
//!
//! ## Search Results
//!
//! ```rust,ignore
//! use rsword_chirho::ListKeyChirho;
//! use rsword_chirho::keys_chirho::StrKeyChirho;
//!
//! // ListKey holds search results
//! let mut results_chirho = ListKeyChirho::new_chirho();
//! results_chirho.add_chirho(Box::new(StrKeyChirho::from_str_chirho("Gen 1:1")));
//! results_chirho.add_chirho(Box::new(StrKeyChirho::from_str_chirho("John 1:1")));
//!
//! println!("Results: {}", results_chirho.count_chirho());
//! ```

pub mod sw_key_chirho;
pub mod verse_key_chirho;
pub mod list_key_chirho;
pub mod tree_key_chirho;
pub mod str_key_chirho;
pub mod date_key_chirho;

pub use sw_key_chirho::SwKeyChirho;
pub use verse_key_chirho::VerseKeyChirho;
pub use list_key_chirho::ListKeyChirho;
pub use tree_key_chirho::TreeKeyChirho;
pub use str_key_chirho::StrKeyChirho;
pub use date_key_chirho::DateKeyChirho;
