use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_all_eras(c: &mut Criterion) {
    c.bench_function("all_eras", |b| b.iter(itihas::era::all_eras));
}

fn bench_eras_containing(c: &mut Criterion) {
    c.bench_function("eras_containing_500bce", |b| {
        b.iter(|| itihas::era::eras_containing(black_box(-500)))
    });
}

fn bench_all_civilizations(c: &mut Criterion) {
    c.bench_function("all_civilizations", |b| {
        b.iter(itihas::civilization::all_civilizations)
    });
}

fn bench_active_at(c: &mut Criterion) {
    c.bench_function("civilizations_active_at_500bce", |b| {
        b.iter(|| itihas::civilization::active_at(black_box(-500)))
    });
}

fn bench_by_region(c: &mut Criterion) {
    c.bench_function("civilizations_by_region", |b| {
        b.iter(|| itihas::civilization::by_region(black_box("Mediterranean")))
    });
}

fn bench_all_events(c: &mut Criterion) {
    c.bench_function("all_events", |b| b.iter(itihas::event::all_events));
}

fn bench_events_by_category(c: &mut Criterion) {
    c.bench_function("events_by_category_war", |b| {
        b.iter(|| itihas::event::by_category(black_box(&itihas::event::EventCategory::War)))
    });
}

fn bench_events_at_year(c: &mut Criterion) {
    c.bench_function("events_at_year_476", |b| {
        b.iter(|| itihas::event::at_year(black_box(476)))
    });
}

fn bench_all_calendars(c: &mut Criterion) {
    c.bench_function("all_calendars", |b| b.iter(itihas::calendar::all_calendars));
}

fn bench_calendar_by_name(c: &mut Criterion) {
    c.bench_function("calendar_by_name_gregorian", |b| {
        b.iter(|| itihas::calendar::by_name(black_box("gregorian")))
    });
}

fn bench_all_figures(c: &mut Criterion) {
    c.bench_function("all_figures", |b| b.iter(itihas::figure::all_figures));
}

fn bench_figures_by_domain(c: &mut Criterion) {
    c.bench_function("figures_by_domain_scientist", |b| {
        b.iter(|| itihas::figure::by_domain(black_box(&itihas::figure::FigureDomain::Scientist)))
    });
}

fn bench_events_between(c: &mut Criterion) {
    c.bench_function("events_between_500bce_500ce", |b| {
        b.iter(|| itihas::event::events_between(black_box(-500), black_box(500)))
    });
}

fn bench_all_causalities(c: &mut Criterion) {
    c.bench_function("all_causalities", |b| {
        b.iter(itihas::causality::all_causalities)
    });
}

fn bench_causes_of(c: &mut Criterion) {
    c.bench_function("causes_of_french_revolution", |b| {
        b.iter(|| itihas::causality::causes_of(black_box("French Revolution")))
    });
}

fn bench_chain(c: &mut Criterion) {
    c.bench_function("chain_writing_depth3", |b| {
        b.iter(|| itihas::causality::chain(black_box("Invention of Writing"), black_box(3)))
    });
}

fn bench_all_interactions(c: &mut Criterion) {
    c.bench_function("all_interactions", |b| {
        b.iter(itihas::interaction::all_interactions)
    });
}

fn bench_interactions_for(c: &mut Criterion) {
    c.bench_function("interactions_for_rome", |b| {
        b.iter(|| itihas::interaction::interactions_for(black_box("Roman Empire")))
    });
}

fn bench_influence_score(c: &mut Criterion) {
    c.bench_function("influence_score_egypt_hittite", |b| {
        b.iter(|| {
            itihas::interaction::influence_score(
                black_box("Ancient Egypt"),
                black_box("Hittite Empire"),
            )
        })
    });
}

fn bench_all_sites(c: &mut Criterion) {
    c.bench_function("all_sites", |b| b.iter(itihas::site::all_sites));
}

fn bench_sites_by_region(c: &mut Criterion) {
    c.bench_function("sites_by_region_near_east", |b| {
        b.iter(|| itihas::site::by_region(black_box("Near East")))
    });
}

fn bench_sites_active_at(c: &mut Criterion) {
    c.bench_function("sites_active_at_500bce", |b| {
        b.iter(|| itihas::site::active_at(black_box(-500)))
    });
}

fn bench_all_routes(c: &mut Criterion) {
    c.bench_function("all_routes", |b| b.iter(itihas::trade::all_routes));
}

fn bench_routes_by_region(c: &mut Criterion) {
    c.bench_function("routes_by_region_east_asia", |b| {
        b.iter(|| itihas::trade::by_region(black_box("East Asia")))
    });
}

fn bench_routes_by_commodity(c: &mut Criterion) {
    c.bench_function("routes_by_commodity_silk", |b| {
        b.iter(|| itihas::trade::by_commodity(black_box("silk")))
    });
}

criterion_group!(
    benches,
    bench_all_eras,
    bench_eras_containing,
    bench_all_civilizations,
    bench_active_at,
    bench_by_region,
    bench_all_events,
    bench_events_by_category,
    bench_events_at_year,
    bench_events_between,
    bench_all_calendars,
    bench_calendar_by_name,
    bench_all_figures,
    bench_figures_by_domain,
    bench_all_causalities,
    bench_causes_of,
    bench_chain,
    bench_all_interactions,
    bench_interactions_for,
    bench_influence_score,
    bench_all_sites,
    bench_sites_by_region,
    bench_sites_active_at,
    bench_all_routes,
    bench_routes_by_region,
    bench_routes_by_commodity,
);
criterion_main!(benches);
