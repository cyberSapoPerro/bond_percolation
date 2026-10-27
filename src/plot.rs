use plotters::prelude::*;
use crate::RandomLatice;

/// Renders a percolation lattice as an SVG image and saves it to the specified path.
///
/// Open edges are drawn and colored according to their cluster labels, while
/// closed edges are omitted. The resulting image is a square with a fixed cell
/// size and margin.
///
/// # Arguments
///
/// * `lattice` - The lattice to visualize.
/// * `output_path` - The file path where the SVG image will be saved.
///
/// # Errors
///
/// Returns an error if drawing the image or saving the SVG file fails.
pub fn graph_lattice(lattice: &RandomLatice, output_path: &str) 
    -> Result<(), Box<dyn std::error::Error>>  {
    let length = lattice.length();
    let cell: usize = 20;
    let margin: usize = 30;
    let size = 2 * margin + (length - 1)* cell;
    let size = size as u32;
    let root = SVGBackend::new(output_path, (size, size))
        .into_drawing_area();
    root.fill(&WHITE)?;

    // Horizontal edges
    for y in 0..length {
        for x in 0..length - 1 {
            if lattice.horizontal_edges()[y][x] == -1 {
                continue;
            }
            let x1 = (margin + x * cell) as i32;
            let y1 = (margin + y * cell) as i32;
            let x2 = x1 + cell as i32;
            let color = Palette99::pick(lattice.horizontal_edges()[y][x] as usize).stroke_width(2);
            root.draw(&PathElement::new(
                    vec![(x1, y1), (x2, y1)],
                    color
            ))?;
        }
    }

    // Vertical edges
    for y in 0..length - 1 {
        for x in 0..length {
            if lattice.vertical_edges()[y][x] == -1 {
                continue;
            }
            let x1 = (margin + x * cell) as i32;
            let y1 = (margin + y * cell) as i32;
            let y2 = y1 + cell as i32;
            let color = Palette99::pick(lattice.vertical_edges()[y][x] as usize).stroke_width(2);
            root.draw(&PathElement::new(
                    vec![(x1, y1), (x1, y2)],
                    color
            ))?;
        }
    }

    root.present()?;

    Ok(())
}
