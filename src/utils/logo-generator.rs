use std::io::Write;
use std::fs::File;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
struct Point {
	x: f64,
	y: f64
}

impl Point {
	fn new(x: f64, y: f64) -> Point {
		Point { x, y }
	}

	fn move_by(&mut self, p: &Point) {
		self.x += p.x;
		self.y += p.y;
	}
}

#[derive(Debug, Clone)]
struct Polygon {
	points: Vec<Point>
}

impl Polygon {
	fn new(sides: usize, radius: f64, offset: &Point) -> Polygon {
		let mut output = Vec::with_capacity(sides);
		let delta_angle: f64 = PI * 2.0 / (sides as f64);
		let mut angle: f64 = 0.0;
		for _ in 0..sides {
			output.push(Point::new(
				radius * angle.cos() + offset.x,
				radius * angle.sin() + offset.y
			));
			angle += delta_angle;
		}
		Polygon { points: output }
	}

	fn move_by(&mut self, m: &Point) {
		for p in self.points.iter_mut() {
			p.move_by(m);
		}
	}

	fn points(&self) -> &Vec<Point> {
		&self.points
	}
}

const IMG_HEIGHT: f64 = 400.0;
const RADIUS: f64 = 40.0;
const STROKE_WIDTH: f64 = 10.0;
const HORIZONTAL_SEPARATOR: f64 = 14.0;
const FILL_COLOR: &str = "#00000000";
const STROKE_COLOR: &str = "white";

fn main() {
	let argvs: Vec<String> = std::env::args().collect();
	let filename = match argvs.len() {
		2 => &argvs[1],
		_ => panic!("usage: {} [filename]", argvs[0]),
	};

	println!("Generating logo...");
	let t = Polygon::new(3, RADIUS + HORIZONTAL_SEPARATOR, &Point::new(0.0, 0.0));
	let mut shapes: Vec<Polygon> = t.points().iter().map(|p| Polygon::new(6, RADIUS, p)).collect();

	let offset = Point::new(
		-shapes[1].points[3].x + STROKE_WIDTH,
		shapes[1].points[1].y + STROKE_WIDTH
	);
	shapes.iter_mut().for_each(|s| s.move_by(&offset));

	let svg_size = Point::new(
		shapes[0].points[0].x + STROKE_WIDTH,
		shapes[1].points[1].y + STROKE_WIDTH
	);

	println!("Writing svg to {}...", filename);
	let mut file = File::create(filename).unwrap();
	file.write(format!(
		"<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
		svg_size.x * IMG_HEIGHT / svg_size.y, IMG_HEIGHT,
		svg_size.x, svg_size.y
	).as_bytes()).unwrap();
	for shape in shapes {
		file.write(format!(
			"  <polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>\n",
			shape.points().iter().map(|p| format!("{},{}", p.x, p.y)).collect::<Vec<String>>().join(" "),
			FILL_COLOR, STROKE_COLOR,
			STROKE_WIDTH
		).as_bytes()).unwrap();
	}
	file.write(b"</svg>\n").unwrap();
	println!("Done!");
}
