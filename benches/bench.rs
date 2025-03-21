use criterion::{criterion_group, criterion_main, Criterion};
use wax::Pattern;

fn simple_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_match");

    const GLOB: &str = "some/**/n*d[k-m]e?txt";
    const PATH: &str = "some/a/bigger/path/to/the/crazy/needle.txt";

    group.bench_function("glob", |b| b.iter(|| glob::Pattern::new(GLOB).unwrap().matches(PATH)));

    group.bench_function("glob-pre-compiled", |b| {
        let matcher = glob::Pattern::new(GLOB).unwrap();
        b.iter(|| matcher.matches(PATH))
    });

    group.bench_function("globset", |b| {
        b.iter(|| {
            globset::Glob::new(GLOB).unwrap().compile_matcher().is_match(PATH);
        })
    });

    group.bench_function("globset-pre-compiled", |b| {
        let matcher = globset::Glob::new(GLOB).unwrap().compile_matcher();
        b.iter(|| matcher.is_match(PATH))
    });

    group.bench_function("glob-match", |b| b.iter(|| glob_match::glob_match(GLOB, PATH)));

    group.bench_function("fast-glob", |b| b.iter(|| fast_glob::glob_match(GLOB, PATH)));

    group.bench_function("wax", |b| b.iter(|| wax::Glob::new(GLOB).unwrap().is_match(PATH)));

    group.bench_function("wax-pre-compiled", |b| {
        let matcher = wax::Glob::new(GLOB).unwrap();
        b.iter(|| matcher.is_match(PATH))
    });

    group.finish();
}

fn brace_expansion(c: &mut Criterion) {
    let mut group = c.benchmark_group("brace_expansion");

    const GLOB: &str = "some/**/{tob,crazy}/?*.{png,txt}";
    const PATH: &str = "some/a/bigger/path/to/the/crazy/needle.txt";

    group.bench_function("globset", |b| {
        b.iter(|| globset::Glob::new(GLOB).unwrap().compile_matcher().is_match(PATH))
    });

    group.bench_function("globset-pre-compiled", |b| {
        let matcher = globset::Glob::new(GLOB).unwrap().compile_matcher();
        b.iter(|| matcher.is_match(PATH))
    });

    group.bench_function("glob-match", |b| b.iter(|| glob_match::glob_match(GLOB, PATH)));

    group.bench_function("fast-glob", |b| {
        b.iter(|| fast_glob::glob_match(GLOB, PATH));
    });

    group.bench_function("wax", |b| b.iter(|| wax::Glob::new(GLOB).unwrap().is_match(PATH)));

    group.bench_function("wax-pre-compiled", |b| {
        let matcher = wax::Glob::new(GLOB).unwrap();
        b.iter(|| matcher.is_match(PATH))
    });

    group.finish();
}

criterion_group!(benches, simple_match, brace_expansion);
criterion_main!(benches);
