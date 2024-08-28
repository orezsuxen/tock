// #![allow(unused)]
use time::macros::offset;
use time::OffsetDateTime;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io, thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};

// println!("start");
// let raw_time = OffsetDateTime::now_utc();
// let offset_time = raw_time.to_offset(offset!(+1));
// let (h, m, s) = offset_time.to_hms();
// println!("now is {}, {}, {}", h, m, s);
//

use vtx::Vtx2;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut center_height = 40;
    let mut center_width = 80;

    //time stuff
    let mut raw_time; //= OffsetDateTime::now_utc();
    let mut offset_time; //= raw_time.to_offset(offset!(+1));
    let (mut h, mut m, mut s); // = offset_time.to_hms();

    // vtx stuff
    let c_hand = Vtx2::new();
    let mut h_hand; // = Vtx2::new();
    let mut m_hand; // = Vtx2::new();
    let mut s_hand; // = Vtx2::new();
    let mut marks: Vec<(f64, f64)> = Vec::new();

    for i in 0..12 {
        let nm = Vtx2::from_rot(
            (i as f64) * -1.0 * (2.0 * std::f64::consts::PI / 12.0) + (std::f64::consts::PI / 2.0),
        );

        marks.push((nm * 0.9).into());
    }

    let mut timer = Instant::now();

    // ========== start main loop ==========
    loop {
        let size = terminal.get_frame().size();
        let frame = size;
        let term_height = frame.height;
        let term_width = frame.width;

        if event::poll(Duration::from_millis(0)).unwrap_or(false) {
            match event::read().unwrap() {
                event::Event::Key(ev) => match ev.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if center_height < term_height {
                            center_height += 1;
                        }
                    }
                    KeyCode::Down => {
                        if center_height > 0 {
                            center_height -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if center_width < term_width {
                            center_width += 1;
                        }
                    }
                    KeyCode::Left => {
                        if center_width > 0 {
                            center_width -= 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            // ========== calculate stuff ===========
            raw_time = OffsetDateTime::now_utc();
            offset_time = raw_time.to_offset(offset!(+1));
            (h, m, s) = offset_time.to_hms();

            h_hand = Vtx2::from_rot(
                (h as f64) * -1.0 * (2.0 * std::f64::consts::PI / 12.0)
                    + (std::f64::consts::PI / 2.0),
            );
            h_hand = h_hand * 0.5;

            m_hand = Vtx2::from_rot(
                (m as f64) * -1.0 * (2.0 * std::f64::consts::PI / 60.0)
                    + (std::f64::consts::PI / 2.0),
            );
            m_hand = m_hand * 0.65;

            s_hand = Vtx2::from_rot(
                (s as f64) * -1.0 * (2.0 * std::f64::consts::PI / 60.0)
                    + (std::f64::consts::PI / 2.0),
            );
            s_hand = s_hand * 0.8;

            // let data = vec![(1.0, 1.0), (3.0, 5.0), (10.0, 10.0)];

            let c_data: (f64, f64) = c_hand.into();
            let h_data: (f64, f64) = h_hand.into();
            let m_data: (f64, f64) = m_hand.into();
            let s_data: (f64, f64) = s_hand.into();
            let h: &[(f64, f64)] = &[h_data, c_data];
            let m: &[(f64, f64)] = &[m_data, c_data];
            let s: &[(f64, f64)] = &[s_data, c_data];

            let marks_set = Dataset::default()
                .graph_type(GraphType::Scatter)
                .marker(Marker::Dot)
                .style(Style::default().fg(Color::White))
                // .name("testdata")
                .data(marks.as_ref());
            let hour_set = Dataset::default()
                .graph_type(GraphType::Line)
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Red))
                // .name("testdata")
                .data(h);
            let minute_set = Dataset::default()
                .graph_type(GraphType::Line)
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Green))
                // .name("testdata")
                .data(m);
            let second_set = Dataset::default()
                .graph_type(GraphType::Line)
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Blue))
                // .name("testdata")
                .data(s);

            // ========== Drawing ==========

            let width = if term_width < center_width {
                term_width
            } else {
                center_width
            };

            let height = if term_height < center_height {
                term_height
            } else {
                center_height
            };

            let top_btm = (term_height - height) / 2;
            let left_right = (term_width - width) / 2;
            let lay_vert_constr = vec![
                Constraint::Length(top_btm),
                Constraint::Length(height),
                Constraint::Length(top_btm),
            ];

            let lay_hori_constr = vec![
                Constraint::Length(left_right),
                Constraint::Length(width),
                Constraint::Length(left_right),
            ];

            terminal.draw(|f| {
                let size = f.size();
                let lay_vert = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(lay_vert_constr.clone())
                    .split(size);
                let lay_hori = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(lay_hori_constr.clone())
                    .split(lay_vert[1]);

                // let timeval = format!(
                //     "time is {} => fps is {}",
                //     timer.elapsed().as_micros(),
                //     1.0 / (timer.elapsed().as_micros() as f64 ) * 1000000.0
                // );
                timer = Instant::now();
                let block = Block::default().title("Tock").borders(Borders::ALL);

                let chartdata = vec![marks_set, hour_set, minute_set, second_set];

                let chart = Chart::new(chartdata)
                    .block(Block::default() /* .title("chart") */)
                    .x_axis(
                        Axis::default()
                            // .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                            // .style(Style::default().fg(Color::White))
                            .bounds([-1.0, 1.0]),
                        // .labels(
                        //     ["0.0", "5.0", "10.0"]
                        //         .iter()
                        //         .cloned()
                        //         .map(Span::from)
                        //         .collect(),
                        // ),
                    )
                    .y_axis(
                        Axis::default()
                            // .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                            // .style(Style::default().fg(Color::White))
                            .bounds([-1.0, 1.0]),
                        // .labels(
                        //     ["0.0", "5.0", "10.0"]
                        //         .iter()
                        //         .cloned()
                        //         .map(Span::from)
                        //         .collect(),
                        // ),
                    );

                // f.render_widget(block, lay_hori[1]);
                f.render_widget(block, lay_hori[1]);
                f.render_widget(chart, lay_hori[1]);
            })?;
        }
        thread::sleep(Duration::from_millis(20));
    }

    // ========== End of Main loop ==========
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
