use icalendar::{Calendar, Component, Event, EventLike};

mod sale;
pub use sale::Sale;

pub fn make_calendar(sales: &[Sale]) {
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

    std::fs::write("/tmp/steam_sales.ics", calendar.to_string()).unwrap();
}
