use icalendar::{Calendar, Component, Event, EventLike};

mod sale;
pub use sale::Sale;

mod sales;

pub fn run() {
    let sales = sales::get_sales();
    let ical = make_calendar(&sales);
    std::fs::create_dir("out").unwrap();
    std::fs::write("out/steam_sales.ics", ical).unwrap();
}

pub fn make_calendar(sales: &[Sale]) -> String {
    let mut calendar = Calendar::new().name("Steam Sales").done();

    sales
        .iter()
        .map(|sale| {
            Event::new()
                .summary(sale.name)
                .starts(sale.start)
                .ends(sale.end)
                .done()
        })
        .for_each(|event| {
            calendar.push(event);
        });

    calendar.to_string()
}
