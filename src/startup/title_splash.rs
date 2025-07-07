pub fn make_title_splash() -> String {
    let title = 
"  // xxxxxxxxxxxxxxxxxxx //
 // --== MAGICIAN == -- //
// xxxxxxxxxxxxxxxxxxx //
    ";
    
    format!(
        "{}\t\tVersion v{}\n\n",
        title,
        env!("CARGO_PKG_VERSION")
    )
}
