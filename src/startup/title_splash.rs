pub fn make_title_splash() -> String {
    let title = 
"  // xxxxxxxxxxxxxxxxxx //
 // --== MAGICIAN ==-- //
// xxxxxxxxxxxxxxxxxx //
    ";
    
    format!(
        "{}\t\tVersion v{}\n\n",
        title,
        env!("CARGO_PKG_VERSION")
    )
}
