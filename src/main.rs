fn main() {
	let url = std::env::args().nth(1).unwrap_or_else(|| {
		eprintln!("Usage: gloc <repo-url>");
		std::process::exit(1);
	});

	let dir = tempfile::tempdir().unwrap_or_else(|e| {
		eprintln!("Failed to create temporary directory: {e}");
		std::process::exit(1);
	});

	git2::Repository::clone(&url, &dir).unwrap_or_else(|e| {
		eprintln!("Failed to clone repository: {e}");
		std::process::exit(1);
	});

	let mut langs = tokei::Languages::new();
	langs.get_statistics(&[dir], &["target", ".git"], &tokei::Config {
		treat_doc_strings_as_comments: Some(true),
		.. Default::default()
	});
	
	langs.into_iter().for_each(|(name, stats)|
		println!("{}: {}", name, stats.code));
}
