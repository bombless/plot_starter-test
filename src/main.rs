use plot_starter::{Plotter, Chart, Color, arange};

fn scale_by_factor(factor: f64, iter: impl Iterator<Item=isize>) -> impl Iterator<Item=f64> {
    iter.map(move |x| x as f64 * factor)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plotter = Plotter::new();

    Chart::on(&plotter)
        .time_series(0.1, -10.0 .. 10.0, f64::sin)
        .color(Color::BLUE);


    Chart::on(&plotter)
        .data(scale_by_factor(0.02, -500 .. 500).map(|x| (x, 3.0 + x.sin())))
        .color(Color::RED);


    Chart::on(&plotter)
        .data(arange(-10.0 .. 10.0, 0.1).map(|x| (x, 6.0 + x.sin())))
        .color(Color::ORANGE);


    plotter.present()
}
