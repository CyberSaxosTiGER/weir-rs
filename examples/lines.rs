use std::f32;
use std::time::Instant;

use image::Rgba;

use weir_rs::geometry::point::Point4;

use weir_rs;
use weir_rs::draw::sandpaint;
use weir_rs::helpers::math::linspace;
use weir_rs::traits::random::{RandomInCircle, RandomOnLine};

fn main() {
    let now = Instant::now();
    let end: std::time::Duration;
    {
        let size = 1000;
        let repeat = 6;
        let grains = 3;

        let line_start = 100;
        let line_end = 900;

        let mut sand =
            sandpaint::Sandpaint::new(size, Rgba([255, 255, 255, 10]), Rgba([0, 0, 0, 255]));

        (line_start..line_end)
            .step_by(line_end / repeat)
            .enumerate()
            .for_each(|(j, i)| {
                let mut wer = weir_rs::grph::Grph::new();

                let mut va = Point4::ZERO.clone();
                let mut vb = Point4::ZERO.clone();

                let p1 = Point4::new(100.0, i as f32, 0.0, 0.0);
                let p2 = Point4::new(900.0, i as f32, 0.0, 0.0);

                linspace(0.0, 1.0, size).for_each(|s| {
                    let p_v1 = p1.random_on_line(&p2, s);
                    let p_v2 = va + p_v1;

                    let ind1 = wer.add_vertex(&p_v1);
                    let ind2 = wer.add_vertex(&p_v2);

                    va = va.random_in_circle(0.7 * j as f32, 1.0);
                    vb = vb.random_in_circle(0.001 * j as f32, 1.0);

                    (0..wer.graph.node_count()).for_each(|v| {
                        wer.move_vertex_by(v, &vb.random_in_circle(0.1, 1.0));
                        wer.add_edge(ind1, ind2);
                    });
                    wer.graph.all_edges().for_each(|(src, dst, _)| {
                        sand.stroke(&wer.get_vertex(src), &wer.get_vertex(dst), grains)
                    });
                    sand.points(&wer)
                });
            });
        end = now.elapsed();
        println!("Elapsed: {:?}", end);
        sand.vals.save("./examples/results/lines.png").unwrap();
    }
}
