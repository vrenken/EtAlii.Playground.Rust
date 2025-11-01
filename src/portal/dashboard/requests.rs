use crate::data::*;
use crate::portal::*;

use askama::Template;
use axum::{ extract::State, response::Html };
use sysinfo::{ System };


pub async fn get_page(State(state): State<AppState>) -> Html<String> {
    let name = state.name.lock().unwrap().clone();
    Html(
        dashboard::PageTemplate {
            title: "Welcome",
            subtitle: "to our page",
        }.render().unwrap()
    )
}

pub async fn get_cpu() -> Html<String> {
    let mut system = System::new_all();

    // Refresh CPU data to get accurate usage
    system.refresh_cpu_all();
    let cpu_usage = system.global_cpu_usage();

    // Print global CPU usage
    println!("Total CPU Usage: {:.2}%", cpu_usage);

    Html(
        dashboard::CpuUsageTemplate
        {
            value: &cpu_usage.to_string(),
        }.render().unwrap()
    )
}

pub async fn get_ram() -> Html<String> {
    let mut system = System::new_all();

    // Refresh CPU data to get accurate usage
    system.refresh_memory();
    let ram_total = system.total_memory();
    let ram_used = system.used_memory();

    let ram_usage = ram_used / (ram_total / 100);

    // Print global RAM usage
    println!("Total RAM usage: {:.2}%", ram_usage);

    Html(
        dashboard::RamUsageTemplate
        {
            value: &ram_usage.to_string(),
        }.render().unwrap()
    )
}