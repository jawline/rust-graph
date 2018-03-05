extern crate gnuplot;

use gnuplot::{Figure, Color, AxesCommon};

pub type Point = (f64, f64);
pub type Points = Vec<Point>;
pub type Line = (Point, Point);

type GraphEntry<T> = (T, String);

pub struct Graph {
	title: String,
	points: Vec<GraphEntry<Points>>,
	lines: Vec<GraphEntry<Line>>,
	range: Option<(Point, Point)>
}

impl Graph {

	pub fn new(title: &str) -> Graph {
		Graph {
			title: title.to_string(),
			points: Vec::new(),
			lines: Vec::new(),
			range: None
		}
	}

	pub fn points(self: &mut Graph, points: &[Point], color: &str) -> &mut Graph {
		self.points.push((points.to_vec(), color.to_string()));
		self
	}

	pub fn line(self: &mut Graph, line: Line, color: &str) -> &mut Graph {
		self.lines.push((line, color.to_string()));
		self
	}

	pub fn range(self: &mut Graph, range: (Point, Point)) -> &mut Graph {
		self.range = Some(range);
		self
	}

	fn extract(points: &[Point]) -> (Vec<f64>, Vec<f64>) {
		let mut xl = Vec::new();
		let mut yl = Vec::new();
		
		points.iter().for_each(|&(x, y)| {
			xl.push(x);
			yl.push(y);
		});

		(xl, yl)
	}

	pub fn show(self: &mut Graph) {
		let mut fg = Figure::new();
		
		//Block scope to construct axis
		{
			let axis = fg.axes2d().set_title(&self.title, &[]);

			if self.range.is_some() {
				let ((minx, miny), (maxx, maxy)) = self.range.unwrap();
				axis.set_x_range(gnuplot::Fix(minx), gnuplot::Fix(maxx));
				axis.set_y_range(gnuplot::Fix(miny), gnuplot::Fix(maxy));
			}

			self.lines.iter().for_each(|&(ref line, ref color)| {
				let &((ax, ay), (bx, by)) = line;
				axis.lines(&[ax, bx], &[ay, by], &[Color(color)]);
			});

			self.points.iter().for_each(|&(ref points, ref color)| {
				let (lx, ly) = Graph::extract(&points);
				axis.points(&lx, &ly, &[Color(&color)]);
			});
		}

		fg.show();
	}
}

fn main() {
	Graph::new("Hello")
		.range(((-200.0, -200.0), (200.0, 200.0)))
		.line(((-50.0, -50.0), (50.0, 50.0)), "black")
		.points(&[(0.0, 1.0), (1.0, 1.4), (90.4, 4.3)], "red")
		.points(&[(50.0, 50.0), (0.0, 100.0)], "blue")
		.show()
}
