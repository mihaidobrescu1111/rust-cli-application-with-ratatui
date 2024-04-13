use std::vec;

use ratatui::{layout::{self, Constraint, Direction, Layout, Size}, style::{Color, Modifier, Style, Stylize}, symbols, text::Text, widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState, Paragraph}, Frame};
use tui_scrollview::ScrollView;
use crate::{app::App, connection::get_temperature};

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
    let paragraph_widget = Paragraph::new(std::format!("Temperatura: {}",app.current_city.as_ref().unwrap().conditions.temp.to_string()))
    .block(Block::default().style(Style::new().bg(Color::Blue)).borders(Borders::ALL));
    frame.render_widget(paragraph_widget, right1);

    // Aici trebuia sa fie un chart pt forecast, imaginati-va voi ca e aici
//     let mut temperatures:Vec<f32> = vec![];

//     for forecast in app.current_city_forecast.as_ref().unwrap() {
//         temperatures.push(forecast.conditions.temp.clone());
//     }

//     let mut hours: Vec<String> = app.current_city_forecast.as_ref().unwrap().into_iter().map(|x| x.current_time.as_ref().unwrap().clone()).collect();

//     let datasets = vec![
//     Dataset::default()
//         .name("data1")
//         .marker(symbols::Marker::Dot)
//         .graph_type(GraphType::Scatter)
//         .style(Style::default().cyan())
//         .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
// ];

//     let x_axis = Axis::default()
//         .title("X Axis".red())
//         .style(Style::default().white())
//         .bounds([0.0, 10.0])
//         .labels(vec!["0.0".into(), "5.0".into(), "10.0".into()]);


//     let y_axis = Axis::default()
//         .title("Y Axis".red())
//         .style(Style::default().white())
//         .bounds([0.0, 10.0])
//         .labels(vec!["0.0".into(), "5.0".into(), "10.0".into()]);

//     let chart_widget = Chart::new(datasets)
//         .block(Block::default().title("Chart"))
//         .x_axis(x_axis)
//         .y_axis(y_axis);
    

}
