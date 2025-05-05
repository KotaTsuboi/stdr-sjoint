use crate::input::HJoint;
use crate::output_util::*;
use anyhow::Result;
use dxf::{tables::Layer, Color, Drawing};

fn set_layer(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let base_layer = Layer {
        name: input.layer_name.base.clone(),
        color: Color::from_index(4),
        ..Default::default()
    };

    drawing.add_layer(base_layer);

    let bolt_layer = Layer {
        name: input.layer_name.bolt.clone(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(bolt_layer);

    let plate_layer = Layer {
        name: input.layer_name.plate.clone(),
        color: Color::from_index(1),
        ..Default::default()
    };

    drawing.add_layer(plate_layer);

    Ok(())
}

fn write_base(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let sec = input.section;
    let gap = 10.0;
    let margin = 1000.0;

    let coords = [
        ((-gap / 2.0, 0.0), (-gap / 2.0, sec.h)),
        ((gap / 2.0, 0.0), (gap / 2.0, sec.h)),
        ((-margin, 0.0), (-gap / 2.0, 0.0)),
        ((gap / 2.0, 0.0), (margin, 0.0)),
        ((-margin, sec.tf), (-gap / 2.0, sec.tf)),
        ((gap / 2.0, sec.tf), (margin, sec.tf)),
        ((-margin, sec.h - sec.tf), (-gap / 2.0, sec.h - sec.tf)),
        ((gap / 2.0, sec.h - sec.tf), (margin, sec.h - sec.tf)),
        ((-margin, sec.h), (-gap / 2.0, sec.h)),
        ((gap / 2.0, sec.h), (margin, sec.h)),
    ];

    for (p1, p2) in coords {
        write_line(drawing, p1.0, p1.1, p2.0, p2.1, &input.layer_name.base)?;
    }

    Ok(())
}

fn write_outer_plate(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let h = input.section.h;
    let t = input.flange.outer_plate.t;
    let l = input.flange.outer_plate.l;
    let layer = input.layer_name.plate.clone();

    write_line(drawing, -l / 2.0, h + t, l / 2.0, h + t, &layer)?;
    write_line(drawing, -l / 2.0, h, l / 2.0, h, &layer)?;
    write_line(drawing, -l / 2.0, h, -l / 2.0, h + t, &layer)?;
    write_line(drawing, l / 2.0, h, l / 2.0, h + t, &layer)?;

    write_line(drawing, -l / 2.0, -t, l / 2.0, -t, &layer)?;
    write_line(drawing, -l / 2.0, 0.0, l / 2.0, 0.0, &layer)?;
    write_line(drawing, -l / 2.0, 0.0, -l / 2.0, -t, &layer)?;
    write_line(drawing, l / 2.0, 0.0, l / 2.0, -t, &layer)?;

    Ok(())
}

fn write_inner_plate(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let h = input.section.h;
    let tf = input.section.tf;
    let t = input.flange.inner_plate.t;
    let l = input.flange.outer_plate.l;
    let layer = input.layer_name.plate.clone();

    write_line(drawing, -l / 2.0, h - tf - t, l / 2.0, h - tf - t, &layer)?;
    write_line(drawing, -l / 2.0, h - tf, l / 2.0, h - tf, &layer)?;
    write_line(drawing, -l / 2.0, h - tf, -l / 2.0, h - tf - t, &layer)?;
    write_line(drawing, l / 2.0, h - tf, l / 2.0, h - tf - t, &layer)?;

    write_line(drawing, -l / 2.0, tf + t, l / 2.0, tf + t, &layer)?;
    write_line(drawing, -l / 2.0, tf, l / 2.0, tf, &layer)?;
    write_line(drawing, -l / 2.0, tf, -l / 2.0, tf + t, &layer)?;
    write_line(drawing, l / 2.0, tf, l / 2.0, tf + t, &layer)?;

    Ok(())
}

fn write_flange_bolt(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let is_staggered = input.flange.bolt.is_staggered;
    let pc = if is_staggered { 45.0 } else { 60.0 };
    let e = 40.0;
    let gap = 10.0;
    let x0 = gap / 2.0 + e;
    let nf = input.flange.bolt.nf;
    let tf = input.section.tf;
    let to = input.flange.outer_plate.t;
    let ti = input.flange.inner_plate.t;
    let margin = 20.0;
    let layer = input.layer_name.bolt.clone();
    let h = input.section.h;

    for i in 0..nf {
        let x = x0 + i as f64 * pc;
        write_line(drawing, x, -to - margin, x, tf + ti + margin, &layer)?;
        write_line(drawing, x, h + to + margin, x, h - tf - ti - margin, &layer)?;
        write_line(drawing, -x, -to - margin, -x, tf + ti + margin, &layer)?;
        write_line(
            drawing,
            -x,
            h + to + margin,
            -x,
            h - tf - ti - margin,
            &layer,
        )?;
    }

    Ok(())
}

fn write_web_plate(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let h = input.section.h;
    let l = input.web.plate.l;
    let b = input.web.plate.b;
    let layer = input.layer_name.plate.clone();

    write_line(
        drawing,
        -l / 2.0,
        h / 2.0 - b / 2.0,
        l / 2.0,
        h / 2.0 - b / 2.0,
        &layer,
    )?;
    write_line(
        drawing,
        -l / 2.0,
        h / 2.0 + b / 2.0,
        l / 2.0,
        h / 2.0 + b / 2.0,
        &layer,
    )?;
    write_line(
        drawing,
        -l / 2.0,
        h / 2.0 - b / 2.0,
        -l / 2.0,
        h / 2.0 + b / 2.0,
        &layer,
    )?;
    write_line(
        drawing,
        l / 2.0,
        h / 2.0 - b / 2.0,
        l / 2.0,
        h / 2.0 + b / 2.0,
        &layer,
    )?;

    Ok(())
}

fn write_web_bolt(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let gap = 10.0;
    let h = input.section.h;
    let mw = input.web.bolt.mw;
    let nw = input.web.bolt.nw;
    let pc = input.web.bolt.pc;
    let e = 40.0;
    let r = input.bolt.diameter as f64 / 2.0 + 1.0;
    let g = 60.0;

    let c = pc * (mw - 1) as f64;
    let mut x = gap / 2.0 + e;
    for _i in 0..nw {
        let mut y = h / 2.0 - c / 2.0;
        for _j in 0..mw {
            let layer = input.layer_name.plate.clone();
            write_circle(drawing, x, y, r, &layer)?;
            write_circle(drawing, -x, y, r, &layer)?;

            let layer = input.layer_name.bolt.clone();
            write_cross(drawing, x, y, 20.0, &layer)?;
            write_cross(drawing, -x, y, 20.0, &layer)?;
            y += pc;
        }
        x += g;
    }
    Ok(())
}

pub fn write_dim(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let h = input.section.h;
    let gap = 10.0;
    let e = 40.0;
    let nw = input.web.bolt.nw;
    let pc = input.web.bolt.pc;
    let layer = "0".to_string();
    let text_height = 100.0;
    let text_rotation_angle = 0.0;
    let distance = -100.0;
    let y0 = 0.0;

    let x1 = -gap / 2.0 - e;
    let x2 = gap / 2.0 + e;

    write_dimension(
        drawing,
        (x1, y0),
        (x2, y0),
        text_height,
        text_rotation_angle,
        distance,
        layer.clone(),
    )?;

    let y0 = -2.0 * text_height;

    let x1 = -gap / 2.0 - 2.0 * e - (nw - 1) as f64 * pc;
    let x2 = -x1;

    write_dimension(
        drawing,
        (x1, y0),
        (x2, y0),
        text_height,
        text_rotation_angle,
        distance,
        layer.clone(),
    )?;

    Ok(())
}

pub fn write_x_view(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    set_layer(drawing, input)?;

    write_base(drawing, input)?;

    write_outer_plate(drawing, input)?;

    write_inner_plate(drawing, input)?;

    write_flange_bolt(drawing, input)?;

    write_web_plate(drawing, input)?;

    write_web_bolt(drawing, input)?;

    write_dim(drawing, input)?;

    Ok(())
}
