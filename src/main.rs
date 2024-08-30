use itertools::Itertools;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Gauge, Padding, Paragraph, Wrap,
    },
    Frame, Terminal,
};
use std::io::{self, stdout};
use sysinfo::System;

const GAUGE3_COLOR: Color = tailwind::BLUE.c800;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut sys = System::new();
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| {
            ui(frame, &mut sys);
        })?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, sys: &mut System) {
    let app_block = Block::new()
        .borders(Borders::ALL)
        .title(Title::from("Deployment Manager"));

    let (left_area, righ_area) = calculate_layout(app_block.inner(frame.area()));
    frame.render_widget(app_block, frame.area());

    render_left(frame, left_area);
    render_right(frame, righ_area, sys);
}

fn calculate_layout(area: Rect) -> (Rect, Rect) {
    let main_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Min(0)]);
    let [left_area, righ_area] = main_layout.areas(area);
    (left_area, righ_area)
}

fn render_left(frame: &mut Frame, area: Rect) {
    let deployments_block = Block::new()
        .borders(Borders::ALL)
        .title(Title::from(String::from("Deployments")));
    // TODO: render list here
    // frame.render_widget(paragraph.clone().block(block), area);
}

fn render_right(frame: &mut Frame, area: Rect, sys: &mut System) {
    let system_block = Block::new()
        .borders(Borders::ALL)
        .title(Title::from(String::from("System Overview")));
    let system_rows = Layout::vertical([Constraint::Percentage(50), Constraint::Min(0)])
        .split(system_block.inner(area));
    let mut count = 0;
    let system_areas = system_rows
        .iter()
        .map(|system_area| {
            let constraints = if count == 0 {
                vec![Constraint::Percentage(50), Constraint::Min(0)]
            } else {
                vec![Constraint::Min(0)]
            };
            let layout = Layout::horizontal(constraints)
                .split(*system_area)
                .iter()
                .copied()
                .collect_vec();
            count += 1;
            return layout;
        })
        .collect_vec();
    frame.render_widget(system_block, area);

    let cpu_block = Block::new().borders(Borders::ALL).title(Title::from("CPU"));

    sys.refresh_cpu_usage();
    let cpus = sys.cpus();
    let cpu_rows = Layout::vertical(vec![Constraint::Min(0); cpus.len() / 2])
        .split(cpu_block.inner(system_areas[0][0]))
        .iter()
        .copied()
        .collect_vec();
    let cpu_areas = cpu_rows
        .iter()
        .map(|cpu_area| {
            Layout::horizontal([Constraint::Percentage(50), Constraint::Min(0)])
                .split(*cpu_area)
                .iter()
                .copied()
                .collect_vec()
        })
        .collect_vec();

    for (index, cpu) in cpus.iter().enumerate() {
        let cpu_name = format!("-- {} --", cpu.name());
        let usage = cpu.cpu_usage();
        let cpu_block = Block::new()
            .borders(Borders::ALL)
            .title(Title::from(cpu_name));
        let label = format!("{:.1}%", usage);
        let cpu_gauge = Gauge::default()
            .block(cpu_block)
            .gauge_style(GAUGE3_COLOR)
            .ratio(usage as f64 / 100.0)
            .label(label);
        let column = if index % 2 == 0 {
            0
        } else if index == 0 {
            0
        } else {
            1
        };
        let row = (index as f32 / 2 as f32).floor() as usize;
        frame.render_widget(cpu_gauge, cpu_areas[row][column]);
    }

    frame.render_widget(cpu_block, system_areas[0][0]);

    let mem_paragraph = Paragraph::new("MEM");
    let mem_block = Block::new().borders(Borders::ALL).title(Title::from("MEM"));
    let storage_paragraph = Paragraph::new("Storage");
    let storage_block = Block::new()
        .borders(Borders::ALL)
        .title(Title::from("Storage"));
    frame.render_widget(mem_paragraph.block(mem_block), system_areas[0][1]);
    frame.render_widget(storage_paragraph.block(storage_block), system_areas[1][0]);
}
