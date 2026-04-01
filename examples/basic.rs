//! Basic usage of itihas — exploring eras, civilizations, events, and calendars.

fn main() {
    // --- Civilizations active in 500 BCE ---
    println!("=== Civilizations Active in 500 BCE ===");
    let civs = itihas::civilization::active_at(-500);
    for c in &civs {
        println!(
            "  {} ({}) — founded ~{} BCE, script: {}",
            c.name,
            c.region,
            -c.founding_year,
            c.script,
        );
    }
    println!();

    // --- Eras containing 500 BCE ---
    println!("=== Eras Containing 500 BCE ===");
    let eras = itihas::era::eras_containing(-500);
    for e in &eras {
        println!(
            "  {} ({:?}) — {} to {}",
            e.name, e.category, e.start_year, e.end_year,
        );
    }
    println!();

    // --- Events by category ---
    println!("=== Inventions ===");
    let inventions = itihas::event::by_category(&itihas::event::EventCategory::Invention);
    for ev in &inventions {
        println!("  {} ({}) — {}", ev.name, ev.year, ev.description);
    }
    println!();

    // --- Calendar systems ---
    println!("=== Calendar Systems ===");
    for cal in itihas::calendar::all_calendars() {
        println!(
            "  {} ({:?}) — {} months, epoch: {}",
            cal.name, cal.calendar_type, cal.months, cal.epoch_year,
        );
    }
    println!();

    // --- Historical figures ---
    println!("=== Scientists ===");
    let scientists = itihas::figure::by_domain(&itihas::figure::FigureDomain::Scientist);
    for f in &scientists {
        let dates = match (f.birth_year, f.death_year) {
            (Some(b), Some(d)) => format!("{} - {}", b, d),
            _ => "dates unknown".into(),
        };
        println!("  {} ({}) — {}", f.name, dates, f.description);
    }
}
