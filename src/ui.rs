use std::vec;

use ratatui::{layout::{self, Constraint, Direction, Layout, Size}, style::{Color, Modifier, Style, Stylize}, symbols, text::{Text}, widgets::{Axis, Block, Borders, Chart, Dataset, GraphType::Line, List, ListItem, ListState, Paragraph}, Frame};
use crate::{app::App, connection::{get_temperature, get_forecast}};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    
    // TODO: Split the layout
    // let [area1, area2, area3 ...] = 
    let [left, right] = *Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ])
    .split(frame.size())
    else {
        panic!("");
    };
    let [right1, right2] = *Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ])
    .split(right)
    else {
        panic!("");
    };
    // TODO: get the list of cities
    let mut cities: Vec<ListItem> = vec![];
    for (i, name) in app.cities.clone().iter().enumerate() {
        let mut city: ListItem = ListItem::new(Text::from(name.clone()));
        if i == app.index as usize{
            let style = Style::default().add_modifier(Modifier::BOLD);
            city = city.style(style);
        }
        cities.push(city);
    }
    let list_component = List::new(cities).block(Block::default().style(Style::new().bg(Color::Blue)).borders(Borders::ALL));
    let mut state = ListState::default();
    state.select(Some(app.index as usize));
    frame.render_stateful_widget(list_component, left, &mut state);

    // TODO: render the list of cities
    // frame.render_widget(list_component, left);


    // TODO: Create the weather info component
    // TODO: Render the weather info component
    let mut tuples_list: Vec<(f64, f64)> = vec![];
    let mut i:f64 = 0.0;
    for forecast in app.current_city_forecast.as_ref().unwrap() {
        tuples_list.push((i, forecast.conditions.temp.clone() as f64));
        i = i + 1.0;
    }
    let paragraph_widget = Paragraph::new(std::format!("Temperatura: {}",app.current_city.as_ref().unwrap().conditions.temp.to_string()))
    .block(Block::default().style(Style::new().bg(Color::Blue)).borders(Borders::ALL));
    frame.render_widget(paragraph_widget, right1);

    // Aici trebuia sa fie un chart pt forecast, imaginati-va voi ca e aici. Edit: A MERS

    let x_max = tuples_list.iter().map(|&(hour, _)| hour).fold(f64::MIN, f64::max);
    let y_max = tuples_list.iter().map(|&(_, temp)| temp).fold(f64::MIN, f64::max);

    let x_bounds = [0.0, x_max + 1.0]; // Add a buffer to the max hour
    let y_bounds = [0.0, y_max + 5.0]; // Add a buffer to the max temperature

    let dataset = vec![
        Dataset::default()
            .style(Style::default().fg(Color::Cyan))
            .graph_type(Line)
            .data(&tuples_list)
    ];

    let x_axis = Axis::default()
        .title("Ora".blue())
        .style(Style::default().white())
        .bounds(x_bounds);

    let y_axis = Axis::default()
        .title("Temperatura".blue())
        .style(Style::default().white())
        .labels(vec!["0".into(), "5".into(), "10".into(), "15".into(), "20".into(), "25".into()])
        .bounds(y_bounds);

    let chart_widget = Chart::new(dataset)
        .block(
            Block::default()
                .title("Chart".cyan().bold())
                .borders(Borders::ALL)
                .style(Style::new().bg(Color::Blue))
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

    frame.render_widget(chart_widget, right2);
    

}   
